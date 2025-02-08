use std::fs;

fn main() {
    let input = read_input_file();
    part_1(input.clone());
    part_2(input);
}

fn part_2(input: Vec<char>) {
    let mapped:Vec<i32> = input
        .iter()
        .map(|x| match x {
            '(' => 1,
            ')' => -1,
            _ => todo!()
        })
        .collect();

    let mut count = 0;
    for (i, &number) in mapped.iter().enumerate() {
        count += number;
        if count == -1 {
            println!("Posicion de entrada a la base -> {}", i+1);
            return;
        }
    }
    eprintln!("Si llegaste hasta aqui, hay un error!");
}

fn part_1(input: Vec<char>) {
    let result: i32 = input
        .iter()
        .map(|x| match x {
            '(' => 1,
            ')' => -1,
            _ => todo!(),
         })
        .sum();
    println!("Resultado -> {}", result);
}

fn read_input_file() -> Vec<char> {
    let file = fs::read_to_string("input.txt")
        .expect("Hola");

    let input: Vec<char> = file
        .trim()
        .chars()
        .collect();

    println!("Se leyeron -> {} caracteres", input.len());
    input
}
