// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod webserver;

#[macro_use]
extern crate log;

use std::sync::Mutex;

use once_cell::sync::OnceCell;
use tauri::Manager;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tokio::sync::broadcast;
use webserver::{webserver, Channel};

static BACK_TO_FRONT_CHANNEL: OnceCell<Mutex<Channel>> = OnceCell::new();
static FRONT_TO_BACK_CHANNEL: OnceCell<Mutex<Channel>> = OnceCell::new();
static IS_LISTENING: OnceCell<Mutex<bool>> = OnceCell::new();

#[tokio::main]
async fn main() {
    IS_LISTENING.set(Mutex::new(false)).unwrap();

    let (b_tx, b_rx) = broadcast::channel(16);
    BACK_TO_FRONT_CHANNEL
        .set(Mutex::new(Channel { tx: b_tx, rx: b_rx }))
        .unwrap();
    let (f_tx, f_rx) = broadcast::channel(16);
    FRONT_TO_BACK_CHANNEL
        .set(Mutex::new(Channel { tx: f_tx, rx: f_rx }))
        .unwrap();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .with_colors(ColoredLevelConfig::default())
                .build(),
        )
        .setup(|app| {
            let b_channel = BACK_TO_FRONT_CHANNEL.get().unwrap();
            let mut b_rx = b_channel.lock().unwrap().tx.subscribe();
            let f_channel = FRONT_TO_BACK_CHANNEL.get().unwrap();
            let f_tx = f_channel.lock().unwrap().tx.clone();

            let app_handle = app.app_handle();

            tokio::spawn(async move {
                loop {
                    let recv = b_rx.try_recv();
                    if let Ok(s) = recv {
                        app_handle.emit_all("back-to-front", s).unwrap();
                        info!("app_handle.emit_all back-to-front");
                    }
                }
            });

            let _ = app.listen_global("front-to-back", move |event| {
                f_tx.send(event.payload().unwrap().to_string()).unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init_web_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn init_web_server() {
    info!("Initializing web server..");

    let b_channel = BACK_TO_FRONT_CHANNEL.get().unwrap();
    let b_tx = b_channel.lock().unwrap().tx.clone();
    let b_rx = b_channel.lock().unwrap().tx.subscribe();
    let f_channel = FRONT_TO_BACK_CHANNEL.get().unwrap();
    let f_tx = f_channel.lock().unwrap().tx.clone();
    let f_rx = f_channel.lock().unwrap().tx.subscribe();

    tokio::spawn(async move {
        webserver(
            Channel { tx: b_tx, rx: b_rx },
            Channel { tx: f_tx, rx: f_rx },
        )
        .await
    });
}
