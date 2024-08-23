use std::path::PathBuf;

use egui::{Ui, Widget};

pub struct Menu {
    file_picked_callback: Box<dyn FnOnce(&mut PathBuf) -> ()>,    
}

impl Widget for Menu {
    fn ui(self: Self, ui: &mut Ui) -> egui::Response {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open file").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.file_picked_callback.into()(path);
                        let new_file = FileData::new(path.to_string_lossy().to_string());
                        files.push(new_file);
                        *selected_index = files.len() - 1;
                    }
                    ui.close_menu();
                }

                if ui.button("Open folder").clicked() {
                    ui.close_menu();
                }

                if ui.button("Settings").clicked() {
                    ui.close_menu();
                }
            });
        }).response
    }
}

impl Menu {
    pub fn new() -> Self {
        let noop = |p| {};

        Menu {
            file_picked_callback: Box::new(noop),
        }
    }

    pub fn on_file_picked(self: &mut Self, callback: impl FnOnce(&mut PathBuf) -> ()) {
        self.file_picked_callback = Box::new(callback);
    }
}