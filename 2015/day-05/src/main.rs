use std::fs;

fn main() {
    let input = read_input_file();
    part_1(input.clone());
}

fn part_2(input: Vec<String>) {
    let result: Vec<&String> = input.iter().filter(|word| verification_2(word)).collect();

    println!("Toal de palabras mas bonitas -> {}", result.len());
}

fn verification_2(word: &String) -> bool {
    // Main loop
    for i in 0..word.len() {
        todo!();
    }
}

fn part_1(input: Vec<String>) {
    let result: Vec<&String> = input.iter().filter(|word| verification(word)).collect();

    println!("Total de palabras bonitas -> {}", result.len());
}

fn verification(word: &String) -> bool {
    // Check for banned words after anything
    if word.contains("ab") || word.contains("cd") || word.contains("pq") || word.contains("xy") {
        return false;
    }

    let mut vowels_pass = 0;
    let mut repeated_pass = false;
    let bytes = word.as_bytes();

    for i in 0..word.len() {
        // Contar las vocales
        if b"aeiou".contains(&bytes[i]) {
            match bytes[i] {
                b'a' => vowels_pass += 1,
                b'e' => vowels_pass += 1,
                b'i' => vowels_pass += 1,
                b'o' => vowels_pass += 1,
                b'u' => vowels_pass += 1,
                _ => {}
            }
        }
        // Verificar la letra repetida excepto la ultima iteracion
        if i < bytes.len() - 1 && bytes[i] == bytes[i + 1] {
            repeated_pass = true;
        }
    }
    vowels_pass >= 3 && repeated_pass
}

fn read_input_file() -> Vec<String> {
    let file = fs::read_to_string("../input.txt").unwrap();

    let input: Vec<String> = file.trim().lines().map(|line| line.to_string()).collect();

    println!("{}", &input.len());
    input
}
