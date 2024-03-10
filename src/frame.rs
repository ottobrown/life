use std::io::{self, Read};

use std::time::{Duration, Instant};

use crate::term;
use crate::Signal;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FrameMode {
    Run,
    Paused,
    FrameAdvance,
}

/// Info collected after advancing frames
#[derive(Clone, Copy)]
pub struct FrameInfo {
    /// The byte of input given to `stdin` on this frame
    pub input: Option<u8>,
    /// Should the state of the matrix be advanced on this frame?
    pub advance_matrix: bool,
}

pub struct FrameHandler {
    /// Milliseconds between each frame
    frame_time: u64,
    mode: FrameMode,
}
impl FrameHandler {
    pub fn new(frame_ms: u64) -> Self {
        Self {
            frame_time: frame_ms,
            mode: FrameMode::Run,
        }
    }

    /// Returns after `self.frame_time` milliseconds.
    /// Returns the first byte given to `stdin` in this time, or `0` by default
    pub fn advance_frame(&mut self) -> io::Result<FrameInfo> {
        let start_frame = Instant::now();

        let mut info = FrameInfo {
            input: None,
            advance_matrix: true,
        };

        let mut input: [u8; 1] = [0];
        // Assumes more than 1000 bytes will not be given to `stdin` in a frame
        let mut ignore: [u8; 1000] = [0; 1000];

        let can_read = term::wait_stdin_ms(self.frame_time)?;
        if can_read && io::stdin().read(&mut input)? > 0 {
            info.input = Some(input[0]);
        }

        info.advance_matrix = match self.mode {
            FrameMode::Run => true,
            FrameMode::Paused => false,
            FrameMode::FrameAdvance => info.input.is_some(),
        };

        // Delay until end of frame
        let total_duration = Duration::from_millis(self.frame_time);
        while start_frame.elapsed() < total_duration {
            if let Some(d) = total_duration.checked_sub(start_frame.elapsed()) {
                let left_ms = d.as_millis();

                // Assumes there are not more than `2^64 - 1` milliseconds left
                let can_read = term::wait_stdin_ms(left_ms as u64)?;
                if can_read {
                    let _ = io::stdin().read(&mut ignore)?;
                }
            }
        }

        Ok(info)
    }

    pub fn handle_signal(&mut self, s: Signal) {
        match s {
            Signal::Quit => {
                std::process::exit(0);
            }
            Signal::Pause => {
                if self.mode == FrameMode::Paused {
                    self.mode = FrameMode::Run;
                } else {
                    self.mode = FrameMode::Paused;
                }
            }
            Signal::FrameAdvance => {
                if self.mode == FrameMode::FrameAdvance {
                    self.mode = FrameMode::Run;
                } else {
                    self.mode = FrameMode::FrameAdvance;
                }
            }

            _ => unreachable!(),
        }
    }
}
