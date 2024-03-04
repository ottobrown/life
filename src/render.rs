use std::io::{self};

use crate::rules;
use crate::term;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Renderer {
    /// Coordinate of the top-left corner of the terminal window
    pub camera: (i16, i16),
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
            camera: (0, 0),
            need_rerender: true,
        }
    }

    /// Converts real coordinates to coordinates on the screen using the camera.
    /// Returns `None` if the coordinates are offscreen
    pub fn get_screen_coords(&self, x: i16, y: i16) -> Option<(u16, u16)> {
        let screen_x = self.camera.0 + x;
        let screen_y = self.camera.1 + y;

        if screen_x > 0 && screen_y > 0 {
            return Some((screen_x as u16, screen_y as u16));
        }

        // TODO: also check if coordinates are larger than the width and height of the terminal

        None
    }

    /// Clears the screen and renders each living cell
    pub fn rerender(&mut self, matrix: &rules::Matrix) -> io::Result<()> {
        if self.need_rerender == false {
            let err = io::Error::new(io::ErrorKind::Other, "erroneous rerender");
            return Err(err);
        }

        term::clear_screen()?;
        for (x, y) in matrix.get_alive() {
            if let Some((sx, sy)) = self.get_screen_coords(*x, *y) {
                term::move_cursor(sx, sy)?;
                print!("{}", self.live_char as char);
            }
        }

        term::flush()?;

        self.need_rerender = false;

        Ok(())
    }

    pub fn render_from_changes(&mut self, changes: Vec<rules::Change>) -> io::Result<()> {
        for change in changes {
            if let rules::Change::Birth(x, y) = change {
                if let Some((sx, sy)) = self.get_screen_coords(x, y) {
                    term::move_cursor(sx, sy)?;
                    print!("{}", self.live_char as char);
                }
            }

            if let rules::Change::Death(x, y) = change {
                if let Some((sx, sy)) = self.get_screen_coords(x, y) {
                    term::move_cursor(sx, sy)?;
                    print!(" ");
                }
            }
        }

        term::flush()?;

        Ok(())
    }
}
