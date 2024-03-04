use std::io::{self};

use crate::rules;
use crate::term;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Renderer {
    /// Coordinate of the top-left corner of the terminal window
    /* camera: (i16 i16),*/
    /// The character that represents a living cell
    live_char: u8,
    /// If `true`, the screen will be cleared and each cell will be re-rendered
    /// on the next frame.
    pub need_rerender: bool,
}
impl Renderer {
    pub fn new() -> Self {
        Self {
            live_char: b'x',
            need_rerender: true,
        }
    }

    /// Clears the screen and renders each living cell
    pub fn rerender(&mut self, matrix: &rules::Matrix) -> io::Result<()> {
        if self.need_rerender == false {
            let err = io::Error::new(io::ErrorKind::Other, "erroneous rerender");
            return Err(err);
        }

        term::clear_screen()?;
        for (x, y) in matrix.get_alive() {
            term::move_cursor(*x as u16, *y as u16)?;
            print!("{}", self.live_char as char);
        }

        term::flush()?;

        self.need_rerender = false;

        Ok(())
    }

    pub fn render_from_changes(&mut self, changes: Vec<rules::Change>) -> io::Result<()> {
        for change in changes {
            if let rules::Change::Birth(x, y) = change {
                term::move_cursor(x as u16, y as u16)?;
                print!("{}", self.live_char as char);
            }
            if let rules::Change::Death(x, y) = change {
                term::move_cursor(x as u16, y as u16)?;
                print!(" ");
            }
        }

        term::flush()?;

        Ok(())
    }
}
