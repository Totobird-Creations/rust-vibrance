use crate::{
    traits::Colourisable,
    strings::ColouredString,
    consts::Formatting
};


macro_rules! function {
    ($name:ident,$value:expr) => {
        function!($name(), $value);
    };
    ($name:ident($($arg_name:ident:$arg_type:ty),*),$value:expr) => {
        pub fn $name<S : Colourisable>(from : S, $($arg_name:$arg_type),*) -> ColouredString {
            from.formatted(vec!($value))
        }
    };
}


pub mod style {
    use super::*;
    function!(bold          , Formatting::Bold          );
    function!(faint         , Formatting::Faint         );
    function!(italic        , Formatting::Italic        );
    function!(underline     , Formatting::Underline     );
    function!(slow_blink    , Formatting::SlowBlink     );
    function!(fast_blink    , Formatting::FastBlink     );
    function!(invert        , Formatting::Invert        );
    function!(conceal       , Formatting::Conceal       );
    function!(strikethrough , Formatting::Strikethrough );
    function!(overline      , Formatting::Overline      );
    pub mod reset {
        use super::*;
        function!(intensity     , Formatting::ResetIntensity     );
        function!(italic        , Formatting::ResetItalic        );
        function!(underline     , Formatting::ResetUnderline     );
        function!(slow_blink    , Formatting::ResetSlowBlink     );
        function!(fast_blink    , Formatting::ResetFastBlink     );
        function!(invert        , Formatting::ResetInvert        );
        function!(conceal       , Formatting::ResetConceal       );
        function!(strikethrough , Formatting::ResetStrikethrough );
        function!(overline      , Formatting::ResetOverline      );
    }
}
pub mod fg {
    use super::*;
    function!(black                   , Formatting::FgBlack         );
    function!(red                     , Formatting::FgRed           );
    function!(green                   , Formatting::FgGreen         );
    function!(yellow                  , Formatting::FgYellow        );
    function!(blue                    , Formatting::FgBlue          );
    function!(magenta                 , Formatting::FgMagenta       );
    function!(cyan                    , Formatting::FgCyan          );
    function!(white                   , Formatting::FgWhite         );
    function!(bright_black            , Formatting::FgBrightBlack   );
    function!(bright_red              , Formatting::FgBrightRed     );
    function!(bright_green            , Formatting::FgBrightGreen   );
    function!(bright_yellow           , Formatting::FgBrightYellow  );
    function!(bright_blue             , Formatting::FgBrightBlue    );
    function!(bright_magenta          , Formatting::FgBrightMagenta );
    function!(bright_cyan             , Formatting::FgBrightCyan    );
    function!(bright_white            , Formatting::FgBrightWhite   );
    function!(cbit_8(n:u8)            , Formatting::Fg8Bit(n)       );
    function!(cbit_24(r:u8,g:u8,b:u8) , Formatting::Fg24Bit(r,g,b)  );
    function!(reset                   , Formatting::FgReset         );
}
pub mod bg {
    use super::*;
    function!(black                   , Formatting::BgBlack         );
    function!(red                     , Formatting::BgRed           );
    function!(green                   , Formatting::BgGreen         );
    function!(yellow                  , Formatting::BgYellow        );
    function!(blue                    , Formatting::BgBlue          );
    function!(magenta                 , Formatting::BgMagenta       );
    function!(cyan                    , Formatting::BgCyan          );
    function!(white                   , Formatting::BgWhite         );
    function!(bright_black            , Formatting::BgBrightBlack   );
    function!(bright_red              , Formatting::BgBrightRed     );
    function!(bright_green            , Formatting::BgBrightGreen   );
    function!(bright_yellow           , Formatting::BgBrightYellow  );
    function!(bright_blue             , Formatting::BgBrightBlue    );
    function!(bright_magenta          , Formatting::BgBrightMagenta );
    function!(bright_cyan             , Formatting::BgBrightCyan    );
    function!(bright_white            , Formatting::BgBrightWhite   );
    function!(cbit_8(n:u8)            , Formatting::Bg8Bit(n)       );
    function!(cbit_24(r:u8,g:u8,b:u8) , Formatting::Bg24Bit(r,g,b)  );
    function!(reset                   , Formatting::BgReset         );
}
