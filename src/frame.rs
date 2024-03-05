use std::io::{self};

use crate::term;
use crate::Signal;

pub struct FrameHandler {
    /// Milliseconds between each frame
    pub frame_time: u64,
    pub paused: bool,
}
impl FrameHandler {
    pub fn new(frame_ms: u64) -> Self {
        Self {
            frame_time: frame_ms,
            paused: false,
        }
    }

    /// Returns when `stdin` can be read from (`true`) or the timeout is reached (`false`)
    pub fn advance_frame(&mut self) -> io::Result<bool> {
        term::wait_stdin_ms(self.frame_time)
    }

    pub fn handle_signal(&mut self, s: Signal) {
        match s {
            Signal::Quit => {
                std::process::exit(0);
            }
            Signal::Pause => {
                self.paused = true;
            }

            _ => unreachable!(),
        }
    }
}
