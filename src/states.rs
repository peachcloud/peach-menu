use std::{process, thread, time};

use chrono::{DateTime, Local};
use log::info;

use peach_lib::network_client::*;
use peach_lib::oled_client::*;
use peach_lib::stats_client::*;

use crate::error::MenuError;

pub fn state_network_mode(mode: u8) -> Result<(), MenuError> {
    match mode {
        0 => {
            oled_clear()?;
            oled_write(24, 16, "ACTIVATING", "6x8")?;
            oled_write(24, 27, "WIRELESS", "6x8")?;
            oled_write(24, 38, "CONNECTION...", "6x8")?;
            oled_flush()?;

            network_activate_client()?;

            oled_clear()?;
            oled_write(0, 0, "> Client mode", "6x8")?;
            oled_write(0, 9, "  Access point mode", "6x8")?;
            oled_flush()?;

            Ok(())
        }
        1 => {
            oled_clear()?;
            oled_write(27, 16, "DEPLOYING", "6x8")?;
            oled_write(27, 27, "ACCESS", "6x8")?;
            oled_write(27, 38, "POINT...", "6x8")?;
            oled_flush()?;

            network_activate_ap()?;

            oled_clear()?;
            oled_write(0, 0, "  Client mode", "6x8")?;
            oled_write(0, 9, "> Access point mode", "6x8")?;
            oled_flush()?;

            Ok(())
        }
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
            oled_write(96, 0, &t, "6x8")?;
            oled_write(0, 0, "PeachCloud", "6x8")?;
            oled_write(0, 18, "> Networking", "6x8")?;
            oled_write(0, 27, "  System Stats", "6x8")?;
            oled_write(0, 36, "  Display Off", "6x8")?;
            oled_write(0, 45, "  Reboot", "6x8")?;
            oled_write(0, 54, "  Shutdown", "6x8")?;
            oled_write(100, 54, "v0.2", "6x8")?;
            oled_flush()?;

            Ok(())
        }
        // Home: networking
        1 => {
            oled_write(0, 18, "> ", "6x8")?;
            oled_write(0, 27, "  ", "6x8")?;
            oled_write(0, 36, "  ", "6x8")?;
            oled_write(0, 45, "  ", "6x8")?;
            oled_write(0, 54, "  ", "6x8")?;
            oled_flush()?;

            Ok(())
        }
        // Home: system stats
        2 => {
            oled_write(0, 18, "  ", "6x8")?;
            oled_write(0, 27, "> ", "6x8")?;
            oled_write(0, 36, "  ", "6x8")?;
            oled_write(0, 45, "  ", "6x8")?;
            oled_write(0, 54, "  ", "6x8")?;
            oled_flush()?;

            Ok(())
        }
        // Home: display off
        3 => {
            oled_write(0, 18, "  ", "6x8")?;
            oled_write(0, 27, "  ", "6x8")?;
            oled_write(0, 36, "> ", "6x8")?;
            oled_write(0, 45, "  ", "6x8")?;
            oled_write(0, 54, "  ", "6x8")?;
            oled_flush()?;

            Ok(())
        }
        // Home: reboot
        4 => {
            oled_write(0, 18, "  ", "6x8")?;
            oled_write(0, 27, "  ", "6x8")?;
            oled_write(0, 36, "  ", "6x8")?;
            oled_write(0, 45, "> ", "6x8")?;
            oled_write(0, 54, "  ", "6x8")?;
            oled_flush()?;

            Ok(())
        }
        // Home: shutdown
        5 => {
            oled_write(0, 18, "  ", "6x8")?;
            oled_write(0, 27, "  ", "6x8")?;
            oled_write(0, 36, "  ", "6x8")?;
            oled_write(0, 45, "  ", "6x8")?;
            oled_write(0, 54, "> ", "6x8")?;
            oled_flush()?;

            Ok(())
        }
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
    let status = match network_state("wlan0") {
        Ok(state) => state,
        Err(_) => "Error".to_string(),
    };
    match status.as_ref() {
        // wlan0 is up or dormant
        // Network: Client mode
        "up" | "dormant" => {
            let show_status = format!("STATUS {}", status);
            let ip = match network_ip("wlan0") {
                Ok(ip) => ip,
                Err(_) => "x.x.x.x".to_string(),
            };
            let show_ip = format!("IP {}", ip);
            let ssid = match network_ssid("wlan0") {
                Ok(ssid) => ssid,
                Err(_) => "Not connected".to_string(),
            };
            let show_ssid = format!("NETWORK {}", ssid);
            let rssi = match network_rssi("wlan0") {
                Ok(rssi) => rssi,
                Err(_) => "_".to_string(),
            };
            let show_rssi = format!("SIGNAL {}dBm", rssi);
            let config = "> Configuration";

            oled_clear()?;
            oled_write(0, 0, "MODE Client", "6x8")?;
            oled_write(0, 9, &show_status, "6x8")?;
            oled_write(0, 18, &show_ssid, "6x8")?;
            oled_write(0, 27, &show_ip, "6x8")?;
            oled_write(0, 36, &show_rssi, "6x8")?;
            oled_write(0, 54, config, "6x8")?;
            oled_flush()?;

            Ok(())
        }
        // wlan0 is down
        // Network: AP mode
        "down" => {
            let status = match network_state("ap0") {
                Ok(state) => state,
                Err(_) => "Error".to_string(),
            };
            let show_status = format!("STATUS {}", status);
            let ip = match network_ip("ap0") {
                Ok(ip) => ip,
                Err(_) => "x.x.x.x".to_string(),
            };
            let show_ip = format!("IP {}", ip);
            let ssid = "peach";
            let show_ssid = format!("NETWORK {}", ssid);
            let config = "> Configuration";

            oled_clear()?;
            oled_write(0, 0, "MODE Access Point", "6x8")?;
            oled_write(0, 9, &show_status, "6x8")?;
            oled_write(0, 18, &show_ssid, "6x8")?;
            oled_write(0, 27, &show_ip, "6x8")?;
            oled_write(0, 54, config, "6x8")?;
            oled_flush()?;

            Ok(())
        }
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
            oled_clear()?;
            oled_write(0, 0, "> Client Mode", "6x8")?;
            oled_write(0, 9, "  Access Point Mode", "6x8")?;
            oled_flush()?;

            Ok(())
        }
        // NetworkConf: client
        1 => {
            oled_write(0, 0, "> ", "6x8")?;
            oled_write(0, 9, "  ", "6x8")?;
            oled_flush()?;

            Ok(())
        }
        // NetworkConf: ap
        2 => {
            oled_write(0, 0, "  ", "6x8")?;
            oled_write(0, 9, "> ", "6x8")?;
            oled_flush()?;

            Ok(())
        }
        // outlier
        _ => Ok(()),
    }
}

