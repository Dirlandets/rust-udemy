// https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#the-tuple-type
pub fn run() {
    println!("\n====4.24 Tuples====");
    let t = tuples();
    println!("t = {:?}", t);
    // Index
    println!("t.0 = {}", t.0);

    let (a, b) = t;
    println!("(a, b) = t");
    println!("a = {}", a);
    println!("b = {}", b);
}

fn tuples() -> (i32, i32) {
    let x = 3;
    let y = 4;
    sum_and_product(x, y)
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}