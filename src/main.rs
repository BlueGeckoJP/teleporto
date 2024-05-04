#![deny(elided_lifetimes_in_paths)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sender;
mod webserver;
mod window;

#[macro_use]
extern crate log;

use crate::window::Window;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([300.0, 300.0])
            .with_drag_and_drop(true)
            .with_decorations(true)
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "dnd_p2p",
        options,
        Box::new(|_cc| Box::new(Window::default())),
    )
}
