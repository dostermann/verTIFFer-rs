use eframe::{epi, egui};
use rfd::FileDialog;

#[derive(Default)]
struct Vtff {}

fn src_folder() -> Option<std::path::PathBuf> {
        let src_folder = FileDialog::new()
            .set_directory("/")
            .pick_folder();
        src_folder
    }

    fn dst_folder() -> Option<std::path::PathBuf> {
        let dst_folder = FileDialog::new()
            .set_directory("/")
            .pick_folder();
        dst_folder
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
                ui.add_space(100.0);
                ui.label("Überschrift");
                ui.label("hier kommt was rein");
                if ui.button("Browse...").clicked() {
                    src_folder();
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
