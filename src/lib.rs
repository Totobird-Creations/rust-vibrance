#![allow(unused_parens)]


mod traits;
#[cfg(not(feature = "us"))]
pub use traits::Colourisable;
#[cfg(feature = "us")]
pub use traits::Colourisable as Colorizable;

pub mod consts;

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
