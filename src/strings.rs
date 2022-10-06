use std::fmt::{
    Display,
    Formatter,
    Result
};

use crate::consts::{
    Formatting,
    FORMAT_RESET
};


#[derive(Debug)]
pub struct ColouredString {
    pub(crate) parts      : Vec<ColouredStringPart>,
    pub(crate) formatting : Vec<Formatting>
}
impl ColouredString {
    pub(crate) fn new() -> ColouredString {
        return ColouredString {
            parts      : Vec::new(),
            formatting : Vec::new()
        };
    }
    pub(crate) fn from<S : Into<String>>(text : S, formatting : Vec<Formatting>) -> ColouredString {
        return ColouredString {
            parts      : vec![ColouredStringPart::String(text.into())],
            formatting : formatting
        };
    }
}

impl Display for ColouredString {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        let mut text = String::new();
        for part in &self.parts {
            text += format!("{}{}{}",
                self.formatting.iter().map(|f| format!("{}", f)).collect::<Vec<String>>().join(""),
                part,
                FORMAT_RESET
            ).as_str();
        }
        return write!(f, "{}", text);
    }
}


#[derive(Debug)]
pub(crate) enum ColouredStringPart {
    String(String),
    Sub(Box<ColouredString>)
}

impl Display for ColouredStringPart {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        return write!(f, "{}", match (self) {
            ColouredStringPart::String(string) => String::from(string),
            ColouredStringPart::Sub(string)    => format!("{}", string)
        });
    }
}
