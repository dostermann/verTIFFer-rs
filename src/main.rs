#![windows_subsystem = "windows"]

use eframe::{egui, epi};
use glob::glob;
use rfd::FileDialog;
use std::process::Command;

#[derive(Default)]
struct Vtff {
    src_path: String,
    dst_path: String,
}

impl epi::App for Vtff {
    fn name(&self) -> &str {
        "verTIFFer-rs 1.0.0"
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
                ui.add_space(48.0);
                ui.horizontal_wrapped(|ui| {
                    ui.add_sized(
                        [480.0, 16.0],
                        egui::TextEdit::singleline(&mut self.src_path).interactive(false),
                    );

                    if ui.button("Quellverzeichnis...").clicked() {
                        if let Some(path) = FileDialog::new().pick_folder() {
                            self.src_path = path.display().to_string().to_owned();
                        }
                    }
                });

                ui.add_space(64.0);

                ui.horizontal_wrapped(|ui| {
                    ui.add_sized(
                        [480.0, 16.0],
                        egui::TextEdit::singleline(&mut self.dst_path).interactive(false),
                    );

                    if ui.button("Zielverzeichnis...").clicked() {
                        if let Some(path) = FileDialog::new().pick_folder() {
                            self.dst_path = path.display().to_string().to_owned();
                        }
                    }
                });

                ui.add_space(96.0);

                if ui.button("verTIFF mich!").clicked() {
                    run_bttn(&self.src_path, &self.dst_path)
                }
            });
        });
    }
}

fn run_bttn(src_path: &str, dst_path: &str) {
    let dir_delimiter = if cfg!(windows) {
        "\\"
    } else if cfg!(unix) {
        "/"
    } else {
        "/"
    };

    for entry in glob(format!("{}{}*.pdf", src_path, dir_delimiter).as_str()).unwrap() {
        match entry {
            Ok(path) => {
                let src_path_string = path.display().to_string();
                let filenames_with_ext: Vec<&str> = src_path_string.split(dir_delimiter).collect();
                let filename_pdf = *filenames_with_ext.last().unwrap();
                let savefile_wo_ext: Vec<&str> = filename_pdf.split(".").collect();
                let savefile_tiff =
                    format!("{}{}{}.tiff", dst_path, dir_delimiter, savefile_wo_ext[0]);

                if cfg!(windows) {
                    Command::new(".\\ImageMagick-7.1.0\\magick.exe")
                        .current_dir(".\\ImageMagick-7.1.0\\")
                        .arg("-density")
                        .arg("300")
                        .arg(src_path_string)
                        .arg("-background")
                        .arg("white")
                        .arg("-alpha")
                        .arg("background")
                        .arg("-alpha")
                        .arg("off")
                        .arg("-colorspace")
                        .arg("gray")
                        .arg("-compress")
                        .arg("zip")
                        .arg(savefile_tiff)
                        .status()
                        .expect("failed to execute process!");
                } else {
                    Command::new("magick")
                        .arg("-density")
                        .arg("300")
                        .arg(src_path_string)
                        .arg("-background")
                        .arg("white")
                        .arg("-alpha")
                        .arg("background")
                        .arg("-alpha")
                        .arg("off")
                        .arg("-colorspace")
                        .arg("gray")
                        .arg("-compress")
                        .arg("zip")
                        .arg(savefile_tiff)
                        .status()
                        .expect("failed to execute process!");
                }
            }
            Err(e) => println!("{}", e),
        }
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
