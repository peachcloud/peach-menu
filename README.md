# peach-menu

[![Build Status](https://travis-ci.com/peachcloud/peach-menu.svg?branch=master)](https://travis-ci.com/peachcloud/peach-menu) ![Generic badge](https://img.shields.io/badge/version-0.2.3-<COLOR>.svg)

OLED menu microservice module for PeachCloud. A state machine which listens for GPIO events (button presses) by subscribing to `peach-buttons` over websockets and makes [JSON-RPC](https://www.jsonrpc.org/specification) calls to relevant PeachCloud microservices (`peach-network`, `peach-oled`, `peach-stats`).

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
Home(u8),
Logo,
Network,
NetworkConf(u8),
NetworkMode(u8),
OledPower(u8),
Reboot,
Shutdown,
Stats,
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
`cargo build --release`

Run the binary:

`./target/target/peach-menu`

_Note: Will currently panic if `peach_buttons` is not running (connection to ws server fails)._

### Debian Packaging

A `systemd` service file and Debian maintainer scripts are included in the `debian` directory, allowing `peach-menu` to be easily bundled as a Debian package (`.deb`). The `cargo-deb` [crate](https://crates.io/crates/cargo-deb) can be used to achieve this.

Install `cargo-deb`:

`cargo install cargo-deb`

Move into the repo:

`cd peach-menu`

Build the package:

`cargo deb`

The output will be written to `target/debian/peach-menu_0.2.1_arm64.deb` (or similar).

Build the package (aarch64):

`cargo deb --target aarch64-unknown-linux-gnu`

Install the package as follows:

`sudo dpkg -i target/debian/peach-menu_0.2.1_arm64.deb`

The service will be automatically enabled and started.

Uninstall the service:

`sudo apt-get remove peach-menu`

Remove configuration files (not removed with `apt-get remove`):

`sudo apt-get purge peach-menu`

### Resources

This work was made much, much easier by the awesome blog post titled [Pretty State Machine Patterns in Rust](https://hoverbear.org/2016/10/12/rust-state-machine-pattern/) by [hoverbear](https://hoverbear.org/about/). Thanks hoverbear!

### Licensing

AGPL-3.0
