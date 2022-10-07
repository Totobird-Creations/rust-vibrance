use std::fmt::{
    Display,
    Formatter,
    Result
};


const FORMAT_PREFIX : &'static str = "\x1b[";
const FORMAT_SUFFIX : &'static str = "m";

pub(crate) const FORMAT_RESET : &'static str = "\x1b[0m";


#[derive(Debug, Clone)]
pub enum Formatting {

    None,
    
    Bold,
    Faint,
    Italic,
    Underline,
    SlowBlink,
    FastBlink,
    Invert,
    Conceal,
    Strikethrough,
    Overline,

    FgBlack,
    FgRed,
    FgGreen,
    FgYellow,
    FgBlue,
    FgMagenta,
    FgCyan,
    FgWhite,
    FgBrightBlack,
    FgBrightRed,
    FgBrightGreen,
    FgBrightYellow,
    FgBrightBlue,
    FgBrightMagenta,
    FgBrightCyan,
    FgBrightWhite,
    Fg8Bit(u8),
    Fg24Bit(u8, u8, u8),

    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
    BgBrightBlack,
    BgBrightRed,
    BgBrightGreen,
    BgBrightYellow,
    BgBrightBlue,
    BgBrightMagenta,
    BgBrightCyan,
    BgBrightWhite,
    Bg8Bit(u8),
    Bg24Bit(u8, u8, u8),


    ResetIntensity,
    ResetItalic,
    ResetUnderline,
    ResetSlowBlink,
    ResetFastBlink,
    ResetInvert,
    ResetConceal,
    ResetStrikethrough,
    ResetOverline,

    FgReset,

    BgReset

}

impl Formatting {
    pub fn get_code(&self) -> String {
        match (self) {

            Formatting::None            => String::from("0"),
            
            Formatting::Bold            => String::from("1"),
            Formatting::Faint           => String::from("2"),
            Formatting::Italic          => String::from("3"),
            Formatting::Underline       => String::from("4"),
            Formatting::SlowBlink       => String::from("5"),
            Formatting::FastBlink       => String::from("6"),
            Formatting::Invert          => String::from("7"),
            Formatting::Conceal         => String::from("8"),
            Formatting::Strikethrough   => String::from("9"),
            Formatting::Overline        => String::from("53"),

            Formatting::FgBlack         => String::from("30"),
            Formatting::FgRed           => String::from("31"),
            Formatting::FgGreen         => String::from("32"),
            Formatting::FgYellow        => String::from("33"),
            Formatting::FgBlue          => String::from("34"),
            Formatting::FgMagenta       => String::from("35"),
            Formatting::FgCyan          => String::from("36"),
            Formatting::FgWhite         => String::from("37"),
            Formatting::FgBrightBlack   => String::from("90"),
            Formatting::FgBrightRed     => String::from("91"),
            Formatting::FgBrightGreen   => String::from("92"),
            Formatting::FgBrightYellow  => String::from("93"),
            Formatting::FgBrightBlue    => String::from("94"),
            Formatting::FgBrightMagenta => String::from("95"),
            Formatting::FgBrightCyan    => String::from("96"),
            Formatting::FgBrightWhite   => String::from("97"),
            Formatting::Fg8Bit(n)       => format!("38;5;{n}"),
            Formatting::Fg24Bit(r,g,b)  => format!("38;2;{r};{g};{b}"),

            Formatting::BgBlack         => String::from("40"),
            Formatting::BgRed           => String::from("41"),
            Formatting::BgGreen         => String::from("42"),
            Formatting::BgYellow        => String::from("43"),
            Formatting::BgBlue          => String::from("44"),
            Formatting::BgMagenta       => String::from("45"),
            Formatting::BgCyan          => String::from("46"),
            Formatting::BgWhite         => String::from("47"),
            Formatting::BgBrightBlack   => String::from("100"),
            Formatting::BgBrightRed     => String::from("101"),
            Formatting::BgBrightGreen   => String::from("102"),
            Formatting::BgBrightYellow  => String::from("103"),
            Formatting::BgBrightBlue    => String::from("104"),
            Formatting::BgBrightMagenta => String::from("105"),
            Formatting::BgBrightCyan    => String::from("106"),
            Formatting::BgBrightWhite   => String::from("107"),
            Formatting::Bg8Bit(n)       => format!("48;5;{n}"),
            Formatting::Bg24Bit(r,g,b)  => format!("48;2;{r};{g};{b}"),

            Formatting::ResetIntensity     => String::from("21"),
            Formatting::ResetItalic        => String::from("23"),
            Formatting::ResetUnderline     => String::from("24"),
            Formatting::ResetSlowBlink     => String::from("25"),
            Formatting::ResetFastBlink     => String::from("26"),
            Formatting::ResetInvert        => String::from("27"),
            Formatting::ResetConceal       => String::from("28"),
            Formatting::ResetStrikethrough => String::from("29"),
            Formatting::ResetOverline      => String::from("55"),

            Formatting::FgReset            => String::from("39"),
            Formatting::BgReset            => String::from("49")

        }.to_string()
    }
}

impl Display for Formatting {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        return write!(f, "{}{}{}",
            FORMAT_PREFIX,
            self.get_code(),
            FORMAT_SUFFIX
        );
    }
}
