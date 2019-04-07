extern crate peach_menu;

use ::std::process;

fn main() {
    // handle errors returned from `run`
    if let Err(e) = peach_menu::run() {
        println!("Application error: {:?}", e);
        process::exit(1);
    }
}
