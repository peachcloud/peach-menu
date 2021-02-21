use log::error;
use std::process;

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = peach_menu::run().await {
        error!("Application error: {:?}", e);
        process::exit(1);
    }
}
