#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::App;

mod code_editor;
pub use code_editor::CodeEditor;

mod explorer;
pub use explorer::Explorer;

mod file_data;
pub use file_data::FileData;