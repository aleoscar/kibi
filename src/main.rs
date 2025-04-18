#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]
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
    Editor::default().run().unwrap_or_else(|error| {
        panic!("Err called from main: {:?}", error)
    })
}