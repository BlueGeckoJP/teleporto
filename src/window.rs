use std::path::PathBuf;

use eframe::egui::{self, Button, Color32, Layout, RichText, Vec2};
use rfd::FileDialog;

pub struct Window {
    recv_mode: bool,
    file_dialog_text: String,
    file_path: PathBuf,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            recv_mode: false,
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
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if ui.button("Change Mode").clicked() {
                        self.recv_mode = !self.recv_mode;
                        info!(
                            "Change mode button clicked!, recv_mode = {}",
                            self.recv_mode
                        );
                    }
                    match self.recv_mode {
                        true => ui.label(RichText::new("Recieve Mode").color(Color32::GREEN)),
                        false => ui.label(RichText::new("Send Mode").color(Color32::YELLOW)),
                    }
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading("Drag and Drop HERE");
            });
            ui.with_layout(Layout::bottom_up(egui::Align::Center), |ui| {
                if ui
                    .add_sized(Vec2::new(285.0, 10.0), Button::new("Send"))
                    .clicked()
                {}
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
