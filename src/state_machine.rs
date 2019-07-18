extern crate crossbeam_channel;
extern crate ws;

use std::{process, thread};

use chrono::{DateTime, Local};
use crossbeam_channel::*;

use crate::error::MenuError;
use crate::network::*;
use crate::oled::*;
use crate::stats::*;

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
    HomeNet,
    HomeStats,
    HomeShut,
    Logo,
    Networking,
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

impl State {
    /// Determines the next state based on current state and event.
    pub fn next(self, event: Event) -> State {
        match (self, event) {
            (State::Logo, Event::A) => State::Home,
            (State::Home, Event::Down) => State::HomeStats,
            (State::Home, Event::Up) => State::HomeShut,
            (State::Home, Event::A) => State::Networking,
            (State::Home, Event::B) => State::Logo,
            (State::HomeNet, Event::Down) => State::HomeStats,
            (State::HomeNet, Event::Up) => State::HomeShut,
            (State::HomeNet, Event::A) => State::Networking,
            (State::HomeStats, Event::Down) => State::HomeShut,
            (State::HomeStats, Event::Up) => State::HomeNet,
            (State::HomeStats, Event::A) => State::Stats,
            (State::Stats, Event::B) => State::Home,
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
            State::Logo => {
                info!("State changed to: Logo.");
                let bytes = PEACH_LOGO.to_vec();
                oled_clear()?;
                oled_draw(bytes, 64, 64, 32, 0)?;
                oled_flush()?;
            }
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
            State::Stats => {
                info!("State changed to: Stats.");
                let c = cpu_stats_percent()?;
                let c_stats = format!(
                    "CPU {} us {} sy {} id",
                    c.user.round(),
                    c.system.round(),
                    c.idle.round()
                );
                let m = mem_stats()?;
                let m_stats = format!(
                    "MEM {}MB f {}MB u",
                    (m.free / 1024).to_string(),
                    (m.used / 1024).to_string()
                );
                let l = load_average()?;
                let l_stats = format!("LOAD {} {} {}", l.one, l.five, l.fifteen);
                let u = uptime()?;
                let u_stats = format!("UPTIME {} hrs", u);

                oled_clear()?;
                oled_write(0, 0, c_stats, "6x8".to_string())?;
                oled_write(0, 9, m_stats, "6x8".to_string())?;
                oled_write(0, 18, l_stats, "6x8".to_string())?;
                oled_write(0, 27, u_stats, "6x8".to_string())?;
                oled_flush()?;
            }
        }
        Ok(())
    }
}

const PEACH_LOGO: [u8; 512] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 0, 0, 0, 0, 0,
    0, 3, 248, 14, 0, 0, 7, 0, 0, 15, 252, 63, 128, 0, 31, 192, 0, 63, 254, 127, 192, 0, 63, 224,
    0, 127, 255, 127, 224, 0, 127, 240, 0, 63, 255, 255, 128, 0, 255, 240, 0, 31, 255, 255, 192,
    31, 255, 248, 0, 15, 252, 64, 112, 63, 255, 248, 0, 24, 240, 96, 24, 127, 255, 255, 192, 48, 0,
    48, 12, 127, 255, 255, 224, 96, 0, 24, 12, 255, 255, 255, 240, 64, 0, 8, 6, 255, 255, 255, 248,
    64, 0, 12, 2, 255, 255, 255, 252, 192, 0, 4, 2, 255, 227, 255, 252, 192, 0, 4, 2, 127, 128,
    255, 252, 128, 0, 4, 2, 63, 0, 127, 252, 128, 0, 6, 2, 126, 0, 63, 252, 128, 0, 6, 3, 252, 0,
    63, 248, 128, 0, 6, 6, 0, 0, 1, 240, 192, 0, 6, 12, 0, 0, 0, 192, 192, 0, 6, 8, 0, 0, 0, 96,
    64, 0, 4, 24, 0, 0, 0, 32, 64, 0, 4, 24, 0, 0, 0, 48, 96, 0, 4, 16, 0, 0, 0, 16, 32, 0, 4, 16,
    0, 0, 0, 16, 48, 0, 12, 24, 0, 0, 0, 16, 24, 0, 8, 56, 0, 0, 0, 16, 12, 0, 24, 104, 0, 0, 0,
    48, 7, 0, 0, 204, 0, 0, 0, 96, 1, 128, 3, 134, 0, 0, 0, 192, 0, 240, 6, 3, 128, 0, 1, 128, 0,
    63, 28, 1, 255, 255, 255, 0, 0, 3, 240, 0, 31, 255, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
