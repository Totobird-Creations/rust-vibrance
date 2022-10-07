use std::ops::{
    Add,
    AddAssign
};

use crate::{
    traits::Colourisable,
    consts::Formatting,
    strings::{
        ColouredString,
        ColouredStringPart
    }
};


impl Colourisable for &str {
    fn formatted(self, formatting : Vec<Formatting>) -> ColouredString {
        return ColouredString::from_formatting(self, formatting);
    }
}
impl Add<ColouredString> for &str {
    type Output = ColouredString;
    fn add(self, other : ColouredString) -> Self::Output {
        let mut string = ColouredString::new();
        string.parts   = vec![
            ColouredStringPart::String(self.to_string()),
            ColouredStringPart::Sub(Box::new(other.formatted(Vec::new())))
        ];
        return string;
    }
}
impl From<&str> for ColouredString {
    fn from(string : &str) -> Self {
        return string.formatted(Vec::new());
    }
}

impl Colourisable for String {
    fn formatted(self, formatting : Vec<Formatting>) -> ColouredString {
        return ColouredString::from_formatting(self, formatting);
    }
}
impl Add<ColouredString> for String {
    type Output = ColouredString;
    fn add(self, other : ColouredString) -> Self::Output {
        let mut string = ColouredString::new();
        string.parts   = vec![
            ColouredStringPart::String(self),
            ColouredStringPart::Sub(Box::new(other.formatted(Vec::new())))
        ];
        return string;
    }
}
impl Into<String> for ColouredString {
    fn into(self) -> String {
        return self.unformat();
    }
}
impl From<String> for ColouredString {
    fn from(string : String) -> Self {
        return string.formatted(Vec::new());
    }
}

impl Colourisable for ColouredString {
    fn formatted(mut self, mut formatting : Vec<Formatting>) -> ColouredString {
        formatting.append(&mut self.formatting);
        self.formatting = formatting;
        return self;
    }
}
impl<S : Colourisable> Add<S> for ColouredString {
    type Output = ColouredString;
    fn add(self, other : S) -> Self::Output {
        let mut string = ColouredString::new();
        string.parts   = vec![
            ColouredStringPart::Sub(Box::new(self)),
            ColouredStringPart::Sub(Box::new(other.formatted(Vec::new())))
        ];
        return string;
    }
}
impl<S : Colourisable> AddAssign<S> for ColouredString {
    fn add_assign(&mut self, other : S) {
        self.parts.push(
            ColouredStringPart::Sub(Box::new(other.formatted(Vec::new())))
        );
    }
}
