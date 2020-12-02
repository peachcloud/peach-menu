use std::{process, thread};

use crossbeam_channel::*;
use log::{error, info, warn};

use crate::error::MenuError;
use crate::oled::*;
use crate::states::*;

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
    Home(u8),
    Logo,
    Network,
    NetworkConf(u8),
    NetworkMode(u8),
    OledPower(u8),
    Reboot,
    Shutdown,
    Stats,
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
        let mut state = State::Logo;
        match state.run() {
            Ok(_) => (),
            Err(e) => warn!("State machine error: {:?}", e),
        };

        loop {
            let button_code = r.recv().unwrap_or_else(|err| {
                error!("Problem receiving button code from server: {}", err);
                process::exit(1);
            });
            let event = match button_code {
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

// 0 - Home
//   1 - Networking
//   2 - System Stats
//   3 - Display Off
//   4 - Reboot
//   5 - Shutdown
// 0 - NetworkConf
//   1 - Client Mode
//   2 - Access Point
// NetworkMode
//   0 - Client Mode
//   1 - Access Point Mode
// OledPower
//   0 - Off
//   1 - On

impl State {
    /// Determines the next state based on current state and event.
    pub fn next(self, event: Event) -> State {
        match (self, event) {
            (State::Logo, Event::A) => State::Home(0),
            (State::Home(_), Event::B) => State::Logo,
            (State::Home(0), Event::Down) => State::Home(2),
            (State::Home(0), Event::Up) => State::Home(5),
            (State::Home(0), Event::A) => State::Network,
            (State::Home(1), Event::Down) => State::Home(2),
            (State::Home(1), Event::Up) => State::Home(5),
            (State::Home(1), Event::A) => State::Network,
            (State::Home(2), Event::Down) => State::Home(3),
            (State::Home(2), Event::Up) => State::Home(1),
            (State::Home(2), Event::A) => State::Stats,
            (State::Home(3), Event::Down) => State::Home(4),
            (State::Home(3), Event::Up) => State::Home(2),
            (State::Home(3), Event::A) => State::OledPower(0),
            (State::Home(4), Event::Down) => State::Home(5),
            (State::Home(4), Event::Up) => State::Home(3),
            (State::Home(4), Event::A) => State::Reboot,
            (State::Home(5), Event::Down) => State::Home(1),
            (State::Home(5), Event::Up) => State::Home(4),
            (State::Home(5), Event::A) => State::Shutdown,
            (State::Network, Event::A) => State::NetworkConf(0),
            (State::Network, Event::B) => State::Home(0),
            (State::NetworkConf(0), Event::A) => State::NetworkMode(0),
            (State::NetworkConf(0), Event::B) => State::Network,
            (State::NetworkConf(0), Event::Down) => State::NetworkConf(2),
            (State::NetworkConf(0), Event::Up) => State::NetworkConf(2),
            (State::NetworkConf(1), Event::A) => State::NetworkMode(0),
            (State::NetworkConf(1), Event::B) => State::Network,
            (State::NetworkConf(1), Event::Down) => State::NetworkConf(2),
            (State::NetworkConf(1), Event::Up) => State::NetworkConf(2),
            (State::NetworkConf(2), Event::A) => State::NetworkMode(1),
            (State::NetworkConf(2), Event::B) => State::Network,
            (State::NetworkConf(2), Event::Down) => State::NetworkConf(1),
            (State::NetworkConf(2), Event::Up) => State::NetworkConf(1),
            (State::NetworkMode(1), Event::B) => State::Network,
            (State::NetworkMode(1), Event::Down) => State::NetworkConf(1),
            (State::NetworkMode(1), Event::Up) => State::NetworkConf(1),
            (State::NetworkMode(0), Event::B) => State::Network,
            (State::NetworkMode(0), Event::Down) => State::NetworkConf(2),
            (State::NetworkMode(0), Event::Up) => State::NetworkConf(2),
            (State::OledPower(0), _) => State::OledPower(1),
            (State::OledPower(1), Event::Down) => State::Home(4),
            (State::OledPower(1), Event::Up) => State::Home(2),
            (State::OledPower(1), Event::A) => State::OledPower(0),
            (State::Stats, Event::B) => State::Home(0),
            // return current state if combination is unmatched
            (s, _) => s,
        }
    }

    /// Executes state-specific logic for current state.
    pub fn run(&self) -> Result<(), MenuError> {
        match *self {
            // home: root
            State::Home(0) => {
                info!("State changed to: Home 0.");
                state_home(0)?;
            }
            // home: networking
            State::Home(1) => {
                info!("State changed to: Home 1.");
                state_home(1)?;
            }
            // home: system stats
            State::Home(2) => {
                info!("State changed to: Home 2.");
                state_home(2)?;
            }
            // home: display off
            State::Home(3) => {
                info!("State changed to: Home 3.");
                state_home(3)?;
            }
            // home: reboot
            State::Home(4) => {
                info!("State changed to: Home 4.");
                state_home(4)?;
            }
            // home: shutdown
            State::Home(5) => {
                info!("State changed to: Home 5.");
                state_home(5)?;
            }
            // home: unknown
            State::Home(_) => {
                info!("State changed to: Home _.");
            }
            State::Logo => {
                info!("State changed to: Logo.");
                state_logo()?;
            }
            State::Network => {
                info!("State changed to: Network.");
                state_network()?;
            }
            State::NetworkConf(0) => {
                info!("State changed to: NetworkConf 0.");
                state_network_conf(0)?;
            }
            State::NetworkConf(1) => {
                info!("State changed to: NetworkConf 1.");
                state_network_conf(1)?;
            }
            State::NetworkConf(2) => {
                info!("State changed to: NetworkConf 2.");
                state_network_conf(2)?;
            }
            State::NetworkConf(_) => {
                info!("State changed to: NetworkConf _.");
            }
            State::NetworkMode(0) => {
                info!("State changed to: NetworkMode 0.");
                state_network_mode(0)?;
            }
            State::NetworkMode(1) => {
                info!("State changed to: NetworkMode 1.");
                state_network_mode(1)?;
            }
            State::NetworkMode(_) => {
                info!("State changed to: NetworkMode _.");
            }
            State::OledPower(0) => {
                info!("State changed to: OledPower 0.");
                oled_power(false)?;
            }
            State::OledPower(1) => {
                info!("State changed to: OledPower 1.");
                oled_power(true)?;
            }
            State::OledPower(_) => {
                info!("State changed to: OledPower _.");
            }
            State::Reboot => {
                info!("State changed to: Reboot.");
                state_reboot()?;
            }
            State::Shutdown => {
                info!("State changed to: Shutdown.");
                state_shutdown()?;
            }
            State::Stats => {
                info!("State changed to: Stats.");
                state_stats()?;
            }
        }
        Ok(())
    }
}
