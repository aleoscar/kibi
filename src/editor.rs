use crate::Terminal;
use crate::Document;
use crate::Row;
use std::env;
use std::io;
use std::time::{Instant, Duration};
use crossterm::style::Color;
use crossterm::{self, execute, style, cursor, terminal, event::{
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
    status_message: StatusMessage,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self { 
            text: message,
            time: Instant::now(),
        }
    }
}

impl Editor {
    pub fn default() -> Self {
        let mut initial_status = String::from("HELP: Ctrl-S = save, Ctrl-Q = quit");
        let args: Vec<String> = env::args().collect();
        let document = if let Some(filename) = args.get(1) {
            let doc = Document::open(&filename);
            if let Ok(doc) = doc {
                doc
            } else {
                initial_status = format!("ERR: Could not open file: {}", filename);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("failed to initalize terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
            status_message: StatusMessage::from(initial_status),
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        self.refresh_screen()?;
        loop {
            let event = event::read().unwrap();
            if let Key(key_event) = event {
                if let KeyEventKind::Press = key_event.kind {
                    self.handle_key_press(&key_event)?;
                }
            } else {continue;}
            
            //will now refresh an extra time before quitting
            self.refresh_screen()?;

            if self.should_quit {
                if let Err(e) = terminal::disable_raw_mode() {
                    eprintln!("failed to disable raw mode");
                    return Err(e);
                }
                break Ok(())
            }
        }
    }
        
    fn refresh_screen(&self) -> Result<(), std::io::Error> {

        //should it move to (1, 1) or (0, 0)? 
        //I think (0, 0)
        execute!(io::stdout(), cursor::Hide).unwrap();

        //TODO: fix clearing flickering, probably only clear when quitting
        //Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
        } else {
            //TODO add error handling
            self.draw_rows();
            self.draw_status_bar()?;
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            })
        }

        execute!(io::stdout(), cursor::Show).unwrap();
        Terminal::flush()
    }

