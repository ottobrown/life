#[derive(Debug)]
pub enum OptionParseError {
    /// e.g. `--fps abcd` when `--fps` expects an integer.
    /// Index `0` is the incorrect argument and
    /// Index `1` is the flag it is given to
    InvalidArgumentToFlag(String, String),
    /// An argument where none is expected
    /// (e.g. if the arguments are `--fps 20 abcd`, `abcd` is an unexpected argument)
    UnexpectedArgument(String),
    /// End of args where an argument is expected
    /// (e.g. last argument is `--fps`)
    MissingArgument,
}

/// Configured by commane line arguments
pub struct Options {
    /// The number of milliseconds in a frame
    ///
    /// configured by `--fps`, `--frame-time`, or `--ft`
    pub frame_time: u64,
    /// The byte for the character that represents living cells
    ///
    /// configured by `--char` or `-c`
    pub character: u8,
}
impl Options {
    pub fn parse_from_args(mut args: std::env::Args) -> Result<Self, OptionParseError> {
        let mut ops = Self::default();

        // Ignore 0th argument, which is the path to the binary
        args.next();

        while let Some(s) = args.next() {
            let tr = s.trim();

            if tr == "--fps" {
                if let Some(nx) = args.next() {
                    if let Ok(fps) = nx.trim().parse::<u64>() {
                        ops.frame_time = 1000 / fps;
                    } else {
                        return Err(OptionParseError::InvalidArgumentToFlag(nx, s));
                    }
                } else {
                    return Err(OptionParseError::MissingArgument);
                }
            } else if tr == "--frame_time" || tr == "--ft" {
                if let Some(nx) = args.next() {
                    if let Ok(ft) = nx.trim().parse::<u64>() {
                        ops.frame_time = ft;
                    } else {
                        return Err(OptionParseError::InvalidArgumentToFlag(nx, s));
                    }
                } else {
                    return Err(OptionParseError::MissingArgument);
                }
            } else if tr == "--char" || tr == "-c" {
                if let Some(nx) = args.next() {
                    if nx.trim().len() > 1 {
                        return Err(OptionParseError::InvalidArgumentToFlag(nx, s));
                    }

                    ops.character = nx.bytes().next().unwrap();
                } else {
                    return Err(OptionParseError::MissingArgument);
                }
            } else {
                return Err(OptionParseError::UnexpectedArgument(s));
            }
        }

        Ok(ops)
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            frame_time: 50,
            character: b'#',
        }
    }
}
