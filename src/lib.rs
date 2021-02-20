//! # peach-menu
//!
//! `peach_menu` is a collection of utilities and data structures for running
//! a menu state machine. I/O takes place using JSON-RPC 2.0 over websockets,
//! with `peach-buttons` providing GPIO input data and `peach-oled` receiving
//! output data for display.
//!
mod buttons;
mod state_machine;
mod states;
mod structs;

use anyhow::Result;
use log::{debug, info};
use tokio::sync::mpsc;

use crate::buttons::Button;

/// Configures channels for message passing, launches the state machine
/// changer thread and connects to the `peach-buttons` JSON-RPC pubsub
/// service over websockets.
///
/// A Receiver is passed into `state_changer` and the corresponding Sender
/// is passed into the websockets client. This allows the `button_code` to
/// be extracted from the received websocket message and passed to the
/// state machine.
///
pub async fn run() -> Result<()> {
    info!("Starting up.");

    debug!("Creating bounded channel for message passing.");
    let (sender, receiver) = mpsc::channel(8);

    let pin = vec![4, 27, 23, 17, 22, 5, 6];
    let code = vec![0, 1, 2, 3, 4, 5, 6];

    debug!("Setting up GPIO event handlers.");
    for i in 0..7 {
        Button::new(pin[i], code[i], sender.clone())
            .listen()
            .await?;
    }

    debug!("Starting state-machine.");
    state_machine::state_changer(receiver).await?;

    Ok(())
}