    fn scroll(&mut self) {
        let Position {x, y} = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let offset = &mut self.offset;
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
        let end = self.offset.x.saturating_add(width);
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    #[allow(clippy::integer_division, clippy::arithmetic_side_effects)]
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(
                self.offset.y.saturating_add(terminal_row as usize)
            ) {
                self.draw_row(row);
            } else if self.document.is_empty() && 
                terminal_row == height / 3 {
                    self.draw_welcome_message()
            } else {
                println!("\r")
            }
        }
    }

    fn draw_status_bar(&self) -> Result<(), std::io::Error> {
        let mut status;
        let width = self.terminal.size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };
        let mut filename;
        if let Some(name) = &self.document.filename {
            filename = name.clone();
            //bad error handling
            filename = filename.split("\\").last().unwrap().to_string();
            filename.truncate(20);
        } else {
            filename = "[No Name]".to_string();
        }
        status = format!(" {} - {} lines{}",
            filename,
            self.document.len(),
            modified_indicator,
        );
        
        let line_indicator = format!(
            "{}/{} ",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );

        let len = status.len() + line_indicator.len();
        status.push_str(&" ".repeat(width.saturating_sub(len)));

        status = format!("{}{}", status, line_indicator);
        status.truncate(width);

        Terminal::set_fg_color(Color::Black)?;
        Terminal::set_bg_color(Color::Green)?;
        println!("{status}\r");
        execute!(io::stdout(), style::ResetColor)?;
        Ok(())
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{text}");
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Kibi text editor -- version {VERSION}");
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        #[allow(clippy::arithmetic_side_effects, clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!(" {}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);

    }

    fn save(&mut self) {
        if self.document.filename.is_none() {
            let new_name = self.prompt("Save as: ").unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from(
                    "Save aborted".to_string()
                );
                return;
            }

            self.document.filename = new_name;
        }

        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from(
                "File saved succesfully".to_string()
            )
        } else {
            self.status_message = StatusMessage::from(
                "Error writing file".to_string()
            )
        }

    }

    fn prompt(&mut self, message: &str) -> Result<Option<String>, std::io::Error> {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(
                format!("{}{}", message, result)
            );

            self.refresh_screen()?;

            let event = event::read()?;
            if let Key(key_event) = event {
                if let KeyEventKind::Press = key_event.kind {
                    match key_event {
                        KeyEvent{code: Enter, ..} => {
                            break;
                        }

                        KeyEvent {code: Char(c), ..} => {
                            result.push(c);
                        }

                        KeyEvent {code: Backspace, ..} => {
                            result.truncate(result.len().saturating_sub(1))
                        }

                        KeyEvent{code: Esc, ..} => {
                            result.truncate(0);
                            break;
                        }

                        _ => {
                            continue;
                        }
                    }
                }
            } else {continue;}
        }

        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None)
        }
        Ok(Some(result))
    }

    fn delete_word(&mut self) {
        let dist = self.jump_to_word_start();
        for _i in 0..dist {
            self.document.delete(&self.cursor_position);
        }
    }

    fn jump_to_next_word(&mut self) -> usize {
        let Position {x, y} = self.cursor_position;
        if self.document.row(y).is_some() {
            let dist = self.document.row(y).unwrap().distance_to_end(x);
            for _i in 0..dist {
                self.move_cursor(Right);
            }
            return dist;
        }
        return 0;
    }

    fn jump_to_word_start(&mut self) -> usize {
        let Position {x, y} = self.cursor_position;
        if self.document.row(y).is_some() {
            let dist = self.document.row(y).unwrap().distance_to_start(x);
            for _i in 0..dist {
                self.move_cursor(Left);
            }
            return dist;
        }
        return 0;
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
            Up | Char('k') => y = y.saturating_sub(1),

            Down | Char('j') => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Left | Char('h') => {
                if x > 0 {
                    x -= 1
                //goes up to previous line if cursor is at start of line
                } else if y > 0 {
                    y -=1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }

            Right | Char('l') => {
                if x < width {
                    x += 1;
                //goes to next line if cursor is at end of line
                } else if y < height {
                    x = 0;
                    y += 1;
                }
            }

            PageUp => {
                y = if y > terminal_height {
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            }

            PageDown => {
                y = if y.saturating_add(terminal_height) >= height {
                    height
                } else {
                    y.saturating_add(terminal_height)
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

    pub fn handle_key_press(&mut self, key_event: &KeyEvent) -> Result<(), std::io::Error> {
        match  key_event {
            KeyEvent {modifiers: KeyModifiers::CONTROL, code: Char('q'), ..} => {
                if self.document.is_dirty() {
                    let result = self.prompt("Are you sure you want to quit? Document has been modified. \'Yes\' to continue, \'Save\' to save and quit: ").unwrap_or(None);
                    if result.is_some() {
                        let answer = result.unwrap();
                        if answer.trim().eq_ignore_ascii_case("save") {
                            self.save();
                            self.should_quit = true;
                        } else {
                            self.should_quit = answer.trim().eq_ignore_ascii_case("yes")
                        }
                    };
                } else {
                    self.should_quit = true;
                }
            }

            KeyEvent {modifiers: KeyModifiers::CONTROL, code: Char('s'), ..} => self.save(),
            
            KeyEvent {modifiers: KeyModifiers::CONTROL, code: Left, ..} => _ = self.jump_to_word_start(), 
            KeyEvent {modifiers: KeyModifiers::CONTROL, code: Right, ..} => _ = self.jump_to_next_word(),

            //also Ctrl + D for deleting entire row
            KeyEvent{code: Up | Down | Left | Right | PageDown | PageUp | Home| End, ..} 
                | KeyEvent{modifiers: KeyModifiers::CONTROL, code: Char('h')
                | Char('j')
                | Char('k')
                | Char('l'), ..}  => {
                let KeyEvent {code, ..} = key_event;
                self.move_cursor(*code)
            }

            KeyEvent {modifiers: KeyModifiers::CONTROL, code: Char('b'), ..} => self.delete_word(),

            KeyEvent {code: Delete, ..} => {
                self.document.delete(&self.cursor_position);
            }

            KeyEvent {code: Backspace, ..} => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Left);
                    self.document.delete(&self.cursor_position);
                }
            }

            KeyEvent {code: Enter, ..} => {
                self.document.new_line(&self.cursor_position);
                self.move_cursor(Right)
            }

            KeyEvent {code: Tab, ..} => {
                for _ in 0..4 {
                    self.document.insert(&self.cursor_position, ' ');
                    self.move_cursor(Right)
                }
            } 

            KeyEvent {code: Char(c), ..} => {
                self.document.insert(&self.cursor_position, *c);
                self.move_cursor(Right)
            }

            /*
            KeyEvent {code, ..} => {
                println!("{code:?}");
            } */

            _ => ()  
        }
        Ok(())
    }
}
