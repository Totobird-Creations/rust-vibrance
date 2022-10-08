//! Text colourisation library.
//!
//! # Features
//! 
//! * `us` - Adds the United States spelling of the names.
//! 
//! # Examples
//! 
//! ```
//! use vibrance::{fg, bg, style};
//! println!("{}", fg::red("red " + style::bold("bold")));
//! ```


#![allow(unused_parens)]


mod traits;
#[cfg(not(feature = "us"))]
pub use traits::Colourisable;
#[cfg(feature = "us")]
pub use traits::Colourisable as Colorizable;

mod consts;
pub use consts::Formatting;

mod strings;
#[cfg(not(feature = "us"))]
pub use strings::ColouredString;
#[cfg(feature = "us")]
pub use strings::ColouredString as ColoredString;

mod functions;
pub use functions::{
    style,
    fg,
    bg
};
