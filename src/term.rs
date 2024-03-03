//! Terminal functions using termios bindings and ANSI escape codes

use std::io::{self, stdout, Write};

extern "C" {
    pub fn term_save_settings();
    pub fn term_restore_settings();
    pub fn term_enable_raw_mode();
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

/*
pub fn set_nonblocking_io() {
    unsafe { term_set_nonblocking_io(); }
}

pub fn unset_nonblocking_io() {
    unsafe { term_unset_nonblocking_io(); }
}
*/

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
