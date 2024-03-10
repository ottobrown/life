//! Terminal functions using termios bindings and ANSI escape codes

use std::ffi;
use std::io::{self, stdout, Write};

extern "C" {
    pub fn term_save_settings();
    pub fn term_restore_settings();
    pub fn term_enable_raw_mode();
    pub fn term_wait_stdin_ms(_: ffi::c_long) -> ffi::c_int;
    pub fn term_get_win_rows() -> ffi::c_ushort;
    pub fn term_get_win_cols() -> ffi::c_ushort;
    /*
    pub fn term_set_nonblocking_io();
    pub fn term_unset_nonblocking_io();
    */
}

pub fn save_settings() {
    unsafe {
        term_save_settings();
    }
}

pub fn restore_settings() {
    unsafe {
        term_restore_settings();
    }
}

pub fn enable_raw_mode() {
    unsafe {
        term_enable_raw_mode();
    }
}

/// Set a `ms` millisecond timeout for input to be given to `stdin`.
///
/// returns `Ok(true)` if `stdin` can be read from, `Ok(false)` if timeout reached.
pub fn wait_stdin_ms(ms: u64) -> io::Result<bool> {
    let result: ffi::c_int = unsafe { term_wait_stdin_ms(ms as ffi::c_long) };

    if result == -1 {
        return Err(io::Error::last_os_error());
    }

    if result == 0 {
        return Ok(false);
    }

    return Ok(true);
}

pub fn get_win_rows() -> u16 {
    (unsafe { term_get_win_rows() }) as u16
}

pub fn get_win_cols() -> u16 {
    (unsafe { term_get_win_cols() }) as u16
}

pub fn flush() -> io::Result<()> {
    stdout().flush()
}

/// Flushes `stdout`
pub fn goto_alternate_screen() -> io::Result<()> {
    stdout().write_all(b"\x1b[?1049h")?;
    stdout().flush()
}

/// Flushes `stdout`
pub fn goto_main_screen() -> io::Result<()> {
    stdout().write_all(b"\x1b[?1049l")?;
    stdout().flush()
}

/// Flushes `stdout`
pub fn hide_cursor() -> io::Result<()> {
    stdout().write_all(b"\x1b[?25l")?;
    stdout().flush()
}

/// Flushes `stdout`
pub fn show_cursor() -> io::Result<()> {
    stdout().write_all(b"\x1b[?25h")?;
    stdout().flush()
}

/// Flushes `stdout`
pub fn clear_screen() -> io::Result<()> {
    stdout().write_all(b"\x1b[2J")?;
    stdout().flush()
}

/// Does not flush `stdout`
pub fn move_cursor(x: u16, y: u16) -> io::Result<()> {
    let escape_string: &str = &format!("\x1b[{};{}H", y, x);
    let bytes: &[u8] = escape_string.as_bytes();
    stdout().write_all(bytes)
}
