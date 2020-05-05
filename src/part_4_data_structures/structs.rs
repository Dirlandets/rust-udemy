struct Point {
    x: usize,
    y: usize,
}

struct Matrix {
    matrix: Vec<Vec<String>>
}

impl Matrix {
    /// Функция которая создает матрицу из height и width
    /// TODO: Вернуться и переписать когда пройду курс до конца.
    /// Сейчас не хватает инструментария и понимания :/
    fn new(height: u8, width: u8) -> Matrix {
        let mut matrix: Vec<Vec<String>> = Vec::new();
        for _ in 0..height {
            let mut line: Vec<String> = vec![];
            for _ in 0..width {
                let v = "[ ]".to_string();
                line.push(v);
            }
            matrix.push(line);
        }
        Matrix { matrix: matrix}
    }
    
    /// Функция должна менять "[ ]" на "[*]" в соответсвии с координатами в Point
    fn place_points(&mut self, points: Vec<&Point> )  {
        for point in points {
           // Берем строку x в ней находим элемент y и меняем на "[*]"
           self.matrix[point.x][point.y] = "[*]".to_string();
        }
    }

    fn print_matrix(&self) {
        println!("");
        for line in self.matrix.iter() {
            println!("");
            for c in line {
                print!("{}", c);
            }
        }
    }
}

pub fn run() {
    let p1 = Point {x: 1, y: 3};
    let p2 = Point {x: 8, y: 6};
    let points = vec![&p1, &p2];

    println!("p1 ({}, {})", p1.x, p1.y);
    println!("p2 ({}, {})", p2.x, p2.y);

    let mut matrix = Matrix::new(10, 10);
    // Печатаем пустую матрицу
    matrix.print_matrix();
    // Ставим точки на матрице
    matrix.place_points(points);
    // Печатаем матрицу с точками
    matrix.print_matrix();
}