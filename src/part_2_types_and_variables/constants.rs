const MEANING_OF_LIFE: u8 = 42;
static mut IS_CHUCHUA: bool = true;

pub fn run() {
    println!("\n====2.11 CONSTANTS====");
    consts();

    println!("let a = 3;");
    let a = 3;
    println!("let mut b = a;");
    let mut b = a;

    assert_eq!(a, b);
    println!("b += 1;");
    b += 1;

    println!("a == {}", a);
    println!("b == {}", b);
}


fn consts() {
    println!("Meaining of life is {}", MEANING_OF_LIFE);
    unsafe {
        println!("Is chuchua? {}", IS_CHUCHUA);
        IS_CHUCHUA = false;
        println!("Turn chuchua off IS_CHUCHUA = false;");
        println!("Is chuchua? {}", IS_CHUCHUA);
    };
}