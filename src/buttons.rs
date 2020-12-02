use std::process;

use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ws::{CloseCode, Error, Handler, Handshake, Message, Sender};

#[derive(Debug, Deserialize)]
pub struct Press {
    pub button_code: u8,
}

#[derive(Serialize, Deserialize)]
struct ButtonMsg {
    jsonrpc: String,
    method: String,
    params: Vec<u8>,
}

/// Websocket client for `peach_buttons`.
#[derive(Debug)]
pub struct Client<'a> {
    pub out: Sender,
    pub s: &'a crossbeam_channel::Sender<u8>,
}

impl<'a> Handler for Client<'a> {
    /// Sends request to `peach_buttons` to subscribe to emitted events.
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        info!("Subscribing to peach_buttons microservice over ws.");
        let subscribe = json!({
            "id":1,
            "jsonrpc":"2.0",
            "method":"subscribe_buttons"
        });
        let data = subscribe.to_string();
        self.out.send(data)
    }

    /// Displays JSON-RPC request from `peach_buttons`.
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        info!("Received ws message from peach_buttons.");
        // button_code must be extracted from the request and passed to
        // state_changer
        let m: String = msg.into_text()?;
        // distinguish button_press events from other received jsonrpc requests
        if m.contains(r"params") {
            // serialize msg string into a struct
            let bm: ButtonMsg = serde_json::from_str(&m).unwrap_or_else(|err| {
                error!("Problem serializing button_code msg: {}", err);
                process::exit(1);
            });
            debug!("Sending button code to state_changer.");
            // send the button_code parameter to state_changer
            self.s.send(bm.params[0]).unwrap_or_else(|err| {
                error!("Problem sending button_code over channel: {}", err);
                process::exit(1);
            });
        }
        Ok(())
    }

    /// Handles disconnection from websocket and displays debug data.
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => {
                info!("The client is done with the connection.");
            }
            CloseCode::Away => {
                info!("The client is leaving the site.");
            }
            CloseCode::Abnormal => {
                warn!("Closing handshake failed! Unable to obtain closing status from client.");
            }
            _ => error!("The client encountered an error: {}", reason),
        }
    }

    fn on_error(&mut self, err: Error) {
        error!("The server encountered an error: {:?}", err);
    }
}
