use chrono::{DateTime, Local};

use std::{process, thread, time};

use crate::error::MenuError;
use crate::network::*;
use crate::oled::*;
use crate::stats::*;

pub fn state_network_mode(mode: u8) -> Result<(), MenuError> {
    match mode {
        0 => {
            oled_clear()?;
            oled_write(24, 16, "ACTIVATING".to_string(), "6x8".to_string())?;
            oled_write(24, 27, "WIRELESS".to_string(), "6x8".to_string())?;
            oled_write(24, 38, "CONNECTION...".to_string(), "6x8".to_string())?;
            oled_flush()?;
            
            network_activate_client()?;
            
            let client = "> Client mode".to_string();
            let ap = "  Access point mode".to_string();
            oled_clear()?;
            oled_write(0, 0, client, "6x8".to_string())?;
            oled_write(0, 9, ap, "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        1 => {
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

            Ok(())
        },
        _ => Ok(()),
    }
}

pub fn state_home(selected: u8) -> Result<(), MenuError> {
    // match on `selected`
    match selected {
        // Home: root
        0 => {
            let dt: DateTime<Local> = Local::now();
            let t = format!("{}", dt.time().format("%H:%M"));

            oled_clear()?;
            oled_write(96, 0, t, "6x8".to_string())?;
            oled_write(0, 0, "PeachCloud".to_string(), "6x8".to_string())?;
            oled_write(0, 18, "> Networking".to_string(), "6x8".to_string())?;
            oled_write(0, 27, "  System Stats".to_string(), "6x8".to_string())?;
            oled_write(0, 36, "  Display Off".to_string(), "6x8".to_string())?;
            oled_write(0, 45, "  Shutdown".to_string(), "6x8".to_string())?;
            oled_write(100, 54, "v0.1".to_string(), "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // Home: networking
        1 => {
            oled_write(0, 18, "> ".to_string(), "6x8".to_string())?;
            oled_write(0, 27, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 36, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 45, "  ".to_string(), "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // Home: system stats
        2 => {
            oled_write(0, 18, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 27, "> ".to_string(), "6x8".to_string())?;
            oled_write(0, 36, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 45, "  ".to_string(), "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // Home: display off
        3 => {
            oled_write(0, 18, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 27, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 36, "> ".to_string(), "6x8".to_string())?;
            oled_write(0, 45, "  ".to_string(), "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // Home: shutdown
        4 => {
            oled_write(0, 18, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 27, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 36, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 45, "> ".to_string(), "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // outlier
        _ => Ok(()),
    }
}

pub fn state_logo() -> Result<(), MenuError> {
    let bytes = PEACH_LOGO.to_vec();
    oled_clear()?;
    oled_draw(bytes, 64, 64, 32, 0)?;
    oled_flush()?;

    Ok(())
}

pub fn state_network() -> Result<(), MenuError> {
    let status = match network_get_state("wlan0".to_string()) {
        Ok(state) => state,
        Err(_) => "Error".to_string(),
    };
    match status.as_ref() {
        // wlan0 is up
        // Network: Client mode
        "up" => {
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

            Ok(())
        },
        // wlan0 is down
        // Network: AP mode
        "down" => {
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
            oled_write(0, 0, "> ".to_string(), "6x8".to_string())?;
            oled_write(0, 9, "  ".to_string(), "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // outlier
        // TODO: account for iface states other than 'up' and 'down'
        _ => Ok(()),
    }
}

pub fn state_network_conf(selected: u8) -> Result<(), MenuError> {
    // match on `selected`
    match selected {
        // NetworkConf: root
        0 => {
            let client = "> Client Mode".to_string();
            let ap = "  Access Point Mode".to_string();
            oled_clear()?;
            oled_write(0, 0, client, "6x8".to_string())?;
            oled_write(0, 9, ap, "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // NetworkConf: client
        1 => {
            oled_write(0, 0, "> ".to_string(), "6x8".to_string())?;
            oled_write(0, 9, "  ".to_string(), "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // NetworkConf: ap
        2 => {
            oled_write(0, 0, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 9, "> ".to_string(), "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // outlier
        _ => Ok(()),
    }
}

pub fn state_shutdown() -> Result<(), MenuError> {
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

    Ok(())
}

pub fn state_stats() -> Result<(), MenuError> {
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

    Ok(())
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
