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
#[derive(Debug)]
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

    pub fn to_string(&self) -> String {
        return self.parts.iter().map(|part| part.to_string()).collect::<Vec<String>>().join("");
    }

    fn push_piece(&mut self, piece : ColouredStringPart) {
        self.parts.push(piece);
    }

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
    /// assert_eq(s.to_string(), "foobar");
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
    /// assert_eq(c.to_string(), "foob");
    /// ```
    pub fn push_char(&mut self, ch : char) {
        self.push_string(ch.to_string());
    }

    // TODO : truncate

    // TODO : pop

    // TODO : remove

    // TODO : remove_matches

    // TODO : retain

    fn insert_piece(&mut self, mut idx : usize, piece : ColouredStringPart) {
        assert!(idx <= self.len(), "Byte index out of bounds.");
        for p in 0..self.parts.len() {
            let part = &mut self.parts[p];
            if (idx == 0) {
                self.parts.insert(p, piece);
                return;
            } else if (idx < part.len()) {
                part.insert_piece(idx, piece);
                return;
            } else {
                idx -= part.len();
            }
        }
    }

    pub fn insert(&mut self, idx : usize, string : ColouredString) {
        self.insert_piece(idx, ColouredStringPart::Sub(Box::new(string)));
    }

    pub fn insert_string<S : Into<String>>(&mut self, idx : usize, string : S) {
        self.insert_piece(idx, ColouredStringPart::String(string.into()));
    }

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

    fn to_string(&self) -> String {
        return match (self) {
            ColouredStringPart::String (string) => String::from(string),
            ColouredStringPart::Sub    (string) => (*string).to_string()
        };
    }

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

    fn len(&self) -> usize {
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
