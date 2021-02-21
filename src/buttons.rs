use anyhow::{Context, Result};
use futures::stream::StreamExt;
use gpio_cdev::{AsyncLineEventHandle, Chip, EventRequestFlags, LineRequestFlags};
use log::debug;
use tokio::sync::mpsc;

pub struct Button {
    pin: u32,
    button_code: u8,
    sender: mpsc::Sender<u8>,
}

impl Button {
    pub fn new(pin: u32, button_code: u8, sender: mpsc::Sender<u8>) -> Self {
        Button {
            pin,
            button_code,
            sender,
        }
    }
}

pub struct ButtonHandle;

impl ButtonHandle {
    pub fn spawn(button: Button) {
        tokio::spawn(listen(button));
    }
}

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
