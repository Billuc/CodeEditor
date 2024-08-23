use egui::style;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Settings {
    small_size: f32,
    body_size: f32,
    monospace_size: f32,
    button_size: f32,
    heading_size: f32,
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
            heading_size: self.heading_size
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
