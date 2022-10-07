use colourful::fg;

fn main() {
    let mut a = fg::red("red");
    a += fg::green("green");
    a.insert_string(6, "none");
    a.insert(2, fg::blue("blue"));
    a.insert(12, fg::yellow("yellow"));
    a.insert(3, fg::magenta("magenta"));
    a.push_string("none");
    println!("{}", a);
    println!("{}", a.unformat());
}
