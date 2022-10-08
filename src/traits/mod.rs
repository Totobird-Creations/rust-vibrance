use super::consts::Formatting;
use crate::strings::ColouredString;

mod impls;

/// Allows for formatting of a certain type.
/// See `src/traits/impls.rs` for examples.
pub trait Colourisable {
    /// Create a new formatted `ColourString`.
    /// 
    /// # Arguments
    /// 
    /// * `formatting` - A `Vec<Formatting>` to add to the `ColouredString`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use vibrance::{
    ///     ColouredString,
    ///     Colourisable,
    ///     Formatting
    /// };
    /// 
    /// struct MyString;
    /// impl Colourisable for MyString {
    ///     fn formatted(self, formatting : Vec<Formatting>) -> ColouredString {
    ///         return ColouredString::from_formatting(String::from("my_string"), formatting);
    ///     }
    /// }
    /// ```
    fn formatted(self, formatting : Vec<Formatting>) -> ColouredString;
}
