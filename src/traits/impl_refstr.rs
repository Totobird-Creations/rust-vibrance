use super::{FgColour, BgColour};
use crate::enums::Colour;
use crate::FormattedString;


impl<'l> FgColour<'l> for &'l str {
    #[cfg(not(feature = "us"))]
    fn uncoloured(self) -> FormattedString<'l> {
        return FormattedString::from(self);
    }
    #[cfg(feature = "us")]
    fn uncolored(self) -> FormattedString<'l> {
        return FormattedString::from(self);
    }
    fn black(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::Black);
    }
    fn red(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::Red);
    }
    fn green(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::Green);
    }
    fn yellow(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::Yellow);
    }
    fn blue(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::Blue);
    }
    fn magenta(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::Magenta);
    }
    fn cyan(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::Cyan);
    }
    fn white(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::White);
    }
    fn bright_black(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::BrightBlack);
    }
    fn bright_red(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::BrightRed);
    }
    fn bright_green(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::BrightGreen);
    }
    fn bright_yellow(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::BrightYellow);
    }
    fn bright_blue(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::BrightBlue);
    }
    fn bright_magenta(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::BrightMagenta);
    }
    fn bright_cyan(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::BrightCyan);
    }
    fn bright_white(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::BrightWhite);
    }
    fn rgb(self, r : u8, g : u8, b : u8) -> FormattedString<'l> {
        return FormattedString::from(self).set_foreground(Colour::Rgb(r, g, b));
    }
}


impl<'l> BgColour<'l> for &'l str {
    #[cfg(not(feature = "us"))]
    fn on_uncoloured(self) -> FormattedString<'l> {
        return FormattedString::from(self);
    }
    #[cfg(feature = "us")]
    fn on_uncolored(self) -> FormattedString<'l> {
        return FormattedString::from(self);
    }
    fn on_black(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::Black);
    }
    fn on_red(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::Red);
    }
    fn on_green(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::Green);
    }
    fn on_yellow(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::Yellow);
    }
    fn on_blue(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::Blue);
    }
    fn on_magenta(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::Magenta);
    }
    fn on_cyan(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::Cyan);
    }
    fn on_white(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::White);
    }
    fn on_bright_black(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::BrightBlack);
    }
    fn on_bright_red(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::BrightRed);
    }
    fn on_bright_green(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::BrightGreen);
    }
    fn on_bright_yellow(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::BrightYellow);
    }
    fn on_bright_blue(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::BrightBlue);
    }
    fn on_bright_magenta(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::BrightMagenta);
    }
    fn on_bright_cyan(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::BrightCyan);
    }
    fn on_bright_white(self) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::BrightWhite);
    }
    fn on_rgb(self, r : u8, g : u8, b : u8) -> FormattedString<'l> {
        return FormattedString::from(self).set_background(Colour::Rgb(r, g, b));
    }
}
