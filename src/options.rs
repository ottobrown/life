use std::str::FromStr;

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

            match tr {
                "--fps" => {
                    let next = args.next();
                    if next.is_none() {
                        return Err(OptionParseError::MissingArgument);
                    }

                    let nx = next.unwrap();
                    let fps: u64 = Self::parse_flag_arg(tr, nx.trim())?;

                    ops.frame_time = 1000 / fps;
                }
                "--frame-time" | "--ft" => {
                    let next = args.next();
                    if next.is_none() {
                        return Err(OptionParseError::MissingArgument);
                    }

                    let nx = next.unwrap();
                    let ft: u64 = Self::parse_flag_arg(tr, nx.trim())?;

                    ops.frame_time = ft;
                }
                "--char" | "-c" => {
                    let next = args.next();
                    if next.is_none() {
                        return Err(OptionParseError::MissingArgument);
                    }

                    let nx = next.unwrap();
                    if nx.trim().len() > 1 {
                        return Err(OptionParseError::InvalidArgumentToFlag(nx, s));
                    }

                    ops.character = nx.bytes().next().unwrap();
                }
                _ => {
                    return Err(OptionParseError::UnexpectedArgument(s));
                }
            }
        }

        Ok(ops)
    }

    fn parse_flag_arg<T: FromStr>(flag: &str, arg: &str) -> Result<T, OptionParseError> {
        let result = arg.parse::<T>();

        if let Ok(t) = result {
            return Ok(t);
        }

        return Err(OptionParseError::InvalidArgumentToFlag(
            arg.into(),
            flag.into(),
        ));
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
