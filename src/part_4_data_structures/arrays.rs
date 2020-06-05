// Very nice explanation: https://learning-rust.github.io/docs/e3.option_and_result.html
use std::mem;

pub fn run() {
    println!("\n====4.22 Arrays / 4.23 Slices====");

    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    // can be defined as: let mut a = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    println!("a took up {} bytes", mem::size_of_val(&a));
    a[0] = 123;
    println!("a has {} elements, first is {}", a.len(), a[0]);
    println!("a took up {} bytes", mem::size_of_val(&a));

    println!("{:?}", a);


    for i in &a {
        println!("{}", i)
    }
    
    // Try to change type of array for example let b: [u16; 10] = [1; 10];
    // And see how size of array changes (mem::size_of_val(&b));)
    // Or u can contol it by let b = [1u8; 10]; <= 1u8 == number: u8 = 1
    // Note: printable array length cant be gt 32
    let b: [u8; 10] = [1; 10];

    println!("b took up {} bytes", mem::size_of_val(&b));
    println!("{:?}", b);

    for i in 0..b.len() {
        println!("{}:{}", i, b[i]);
    }

    let small_matrix: [[u8;10]; 2] = [[0; 10],[0; 10]];
    println!("\nSmall matrix");
    for l in &small_matrix {
        println!("{:?}", l);
    }

   
    let marked_cell = "[X]".to_string();
    let cell = "[ ]".to_string();

    let mut big_matrix: [[&String; 10]; 10] = [[&cell; 10]; 10];

    println!("\nBig matrix");
    for l in &big_matrix {
        println!("");
        for c in l {
            print!("{}", c);
        }
    }

    big_matrix[3][2] = &marked_cell;
    println!("\nChanged Big matrix");
    for l in &big_matrix {
        println!("");
        for c in l {
            print!("{}", c);
        }
    }
    println!("");
    use_slice(&big_matrix[0][1..9])
}

fn use_slice(slice: &[&String]) {
    println!("\nUse slice");
    // You can do every thing what you can do with arrr or vector
    println!("len of slice {}", slice.len());
    println!("First element is {:?}", slice.first());
}