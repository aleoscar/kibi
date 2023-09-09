use std::io::{self};
use crate::Terminal;
use crossterm::{self, execute, cursor, terminal::{ClearType, self}, event::{
    self,
    Event::Key,
    KeyCode::Char,
    KeyEventKind,
    KeyEvent,
    KeyModifiers
}};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("failed to initalize terminal"),
        }
    }

    pub fn run(&mut self) {
        terminal::enable_raw_mode().ok();

        loop {

            self.refresh_screen();

            if self.should_quit {break;}

            let event = event::read().unwrap();
            if let Key(key_event) = event {
                if let KeyEventKind::Press = key_event.kind {
                    self.handle_key_press(&key_event);
                }
            }
        }
    }
        
    fn handle_key_press(&mut self,key_event: &KeyEvent) {
        match  key_event {
            KeyEvent {modifiers: KeyModifiers::CONTROL, code: Char('q'), ..} => {
                self.should_quit = true;
            }

            KeyEvent {code: Char(c), ..} => {
                println!("{c}");
            }

            KeyEvent {code, ..} => {
                println!("{code:?}");
            }

            //_ => false  
        }
    }

    fn refresh_screen(&self) {

        //should it move to (1, 1) or (0, 0)?
        execute!(io::stdout(), cursor::MoveTo(1, 1)).unwrap();
        execute!(io::stdout(), terminal::Clear(ClearType::All)).unwrap();
        if self.should_quit {
            println!("goodbye!")
        } else {
            self.draw_rows();
        }
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r")
        }
    }
}