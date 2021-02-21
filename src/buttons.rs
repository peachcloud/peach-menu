use anyhow::{Context, Result};
use futures::stream::StreamExt;
use gpio_cdev::{AsyncLineEventHandle, Chip, EventRequestFlags, LineRequestFlags};
use log::debug;
use tokio::sync::mpsc;

/// A `struct` which represents a single button. Each `Button` is comprised of
/// a GPIO pin address, a button code and the sender of a channel. The sender
/// allows button-press events to be communicated to the listening state machine.
pub struct Button {
    pin: u32,
    button_code: u8,
    sender: mpsc::Sender<u8>,
}

/// Return a new instance of the `Button` `struct`.
impl Button {
    pub fn new(pin: u32, button_code: u8, sender: mpsc::Sender<u8>) -> Self {
        Button {
            pin,
            button_code,
            sender,
        }
    }
}

/// A unit `struct` to serve as a handle for our asynchronous GPIO listener.
pub struct ButtonHandle;

/// Spawn a GPIO event loop to listen for button-press events.
impl ButtonHandle {
    pub fn spawn(button: Button) {
        tokio::spawn(listen(button));
    }
}

/// Initializes a GPIO pin and listens for button-press events. Each event
/// results in the `button_code` being sent to the state machine.
pub async fn listen(mut button: Button) -> Result<()> {
    let mut chip = Chip::new("/dev/gpiochip0")?;

    let input = chip.get_line(button.pin)?;

    let mut events = AsyncLineEventHandle::new(input.events(
        LineRequestFlags::INPUT,
        EventRequestFlags::RISING_EDGE,
        "gpioevents",
    )?)?;

    while let Some(event) = events.next().await {
        debug!("{:?}", event?);
        button
            .sender
            .send(button.button_code)
            .await
            .context("State machine receiver has closed")?;
    }

    Ok(())
}
