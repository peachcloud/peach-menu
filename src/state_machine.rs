extern crate crossbeam_channel;
extern crate ws;

use std::{process, thread};

use chrono::{DateTime, Local};
use crossbeam_channel::*;

use crate::error::MenuError;
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
    Home,
    HomeNet,   // Home with Networking selected
    HomeStats, // Home with System Stats selected
    HomeShut,  // Home with Shutdown selected
    //Welcome,
    //Help,
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
        let mut state = State::Home;
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
            match state.run() {
                Ok(_) => (),
                Err(e) => warn!("State machine error: {:?}", e),
            };
        }
    });
}

// state machine functionality
impl State {
    /// Determines the next state based on current state and event.
    pub fn next(self, event: Event) -> State {
        match (self, event) {
            (State::Home, Event::Down) => State::HomeStats,
            (State::Home, Event::Up) => State::HomeShut,
            (State::Home, Event::A) => State::Networking,
            (State::HomeNet, Event::Down) => State::HomeStats,
            (State::HomeNet, Event::Up) => State::HomeShut,
            (State::HomeNet, Event::A) => State::Networking,
            (State::HomeStats, Event::Down) => State::HomeShut,
            (State::HomeStats, Event::Up) => State::HomeNet,
            (State::HomeShut, Event::Down) => State::HomeNet,
            (State::HomeShut, Event::Up) => State::HomeStats,
            (State::Networking, Event::B) => State::Home,
            // return current state if combination is unmatched
            (s, _) => s,
        }
    }

    /// Executes state-specific logic for current state.
    pub fn run(&self) -> Result<(), MenuError> {
        match *self {
            State::Home => {
                info!("State changed to: Home.");
                let dt: DateTime<Local> = Local::now();
                let t = format!("{}", dt.time().format("%H:%M"));
                oled_clear()?;
                oled_write(96, 0, t, "6x8".to_string())?;
                oled_write(0, 0, "PeachCloud".to_string(), "6x8".to_string())?;
                oled_write(0, 18, "> Networking".to_string(), "6x8".to_string())?;
                oled_write(12, 27, "System Stats".to_string(), "6x8".to_string())?;
                oled_write(12, 36, "Shutdown".to_string(), "6x8".to_string())?;
                oled_write(100, 54, "v0.1".to_string(), "6x8".to_string())?;
                oled_flush()?;
            }
            State::HomeNet => {
                info!("State changed to: Home.");
                oled_write(0, 18, "> ".to_string(), "6x8".to_string())?;
                oled_write(0, 27, "  ".to_string(), "6x8".to_string())?;
                oled_write(0, 36, "  ".to_string(), "6x8".to_string())?;
                oled_flush()?;
            }
            State::HomeStats => {
                info!("State changed to: Home.");
                oled_write(0, 18, "  ".to_string(), "6x8".to_string())?;
                oled_write(0, 27, "> ".to_string(), "6x8".to_string())?;
                oled_write(0, 36, "  ".to_string(), "6x8".to_string())?;
                oled_flush()?;
            }
            State::HomeShut => {
                info!("State changed to: Home.");
                oled_write(0, 18, "  ".to_string(), "6x8".to_string())?;
                oled_write(0, 27, "  ".to_string(), "6x8".to_string())?;
                oled_write(0, 36, "> ".to_string(), "6x8".to_string())?;
                oled_flush()?;
            }
            /*State::Welcome => {
                oled_clear()?;
                info!("State changed to: Welcome.");
                oled_flush()?;
            }
            State::Help => {
                oled_clear()?;
                info!("State changed to: Help.");
                // show buttons
                // [ A ] - Select
                // [ B ] - Back or Help
                // arrows - Navigation
                oled_flush()?;
            }*/
            State::Networking => {
                info!("State changed to: Networking.");
                let mode = "MODE Client".to_string();
                let status = "STATUS Active".to_string();
                let ip = match network_get_ip("wlan1".to_string()) {
                    Ok(ip) => ip,
                    Err(_) => "x.x.x.x".to_string(),
                };
                let show_ip = format!("IP {}", ip);
                let ssid = match network_get_ssid("wlan1".to_string()) {
                    Ok(ssid) => ssid,
                    Err(_) => "Not connected".to_string(),
                };
                let show_ssid = format!("NETWORK {}", ssid);
                let rssi = match network_get_rssi("wlan1".to_string()) {
                    Ok(rssi) => rssi,
                    Err(_) => "Not connected".to_string(),
                };
                let show_rssi = format!("SIGNAL {}dBm", rssi);
                let config = "> Configuration".to_string();

                oled_clear()?;
                oled_write(0, 0, mode, "6x8".to_string())?;
                oled_write(0, 9, status, "6x8".to_string())?;
                oled_write(0, 18, show_ssid, "6x8".to_string())?;
                oled_write(0, 27, show_ip, "6x8".to_string())?;
                oled_write(0, 36, show_rssi, "6x8".to_string())?;
                oled_write(0, 54, config, "6x8".to_string())?;
                oled_flush()?;
            }
        }
        Ok(())
    }
}
