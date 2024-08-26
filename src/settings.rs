use egui::style;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Settings {
    pub small_size: f32,
    pub body_size: f32,
    pub monospace_size: f32,
    pub button_size: f32,
    pub heading_size: f32,

    pub show_explorer: bool,
}

impl Default for Settings {
    fn default() -> Self {
        let defaults = style::default_text_styles();

        Settings {
            small_size: defaults.get(&style::TextStyle::Small).unwrap().size,
            body_size: defaults.get(&style::TextStyle::Body).unwrap().size,
            monospace_size: defaults.get(&style::TextStyle::Monospace).unwrap().size,
            button_size: defaults.get(&style::TextStyle::Button).unwrap().size,
            heading_size: defaults.get(&style::TextStyle::Heading).unwrap().size,

            show_explorer: true,
        }
    }
}

impl Clone for Settings {
    fn clone(&self) -> Self {
        Settings {
            small_size: self.small_size,
            body_size: self.body_size,
            monospace_size: self.monospace_size,
            button_size: self.button_size,
            heading_size: self.heading_size,

            show_explorer: self.show_explorer,
        }
    }
}

impl Copy for Settings {}

impl Settings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn copy(other: &Settings) -> Self {
        other.clone()
    }
}
