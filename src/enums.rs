#[derive(Clone)]
pub enum Colour {
    None,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Rgb(u8, u8, u8)
}
impl Colour {
    pub fn code_fg(&self) -> String {
        return self.code("3", "9");
    }
    fn code(&self, normal : &str, bright : &str) -> String {
        return match (self) {
            Colour::None          => format!("\x1b[{}9m", normal),
            Colour::Black         => format!("\x1b[{}0m", normal),
            Colour::Red           => format!("\x1b[{}1m", normal),
            Colour::Green         => format!("\x1b[{}2m", normal),
            Colour::Yellow        => format!("\x1b[{}3m", normal),
            Colour::Blue          => format!("\x1b[{}4m", normal),
            Colour::Magenta       => format!("\x1b[{}5m", normal),
            Colour::Cyan          => format!("\x1b[{}6m", normal),
            Colour::White         => format!("\x1b[{}7m", normal),
            Colour::BrightBlack   => format!("\x1b[{}0m", bright),
            Colour::BrightRed     => format!("\x1b[{}1m", bright),
            Colour::BrightGreen   => format!("\x1b[{}2m", bright),
            Colour::BrightYellow  => format!("\x1b[{}3m", bright),
            Colour::BrightBlue    => format!("\x1b[{}4m", bright),
            Colour::BrightMagenta => format!("\x1b[{}5m", bright),
            Colour::BrightCyan    => format!("\x1b[{}6m", bright),
            Colour::BrightWhite   => format!("\x1b[{}7m", bright),
            Colour::Rgb(r, g, b)  => format!("\x1b[{};2;{};{};{}m", normal, r, g, b)
        };
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Style {
    Bold,
    Faint,
    Italic,
    Underline,
    Blink,
    FastBlink,
    Invert,
    Conceal,
    Strikethrough
}

#[derive(Clone)]
pub enum Font {
    Primary,
    Alternative1,
    Alternative2,
    Alternative3,
    Alternative4,
    Alternative5,
    Alternative6,
    Alternative7,
    Alternative8,
    Alternative9
}
