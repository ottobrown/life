#include <termios.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>

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

/*
void term_set_nonblocking_io() {
    fcntl(STDIN_FILENO, F_SETFL, fcntl(0, F_GETFL) | O_NONBLOCK);
}

void term_unset_nonblocking_io() {
    fcntl(STDIN_FILENO, F_SETFL, fcntl(0, F_GETFL) & ~O_NONBLOCK);
}
*/
