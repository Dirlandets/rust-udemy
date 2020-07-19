// https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#the-tuple-type
pub fn run() {
    println!("\n====4.25 Pattern matching====");

    let range = 0..24;
    for x in range {
        println!{"{}: I have {} oranges", x, how_many(x)}
    }

    let point = (0, 0);
    // try:
    // let point = (0, 3);
    // let point = (3, 0);
    // let point = (3, 5);

    match point {
        (0, 0) => println!("Start"),
        (0, y) => println!("X axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (x, y) => println!("x = {}, y = {}", x, y),
    }
}


fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        9..=11 => "lots of",
        12 => "a dozen",
        _ if (x > 12) => "a huge ammount of",
        _ => "a few"
    }
}