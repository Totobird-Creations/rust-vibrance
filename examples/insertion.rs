use colourful::fg;

fn main() {
    let mut a = fg::red("red");
    a += fg::green("green");
    a.insert_string(5, "text");
    println!("{}", a);
    println!("{}", a.to_string());
}
