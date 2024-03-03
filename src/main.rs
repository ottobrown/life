mod term;

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
    use std::collections::HashSet;

    setup()?;

    // not called on `SIGINT`, which is given by `^C`.
    unsafe { libc::atexit(reset); }

    let mut live: HashSet<(u16, u16)> = HashSet::new();
    live.insert((1, 1));
    live.insert((1, 5));
    live.insert((2, 9));

    for &(x, y) in &live {
        term::move_cursor(x, y)?;
        print!("a");
        term::flush()?;
    }

    loop {
        use std::io::Read;
        let mut byte: [u8; 1] = [0];
        if std::io::stdin().read(&mut byte)? > 0 {
            if byte[0] == b'q' {
                std::process::exit(0);
            }
        }
    }
}
