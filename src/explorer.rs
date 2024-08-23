use egui::{Color32, RichText, Ui};

use crate::FileData;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Explorer {}

impl Explorer {
    pub fn new() -> Self {
        Explorer {}
    }

    pub fn show(self: &mut Self, ui: &mut Ui, files: &Vec<FileData>, selected: &mut usize) {
        for i in 0..files.len() {
            let file = files.get(i);

            if let Some(f) = file {
                let color = if i == *selected {
                    Color32::RED
                } else {
                    Color32::WHITE
                };

                if ui
                    .button(RichText::new(f.get_name()).color(color))
                    .clicked()
                {
                    *selected = i
                }
            }
        }
    }
}
