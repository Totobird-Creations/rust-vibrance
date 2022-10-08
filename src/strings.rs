use std::{
    fmt::{
        Display,
        Formatter,
        Result
    },
    ops::{
        RangeBounds,
        Bound
    }
};

use crate::consts::{
    Formatting,
    FORMAT_RESET
};


/// An object representing pieces of text and
/// formatting for each part.
///
/// # Examples
///
/// Direct usage.
/// ```
/// use vibrance::ColouredString;
/// let a = ColouredString::new();
/// let b = ColouredString::from("foo");
/// ```
///
/// Shorthand usage.
/// ```
/// use vibrance::{fg, bg, style};
/// let a = fg::red("foo");
/// let b = bg::green("bar" + style::bold("baz"));
/// ```
///
/// # Aliases
///
/// Disable the `us` feature to use `ColouredString` name.
/// This is used by default.
/// 
/// Enable the `us` feature to use `ColoredString` alias.
#[derive(Debug, Clone)]
pub struct ColouredString {
    pub(crate) parts      : Vec<ColouredStringPart>,
    pub(crate) formatting : Vec<Formatting>
}

/// Initialisation
impl ColouredString {
    /// Create a new empty, unformatted, `ColouredString`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vibrance::ColouredString;
    /// let m = ColouredString::new();
    /// assert_eq!(m.unformat(), "");
    /// ```
    pub fn new() -> ColouredString {
        return ColouredString {
            parts      : Vec::new(),
            formatting : Vec::new()
        };
    }
    /// Create a new unformatted `ColouredString` containing some text.
    ///
    /// # Arguments
    /// 
    /// * `text` - Any object that can be converted into a `String`.
    /// 
    /// # Examples
    ///
    /// ```
    /// use vibrance::ColouredString;
    /// let f = ColouredString::from("foo");
    /// assert_eq!(f.unformat(), String::from("foo"));
    /// ```
    pub fn from<S : Into<String>>(text : S) -> ColouredString {
        return ColouredString::from_part(ColouredStringPart::String(text.into()));
    }
    /// Creates a new `ColouredString` contining a single part.
    /// 
    /// # Internal
    pub(crate) fn from_part(part : ColouredStringPart) -> ColouredString {
        return ColouredString {
            parts      : vec![part],
            formatting : Vec::new()
        };
    }
    /// Create a new `ColouredString` containing some text and formatting.
    /// 
    /// Commonly used by implementations of `Colourisable`.
    pub fn from_formatting<S : Into<String>>(text : S, formatting : Vec<Formatting>) -> ColouredString {
        return ColouredString {
            parts      : vec![ColouredStringPart::String(text.into())],
            formatting : formatting
        };
    }
}

/// Mutation and Getters
impl ColouredString {

    /// Create an unformatted string from the parts.
    /// The resulting string will not contain any 
    /// escape codes. If you want one that has the
    /// codes, see [ColouredString::unformat] or use
    /// `format!`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vibrance::fg;
    /// let u = fg::red("foo");
    /// assert_eq!(u.unformat(), "foo");
    /// ```
    /// 
    /// # Aliases
    /// 
    /// [ColouredString::deformat].
    pub fn unformat(&self) -> String {
        return self.parts.iter().map(|part| part.unformat()).collect::<Vec<String>>().join("");
    }

    /// Alias to [ColouredString::unformat].
    pub fn deformat(&self) -> String {
        return self.unformat();
    }


    /// Format the parts with the formatting and given prefix.
    fn format_next(&self, prefix : &Vec<String>) -> String {
        let mut result = String::new();
        let mut next_prefix = prefix.clone();
        next_prefix.append(&mut self.formatting.iter().map(|f| format!("{}", f)).collect::<Vec<String>>());
        for part in &self.parts {
            result += part.format_next(&next_prefix).as_str();
        }
        return result;
    }

    /// Create a formatted string from the parts.
    /// The resulting string will contain several escape
    /// codes. If you want one that doesn't have the
    /// codes, see [ColouredString::unformat].
    pub fn format(&self) -> String {
        return self.format_next(&Vec::new());
    }


    /// Append a `ColouredStringPart` to the end of this `ColouredString`.
    /// 
    /// # Internal
    fn push_piece(&mut self, piece : ColouredStringPart) {
        let mut result = ColouredString::new();
        result.parts.push(ColouredStringPart::Sub(Box::new(self.clone())));
        result.parts.push(piece);
        *self = result;
    }

    /// Append a `ColouredString` to the end of this `ColouredString`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vibrance::fg;
    /// let mut s = fg::red("foo");
    /// s.push(String::from("bar"));
    /// s.push("baz");
    /// s.push(fg::red("foo"));
    /// assert_eq!(s.unformat(), "foobarbazfoo");
    /// ```
    pub fn push<S : Into<ColouredString>>(&mut self, string : S) {
        self.push_piece(ColouredStringPart::Sub(Box::new(string.into())));
    }


