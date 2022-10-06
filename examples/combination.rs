use colourful::fg;


fn main() {
    println!("{}", fg::red("r"));
    println!("{}", fg::red("r") + fg::green("g"));
    println!("{}", fg::green(fg::red("r") + "g"));
    println!("{}", fg::red("r") + "=");
    println!("{}", "=" + fg::green("g"));
    println!("{}", fg::red("r" + fg::green("g")));
}
