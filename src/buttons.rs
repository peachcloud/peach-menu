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

    pub async fn listen(&mut self) -> Result<()> {
        let mut chip = Chip::new("/dev/gpiochip0")?;

        let input = chip.get_line(self.pin)?;

        let mut events = AsyncLineEventHandle::new(input.events(
            LineRequestFlags::INPUT,
            EventRequestFlags::BOTH_EDGES,
            "gpioevents",
        )?)?;

        while let Some(event) = events.next().await {
            debug!("{:?}", event?);
            self.sender
                .send(self.button_code)
                .await
                .context("State machine receiver has closed")?;
        }

        Ok(())
    }
}
