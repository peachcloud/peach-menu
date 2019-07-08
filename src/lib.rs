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
extern crate ws;

mod buttons;
mod error;
mod oled;

use std::{env, process, thread};

use crossbeam_channel::unbounded;
use crossbeam_channel::*;

#[allow(unused_imports)]
use jsonrpc_test as test;

use ws::connect;

use crate::buttons::*;
use crate::oled::*;

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
            let button_code = r.recv().unwrap_or_else(|err| {
                error!("Problem receiving button code from server: {}", err);
                process::exit(1);
            });
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
                error!("State machine entered a failure state.");
                process::exit(1);
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
pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    info!("Starting up.");

    debug!("Creating unbounded channel for message passing.");
    let (s, r) = unbounded();
    // clone channel so receiver can be moved into `state_changer`
    let (mut s1, r1) = (s.clone(), r.clone());

    debug!("Spawning state-machine thread.");
    state_changer(r1);

    let s2 = &mut s1;

    let ws_addr = env::var("PEACH_BUTTONS_SERVER").unwrap_or_else(|_| "127.0.0.1:5111".to_string());

    let ws_server = format!("ws://{}", ws_addr);

    connect(ws_server, |out| Client { out, s: s2 })?;

    Ok(())
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
                oled_clear().unwrap();
                info!("State changed to: Welcome.");
                let x_coord = 0;
                let y_coord = 0;
                let string = "Welcome to PeachCloud".to_string();
                let font_size = "6x8".to_string();
                // perform write() call to peach-oled
                oled_write(x_coord, y_coord, string, font_size)
                    // this needs to be handled better! impl Display !!!
                    .unwrap_or_else(|_err| {
                        error!("Problem executing OLED client call.");
                    });
                oled_flush().unwrap();
            }
            State::Help => {
                oled_clear().unwrap();
                info!("State changed to: Help.");
                let x_coord = 0;
                let y_coord = 0;
                let string = "Navigation".to_string();
                let font_size = "6x8".to_string();
                // perform write() call to peach-oled
                oled_write(x_coord, y_coord, string, font_size).unwrap_or_else(|_err| {
                    error!("Problem executing OLED client call.");
                });
                oled_flush().unwrap();
            }
            State::Clock => {
                oled_clear().unwrap();
                info!("State changed to: Clock.");
                let x_coord = 0;
                let y_coord = 0;
                let string = "Clock".to_string();
                let font_size = "6x8".to_string();
                // perform write() call to peach-oled
                oled_write(x_coord, y_coord, string, font_size).unwrap_or_else(|_err| {
                    error!("Problem executing OLED client call.");
                });
                oled_flush().unwrap();
            }
            State::Networking => {
                info!("State changed to: Networking.");
                oled_clear().unwrap();
                oled_write(0, 0, "Network mode: Client".to_string(), "6x8".to_string())
                    .unwrap_or_else(|_| {
                        error!("Problem executing OLED client call.");
                    });
                oled_write(0, 10, "Wireless AP: Home".to_string(), "6x8".to_string())
                    .unwrap_or_else(|_err| {
                        error!("Problem executing OLED client call.");
                    });
                oled_write(0, 20, "IP: 192.168.1.34".to_string(), "6x8".to_string())
                    .unwrap_or_else(|_err| {
                        error!("Problem executing OLED client call.");
                    });
                oled_flush().unwrap();
            }
            State::Failure(_) => {
                error!("State machine failed during run method.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test to ensure correct success response
    #[test]
    fn rpc_success() {
        let rpc = {
            let mut io = IoHandler::new();
            io.add_method("rpc_success_response", |_| {
                Ok(Value::String("success".into()))
            });
            test::Rpc::from(io)
        };

        assert_eq!(rpc.request("rpc_success_response", &()), r#""success""#);
    }
}
