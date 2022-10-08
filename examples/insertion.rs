use vibrance::fg;


fn main() {
    let mut a = fg::red("red");
    a += fg::green("green");
    assert!(a.unformat() == "redgreen");
    a.insert(6, "none");
    assert!(a.unformat() == "redgrenoneen");
    a.insert(2, fg::blue("blue"));
    assert!(a.unformat() == "rebluedgrenoneen");
    a.insert(12, fg::yellow("yellow"));
    assert!(a.unformat() == "rebluedgrenoyellowneen");
    a.insert(3, fg::magenta("magenta"));
    assert!(a.unformat() == "rebmagentaluedgrenoyellowneen");
    a.push("none");
    assert!(a.unformat() == "rebmagentaluedgrenoyellowneennone");

    println!("{}", a);
    println!("{}", a.unformat());

    a.truncate(24);
    assert!(a.unformat() == "rebmagentaluedgrenoyello");
    a.replace_range(..4, fg::cyan("cyan"));
    assert!(a.unformat() == "cyanagentaluedgrenoyello");
    a.replace_range(3..6, fg::white("white"));
    assert!(a.unformat() == "cyawhiteentaluedgrenoyello");
    a.replace_range(23.., fg::red("red"));
    assert!(a.unformat() == "cyawhiteentaluedgrenoyered");

    println!("{}", a);
    println!("{}", a.unformat());
}
