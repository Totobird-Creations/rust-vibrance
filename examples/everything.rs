use vibrance::{fg, bg, style};


fn main() {
    for invert in [
        style::reset::invert,
        style::invert
    ] {
        for fg in [
            fg::black,
            fg::red,
            fg::green,
            fg::yellow,
            fg::blue,
            fg::magenta,
            fg::cyan,
            fg::white,
            fg::bright_black,
            fg::bright_red,
            fg::bright_green,
            fg::bright_yellow,
            fg::bright_blue,
            fg::bright_magenta,
            fg::bright_cyan,
            fg::bright_white
        ] {
            for style in [
                style::bold,
                style::faint,
                style::italic,
                style::underline,
                style::slow_blink,
                style::fast_blink,
                style::conceal,
                style::strikethrough,
                style::overline
            ] {
                for bg in [
                    bg::black,
                    bg::red,
                    bg::green,
                    bg::yellow,
                    bg::blue,
                    bg::magenta,
                    bg::cyan,
                    bg::white,
                    bg::bright_black,
                    bg::bright_red,
                    bg::bright_green,
                    bg::bright_yellow,
                    bg::bright_blue,
                    bg::bright_magenta,
                    bg::bright_cyan,
                    bg::bright_white
                ] {
                    print!("{}", invert(fg(bg(style("@")))));
                }
                print!(" ");
            }
            print!("\n");
        }
        print!("\n");
    }
}
