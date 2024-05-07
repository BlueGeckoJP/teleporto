use std::path::PathBuf;

use eframe::egui::{self, Button, Layout, Vec2};
use rfd::FileDialog;
use tokio::sync::broadcast;

use crate::webserver::{webserver, Channel};

pub struct Window {
    file_dialog_text: String,
    file_path: PathBuf,
    channel: Channel,
    try_recv: TryRecv,
}

struct TryRecv {
    try_recv: bool,
    content: String,
}

impl Default for Window {
    fn default() -> Self {
        let (tx, rx) = broadcast::channel(16);
        let cloned_channel = Channel {
            tx: tx.clone(),
            rx: tx.subscribe(),
        };
        let channel = Channel { tx, rx };
        tokio::spawn(async move { webserver(cloned_channel).await });
        Self {
            file_dialog_text: String::new(),
            file_path: PathBuf::new(),
            channel: channel,
            try_recv: TryRecv {
                try_recv: false,
                content: String::from(""),
            },
        }
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let recv = self.channel.rx.try_recv();
        if let Ok(content) = recv {
            self.try_recv.try_recv = true;
            self.try_recv.content = content;
        }
        if self.try_recv.try_recv {
            egui::Window::new("Confirm recieve of file").show(ctx, |ui| {
                ui.heading(self.try_recv.content.clone());
            });
        }
        egui::TopBottomPanel::bottom("bottom_panel")
            .min_height(25.0)
            .default_height(25.0)
            .show(ctx, |ui| {});
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading("Drag and Drop HERE");
            });
            ui.with_layout(Layout::bottom_up(egui::Align::Center), |ui| {
                if ui
                    .add_sized(Vec2::new(285.0, 10.0), Button::new("Send"))
                    .clicked()
                {
                    todo!()
                }
                ui.with_layout(Layout::right_to_left(egui::Align::Max), |ui| {
                    if ui.button("..").clicked() {
                        let file_path = FileDialog::new().pick_file().unwrap();
                        self.file_path = file_path.clone();
                        self.file_dialog_text = file_path.to_str().unwrap().to_string();
                    }
                    ui.text_edit_singleline(&mut self.file_dialog_text);
                });
                ui.label("or use the file dialog");
            });
        });
    }
}
