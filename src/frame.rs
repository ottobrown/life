use std::io::{self, Read};

use crate::render::Renderer;
use crate::rules::Matrix;
use crate::term;

pub struct FrameHandler {
    /// Milliseconds between each frame
    pub frame_time: u64,
    pub paused: bool,
}
impl FrameHandler {
    pub fn advance_frame(&mut self, matrix: &mut Matrix, rend: &mut Renderer) -> io::Result<()> {
        if term::wait_stdin_ms(self.frame_time)? {
            let mut byte: [u8; 1] = [0];
            if std::io::stdin().read(&mut byte)? > 0 {
                if byte[0] == b'q' {
                    std::process::exit(0);
                }
                if byte[0] == b' ' {
                    self.paused = !self.paused;
                }
            }
        }

        if self.paused {
            return Ok(());
        }

        let changes = matrix.advance();

        if rend.need_rerender {
            rend.rerender(&matrix)?;
        } else {
            rend.render_from_changes(changes)?;
        }

        Ok(())
    }
}
