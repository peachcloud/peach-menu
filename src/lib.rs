//! # peach-menu
//!
//! `peach_menu` implements a GPIO event loop which listens for button press
//! events on seven GPIO lines. These events are used to drive a state machine.
//! The state machine initiates requests to JSON-RPC microservices in order
//! to retrieve device state data and update device state. These actions
//! result in the OLED display being updated. Together, this functionality
//! comprises a physical menu system with input (buttons) and output (OLED).
//!
//! Retrieving and updating device state is handled via the `peach-lib`
//! JSON-RPC client library. With each transition of the state machine, RPC
//! requests are sent to the respective microservices (`peach-network`,
//! `peach-stats`, `peach-oled`).
//!
mod buttons;
mod state_machine;
mod states;

use anyhow::Result;
use log::{debug, info};
use tokio::sync::mpsc;

use crate::buttons::{Button, ButtonHandle};

/// Configures channels for message passing, spawns the GPIO event listeners,
/// and launches the state machine.
///
/// A Receiver is passed into the `state_changer` and the corresponding Sender
/// is cloned and passed into each GPIO event listener. This allows the
/// `button_code` for each pressed button to be passed to the state machine,
/// where it serves as an event which determines the state transition of the
/// menu system.
pub async fn run() -> Result<()> {
    info!("Starting up.");

    debug!("Creating bounded channel for message passing.");
    let (sender, receiver) = mpsc::channel(8);

    let pin = vec![4, 27, 23, 17, 22, 5, 6];
    let code = vec![0, 1, 2, 3, 4, 5, 6];

    debug!("Setting up GPIO event handlers.");
    for i in 0..7 {
        let button = Button::new(pin[i], code[i], sender.clone());
        ButtonHandle::spawn(button);
    }

    debug!("Starting state-machine.");
    state_machine::state_changer(receiver).await?;

    Ok(())
}
