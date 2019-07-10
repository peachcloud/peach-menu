extern crate crossbeam_channel;
extern crate ws;

use std::{process, thread};

use crossbeam_channel::*;

use crate::network::*;
use crate::oled::*;

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

#[derive(Debug, PartialEq)]
/// The states of the state machine.
pub enum State {
    Welcome,
    Help,
    Clock,
    Networking,
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
            state.run();
        }
    });
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
                oled_write(x_coord, y_coord, string, font_size).unwrap_or_else(|_err| {
                    error!("Problem executing OLED client call.");
                });
                oled_flush().unwrap();
            }
            State::Networking => {
                info!("State changed to: Networking.");
                // add rpc to peach_network: get_mode(iface)
                //  use wpactrl STATUS (mode=station)
                let mode = "MODE: Client".to_string();
                // add rpc to peach_network: get_status(iface)
                let status = "STATUS: Up".to_string();
                let ip = match network_get_ip("wlan1".to_string()) {
                    Ok(ip) => ip,
                    Err(_) => "x.x.x.x".to_string(),
                };
                let address = format!("IP: {}", ip);

                let ssid = match network_get_ssid("wlan1".to_string()) {
                    Ok(ssid) => ssid,
                    Err(_) => "Not connected".to_string(),
                };
                let network = format!("NETWORK: {}", ssid);

                let rssi = match network_get_rssi("wlan1".to_string()) {
                    Ok(rssi) => rssi,
                    Err(_) => "Not connected".to_string(),
                };
                let signal = format!("SIGNAL: {}dBm", rssi);

                oled_clear().unwrap();
                oled_write(0, 0, mode, "6x8".to_string()).unwrap_or_else(|_err| {
                    error!("Problem executing OLED client call.");
                });
                oled_write(0, 9, status, "6x8".to_string()).unwrap_or_else(|_err| {
                    error!("Problem executing OLED client call.");
                });
                oled_write(0, 18, network, "6x8".to_string()).unwrap_or_else(|_err| {
                    error!("Problem executing OLED client call.");
                });
                oled_write(0, 27, address, "6x8".to_string()).unwrap_or_else(|_| {
                    error!("Problem executing OLED client call.");
                });
                oled_write(0, 36, signal, "6x8".to_string()).unwrap_or_else(|_| {
                    error!("Problem executing OLED client call.");
                });
                oled_flush().unwrap();
            }
        }
    }
}
