use std::path::PathBuf;

use egui::Ui;

pub struct Menu {
    file_picked: Option<PathBuf>,
}

impl Menu {
    pub fn new() -> Self {
        Menu { file_picked: None }
    }

    pub fn show(self: &mut Self, ui: &mut Ui) -> egui::InnerResponse<Option<MenuEvent>> {
        egui::menu::bar(ui, |ui| {
            let mut return_value: Option<MenuEvent> = None;

            ui.menu_button("File", |ui| {
                if ui.button("Open file").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        return_value = Some(MenuEvent::FilePicked(path));
                    }
                    ui.close_menu();
                }

                if ui.button("Open folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        return_value = Some(MenuEvent::FolderPicked(path));
                    }
                    ui.close_menu();
                }

                if ui.button("Settings").clicked() {
                    return_value = Some(MenuEvent::Click(String::from("settings")));
                    ui.close_menu();
                }
            });

            return_value
        })
    }

    pub fn file_picked(self: &Self) -> &Option<PathBuf> {
        let Self { file_picked } = self;
        file_picked
    }
}

pub enum MenuEvent {
    Click(String),
    FilePicked(PathBuf),
    FolderPicked(PathBuf),
}
