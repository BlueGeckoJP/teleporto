use std::path::PathBuf;

use eframe::egui::{self, Button, Layout, Vec2};
use rfd::FileDialog;

use crate::webserver::webserver;

pub struct Window {
    file_dialog_text: String,
    file_path: PathBuf,
}

impl Default for Window {
    fn default() -> Self {
        tokio::spawn(async { webserver().await });
        Self {
            file_dialog_text: String::new(),
            file_path: PathBuf::new(),
        }
    }
}

impl eframe::App for Window {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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
