use std::{collections::HashSet, fs};

fn main() {
    let input = read_input_file();
    part_1(input.clone());
    part_2(input);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn part_2(input: Vec<char>) {
    let mut santa = Point::new(0, 0);
    let mut bot = Point::new(0, 0);

    let mut flag = false;

    let mut visited: HashSet<Point> = HashSet::new();

    visited.insert(santa.clone());

    for direction in input.iter() {
        let tracker = match flag {
            false => &mut santa,
            true => &mut bot,
        };

        match direction {
            '>' => tracker.right(),
            '<' => tracker.left(),
            '^' => tracker.up(),
            'v' => tracker.down(),
            _ => todo!(),
        };

        visited.insert(tracker.clone());
        flag = !flag;
    }

    let result = visited.len();

    println!("Parte 2 -> {}", result);
}

// Usar hashset en lugar de un vector ya que en el pero de los casos tendre una busqueda O(n)
fn part_1(input: Vec<char>) {
    let mut tracker = Point::new(0, 0);

    let mut visited: Vec<Point> = Vec::new();
    visited.push(tracker.clone());

    for direction in input.iter() {
        match direction {
            '>' => tracker.right(),
            '<' => tracker.left(),
            '^' => tracker.up(),
            'v' => tracker.down(),
            _ => todo!(),
        };

        if !visited.contains(&tracker) {
            visited.push(Point::new(tracker.x, tracker.y));
        }
    }

    let result = visited.len();

    println!("This many houses with at least one present -> {}", result);
}

fn read_input_file() -> Vec<char> {
    let file = fs::read_to_string("input.txt").expect("Algo salio mal");

    let input: Vec<char> = file.trim().chars().collect();

    println!("input lenght-> {}", &input.len());
    input
}
