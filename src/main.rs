use log::error;
use std::process;

#[tokio::main]
async fn main() {
    // initialize the logger
    env_logger::init();

    // handle errors returned from `run`
    if let Err(e) = peach_menu::run().await {
        error!("Application error: {:?}", e);
        process::exit(1);
    }
}
