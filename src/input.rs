#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Signal {
    NoOp,
    Quit,
    Pause,
    MoveUp,
    MoveLeft,
    MoveRight,
    MoveDown,
}
impl Signal {
    pub fn dest(&self) -> Destination {
        use Signal::*;

        match self {
            NoOp => Destination::Nowhere,
            Quit | Pause => Destination::FrameHandler,
            MoveUp | MoveLeft | MoveRight | MoveDown => Destination::Renderer,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Destination {
    Nowhere,
    Renderer,
    FrameHandler,
}

/// Given a byte from the keyboard
pub fn handle_input(b: u8) -> Signal {
    match b {
        b'q' => Signal::Quit,
        b' ' => Signal::Pause,
        b'h' => Signal::MoveLeft,
        b'j' => Signal::MoveDown,
        b'k' => Signal::MoveUp,
        b'l' => Signal::MoveRight,
        _ => Signal::NoOp,
    }
}
