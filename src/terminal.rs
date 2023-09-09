use std::io::{self, Write};
use crossterm::{terminal::{self, ClearType},
    execute, cursor, 
    };
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
                height: height
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

    pub fn cursor_position(x: u16, y: u16) {
        execute!(io::stdout(), cursor::MoveTo(x, y)).unwrap();
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
}