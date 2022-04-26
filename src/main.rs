use eframe::{epi, egui};
use rfd::FileDialog;

#[derive(Default)]
struct Vtff {}

impl epi::App for Vtff {
    fn name(&self) -> &str {
        "VerTIFFer-rs"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
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
            ui.centered_and_justified(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("Überschrift");
                    ui.label("hier kommt was rein");
                });
            });
        });
    }
}

fn main() {
    let app = Vtff::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
