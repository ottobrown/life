#include <termios.h>
#include <unistd.h>
#include <stdlib.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/ioctl.h>
// #include <fcntl.h>

static struct termios ORIG_TERM;

void term_enable_raw_mode() {
    struct termios term;
    tcgetattr(STDIN_FILENO, &term);

    term.c_lflag &= ~(ECHO | ICANON);

    tcsetattr(STDOUT_FILENO, TCSAFLUSH, &term);
}

void term_save_settings() {
    tcgetattr(STDOUT_FILENO, &ORIG_TERM);
}

void term_restore_settings() {
    tcsetattr(STDOUT_FILENO, TCSAFLUSH, &ORIG_TERM);
}

/// Set a `ms` millisecond timeout for input to be given to `stdin`.
///
/// returns `-1` for error, `0` if timeout reached.
int term_wait_stdin_ms(long ms) {
    struct timeval timeout;
    timeout.tv_sec = ms / 1000;
    timeout.tv_usec = (ms % 1000) * 1000;

    fd_set readfds;
    FD_ZERO(&readfds);
    FD_SET(STDIN_FILENO, &readfds);

    return select(STDIN_FILENO + 1, &readfds, NULL, NULL, &timeout);
}

unsigned short term_get_win_rows() {
    struct winsize ws;
    ioctl(STDIN_FILENO, TIOCGWINSZ, &ws);

    return ws.ws_row;
}

unsigned short term_get_win_cols() {
    struct winsize ws;
    ioctl(STDIN_FILENO, TIOCGWINSZ, &ws);

    return ws.ws_col;
}

/*
void term_set_nonblocking_io() {
    fcntl(STDIN_FILENO, F_SETFL, fcntl(0, F_GETFL) | O_NONBLOCK);
}

void term_unset_nonblocking_io() {
    fcntl(STDIN_FILENO, F_SETFL, fcntl(0, F_GETFL) & ~O_NONBLOCK);
}
*/
