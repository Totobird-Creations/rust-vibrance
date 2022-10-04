#![allow(unused_parens)]


mod traits;
mod strings;
mod enums;


#[cfg(not(feature="us"))]
pub use traits::FgColour as FgColour;
#[cfg(feature="us")]
pub use traits::FgColour as FgColor;

#[cfg(not(feature="us"))]
pub use traits::BgColour as BgColour;
#[cfg(feature="us")]
pub use traits::BgColour as BgColor;

pub use strings::FormattedString;
