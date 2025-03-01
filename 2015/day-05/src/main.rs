use std::fs;

fn main() {
    let input = read_input_file();
    part_1(input.clone());
}

fn part_1(input: Vec<String>) {
    let result: Vec<&String> = input.iter().filter(|word| verification(word)).collect();

    println!("Total de palabras bonitas -> {}", result.len());
}

fn verification(word: &String) -> bool {
    let mut vowels_pass = 0;
    let mut repeated_pass = false;
    let bytes = word.as_bytes();

    for i in 0..word.len() {
        // search for banned words
        if i < bytes.len() - 1 {
            let word_bytes: [u8; 2] = [bytes[i], bytes[i + 1]];
            match &word_bytes {
                b"ab" | b"cd" | b"pq" | b"xy" => return false,
                _ => {}
            }
        }

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
