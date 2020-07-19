pub fn run() {
    println!("\n====5.28 Vector====");
    // https://doc.rust-lang.org/stable/book/ch08-01-vectors.html
    let mut v = vectors();
    println!("{:?}", v);
    v.push(5);
    println!("v.push(5); -> {:?}", v);
    let third: i32 = v[2];
    println!("The third element is {}", third);

    // does not work
    // let idx: i32 = 0;
    // v[idx] the type `[i32]` cannot be indexed by `i32`

    // work 
    let idx: usize = 3;
    v[idx];

    // panic if we try get index that is do not exists
    // like v[500]
    // solution is use get function
    match v.get(8) {
        Some(x) => println!("v.get(8) = {}", x),
        None => println!("No such ellement")
    }

    println!("Iterating");
    for val in &v {
        println!("{}", val);
    }

    for val in &v {
        println!("{}", val);
    }

    let last_value = v.pop();  // Option
    println!("Last element is {:?}", last_value);

    while let Some(x) = v.pop() {
        println!("v.pop() {}, {:?}", x, v);
    }

    let last_value = v.pop();  // Option
    println!("Last element is {:?}", last_value);
}


fn vectors() -> Vec<i32>{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    a
}