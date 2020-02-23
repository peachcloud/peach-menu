//! # peach-menu
//!
//! `peach_menu` is a collection of utilities and data structures for running
//! a menu state machine. I/O takes place using JSON-RPC 2.0 over websockets,
//! with `peach-buttons` providing GPIO input data and `peach-oled` receiving
//! output data for display.

#[macro_use]
pub extern crate log;
extern crate crossbeam_channel;
#[macro_use]
extern crate jsonrpc_client_core;
extern crate ws;

mod buttons;
mod error;
mod network;
mod oled;
mod state_machine;
mod states;
mod stats;
mod structs;

use std::env;

use crossbeam_channel::unbounded;

#[allow(unused_imports)]
use jsonrpc_test as test;

use ws::connect;

use crate::buttons::*;
use crate::state_machine::*;

/// Configures channels for message passing, launches the state machine
/// changer thread and connects to the `peach-buttons` JSON-RPC pubsub
/// service over websockets.
///
/// A Receiver is passed into `state_changer` and the corresponding Sender
/// is passed into the websockets client. This allows the `button_code` to
/// be extracted from the received websocket message and passed to the
/// state machine.
///
pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    info!("Starting up.");

    debug!("Creating unbounded channel for message passing.");
    let (s, r) = unbounded();

    debug!("Spawning state-machine thread.");
    state_changer(r);

    let ws_addr = env::var("PEACH_BUTTONS_SERVER").unwrap_or_else(|_| "127.0.0.1:5111".to_string());

    let ws_server = format!("ws://{}", ws_addr);

    connect(ws_server, |out| Client { out, s: &s })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // test to ensure correct success response
    #[test]
    fn rpc_success() {
        let rpc = {
            let mut io = IoHandler::new();
            io.add_method("rpc_success_response", |_| {
                Ok(Value::String("success".into()))
            });
            test::Rpc::from(io)
        };

        assert_eq!(rpc.request("rpc_success_response", &()), r#""success""#);
    }
}
