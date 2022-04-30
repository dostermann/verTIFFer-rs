use eframe::{epi, egui};
use std::path::PathBuf;
use rfd::FileDialog;

#[derive(Default)]
struct Vtff {
    picked_src_path: Option<PathBuf>,
    picked_src_path_display: Option<String>,
    picked_dst_path: Option<PathBuf>,
    picked_dst_path_display: Option<String>,
}

impl epi::App for Vtff {
    fn name(&self) -> &str {
        "verTIFFer-rs"
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
                ui.add_space(32.0);
                ui.horizontal_wrapped(|ui| {
                    if let Some(picked_src_path_display) = &self.picked_src_path_display {
                        ui.label(picked_src_path_display);
                    }
                    if ui.button("Quellverzeichnis wählen...").clicked() {
                        if let Some(path) = FileDialog::new().set_directory("/").pick_folder() {
                            self.picked_src_path_display = Some(path.display().to_string().to_owned());
                            self.picked_src_path = Some(path);
                        }
                    }
                });
            });
        });
    }
}

fn main() {
    let app = Vtff::default();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(512.0, 256.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
