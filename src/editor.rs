use crate::Terminal;
use crate::Document;
use crate::Row;
use std::env;
use std::io;
use crossterm::{self, execute, cursor, event::{
    self,
    Event::Key,
    KeyCode::{self, *},
    KeyEventKind, KeyEvent, KeyModifiers,
}};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let filename = &args[1];
            Document::open(filename).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("failed to initalize terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document: document,
        }
    }

    pub fn run(&mut self) {
        loop {
            let event = event::read().unwrap();
            if let Key(key_event) = event {
                if let KeyEventKind::Press = key_event.kind {
                    self.handle_key_press(&key_event);
                }
            } else {continue;}
            
            //will now refresh an extra time before quitting
            self.refresh_screen();

            if self.should_quit {break;}
        }
    }
        
    fn refresh_screen(&self) {

        //should it move to (1, 1) or (0, 0)? 
        //I think (0, 0)
        execute!(io::stdout(), cursor::Hide).unwrap();

        //TODO: fix clearing flickering, probably only clear when quitting
        //Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("goodbye!")
        } else {
            self.draw_rows();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            })
        }

        execute!(io::stdout(), cursor::Show).unwrap();
        //bad error handling
        Terminal::flush().ok();
    }

    fn scroll(&mut self) {
        let Position {x, y} = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(
                terminal_row as usize + self.offset.y
            ) {
                self.draw_row(row);
            } else if self.document.is_empty() && 
                terminal_row == height / 3 {
                    self.draw_welcome_message()
            } else {
                println!("~\r")
            }
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Kibi text edior -- version {VERSION}");
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);

    }

    fn move_cursor(&mut self, code: KeyCode) {
        let terminal_height = self.terminal.size().height as usize;
        let Position {mut x, mut y} = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        
        match code {
            Up => y = y.saturating_sub(1),

            Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Left => x = x.saturating_sub(1),

            Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }

            PageUp => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                }
            }

            PageDown => {
                y = if y.saturating_add(terminal_height) >= height {
                    height
                } else {
                    y + terminal_height
                }
            }

            Home => x = 0,

            End => x = width,

            _ => ()
        }

        //sets cursor position to end of line if cursor is further to the right
        //prevents going to eol on long line then moving cursor up/down
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position {x, y};
        self.scroll()
    }

    pub fn handle_key_press(&mut self, key_event: &KeyEvent) {
        match  key_event {
            KeyEvent {modifiers: KeyModifiers::CONTROL, code: Char('q'), ..} => {
                self.should_quit = true;
            }

            //TODO: add Ctrl + Vim keybinds to move cursor
            KeyEvent{code: Up, ..}       |  
            KeyEvent{code: Down, ..}     |
            KeyEvent{code: Left, ..}     |
            KeyEvent{code: Right, ..}    |
            KeyEvent{code: PageDown, ..} |
            KeyEvent{code: PageUp, ..}   |
            KeyEvent{code: Home, ..}     |
            KeyEvent{code: End, ..} => {
                let KeyEvent {code, ..} = key_event;
                self.move_cursor(*code)
            }

            /* KeyEvent {code: Char(c), ..} => {
                println!("{c}");
            }

            KeyEvent {code, ..} => {
                println!("{code:?}");
            } */

            _ => ()  
        }
    }
}