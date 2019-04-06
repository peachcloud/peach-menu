// https://play.rust-lang.org/?gist=ee3e4df093c136ced7b394dc7ffb78e1&version=stable&backtrace=0

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

use std::thread;
use failure::Fail;
use jsonrpc_http_server::jsonrpc_core::types::error::Error;
use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::*;
use serde::Deserialize;
use crossbeam_channel::unbounded;

fn build_msg(x_coord: i32, y_coord: i32, string: String) -> Msg {
    Msg {
        x_coord,
        y_coord,
        string,
    }
}

fn main() {
   
    // create an unbounded channel
    let (s, r) = unbounded();
    let (s1, r1) = (s.clone(), r.clone());

    // state_changer thread
    thread::spawn(move || {
        let mut state = State::Welcome;
        loop {
            let button_code = r1.recv().unwrap();
            println!("{:}", button_code);
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
            println!(" to {:?}", state);
            if let State::Failure(string) = state {
                println!("{}", string);
                break;
            } else {
                state.run();
            }
        }
    });

    let mut io = IoHandler::default();
    let (s2, r2) = (s.clone(), r.clone());

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

#[derive(Debug)]
struct Msg {
    x_coord: i32,
    y_coord: i32,
    string: String,
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
            State::Welcome => println!("Welcome to PeachCloud!"),
            State::Help => println!("Navigation"),
            State::Clock => println!("Clock"),
            State::Networking => println!("Network Stats"),
            State::Failure(_) => {},
        }
    }
}
