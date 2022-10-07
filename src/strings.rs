use std::fmt::{
    Display,
    Formatter,
    Result
};

use crate::consts::{
    Formatting,
    FORMAT_RESET
};


/// An object representing pieces of text and formatting for each part.
///
/// # Examples
///
/// Direct usage.
/// ```
/// use coloured::ColouredString;
/// use coloured::consts::Formatting;
/// let a = ColouredString::new();
/// let b = ColouredString::from("foo");
/// let c = ColouredString::from_formatting("bar", Formatting::FgRed);
/// ```
///
/// Shorthand usage.
/// ```
/// use coloured::fg;
/// let a = fg::red("bar");
/// ```
///
/// # Aliases
///
/// Disable feature `us` to use `ColouredString` name.
/// Enable feature `us` to use `ColoredString` alias.
#[derive(Debug, Clone)]
pub struct ColouredString {
    pub(crate) parts      : Vec<ColouredStringPart>,
    pub(crate) formatting : Vec<Formatting>
}

/// Initialisation
impl ColouredString {
    /// Create a new empty, unformatted, `ColouredString`.
    pub fn new() -> ColouredString {
        return ColouredString {
            parts      : Vec::new(),
            formatting : Vec::new()
        };
    }
    /// Create a new unformatted `ColouredString` containing some text.
    pub fn from<S : Into<String>>(text : S) -> ColouredString {
        return ColouredString {
            parts      : vec![ColouredStringPart::String(text.into())],
            formatting : Vec::new()
        }
    }
    /// Create a new `ColouredString` containing some text and formatting.
    pub(crate) fn from_formatting<S : Into<String>>(text : S, formatting : Vec<Formatting>) -> ColouredString {
        return ColouredString {
            parts      : vec![ColouredStringPart::String(text.into())],
            formatting : formatting
        };
    }
}

/// Mutation and Getters
impl ColouredString {

    /// TODO
    pub fn unformat(&self) -> String {
        return self.parts.iter().map(|part| part.unformat()).collect::<Vec<String>>().join("");
    }
    
    /// TODO
    fn format_next(&self, prefix : &Vec<String>) -> String {
        let mut result = String::new();
        let mut next_prefix = prefix.clone();
        next_prefix.append(&mut self.formatting.iter().map(|f| format!("{}", f)).collect::<Vec<String>>());
        for part in &self.parts {
            result += part.format_next(&next_prefix).as_str();
        }
        return result;
    }

    /// TODO
    pub fn format(&self) -> String {
        return self.format_next(&Vec::new());
    }

    /// TODO
    fn push_piece(&mut self, piece : ColouredStringPart) {
        let mut result = ColouredString::new();
        result.parts.push(ColouredStringPart::Sub(Box::new(self.clone())));
        result.parts.push(piece);
        *self = result;
    }

    /// TODO
    pub fn push(&mut self, string : ColouredString) {
        self.push_piece(ColouredStringPart::Sub(Box::new(string)));
    }

    /// Append an unformatted `String` to the end of this `ColouredString`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut s = fg::red("foo");
    /// s.push_string(String::from("bar"));
    /// assert_eq(s.unformat(), "foobar");
    /// ```
    pub fn push_string<S : Into<String>>(&mut self, string : S) {
        self.push_piece(ColouredStringPart::String(string.into()));
    }

    /// Append an unformatted `char` to the end of this `ColouredString`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut c = fg::red("foo");
    /// c.push_char('b');
    /// assert_eq(c.unformat(), "foob");
    /// ```
    pub fn push_char(&mut self, ch : char) {
        self.push_string(ch);
    }

    /// TODO
    fn split_at(&mut self, mut idx : usize) -> [ColouredString; 2] {
        let mut left  = Vec::new();
        let mut right = Vec::new();
        for p in 0..self.parts.len() {
            let part = &mut self.parts[p];
            if (idx == 0) {
                // TODO : 
            } else if (idx < part.len()) {
                // TODO : split the part.
            } else {
                idx -= part.len();
            }
        }
        let mut left_string = ColouredString::new();
        left_string.parts = left;
        let mut right_string = ColouredString::new();
        right_string.parts = right;
        return [left_string, right_string];
    }

    // TODO : truncate

    // TODO : pop

    // TODO : remove

    // TODO : remove_matches

    // TODO : retain

    /// TODO
    fn insert_piece(&mut self, mut idx : usize, piece : ColouredStringPart) {
        assert!(idx <= self.len(), "Byte index out of bounds.");
        let [left, right] = self.split_at(idx);
        let result        = ColouredString::new();
        result.parts.push(ColouredStringPart::Sub(Box::new(left)));
        result.parts.push(piece);
        result.parts.push(ColouredStringPart::Sub(Box::new(right)));
        *self = result;
    }

    /// TODO
    pub fn insert(&mut self, idx : usize, string : ColouredString) {
        self.insert_piece(idx, ColouredStringPart::Sub(Box::new(string)));
    }

    /// TODO
    pub fn insert_string<S : Into<String>>(&mut self, idx : usize, string : S) {
        self.insert_piece(idx, ColouredStringPart::String(string.into()));
    }

    /// TODO
    pub fn insert_char(&mut self, idx : usize, ch : char) {
        self.insert_string(idx, ch.to_string());
    }

    /// Returns the sum of the lengths of each part.
    /// This uses the [String::len] method, so it
    /// might not be what a human considers the
    /// length of the string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use colourful::fg;
    /// let a = fg::red("foo");
    /// assert_eq!(3, a.len());
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

/// TODO
impl Display for ColouredString {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        return write!(f, "{}", self.format());
    }
}


/// TODO
#[derive(Debug, Clone)]
pub(crate) enum ColouredStringPart {
    /// TODO
    String(String),
    /// TODO
    Sub(Box<ColouredString>)
}

/// TODO
impl ColouredStringPart {

    /// TODO
    fn unformat(&self) -> String {
        return match (self) {
            ColouredStringPart::String (string) => String::from(string),
            ColouredStringPart::Sub    (string) => (*string).unformat()
        };
    }

    /// TODO
    fn format_next(&self, prefix : &Vec<String>) -> String {
        return match (self) {
            ColouredStringPart::String(string) => format!(
                "{}{}{}",
                prefix.join(""),
                string,
                FORMAT_RESET
            ),
            ColouredStringPart::Sub(string) => string.format_next(prefix)
        };
    }

    /// TODO
    fn insert_piece(&mut self, idx : usize, piece : ColouredStringPart) {
        match (self) {
            ColouredStringPart::String(string) => {
                assert!(idx <= string.len(), "Byte index out of bounds.");
                let mut coloured_string = ColouredString::new();
                coloured_string.push_string(string[..idx].to_string());
                coloured_string.push_piece(piece);
                coloured_string.push_string(string[idx..].to_string());
                *self = ColouredStringPart::Sub(Box::new(coloured_string));
            },
            ColouredStringPart::Sub(string) => string.insert_piece(idx, piece)
        }
    }

    /// TODO
    fn len(&self) -> usize {
        return match (self) {
            ColouredStringPart::String (string) => string.len(),
            ColouredStringPart::Sub    (string) => string.len()
        };
    }

}
