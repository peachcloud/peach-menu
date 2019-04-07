//! # peach-menu
//!
//! `peach_menu` is a collection of utilities and data structures for running
//! a menu state machine. I/O takes place using JSON-RPC 2.0 over http
//! transports, with `peach-buttons` providing GPIO input data and `peach-oled`
//! receiving output data for display.

extern crate crossbeam_channel;
#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

use crossbeam_channel::unbounded;
use crossbeam_channel::Receiver;
use failure::Fail;
use jsonrpc_client_http::HttpTransport;
use jsonrpc_http_server::jsonrpc_core::types::error::Error;
use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::*;
use serde::Deserialize;
use std::thread;

/// Creates a JSON-RPC client with http transport and calls the `peach-oled`
/// `write` method.
///
/// # Arguments
///
/// * `x_coord` - A 32 byte signed int.
/// * `y_coord` - A 32 byte signed int.
/// * `string` - A String containing the message to be displayed.
pub fn oled_client(x_coord: i32, y_coord: i32, string: String) {
    // create http transport for json-rpc comms
    let transport = HttpTransport::new().standalone().unwrap();
    let transport_handle = transport.handle("http://127.0.0.1:3030").unwrap();
    let mut client = PeachOledClient::new(transport_handle);

    // send msg to oled for display
    client.write(x_coord, y_coord, string).call().unwrap();
}

/// Initializes the state machine, listens for button events and drives
/// corresponding state changes.
///
/// # Arguments
///
/// * `r` - An unbounded `crossbeam_channel::Receiver` for unsigned 8 byte int.
pub fn state_changer(r: Receiver<u8>) {
    thread::spawn(move || {
        // initialize the state machine as Welcome
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
                panic!("Failed");
            } else {
                state.run();
            }
        }
    });
}

/// Configures channels for message passing, launches the state machine
/// changer thread, creates JSON-RPC server method for button press events
/// and launches the JSON-RPC server.
pub fn run() -> Result<()> {
    // create an unbounded channel
    let (s, r) = unbounded();
    // clone channel so receiver can be moved into `state_changer`
    let (_s1, r1) = (s.clone(), r.clone());

    // spawn state-machine thread
    state_changer(r1);

    let mut io = IoHandler::default();
    // clone channel so sender can be moved into `press` method
    let (s2, _r2) = (s.clone(), r.clone());

    io.add_method("press", move |params: Params| {
        let p: Result<Press> = params.parse();
        match p {
            // if result contains `button_code`, unwrap
            Ok(_) => {
                let p: Press = p.unwrap();
                // send p.button_code to state_changer via channel sender
                s2.send(p.button_code).unwrap();
                Ok(Value::String("success".into()))
            }
            Err(e) => Err(Error::from(PressError::MissingParams { e })),
        }
    });

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(&"127.0.0.1:3031".parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Press {
    pub button_code: u8,
}

// jsonrpc client
jsonrpc_client!(pub struct PeachOledClient {
    /// Creates a JSON-RPC request to write to the OLED display.
    pub fn write(&mut self, x_coord: i32, y_coord: i32, string: String) -> RpcRequest<String>;
});

// error handling for jsonrpc methods
#[derive(Debug, Fail)]
pub enum PressError {
    /// The errors for the `peach_menu` JSON-RPC server.
    #[fail(display = "missing expected parameters")]
    MissingParams { e: Error },
}

impl From<PressError> for Error {
    fn from(err: PressError) -> Self {
        match &err {
            PressError::MissingParams { e } => Error {
                code: ErrorCode::ServerError(-32602),
                message: "invalid params".into(),
                data: Some(format!("{}", e.message).into()),
            },
            err => Error {
                code: ErrorCode::InternalError,
                message: "internal error".into(),
                data: Some(format!("{:?}", err).into()),
            },
        }
    }
}

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
                let x_coord = 0;
                let y_coord = 0;
                let string = "Welcome to PeachCloud".to_string();
                // perform write() call to peach-oled
                oled_client(x_coord, y_coord, string);
            }
            State::Help => println!("Navigation"),
            State::Clock => println!("Clock"),
            State::Networking => println!("Network Stats"),
            State::Failure(_) => panic!("State machine failed"),
        }
    }
}
