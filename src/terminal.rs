use std::io::{self, Write};
use crate::editor::Position;
use crossterm::{terminal::{self, ClearType},
    execute, cursor, style::{self, Color,}};

pub struct Size {
    pub width: u16,
    pub height: u16
}

pub struct Terminal {
    size: Size,
    _stdout: io::Result<()>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let width = terminal::size()?.0;
        let height = terminal::size()?.1;

        Ok(
            Self {size: Size {
                width: width,
                height: height.saturating_sub(2),
            },
            //bad error handling
            _stdout: terminal::enable_raw_mode(),
        }
        )
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        //bad error handling
        execute!(io::stdout(), terminal::Clear(ClearType::All)).unwrap();
    }

    pub fn clear_current_line() {
        execute!(io::stdout(), terminal::Clear(ClearType::CurrentLine)).unwrap();
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(pos: &Position) {
        let Position{x, y} = pos;
        let x = *x as u16;
        let y = *y as u16;
        execute!(io::stdout(), cursor::MoveTo(x, y)).unwrap();
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn reset_color() -> Result<(), std::io::Error> {
        execute!(io::stdout(), style::ResetColor)
    }

    pub fn set_bg_color(color: Color) -> Result<(), std::io::Error> {
        execute!(io::stdout(), style::SetBackgroundColor(color))
    }

    pub fn set_fg_color(color: Color) -> Result<(), std::io::Error> {
        execute!(io::stdout(), style::SetForegroundColor(color))
    }
}