//! # peach-menu
//!
//! `peach_menu` is a collection of utilities and data structures for running
//! a menu state machine. I/O takes place using JSON-RPC 2.0 over websockets, 
//! with `peach-buttons` providing GPIO input data and `peach-oled` receiving 
//! output data for display.

#[macro_use]
pub extern crate log;
extern crate crossbeam_channel;
#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;
extern crate ws;
extern crate regex;

use std::thread;

use crossbeam_channel::unbounded;
use crossbeam_channel::*;

use jsonrpc_client_http::HttpTransport;
use jsonrpc_http_server::jsonrpc_core::*;

use ws::{connect, Handler, Sender, Result, Message, Handshake, CloseCode, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use regex::Regex;

/// Creates a JSON-RPC client with http transport and calls the `peach-oled`
/// `clear` and `write` methods.
///
/// # Arguments
///
/// * `x_coord` - A 32 byte signed int.
/// * `y_coord` - A 32 byte signed int.
/// * `string` - A String containing the message to be displayed.
/// * `font_size` - A String containing `6x8`, `6x12`, `8x16` or `12x16`
///
pub fn oled_client(x_coord: i32, y_coord: i32, string: String, font_size: String) {
    debug!("Creating HTTP transport for OLED client.");
    // create http transport for json-rpc comms
    let transport = HttpTransport::new().standalone().unwrap();
    debug!("Creating HTTP transport handle on 127.0.0.1:3031.");
    let transport_handle = transport.handle("http://127.0.0.1:3031").unwrap();
    info!("Creating client for peach_oled service.");
    let mut client = PeachOledClient::new(transport_handle);

    // clear oled display before writing new message
    client.clear().call().unwrap();
    debug!("Cleared the OLED display.");
    // send msg to oled for display
    client.write(x_coord, y_coord, string, font_size).call().unwrap();
    debug!("Wrote to the OLED display.");
}

/// Initializes the state machine, listens for button events and drives
/// corresponding state changes.
///
/// # Arguments
///
/// * `r` - An unbounded `crossbeam_channel::Receiver` for unsigned 8 byte int.
///
pub fn state_changer(r: Receiver<u8>) {
    thread::spawn(move || {
        // initialize the state machine as Welcome
        info!("Initializing the state machine.");
        let mut state = State::Welcome;
        loop {
            // listen for button_code from json-rpc server
            let button_code = r.recv().unwrap();
            // match on button_code & pass event to state.next
            let event = match button_code {
                // button code mappings
                0 => Event::Center,
                1 => Event::Left,
                2 => Event::Right,
                3 => Event::Up,
                4 => Event::Down,
                5 => Event::A,
                6 => Event::B,
                _ => Event::Unknown,
            };
            state = state.next(event);
            if let State::Failure(_string) = state {
                //break;
                error!("State machine entered a failure state.");
                panic!("Failed");
            } else {
                state.run();
            }
        }
    });
}


/// Configures channels for message passing, launches the state machine
/// changer thread and connects to the `peach-buttons` JSON-RPC pubsub
/// service over websockets.
///
/// A Receiver is passed into `state_changer` and the corresponding Sender
/// is passed into the websockets client. This allows the `button_code` to
/// be extracted from the received websocket message and passed to the
/// state machine.
///
pub fn run() -> Result<()> {
    info!("Starting up.");

    debug!("Creating unbounded channel for message passing.");
    let (s, r) = unbounded();
    // clone channel so receiver can be moved into `state_changer`
    let (mut s1, r1) = (s.clone(), r.clone());

    debug!("Spawning state-machine thread.");
    state_changer(r1);

    let s2 = &mut s1;
    
    connect("ws://127.0.0.1:3030", |out| Client { out: out, s: s2 } ).unwrap();
    
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct SubscribeMsg {
    pub id: u8,
    pub jsonrpc: String,
    pub method: String,
}

#[derive(Debug, Deserialize)]
pub struct Press {
    pub button_code: u8,
}

// jsonrpc peach-oled client
jsonrpc_client!(pub struct PeachOledClient {
    /// Creates a JSON-RPC request to write to the OLED display.
    pub fn write(&mut self, x_coord: i32, y_coord: i32, string: String, font_size: String) -> RpcRequest<String>;

    /// Creates a JSON-RPC request to clear the OLED display.
    pub fn clear(&mut self) -> RpcRequest<String>;
});

#[derive(Debug, PartialEq)]
/// The states of the state machine.
pub enum State {
    Welcome,
    Help,
    Clock,
    Networking,
    Failure(String),
}

#[derive(Debug, Clone, Copy)]
/// The button press events.
pub enum Event {
    Center,
    Left,
    Right,
    Down,
    Up,
    A,
    B,
    Unknown,
}

// state machine functionality
impl State {
    /// Determines the next state based on current state and event.
    pub fn next(self, event: Event) -> State {
        match (self, event) {
            // always set state to `Welcome` on center-joystick keypress
            (_, Event::Center) => State::Welcome,
            (State::Welcome, Event::Left) => State::Networking,
            (State::Welcome, Event::Right) => State::Help,
            (State::Help, Event::Left) => State::Welcome,
            (State::Help, Event::Right) => State::Clock,
            (State::Clock, Event::Left) => State::Help,
            (State::Clock, Event::Right) => State::Networking,
            (State::Networking, Event::Left) => State::Clock,
            (State::Networking, Event::Right) => State::Welcome,
            // return current state if combination is unmatched
            (s, _) => s,
        }
    }

    /// Executes state-specific logic for current state.
    pub fn run(&self) {
        match *self {
            State::Welcome => {
                info!("State changed to: Welcome.");
                let x_coord = 0;
                let y_coord = 0;
                let string = "Welcome to PeachCloud".to_string();
                let font_size = "6x8".to_string();
                // perform write() call to peach-oled
                oled_client(x_coord, y_coord, string, font_size);
            }
            State::Help => {
                info!("State changed to: Help.");
                let x_coord = 0;
                let y_coord = 0;
                let string = "Navigation".to_string();
                let font_size = "6x8".to_string();
                // perform write() call to peach-oled
                oled_client(x_coord, y_coord, string, font_size);
            }
            State::Clock => {
                info!("State changed to: Clock.");
                let x_coord = 0;
                let y_coord = 0;
                let string = "Clock".to_string();
                let font_size = "6x8".to_string();
                // perform write() call to peach-oled
                oled_client(x_coord, y_coord, string, font_size);
            }
            State::Networking => {
                info!("State changed to: Networking.");
                let x_coord = 0;
                let y_coord = 0;
                let string = "Networking".to_string();
                let font_size = "6x8".to_string();
                // perform write() call to peach-oled
                oled_client(x_coord, y_coord, string, font_size);
            }
            State::Failure(_) => {
                error!("State machine failed during run method.");
                panic!("State machine failed")
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ButtonMsg {
    jsonrpc: String,
    method: String,
    params: Vec<u8>,
}

// websocket client
#[derive(Debug)]
pub struct Client<'a> {
    out: Sender,
    s: &'a crossbeam_channel::Sender<u8>,
}

impl <'a> Handler for Client<'a> {
    /// Sends request to `peach_buttons` to subscribe to emitted events.
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        info!("Subscribing to peach_buttons microservice over ws.");
        let subscribe = json!({
            "id":1,
            "jsonrpc":"2.0",
            "method":"subscribe_buttons"
        });
        let data = subscribe.to_string();
        self.out.send(data)
    }
        
    /// Displays JSON-RPC request from `peach_buttons`.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        info!("Received ws message from peach_buttons.");
        // button_code must be extracted from the request and passed to
        // state_changer
        let m : String = msg.into_text().unwrap();
        // regex is used to distinguish button_press events from 
        // other received jsonrpc requests
        let re = Regex::new(r"params").unwrap();
        if re.is_match(&m) {
            // serialize msg string into a struct
            let bm : ButtonMsg = serde_json::from_str(&m).unwrap();
            debug!("Sending button code to state_changer.");
            // send the button_code parameter to state_changer
            self.s.send(bm.params[0]).unwrap();
        }
        Ok(())
    }

    /// Handles disconnection from websocket and displays debug data.
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => {
                info!("The client is done with the connection.");
            },
            CloseCode::Away => {
                info!("The client is leaving the site.");
            },
            CloseCode::Abnormal => {
                warn!("Closing handshake failed! Unable to obtain closing status from client.");
            },
            _ => error!("The client encountered an error: {}", reason),
        }
    }

    fn on_error(&mut self, err: Error) {
        error!("The server encountered an error: {:?}", err);
    }
}
