use std::io::{self};

use crate::Signal;
use crate::term;

pub struct FrameHandler {
    /// Milliseconds between each frame
    pub frame_time: u64,
    pub paused: bool,
}
impl FrameHandler {
    /// Returns whether stdout can be read from
    pub fn advance_frame(&mut self) -> io::Result<bool> {
        term::wait_stdin_ms(self.frame_time)
    }

    pub fn handle_signal(&mut self, s: Signal) {
        match s {
            Signal::Quit => { std::process::exit(0); },
            Signal::Pause => { self.paused = true; }

            _ => unreachable!(),
        }
    }

}
