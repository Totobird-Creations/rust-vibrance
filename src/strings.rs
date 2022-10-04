use std::fmt::{Debug, Display, Formatter, Result};
use std::collections::HashSet;

use crate::enums::{Colour, Style, Font};


#[derive(Clone)]
pub struct FormattedString<'l> {
    text       : &'l str,
    foreground : Colour,
    background : Colour,
    style      : HashSet<Style>,
    font       : Font,
    next       : Option<Box<FormattedString<'l>>>
}

impl<'l> FormattedString<'l> {
    pub fn from<S : Into<&'l str>>(text : S) -> FormattedString<'l> {
        return FormattedString {
            text       : text.into(),
            foreground : Colour::None,
            background : Colour::None,
            style      : HashSet::new(),
            font       : Font::Primary,
            next       : None
        };
    }
}

impl<'l> FormattedString<'l> {
    pub fn set_foreground(mut self, colour : Colour) -> FormattedString<'l> {
        self.foreground = colour;
        return self;
    }
    pub fn set_background(mut self, colour : Colour) -> FormattedString<'l> {
        self.background = colour;
        return self;
    }
    pub fn add_style(mut self, style : Style) -> FormattedString<'l> {
        self.style.insert(style);
        return self;
    }
    pub fn set_font(mut self, font : Font) -> FormattedString<'l> {
        self.font = font;
        return self;
    }
}

impl Debug for FormattedString<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return write!(f,
            "{}{}",
            self.text,
            match (&self.next) {
                Some(next) => format!("{:?}", next),
                None       => String::new()
            }
        );
    }
}

impl Display for FormattedString<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return write!(f,
            "{}{}",
            self.text,
            match (&self.next) {
                Some(next) => format!("\x{:?}", next),
                None       => String::new()
            }
        );
    }
}
