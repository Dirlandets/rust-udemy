// https://doc.rust-lang.org/stable/book/ch03-02-data-types.html#the-tuple-type
pub fn run() {
    println!("\n====4.26 Generics====");
    generics()
}


struct Point<T> {
    x: T,
    y: T
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

fn generics() {
    let a = Point { x: 0, y: 0 };
    let b = Point { x: 0.1, y: 0.2 };
    let c: Point<i32> = Point { x: 1, y: 2 };
    let line = Line {start: a, end: c};
    // but cant do
    // let line = Line {start: a, end: b};
    // couse b is different type
}