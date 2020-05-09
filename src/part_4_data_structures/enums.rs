use std::fmt;

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8), // tuple
    CmykColor { // struct
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8
    }
}

enum Gender {
    TheMan,
    Chupakabra,
    TheWoman
}

struct User {
    gender: Gender,
    favorite_color: Color
}

pub fn run() {
    println!("\n====4.19 ENUMS====");

    // Create new color
    let c:Color = Color::Green;

    match c {
        Color::Red => println!("RED"),
        // Color::Green => println!("GREEN"),
        Color::Blue => println!("BLUE"),
        Color::RGBColor(0, 0, 0) => println!("BLACK"),
        // Если не сматчитлся, то напечатать "SOME COLOR"
        // Попробуй закомментить Color::Green => println!("GREEN").
        // Так же попробуй закоментить Color::Green => println!("GREEN") и строку _ => println!("SOME COLOR")  ниже.
        // Компилятор наорет на тебя, что не все случаи обраьотаны "pattern <Pattern> not covered"
        _ => println!("SOME COLOR")
    };

    let rgb_black: Color = Color::RGBColor(0, 0, 0);
    match_color(rgb_black);

    let some_user: User = User {
        gender: Gender::Chupakabra,
        favorite_color: Color::RGBColor(1,2,3)
    };
    println!("The user gender is {}, and user like {} color", 
        match_gender(some_user.gender), 
        match_color(some_user.favorite_color)
    );

    let enother_user: User = User {
        gender: Gender::TheMan,
        favorite_color: Color::CmykColor{
            cyan: 1, magenta: 2, yellow: 3, black: 255
        }
    };
    println!("The user gender is {}, and user like {} color", 
        match_gender(enother_user.gender), 
        match_color(enother_user.favorite_color)
    );

    let one_more_user: User = User {
        gender: Gender::TheWoman,
        favorite_color: Color::CmykColor{
            cyan: 1, magenta: 2, yellow: 3, black: 30
        }
    };
    println!("The user gender is {}, and user like {} color", 
        match_gender(one_more_user.gender), 
        match_color(one_more_user.favorite_color)
    );
}

fn match_gender(gender: Gender) -> String {
    // Лучше сделать, impl fmt::Display for Gender но это будет в следующих уроках.
    match gender {
        Gender::TheMan => String::from("man"),
        Gender::TheWoman => String::from("woman "),
        Gender::Chupakabra => String::from("chupakabra"),
    }
}

fn match_color(color: Color) -> String {
    // Не переношу повторяющийся код из run() для того чтобы пример был понятным.
    // Лучше сделать, impl fmt::Display for Color но это будет в следующих уроках.
    match color {
        Color::Red => String::from("RED"),
        Color::Green => String::from("GREEN"),
        Color::Blue => String::from("BLUE"),
        Color::RGBColor(0, 0, 0)  // rgb black = 000
        | Color::CmykColor{cyan: _, magenta: _, yellow: _, black: 255} => format!("BLACK"), // Cmyk blaack is when black 255 "_" mean any value
        Color::RGBColor(r, g, b) => format!("rgb({}, {}, {})", r, g, b),
        Color::CmykColor{cyan: c, magenta: m, yellow: y, black: b} => format!("cmyk({}, {}, {}, {})", c, m, y, b),
        _ => String::from("SOME COLOR")
    }
}