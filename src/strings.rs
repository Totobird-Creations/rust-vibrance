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
    pub fn new() -> ColouredString {
        return ColouredString {
            parts      : Vec::new(),
            formatting : Vec::new()
        };
    }
    pub fn from<S : Into<String>>(text : S) -> ColouredString {
        return ColouredString {
            parts      : vec![ColouredStringPart::String(text.into())],
            formatting : Vec::new()
        }
    }
    pub(crate) fn from_formatting<S : Into<String>>(text : S, formatting : Vec<Formatting>) -> ColouredString {
        return ColouredString {
            parts      : vec![ColouredStringPart::String(text.into())],
            formatting : formatting
        };
    }
}
impl ColouredString {
    pub fn push_string(&mut self, string : String) {
        *self += string;
    }
    pub fn push(&mut self, ch : char) {
        self.parts.push(ColouredStringPart::String(ch.to_string()));
    }
    // TODO : truncate
    // TODO : pop
    // TODO : remove
    // TODO : remove_matches
    // TODO : retain
    // TODO : insert
    
    /// Returns the sum of the lengths of each part.
    /// This uses the `String.len` method, so it
    /// might not be what a human considers the
    /// length of the string.
    /// 
    /// # Examples
    /// 
    /// ```
        use colourful::fg;
        let a = fg::red("foo");
        assert_eq!(3, a.len());
    /// ```
    pub fn len(&self) -> usize {
        return self.parts.iter().map(|part| part.len()).sum();
    }
    /// Returns `true` is this `ColouredString` has a length of zero, and `false` otherwise.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let v = ColouredString::new();
    /// assert!(v.is_empty());
    /// 
    /// v.push('a');
    /// assert!(! v.is_empty())
    /// ```
    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }
    /// Remove all characters and formatting from this `ColouredString`.
    ///
    /// # Examples
    /// 
    /// ```
    /// use colourful::fg;
    /// let mut s = fg::red("foo");
    /// 
    /// s.clear();
    /// 
    /// assert!(s.is_empty());
    /// assert_eq!(0, s.len());
    /// ```
    pub fn clear(&mut self) {
        self.parts.clear();
        self.formatting.clear();
    }
    // TODO : drain
    // TODO : replace_range
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
impl ColouredStringPart {
    pub fn to_string(&self) -> String {
        return match (self) {
            ColouredStringPart::String (string) => String::from(string),
            ColouredStringPart::Sub    (string) => string.to_string()
        };
    }
    pub fn len(&self) -> usize {
        return match (self) {
            ColouredStringPart::String (string) => string.len(),
            ColouredStringPart::Sub    (string) => string.len()
        };
    }
}

impl Display for ColouredStringPart {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        return write!(f, "{}", match (self) {
            ColouredStringPart::String (string) => String::from(string),
            ColouredStringPart::Sub    (string) => format!("{}", string)
        });
    }
}
