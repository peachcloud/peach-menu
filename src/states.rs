use chrono::{DateTime, Local};

use crate::error::MenuError;
use crate::oled::*;

pub fn state_home(selected: u8) -> Result<(), MenuError> {
    // match on `selected`
    match selected {
        // home: root
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
        // home: networking
        1 => {
            oled_write(0, 18, "> ".to_string(), "6x8".to_string())?;
            oled_write(0, 27, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 36, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 45, "  ".to_string(), "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // home: system stats
        2 => {
            oled_write(0, 18, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 27, "> ".to_string(), "6x8".to_string())?;
            oled_write(0, 36, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 45, "  ".to_string(), "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // home: display off
        3 => {
            oled_write(0, 18, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 27, "  ".to_string(), "6x8".to_string())?;
            oled_write(0, 36, "> ".to_string(), "6x8".to_string())?;
            oled_write(0, 45, "  ".to_string(), "6x8".to_string())?;
            oled_flush()?;

            Ok(())
        },
        // home: shutdown
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
