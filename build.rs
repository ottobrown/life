fn main() {
    cc::Build::new()
        .file("src/term.c")
        .compile("test");
}
