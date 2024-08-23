use core::f32;

use egui::Ui;

use crate::FileData;

pub struct CodeEditor {}

impl CodeEditor {
    pub fn show(ui: &mut Ui, filedata: Option<&mut FileData>) {
        let mut language = String::from("rs");
        let mut content: &mut String = &mut "// Welcome to my Code Editor".to_string();

        if let Some(file) = filedata {
            language = file.get_language().to_string();
            content = file.borrow_content();
        }
        let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
        
        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job = egui_extras::syntax_highlighting::highlight(
                ui.ctx(),
                &theme,
                string,
                &language,
            );
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(content)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_rows(20)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            )
        });
    }
}
