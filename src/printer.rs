/// Colors
/// 
/// Available colors to print in various colors.
/// Translates to their ASNI Escape Code representation.
pub enum Colors {
    None,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BlackBright,
    RedBright,
    GreenBright,
    YellowBright,
    BlueBright,
    MagentaBright,
    CyanBright,
    WhiteBright
}

impl AsRef<str> for Colors {
    fn as_ref(&self) -> &'static str {
        match self {
            Colors::None => "\x1b[0m",
            // Default
            Colors::Black => "\x1b[30m",
            Colors::Red => "\x1b[31m",
            Colors::Green => "\x1b[32m",
            Colors::Yellow => "\x1b[33m",
            Colors::Blue => "\x1b[34m",
            Colors::Magenta => "\x1b[35m",
            Colors::Cyan => "\x1b[36m",
            Colors::White => "\x1b[37m",
            // Bright
            Colors::BlackBright => "\x1b[90m",
            Colors::RedBright => "\x1b[91m",
            Colors::GreenBright => "\x1b[92m",
            Colors::YellowBright => "\x1b[93m",
            Colors::BlueBright => "\x1b[94m",
            Colors::MagentaBright => "\x1b[95m",
            Colors::CyanBright => "\x1b[96m",
            Colors::WhiteBright => "\x1b[97m"
        }
    }
}

/// Println with color
/// 
/// Prints with color and new line to stdout
pub fn write(msg: &str, color: Colors) {
    print!("{}{}{}", color.as_ref(), msg, Colors::None.as_ref());
}

/// Print with color
/// 
/// Prints with color to stdout
pub fn write_line(msg: &str, color: Colors) {
    println!("{}{}{}", color.as_ref(), msg, Colors::None.as_ref());
}

/// Println with color
/// 
/// Prints with color and new line to stdout
pub fn error(msg: &str, color: Colors) {
    eprint!("{}{}{}", color.as_ref(), msg, Colors::None.as_ref());
}

/// Print with color
/// 
/// Prints with color to stdout
pub fn error_line(msg: &str, color: Colors) {
    eprintln!("{}{}{}", color.as_ref(), msg, Colors::None.as_ref());
}