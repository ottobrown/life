use std::io::Read;

mod render;
pub mod rules;
pub mod term;

fn setup() -> std::io::Result<()> {
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

    // not called on `SIGINT`, which is given by `^C`.
    unsafe {
        libc::atexit(reset);
    }

    let mut matrix = rules::Matrix::blank();
    matrix.insert(4, 4);
    matrix.insert(4, 6);
    matrix.insert(5, 5);
    matrix.insert(5, 6);
    matrix.insert(6, 5);

    let mut rend = render::Renderer::new();

    loop {
        rend.render(&mut matrix)?;

        if term::wait_stdin_ms(100)? {
            let mut byte: [u8; 1] = [0];
            if std::io::stdin().read(&mut byte)? > 0 {
                if byte[0] == b'q' {
                    std::process::exit(0);
                }
            }
        }
    }
}
