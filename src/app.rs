use std::fs;

use crate::{FileData, Settings};

pub struct App {
    files: Vec<FileData>,
    selected_index: usize,
    settings: Settings,
}

impl Default for App {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            selected_index: 0,
            settings: Settings::new(),
        }
    }
}

impl App {
    // Before first frame
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // App customization (see `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`)

        Default::default()
    }
}

impl eframe::App for App {
    /// Save state in storage
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            files,
            selected_index,
            settings,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if let Some(response) = crate::Menu::new().show(ui).inner {
                match response {
                    crate::menu::MenuEvent::Click(ev) => {
                        match ev.as_str() {
                            "settings" => println!("Settings"),
                            _ => {}
                        }
                    }
                    crate::menu::MenuEvent::FilePicked(path) => {
                        let new_file = FileData::from_path(path);
                        files.push(new_file);
                        *selected_index = files.len() - 1;
                    }
                    crate::menu::MenuEvent::FolderPicked(path) => {
                        let folder_files = fs::read_dir(path);

                        if let Ok(iter) = folder_files {
                            for file in iter {
                                if let Ok(f) = file {
                                    if f.metadata().is_ok_and(|m| m.is_file()) {
                                        let new_file = FileData::from_path(f.path());
                                        files.push(new_file);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        egui::SidePanel::left("explorer")
            .resizable(true)
            .show_animated(ctx, settings.show_explorer, |ui| {
                crate::Explorer::new().show(ui, files, selected_index)
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            crate::CodeEditor::show(ui, files.get_mut(*selected_index));
        });
    }
}
