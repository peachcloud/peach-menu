extern crate crossbeam_channel;
extern crate ws;

use std::{process, thread, time};

//use chrono::{DateTime, Local};
use crossbeam_channel::*;

use crate::error::MenuError;
use crate::network::*;
use crate::oled::*;
use crate::states::*;
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
    ActivateAp,
    ActivateClient,
    Home(u8),
    //HomeNet,
    //HomeStats,
    //HomePower,
    //HomeShut,
    Logo,
    Network,
    NetworkConf,
    NetworkConfAp,
    NetworkConfClient,
    PowerOff,
    PowerOn,
    Shutdown,
    Stats,
}

//#[derive(Debug, PartialEq)]
///// The states of the Home view.
//pub enum Home {

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
//   4 - Shutdown

impl State {
    /// Determines the next state based on current state and event.
    pub fn next(self, event: Event) -> State {
        match (self, event) {
            (State::Logo, Event::A) => State::Home(0),
            (State::Home(0), Event::Down) => State::Home(1),
            (State::Home(0), Event::Up) => State::Home(4),
            (State::Home(0), Event::A) => State::Network,
            (State::Home(_), Event::B) => State::Logo,
            (State::Home(1), Event::Down) => State::Home(2),
            (State::Home(1), Event::Up) => State::Home(4),
            (State::Home(1), Event::A) => State::Network,
            (State::Home(2), Event::Down) => State::Home(3),
            (State::Home(2), Event::Up) => State::Home(1),
            (State::Home(2), Event::A) => State::Stats,
            (State::Home(3), Event::Down) => State::Home(4),
            (State::Home(3), Event::Up) => State::Home(2),
            (State::Home(3), Event::A) => State::PowerOff,
            (State::Home(4), Event::Down) => State::Home(1),
            (State::Home(4), Event::Up) => State::Home(3),
            (State::Home(4), Event::A) => State::Shutdown,
            (State::Home(_), _) => State:: Home(0),
            (State::Network, Event::A) => State::NetworkConf,
            (State::Network, Event::B) => State::Home(0),
            (State::NetworkConf, Event::A) => State::ActivateClient,
            (State::NetworkConf, Event::B) => State::Network,
            (State::NetworkConf, Event::Down) => State::NetworkConfAp,
            (State::NetworkConf, Event::Up) => State::NetworkConfAp,
            (State::NetworkConfClient, Event::A) => State::ActivateClient,
            (State::NetworkConfClient, Event::B) => State::Network,
            (State::NetworkConfClient, Event::Down) => State::NetworkConfAp,
            (State::NetworkConfClient, Event::Up) => State::NetworkConfAp,
            (State::NetworkConfAp, Event::A) => State::ActivateAp,
            (State::NetworkConfAp, Event::B) => State::Network,
            (State::NetworkConfAp, Event::Down) => State::NetworkConfClient,
            (State::NetworkConfAp, Event::Up) => State::NetworkConfClient,
            (State::PowerOff, _) => State::PowerOn,
            (State::Stats, Event::B) => State::Home(0),
            (State::ActivateAp, Event::B) => State::Network,
            (State::ActivateAp, Event::Down) => State::NetworkConfClient,
            (State::ActivateAp, Event::Up) => State::NetworkConfClient,
            (State::ActivateClient, Event::B) => State::Network,
            (State::ActivateClient, Event::Down) => State::NetworkConfAp,
            (State::ActivateClient, Event::Up) => State::NetworkConfAp,
            // return current state if combination is unmatched
            (s, _) => s,
        }
    }

