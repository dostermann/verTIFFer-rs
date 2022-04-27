use eframe::{epi, egui};
use std::path::PathBuf;
use rfd::FileDialog;

#[derive(Default)]
struct Vtff {}

fn src_folder() -> PathBuf {
        let src_folder = FileDialog::new()
            .set_directory("/")
            .pick_folder();
        match src_folder {
            None => PathBuf::new(),
            Some(path) => path,
        }
    }

fn dst_folder() -> PathBuf {
    let dst_folder = FileDialog::new()
        .set_directory("/")
        .pick_folder();
        match dst_folder {
            None => PathBuf::new(),
            Some(path) => path,
        }
}

impl epi::App for Vtff {
    fn name(&self) -> &str {
        "VerTIFFer-rs"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // GUI
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Datei", |ui| {
                    if ui.button("Beenden...").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(150.0);
                ui.heading("Überschrift");
                ui.label("hier kommt was rein");
                if ui.button("Quellverzeichnis wählen...").clicked() {
                    let src_path = src_folder();
                }
            })
        });
    }
}

fn main() {
    let app = Vtff::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
