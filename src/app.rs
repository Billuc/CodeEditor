use crate::FileData;

pub struct App {
    files: Vec<FileData>,
    selected_index: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            selected_index: 0,
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
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open file").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let new_file = FileData::new(path.to_string_lossy().to_string());
                            files.push(new_file);
                            *selected_index = files.len() - 1;
                        }
                    }
                });
            })
        });

        egui::SidePanel::left("explorer").show(ctx, |ui| {
            crate::Explorer::new().show(ui, files, selected_index)
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            crate::CodeEditor::show(ui, files.get_mut(*selected_index));
        });
    }
}
