// jsonrpc call is triggered by peach-buttons
//   -> press() listens for button_code & passes to state_changer thread
//      via a channel
//   -> state_changer instantiates an instance of the state machine
//      then listens on the channel receiver
//        -> received button_code is mapped to an Event
//        -> Event is passed with State to .next()
//        -> New state code is calculated and executed with .run()
//        -> Fails currently break
//

#[macro_use]
extern crate crossbeam_channel;
#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

use std::thread;
use failure::Fail;
use jsonrpc_http_server::jsonrpc_core::types::error::Error;
use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::*;
use serde::Deserialize;
use crossbeam_channel::unbounded;
use jsonrpc_client_http::HttpTransport;

fn oled_client(x_coord: i32, y_coord: i32, string: String) {
    // create http transport for jsonrpc comms
    let transport = HttpTransport::new().standalone().unwrap();
    let transport_handle = transport.handle("http://127.0.0.1:3030").unwrap();
    let mut client = PeachOledClient::new(transport_handle);

    // send msg to oled for display
    client.write(x_coord, y_coord, string).call().unwrap();
}

fn main() {
   
    // create an unbounded channel
    let (s, r) = unbounded();
    let (_s1, r1) = (s.clone(), r.clone());

    // state_changer thread
    thread::spawn(move || {
        let mut state = State::Welcome;
        loop {
            let button_code = r1.recv().unwrap();
            // match on button_code & pass event to state.next
            let event = match button_code {
                // button code mappings
                0 => Event::Center,
                1 => Event::Left,
                2 => Event::Right,
                3 => Event::Up,
                4 => Event::Down,
                5 => Event::A,
                6 => Event::B,
                _ => Event::Unknown,
            };
            state = state.next(event);
            if let State::Failure(string) = state {
                break;
            } else {
                state.run();
            }
        }
    });

    let mut io = IoHandler::default();
    let (s2, _r2) = (s.clone(), r.clone());

    io.add_method("press", move |params: Params| {
        let p: Result<Press> = params.parse();
        match p {
            // if result contains `button`, unwrap
            Ok(_) => {
                let p: Press = p.unwrap();
                // send p.button_code to state_changer via channel sender
                s2.send(p.button_code).unwrap();
                Ok(Value::String("success".into()))
            },
            Err(e) => Err(Error::from(PressError::MissingParams { e })),
        }
    });

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(&"127.0.0.1:3031".parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();
}

#[derive(Debug, Deserialize)]
struct Press {
    button_code: u8,
}

// jsonrpc client
jsonrpc_client!(pub struct PeachOledClient {
    // send msg(s) to oled display for printing
    pub fn write(&mut self, x_coord: i32, y_coord: i32, string: String) -> RpcRequest<String>;
});

// error handling for jsonrpc methods
#[derive(Debug, Fail)]
pub enum PressError {
    #[fail(display = "missing expected parameters")]
    MissingParams { e: Error },
}

impl From<PressError> for Error {
    fn from(err: PressError) -> Self {
        match &err {
            PressError::MissingParams { e } => Error {
                code: ErrorCode::ServerError(-32602),
                message: "invalid params".into(),
                data: Some(format!("{}", e.message).into()),
            },
            err => Error {
                code: ErrorCode::InternalError,
                message: "internal error".into(),
                data: Some(format!("{:?}", err).into()),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Welcome,
    Help,
    Clock,
    Networking,
    Failure(String),
}

#[derive(Debug, Clone, Copy)]
enum Event {
    Center,
    Left,
    Right,
    Down,
    Up,
    A,
    B,
    Unknown,
}

// state machine functionality
impl State {
    fn next(self, event: Event) -> State {
        match (self, event) {
            (_, Event::Center) => State::Welcome,
            (State::Welcome, Event::Left) => State::Networking,
            (State::Welcome, Event::Right) => State::Help,
            (State::Help, Event::Left) => State::Welcome,
            (State::Help, Event::Right) => State::Clock,
            (State::Clock, Event::Left) => State::Help,
            (State::Clock, Event::Right) => State::Networking,
            (State::Networking, Event::Left) => State::Clock,
            (State::Networking, Event::Right) => State::Welcome,
            (s, e) => {
                State::Failure(format!("Wrong state, event combination: {:#?} {:#?}", s, e)
                    .to_string())
            }
        }
    }

    fn run(&self) {
        match *self {
            State::Welcome => {
                let x_coord = 0;
                let y_coord = 0;
                let string = "Welcome to PeachCloud".to_string();
                // perform write() call to peach-oled
                oled_client(x_coord, y_coord, string);
            },
            State::Help => println!("Navigation"),
            State::Clock => println!("Clock"),
            State::Networking => println!("Network Stats"),
            State::Failure(_) => {},
        }
    }
}
