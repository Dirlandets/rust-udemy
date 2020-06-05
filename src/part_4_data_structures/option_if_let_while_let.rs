// Very nice explanation: https://learning-rust.github.io/docs/e3.option_and_result.html

pub fn run() {
    println!("\n====4.21 Option<T> and if let/while let====");

    let x = 3.0;
    let y = 0.0;

    // Option -> Some(v) | None
    let result = if y != 0.0 { Some(x / y) } else { None };

    match result {
        Some(z) => println!("{} / {} = {}", x, y, z),
        None => println!("Cannot divide by zero")
    }

    // Same result if let
    if let Some(z) = result {
        println!("Result = {}", z)
    } else {
        println!("Cannot devide by zero")
    }


    let mut norm_result = Some(0.0);

    while let Some(z) = norm_result {
        norm_result = Some(z + 1.0);

        if norm_result > Some(5.0) {
            println!("norm_result is {:?} break", norm_result);
            norm_result = None;
        } else {
            println!("norm_result {:?}", norm_result);
        }

    }
}