    /// Splits this `ColouredString` into two parts
    /// at the given index.
    /// 
    /// # Internal
    fn split_2(&self, mut idx : usize) -> [ColouredString; 2] {
        assert!(idx <= self.len(), "Byte index out of bounds.");
        let mut left     = Vec::new();
        let mut right    = Vec::new();
        let mut is_right = false;
        for part in &self.parts {
            if (! is_right) {
                if (idx == 0) {
                    is_right = true;
                    right.push(part.clone());
                } else if (idx < part.len()) {
                    is_right = true;
                    let [part_left, part_right] = part.split_2(idx);
                    left.push(ColouredStringPart::Sub(Box::new(part_left)));
                    right.push(ColouredStringPart::Sub(Box::new(part_right)));
                } else {
                    idx -= part.len();
                    left.push(part.clone());
                }
            } else {
                right.push(part.clone());
            }
        }
        let mut left_string = ColouredString::new();
        left_string.parts      = left;
        left_string.formatting = self.formatting.clone();
        let mut right_string = ColouredString::new();
        right_string.parts      = right;
        right_string.formatting = self.formatting.clone();
        return [left_string, right_string];
    }

    /// Splits this `ColouredString` into three parts
    /// at the start and end of the given range.
    /// 
    /// # Internal
    fn split_3<R : RangeBounds<usize>>(&self, range : R) -> [ColouredString; 3] {
        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded    => 0
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n + 1,
            Bound::Excluded(&n) => n,
            Bound::Unbounded    => self.len()
        };
        assert!(end   >= start,      "End index nust be greater than or equal to start index.");
        assert!(start <= self.len(), "Start byte index out of bounds.");
        assert!(end   <= self.len(), "End byte index out of bounds.");
        let [left,   right] = self.split_2(start);
        let [center, right] = right.split_2(end - left.len());
        return [left, center, right];
    }

    /// Remove all characters and formatting from this `ColouredString`.
    ///
    /// # Examples
    /// 
    /// ```
    /// use vibrance::fg;
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

    /// Returns a `ColouredString` of the text within
    /// the given range with formatting.
    /// 
    /// # Arguments
    /// 
    /// * range : A `RangeBounds<usize>` which specifies the start and end of the range to get.
    /// 
    /// # Panics
    /// 
    /// * The end of the range is greater than the length of this `ColouredString`.
    /// * The start of the range is greater than the end.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vibrance::fg;
    /// let mut r = fg::red("red" + fg::green("green")) + "none" + fg::yellow("yellow");
    /// assert_eq!(r.unformat(), "redgreennoneyellow");
    /// assert_eq!(r.get_range(5..14).unformat(), "eennoneye");
    /// ```
    pub fn get_range<R : RangeBounds<usize>>(&mut self, range : R) -> ColouredString {
        let [_, center, _] = self.split_3(range);
        return center;
    }

    /// Removes text within the given range.
    /// Returns the removed text with formatting.
    /// 
    /// # Arguments
    /// 
    /// * range : A `RangeBounds<usize>` which specifies the start and end of the range to remove.
    /// 
    /// # Panics
    /// 
    /// * The end of the range is greater than the length of this `ColouredString`.
    /// * The start of the range is greater than the end.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vibrance::fg;
    /// let mut r = fg::red("red" + fg::green("green")) + "none" + fg::yellow("yellow");
    /// assert_eq!(r.unformat(), "redgreennoneyellow");
    /// assert_eq!(r.remove_range(5..14).unformat(), "eennoneye");
    /// assert_eq!(r.unformat(), "redgrllow");
    /// ```
    pub fn remove_range<R : RangeBounds<usize>>(&mut self, range : R) -> ColouredString {
        let [left, center, right] = self.split_3(range);
        self.parts.clear();
        self.parts.push(ColouredStringPart::Sub(Box::new(left)));
        self.parts.push(ColouredStringPart::Sub(Box::new(right)));
        return center;
    }

    /// Replaced text within the given range
    /// with other text.
    /// Returns the replaced text with formatting.
    /// 
    /// # Arguments
    /// 
    /// * range : A `RangeBounds<usize>` which specifies the start and end of the range to replace.
    /// 
    /// # Panics
    /// 
    /// * The end of the range is greater than the length of this `ColouredString`.
    /// * The start of the range is greater than the end.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vibrance::fg;
    /// let mut r = fg::red("red" + fg::green("green")) + "none" + fg::yellow("yellow");
    /// assert_eq!(r.unformat(), "redgreennoneyellow");
    /// assert_eq!(r.replace_range(5..14, "replaced").unformat(), "eennoneye");
    /// assert_eq!(r.unformat(), "redgrreplacedllow");
    /// ```
    pub fn replace_range<R : RangeBounds<usize>, S : Into<ColouredString>>(&mut self, range : R, replace_with : S) -> ColouredString {
        let [left, center, right] = self.split_3(range);
        self.parts.clear();
        self.parts.push(ColouredStringPart::Sub(Box::new(left)));
        self.parts.push(ColouredStringPart::Sub(Box::new(replace_with.into())));
        self.parts.push(ColouredStringPart::Sub(Box::new(right)));
        return center;
    }

    /// Shortens this `ColouredString` to the specified length.
    /// 
    /// # Arguments
    /// 
    /// * length : The maximum length of the new `ColouredString`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vibrance::fg;
    /// let mut t = fg::red("red" + fg::green("green")) + "none" + fg::yellow("yellow");
    /// assert_eq!(t.unformat(), "redgreennoneyellow");
    /// t.truncate(7);
    /// assert_eq!(t.unformat(), "redgree");
    /// ```
    pub fn truncate(&mut self, length : usize) {
        let     [left, _] = self.split_2(length);
        let mut result    = ColouredString::new();
        result.parts.push(ColouredStringPart::Sub(Box::new(left)));
        *self = result;
    }

    /// Inserts a `ColouredStringPart` at the given index.
    /// 
    /// # Internal
    fn insert_piece(&mut self, idx : usize, piece : ColouredStringPart) {
        let     [left, right] = self.split_2(idx);
        let mut result        = ColouredString::new();
        result.parts.push(ColouredStringPart::Sub(Box::new(left)));
        result.parts.push(piece);
        result.parts.push(ColouredStringPart::Sub(Box::new(right)));
        *self = result;
    }

    /// Inserts text at the given index.
    /// 
    /// # Arguments
    /// 
    /// * index : A usize which specifies the position at which to insert.
    /// 
    /// # Panics
    /// 
    /// * The index is greater than the length of this `ColouredString`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vibrance::fg;
    /// let mut i = fg::red("red" + fg::green("green")) + "none" + fg::yellow("yellow");
    /// assert_eq!(i.unformat(), "redgreennoneyellow");
    /// i.insert(13, "insertion");
    /// assert_eq!(i.unformat(), "redgreennoneyinsertionellow");
    /// ```
    pub fn insert<S : Into<ColouredString>>(&mut self, index : usize, string : S) {
        self.insert_piece(index, ColouredStringPart::Sub(Box::new(string.into())));
    }
    
    /// Returns the sum of the lengths of each part.
    /// This uses the [String::len] method, so it
    /// might not be what a human considers the
    /// length of the string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vibrance::fg;
    /// let a = fg::red("foo");
    /// assert_eq!(3, a.len());
    /// ```
    pub fn len(&self) -> usize {
        return self.parts.iter().map(|part| part.len()).sum();
    }

    /// Returns `true` is this `ColouredString` has a
    /// length of zero, and `false` otherwise.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vibrance::ColouredString;
    /// let mut v = ColouredString::new();
    /// assert!(v.is_empty());
    /// 
    /// v.push('a');
    /// assert!(! v.is_empty())
    /// ```
    pub fn is_empty(&self) -> bool {
        return self.len() == 0;
    }

    /// Removes that last character from this `ColouredString`
    /// and returns it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vibrance::fg;
    /// let mut p = fg::red("red") + fg::blue("blue");
    /// assert_eq!(p.unformat(), "redblue");
    /// 
    /// assert_eq!(p.pop().unformat(), "e");
    /// assert_eq!(p.unformat(), "redblu");
    /// ```
    pub fn pop(&mut self) -> ColouredString {
        return self.remove_range(self.len() - 1 .. self.len());
    }

}

/// Formatting
impl Display for ColouredString {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        return write!(f, "{}", self.format());
    }
}


/// A fragment of a `ColouredString`.
#[derive(Debug, Clone)]
pub(crate) enum ColouredStringPart {
    /// A string.
    String(String),
    /// A coloured string as a child.
    Sub(Box<ColouredString>)
}

/// `ColouredString` helpers.
impl ColouredStringPart {

    /// See [ColouredString::unformat].
    fn unformat(&self) -> String {
        return match (self) {
            ColouredStringPart::String (string) => String::from(string),
            ColouredStringPart::Sub    (string) => (*string).unformat()
        };
    }

    /// See [ColouredString::format].
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

    /// See [ColouredString::split_2].
    fn split_2(&self, idx : usize) -> [ColouredString; 2] {
        match (self) {
            ColouredStringPart::String(string) => {
                return [ColouredString::from(&string[..idx]), ColouredString::from(&string[idx..])]
            },
            ColouredStringPart::Sub(string) => string.split_2(idx)
        }
    }

    /// See [ColouredString::len].
    fn len(&self) -> usize {
        return match (self) {
            ColouredStringPart::String (string) => string.len(),
            ColouredStringPart::Sub    (string) => string.len()
        };
    }

}
