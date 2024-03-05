#![allow(clippy::manual_range_contains)]
#![allow(clippy::needless_return)]

use std::io::{self, Read};

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
    //term::set_nonblocking_io();

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
    /*
    matrix.insert(4, 4);
    matrix.insert(4, 6);
    matrix.insert(5, 5);
    matrix.insert(5, 6);
    matrix.insert(6, 5);
    */

    let mut rend = render::Renderer::new();

    let mut frame_handler = frame::FrameHandler::new(100);

    loop {
        let can_read = frame_handler.advance_frame()?;

        let mut byte: [u8; 1] = [0];
        if can_read && io::stdin().read(&mut byte)? > 0 {
            let signal = input::handle_input(byte[0]);

            match signal.dest() {
                Destination::Nowhere => {}
                Destination::Renderer => rend.handle_signal(signal),
                Destination::FrameHandler => frame_handler.handle_signal(signal),
            }
        }

        let changes = if !frame_handler.paused {
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
