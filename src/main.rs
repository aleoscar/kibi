mod editor;
mod terminal;
mod document;
mod row;

pub use document::Document;
pub use row::Row;
pub use terminal::Terminal;
use editor::Editor;

fn main() {
    //Error now bubbles up, how should it be handled?
    Editor::default().run();
}