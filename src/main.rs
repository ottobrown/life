#![allow(clippy::manual_range_contains)]
#![allow(clippy::needless_return)]

use std::io::{self};

mod frame;
mod input;
mod render;
mod rules;
mod term;

use input::Destination;
pub use input::Signal;

fn setup() -> io::Result<()> {
    term::save_settings();
    term::goto_alternate_screen()?;
    term::hide_cursor()?;
    term::enable_raw_mode();

    Ok(())
}

extern "C" fn reset() {
    let _ = term::show_cursor();
    let _ = term::goto_main_screen();
    term::restore_settings();
    //term::unset_nonblocking_io();
}

fn main() -> std::io::Result<()> {
    setup()?;

    unsafe {
        // not called on `SIGINT`, which is given by `^C`.
        libc::atexit(reset);
        libc::srand(libc::time(std::ptr::null_mut()) as u32);
    }

    let mut matrix = rules::Matrix::blank();
    for _ in 0..400 {
        let x = unsafe { libc::rand() } % 50;
        let y = unsafe { libc::rand() } % 50;
        matrix.insert(x as i16, y as i16);
    }

    let mut rend = render::Renderer::new();

    let mut frame_handler = frame::FrameHandler::new(100);

    loop {
        let frame_info = frame_handler.advance_frame()?;

        if let Some(b) = frame_info.input {
            let signal = input::handle_input(b);

            match signal.dest() {
                Destination::Nowhere => {}
                Destination::Renderer => rend.handle_signal(signal),
                Destination::FrameHandler => frame_handler.handle_signal(signal),
            }
        }

        let changes = if frame_info.advance_matrix {
            matrix.advance()
        } else {
            vec![]
        };

        if rend.need_rerender {
            rend.rerender(&matrix)?;
        } else {
            rend.render_from_changes(changes)?;
        }
    }
}
