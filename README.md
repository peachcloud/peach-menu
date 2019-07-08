# peach-menu

[![Build Status](https://travis-ci.com/peachcloud/peach-menu.svg?branch=master)](https://travis-ci.com/peachcloud/peach-menu)

OLED menu microservice module for PeachCloud. A state machine which listens for GPIO events (button presses) by subscribing to `peach-buttons` over websockets and makes [JSON-RPC](https://www.jsonrpc.org/specification) calls to relevant PeachCloud microservices (eg. peach-oled & peach-network).

_Note: This module is a work-in-progress._

### Button Code Mappings

```
0 => Center,  
1 => Left,  
2 => Right,  
3 => Up,  
4 => Down,  
5 => A,  
6 => B
```

### States

```
Welcome,  
Help,  
Clock,  
Networking
```

### Environment

The JSON-RPC HTTP server address and port for the OLED microservice can be configured with the `PEACH_OLED_SERVER` environment variable:

`export PEACH_OLED_SERVER=127.0.0.1:5000`

When not set, the value defaults to `127.0.0.1:5112`.

Logging is made available with `env_logger`:

`export RUST_LOG=info`

Other logging levels include `debug`, `warn` and `error`.

### Setup

Clone this repo:

`git clone https://github.com/peachcloud/peach-menu.git`

Move into the repo and compile:

`cd peach-menu`  
`cargo build`

Run the binary:

`./target/debug/peach-menu`

_Note: Will currently panic if `peach_buttons` is not running (connection to ws server fails)._

### Resources

This work was made much, much easier by the awesome blog post titled [Pretty State Machine Patterns in Rust](https://hoverbear.org/2016/10/12/rust-state-machine-pattern/) by [hoverbear](https://hoverbear.org/about/). Thanks hoverbear!

### Licensing

AGPL-3.0
