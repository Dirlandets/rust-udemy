// 32 bits
union IntOrFloat {
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat) {
    // Т.к. компилятор хз что там у тебя,
    // Нужен блок Unsafe
    unsafe {
        match iof {
            IntOrFloat { i: 42} => {
                println!("Meaning of life");
            }

            IntOrFloat { i } => {
                println!("i Value = {}", i);
            }

        }
    }
}

pub fn run() {
    println!("\n====4.20 Unions====");

    // Тут впервые появляется unsafe 
    // Подробнее тут: https://doc.rust-lang.ru/book/ch19-01-unsafe-rust.html

    // Подробее про unions https://doc.rust-lang.org/stable/reference/items/unions.html
    let mut iof = IntOrFloat {i: 123};
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    // Так как компилятор хз что мы туда запихнули, 
    // надо матчить, при помощи отдельной функции.
    process_value(IntOrFloat { i: 32});
    process_value(IntOrFloat { i: 42});

    // Обрати внимание что напечатает эта функция
    // Она возьмет байты которые представляют это число
    // и так как оно хз что это. Оно воспримет это как i: 32
    process_value(IntOrFloat { f: 56.0});
}

