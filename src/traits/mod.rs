use crate::enums::Colour;
use crate::FormattedString;

mod impl_refstr;


pub trait FgColour<'l> {
    #[cfg(not(feature = "us"))]
    fn uncoloured     (self                         ) -> FormattedString<'l>;
    #[cfg(feature = "us")]
    fn uncolored      (self                         ) -> FormattedString<'l>;
    fn black          (self                         ) -> FormattedString<'l>;
    fn red            (self                         ) -> FormattedString<'l>;
    fn green          (self                         ) -> FormattedString<'l>;
    fn yellow         (self                         ) -> FormattedString<'l>;
    fn blue           (self                         ) -> FormattedString<'l>;
    fn magenta        (self                         ) -> FormattedString<'l>;
    fn cyan           (self                         ) -> FormattedString<'l>;
    fn white          (self                         ) -> FormattedString<'l>;
    fn bright_black   (self                         ) -> FormattedString<'l>;
    fn bright_red     (self                         ) -> FormattedString<'l>;
    fn bright_green   (self                         ) -> FormattedString<'l>;
    fn bright_yellow  (self                         ) -> FormattedString<'l>;
    fn bright_blue    (self                         ) -> FormattedString<'l>;
    fn bright_magenta (self                         ) -> FormattedString<'l>;
    fn bright_cyan    (self                         ) -> FormattedString<'l>;
    fn bright_white   (self                         ) -> FormattedString<'l>;
    fn rgb            (self, r : u8, g : u8, b : u8 ) -> FormattedString<'l>;
}


pub trait BgColour<'l> {
    #[cfg(not(feature = "us"))]
    fn on_uncoloured     (self                         ) -> FormattedString<'l>;
    #[cfg(feature = "us")]
    fn on_uncolored      (self                         ) -> FormattedString<'l>;
    fn on_black          (self                         ) -> FormattedString<'l>;
    fn on_red            (self                         ) -> FormattedString<'l>;
    fn on_green          (self                         ) -> FormattedString<'l>;
    fn on_yellow         (self                         ) -> FormattedString<'l>;
    fn on_blue           (self                         ) -> FormattedString<'l>;
    fn on_magenta        (self                         ) -> FormattedString<'l>;
    fn on_cyan           (self                         ) -> FormattedString<'l>;
    fn on_white          (self                         ) -> FormattedString<'l>;
    fn on_bright_black   (self                         ) -> FormattedString<'l>;
    fn on_bright_red     (self                         ) -> FormattedString<'l>;
    fn on_bright_green   (self                         ) -> FormattedString<'l>;
    fn on_bright_yellow  (self                         ) -> FormattedString<'l>;
    fn on_bright_blue    (self                         ) -> FormattedString<'l>;
    fn on_bright_magenta (self                         ) -> FormattedString<'l>;
    fn on_bright_cyan    (self                         ) -> FormattedString<'l>;
    fn on_bright_white   (self                         ) -> FormattedString<'l>;
    fn on_rgb            (self, r : u8, g : u8, b : u8 ) -> FormattedString<'l>;
}