    /// Executes state-specific logic for current state.
    pub fn run(&self) -> Result<(), MenuError> {
        match *self {
            State::ActivateAp => {
                info!("State changed to: ActivateAp.");
                oled_clear()?;
                oled_write(27, 16, "DEPLOYING".to_string(), "6x8".to_string())?;
                oled_write(27, 27, "ACCESS".to_string(), "6x8".to_string())?;
                oled_write(27, 38, "POINT...".to_string(), "6x8".to_string())?;
                oled_flush()?;
                network_activate_ap()?;

                let client = "  Client mode".to_string();
                let ap = "> Access point mode".to_string();

                oled_clear()?;
                oled_write(0, 0, client, "6x8".to_string())?;
                oled_write(0, 9, ap, "6x8".to_string())?;
                oled_flush()?;
            }
            State::ActivateClient => {
                info!("State changed to: ActivateClient.");
                oled_clear()?;
                oled_write(24, 16, "ACTIVATING".to_string(), "6x8".to_string())?;
                oled_write(24, 27, "WIRELESS".to_string(), "6x8".to_string())?;
                oled_write(24, 38, "CONNECTION...".to_string(), "6x8".to_string())?;
                oled_flush()?;
                network_activate_client()?;

                let client = "> Client mode".to_string();
                let ap = "Access point mode".to_string();

                oled_clear()?;
                oled_write(0, 0, client, "6x8".to_string())?;
                oled_write(12, 9, ap, "6x8".to_string())?;
                oled_flush()?;
            }
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
            // home: shutdown
            State::Home(4) => {
                info!("State changed to: Home 4.");
                state_home(4)?;
            }
            // home: unknown
            State::Home(_) => {
                info!("State changed to: Home _.");
            }
            State::Logo => {
                info!("State changed to: Logo.");
                let bytes = PEACH_LOGO.to_vec();
                oled_clear()?;
                oled_draw(bytes, 64, 64, 32, 0)?;
                oled_flush()?;
            }
            State::Network => {
                info!("State changed to: Network.");
                let status = match network_get_state("wlan0".to_string()) {
                    Ok(state) => state,
                    Err(_) => "Error".to_string(),
                };
                if status == "up" {
                    let mode = "MODE Client".to_string();
                    let show_status = format!("STATUS {}", status);
                    let ip = match network_get_ip("wlan0".to_string()) {
                        Ok(ip) => ip,
                        Err(_) => "x.x.x.x".to_string(),
                    };
                    let show_ip = format!("IP {}", ip);
                    let ssid = match network_get_ssid("wlan0".to_string()) {
                        Ok(ssid) => ssid,
                        Err(_) => "Not connected".to_string(),
                    };
                    let show_ssid = format!("NETWORK {}", ssid);
                    let rssi = match network_get_rssi("wlan0".to_string()) {
                        Ok(rssi) => rssi,
                        Err(_) => "_".to_string(),
                    };
                    let show_rssi = format!("SIGNAL {}dBm", rssi);
                    let config = "> Configuration".to_string();

                    oled_clear()?;
                    oled_write(0, 0, mode, "6x8".to_string())?;
                    oled_write(0, 9, show_status, "6x8".to_string())?;
                    oled_write(0, 18, show_ssid, "6x8".to_string())?;
                    oled_write(0, 27, show_ip, "6x8".to_string())?;
                    oled_write(0, 36, show_rssi, "6x8".to_string())?;
                    oled_write(0, 54, config, "6x8".to_string())?;
                    oled_flush()?;
                } else {
                    let mode = "MODE Access Point".to_string();
                    let status = match network_get_state("ap0".to_string()) {
                        Ok(state) => state,
                        Err(_) => "Error".to_string(),
                    };
                    let show_status = format!("STATUS {}", status);
                    let ip = match network_get_ip("ap0".to_string()) {
                        Ok(ip) => ip,
                        Err(_) => "x.x.x.x".to_string(),
                    };
                    let show_ip = format!("IP {}", ip);
                    let ssid = "peach".to_string();
                    let show_ssid = format!("NETWORK {}", ssid);
                    let config = "> Configuration".to_string();

                    oled_clear()?;
                    oled_write(0, 0, mode, "6x8".to_string())?;
                    oled_write(0, 9, show_status, "6x8".to_string())?;
                    oled_write(0, 18, show_ssid, "6x8".to_string())?;
                    oled_write(0, 27, show_ip, "6x8".to_string())?;
                    oled_write(0, 54, config, "6x8".to_string())?;
                    oled_flush()?;
                }
            }
            State::NetworkConf => {
                info!("State changed to: NetworkConf.");
                let client = "> Client mode".to_string();
                let ap = "Access point mode".to_string();

                oled_clear()?;
                oled_write(0, 0, client, "6x8".to_string())?;
                oled_write(12, 9, ap, "6x8".to_string())?;
                oled_flush()?;
            }
            State::NetworkConfAp => {
                info!("State changed to: NetworkConfAp.");
                oled_write(0, 0, "  ".to_string(), "6x8".to_string())?;
                oled_write(0, 9, "> ".to_string(), "6x8".to_string())?;
                oled_flush()?;
            }
            State::NetworkConfClient => {
                info!("State changed to: NetworkConfClient.");
                oled_write(0, 0, "> ".to_string(), "6x8".to_string())?;
                oled_write(0, 9, "  ".to_string(), "6x8".to_string())?;
                oled_flush()?;
            }
            State::PowerOff => {
                info!("State changed to: PowerOff.");
                oled_power(false)?;
            }
            State::PowerOn => {
                info!("State changed to: PowerOn.");
                oled_power(true)?;
            }
            State::Shutdown => {
                info!("State changed to: Shutdown.");
                oled_clear()?;
                oled_write(27, 16, "SHUTTING".to_string(), "6x8".to_string())?;
                oled_write(27, 27, "DOWN".to_string(), "6x8".to_string())?;
                oled_write(27, 38, "DEVICE...".to_string(), "6x8".to_string())?;
                oled_flush()?;
                
                let three_secs = time::Duration::from_millis(3000);
                thread::sleep(three_secs);
                oled_power(false)?;
                info!("Shutting down device");
                process::Command::new("sudo")
                    .arg("shutdown")
                    .arg("now")
                    .output()
                    .expect("Failed to shutdown");
            }

            State::Stats => {
                info!("State changed to: Stats.");
                let cpu = cpu_stats_percent()?;
                let cpu_stats = format!(
                    "CPU {} us {} sy {} id",
                    cpu.user.round(),
                    cpu.system.round(),
                    cpu.idle.round()
                );
                let mem = mem_stats()?;
                let mem_stats = format!(
                    "MEM {}MB f {}MB u",
                    (mem.free / 1024).to_string(),
                    (mem.used / 1024).to_string()
                );
                let load = load_average()?;
                let load_stats = format!("LOAD {} {} {}", load.one, load.five, load.fifteen);
                let uptime = uptime()?;
                let uptime_stats = format!("UPTIME {} hrs", uptime);

                let traffic = network_get_traffic("wlan0".to_string())?;
                let rx = (traffic.received / 1024 / 1024).to_string();
                let rx_stats = format!("DATA RX {}MB", rx);
                let tx = (traffic.transmitted / 1024 / 1024).to_string();
                let tx_stats = format!("DATA TX {}MB", tx);

                oled_clear()?;
                oled_write(0, 0, cpu_stats, "6x8".to_string())?;
                oled_write(0, 9, mem_stats, "6x8".to_string())?;
                oled_write(0, 18, load_stats, "6x8".to_string())?;
                oled_write(0, 27, uptime_stats, "6x8".to_string())?;
                oled_write(0, 36, rx_stats, "6x8".to_string())?;
                oled_write(0, 45, tx_stats, "6x8".to_string())?;
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
