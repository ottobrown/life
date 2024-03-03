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
    need_rerender: bool,
}
impl Renderer {
    pub fn new() -> Self {
        Self {
            live_char: b'x',
            need_rerender: true,
        }
    }

    /// This should be called with the same `Matrix` every time
    pub fn render(&mut self, matrix: &mut rules::Matrix) -> io::Result<()> {
        let _changes = matrix.advance();

        if self.need_rerender {
            term::clear_screen()?;
            for (x, y) in matrix.get_alive() {
                term::move_cursor(*x as u16, *y as u16)?;
                print!("{}", self.live_char as char);
            }

            term::flush()?;
        }

        Ok(())
    }
}
