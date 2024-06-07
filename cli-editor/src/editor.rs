mod terminal;
use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers,
};
use terminal::Terminal;
pub struct Editor {
    should_quit: bool,
    current_position: (u16, u16),
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            current_position: (0, 0),
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen().unwrap();

            if self.should_quit {
                break;
            }

            let event = read()?;
            self.handle_event(&event);
        }

        Ok(())
    }

    fn handle_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => {
                    // let x = self.current_position.0 + 1;
                    // let y = self.current_position.1;

                    // self.current_position = (x, y);
                    // println!("{x:?} {y:?}");
                }
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen();
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(self.current_position.0, self.current_position.1).unwrap();
        }
        Ok(())
    }

    pub fn draw_rows() -> Result<(), std::io::Error> {
        let height = Terminal::size()?.1;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }
}