pub fn state_reboot() -> Result<(), MenuError> {
    oled_clear()?;
    oled_write(27, 16, "REBOOTING", "6x8")?;
    oled_write(27, 27, "DEVICE...", "6x8")?;
    oled_flush()?;

    let three_secs = time::Duration::from_millis(3000);
    thread::sleep(three_secs);

    oled_power(false)?;
    info!("Rebooting device");
    process::Command::new("sudo")
        .arg("/sbin/shutdown")
        .arg("-r")
        .arg("now")
        .output()
        .expect("Failed to reboot");

    Ok(())
}

pub fn state_shutdown() -> Result<(), MenuError> {
    oled_clear()?;
    oled_write(27, 16, "SHUTTING", "6x8")?;
    oled_write(27, 27, "DOWN", "6x8")?;
    oled_write(27, 38, "DEVICE...", "6x8")?;
    oled_flush()?;

    let three_secs = time::Duration::from_millis(3000);
    thread::sleep(three_secs);

    oled_power(false)?;
    info!("Shutting down device");
    process::Command::new("sudo")
        .arg("/sbin/shutdown")
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
    let mem_stats = format!("MEM {}MB f {}MB u", mem.free / 1024, mem.used / 1024);
    let load = load_average()?;
    let load_stats = format!("LOAD {} {} {}", load.one, load.five, load.fifteen);
    let uptime = uptime()?;
    let uptime_stats = format!("UPTIME {} mins", uptime);
    let traffic = network_traffic("wlan0")?;
    let rx = traffic.received / 1024 / 1024;
    let rx_stats = format!("DATA RX {}MB", rx);
    let tx = traffic.transmitted / 1024 / 1024;
    let tx_stats = format!("DATA TX {}MB", tx);

    oled_clear()?;
    oled_write(0, 0, &cpu_stats, "6x8")?;
    oled_write(0, 9, &mem_stats, "6x8")?;
    oled_write(0, 18, &load_stats, "6x8")?;
    oled_write(0, 27, &uptime_stats, "6x8")?;
    oled_write(0, 36, &rx_stats, "6x8")?;
    oled_write(0, 45, &tx_stats, "6x8")?;
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
