## peach-menu

OLED menu microservice module for PeachCloud. A state machine which listens for GPIO events (button presses) and makes [JSON-RPC](https://www.jsonrpc.org/specification) calls to relevant PeachCloud microservices (eg. peach-oled & peach-network).

_Note: This module is a work-in-progress._

### JSON-RPC API

| Method | Parameters | Description |
| --- | --- | --- |
| `press` | `button_code` | Compute next state based on current state and button pressed |

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
Networking,  
Failure
```

### Setup

Clone this repo:

`git clone https://github.com/peachcloud/peach-menu.git`

Move into the repo and compile:

`cd peach-menu`  
`cargo build`

Run the binary:

`./target/debug/peach-menu`

### Example Usage

**Send button code to menu**

With microservice running, open a second terminal window and use `curl` to call server method:

`curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "press", "params" : {"button_code": 0}, "id":1}' 127.0.0.1:3031`

Server responds with:

`{"jsonrpc":"2.0","result":"success","id":1}`

First terminal output:

```
0  
 to Welcome  
Welcome to PeachCloud!
```

Output includes: 1) button_code, 2) event, 3) new state output

### Resources

This work was made much, much easier by the awesome blog post titled [Pretty State Machine Patterns in Rust](https://hoverbear.org/2016/10/12/rust-state-machine-pattern/) by [hoverbear](https://hoverbear.org/about/). Thanks hoverbear!

### Licensing

AGPL-3.0
