use eframe::{epi, egui::{self, RichText}};
use std::path::PathBuf;
use rfd::FileDialog;

#[derive(Default)]
struct Vtff {
    src_path: Option<PathBuf>,
    src_path_display: String,
    dst_path: Option<PathBuf>,
    dst_path_display: String,
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
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Center), 
                |ui| {

                    ui.add_space(48.0);
                    ui.horizontal_wrapped(|ui| {

                        ui.add_sized(
                            [480.0, 16.0],
                            egui::Label::new(
                                RichText::new(&self.src_path_display)
                                .monospace())
                        );

                        if ui.button("Quellverzeichnis...").clicked() {
                            if let Some(path) = FileDialog::new()
                                .set_directory("/")
                                .pick_folder() {
                                    self.src_path_display = path.display().to_string().to_owned();
                                    self.src_path = Some(path);
                            }
                        }
                    });

                    ui.add_space(64.0);
                    ui.horizontal_wrapped(|ui| {

                        ui.add_sized(
                            [480.0, 16.0],
                            egui::Label::new(
                                RichText::new(&self.dst_path_display)
                                .monospace())
                        );

                        if ui.button("Zielverzeichnis...").clicked() {
                            if let Some(path) = FileDialog::new()
                                .set_directory("/")
                                .pick_folder() {
                                    self.dst_path_display = path.display().to_string().to_owned();
                                    self.dst_path = Some(path);
                            }
                        }
                    });

                    ui.add_space(96.0);

                    if ui.button("verTIFF mich!").clicked() {
                        
                    }
            });
        });
    }
}

fn main() {
    let app = Vtff::default();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 350.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
