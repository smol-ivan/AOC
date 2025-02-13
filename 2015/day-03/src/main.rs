use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Hello, world!");
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(axis_x: i32, axis_y: i32) -> Point {
        Point {
            x: axis_x,
            y: axis_y,
        }
    }

    fn right(&mut self) {
        self.x += 1;
    }

    fn left(&mut self) {
        self.x -= 1;
    }

    fn up(&mut self) {
        self.y += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
    }
}

fn part_1(input: Vec<char>) {
    let mut places: HashMap<Point> = HashMap::new();

    let mut tracker = Point::new(0, 0);

    input.iter().map(|c| match c {
        '>' => tracker.right(),
        '<' => tracker.left(),
        '^' => tracker.up(),
        'v' => tracker.down(),
        _ => todo!(),
    });
}

fn read_input_file() -> Vec<char> {
    let file = fs::read_to_string("input.txt").expect("Algo salio mal");

    let input: Vec<char> = file.trim().chars().collect();

    input
}
