use colourful::fg;


fn main() {
    println!("{}", fg::red("red"));
    println!("{}", fg::red("red") + fg::green("green"));
    println!("{}", fg::green(fg::red("red") + "green"));
    println!("{}", fg::red("red") + "none");
    println!("{}", "none" + fg::green("green"));
    println!("{}", fg::red("red" + fg::green("green")));
}
