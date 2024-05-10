// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod webserver;

#[macro_use]
extern crate log;

use std::{sync::Mutex, thread, time::Duration};

use once_cell::sync::OnceCell;
use tauri::{AppHandle, Event, Manager};
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tokio::sync::broadcast;
use webserver::{webserver, Channel};

static CHANNEL: OnceCell<Mutex<Channel>> = OnceCell::new();
static IS_LISTENING: OnceCell<Mutex<bool>> = OnceCell::new();

#[tokio::main]
async fn main() {
    IS_LISTENING.set(Mutex::new(false)).unwrap();

    let (tx, rx) = broadcast::channel(16);
    let channel = Channel { tx, rx };
    CHANNEL.set(Mutex::new(channel)).unwrap();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .with_colors(ColoredLevelConfig::default())
                .build(),
        )
        /*.setup(|app| {
            let app_handle = app.app_handle();
            tokio::spawn(async move { back_to_front_handler(app_handle) });
            Ok(())
        })
        .setup(|app| {
            let _ = app.listen_global("front-to-back", |event| front_to_back_handler(event));
            Ok(())
        })*/
        .invoke_handler(tauri::generate_handler![init_web_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn init_web_server() {
    info!("Initializing web server..");

    let ch = CHANNEL.get().unwrap();
    let tx = ch.lock().unwrap().tx.clone();
    let rx = ch.lock().unwrap().tx.subscribe();
    let channel = Channel { tx, rx };

    tokio::spawn(async move { webserver(channel).await });
}

/*
async fn back_to_front_handler(app_handle: AppHandle) {
    let channel = CHANNEL.get().unwrap();
    let mut rx = channel.lock().unwrap().tx.subscribe();

    loop {
        let ip = loop {
            let recv = rx.try_recv();
            info!("back_to_front_handler");
            if let Ok(ip) = recv {
                break ip;
            }
            thread::sleep(Duration::from_secs(1));
        };
        app_handle.emit_all("back-to-front", ip).unwrap();
        {
            let mut is_listening = IS_LISTENING.get().unwrap().lock().unwrap();
            *is_listening = true;
        }
        thread::sleep(Duration::from_secs(1));
    }
}

fn front_to_back_handler(event: Event) {
    info!("front_to_back_handler");
    if event.payload() == Some("NU") {
        let mut is_listening = IS_LISTENING.get().unwrap().lock().unwrap();
        if *is_listening {
            let channel = CHANNEL.get().unwrap();
            let tx = channel.lock().unwrap().tx.clone();
            tx.send(String::new()).unwrap();
            *is_listening = false;
        }
    }
}
*/
