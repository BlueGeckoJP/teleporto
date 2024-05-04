#![deny(elided_lifetimes_in_paths)]

mod sender;
mod webserver;

#[macro_use]
extern crate log;

use std::thread;

use webserver::webserver;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("trace"));

    let ws = thread::spawn(move || webserver());
    ws.join().unwrap().await;
}
