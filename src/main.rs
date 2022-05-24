use std::collections::HashMap;
use adventofcode_lmh01_lib::read_file;

fn main() {
    let input = read_file("input/main.txt").unwrap();
    let mut decryption_pairs = HashMap::new();

    decryption_pairs.insert('n', 'a');
    decryption_pairs.insert('o', 'b');
    decryption_pairs.insert('p', 'c');
    decryption_pairs.insert('q', 'd');
    decryption_pairs.insert('r', 'e');
    decryption_pairs.insert('s', 'f');
    decryption_pairs.insert('t', 'g');
    decryption_pairs.insert('u', 'h');
    decryption_pairs.insert('v', 'i');
    decryption_pairs.insert('x', 'k');
    decryption_pairs.insert('y', 'l');
    decryption_pairs.insert('z', 'm');
    decryption_pairs.insert('a', 'n');
    decryption_pairs.insert('b', 'o');
    decryption_pairs.insert('c', 'p');
    decryption_pairs.insert('e', 'r');
    decryption_pairs.insert('f', 's');
    decryption_pairs.insert('g', 't');
    decryption_pairs.insert('h', 'u');
    decryption_pairs.insert('i', 'v');
    decryption_pairs.insert('j', 'w');
    decryption_pairs.insert('l', 'y');
    decryption_pairs.insert('m', 'z');
    decryption_pairs.insert('ä', 'ä');
    decryption_pairs.insert('ö', 'ö');
    decryption_pairs.insert('ü', 'ü');

    for string in input {
        let mut current_decrypted = String::new();
        for char in string.to_lowercase().chars() {
            print!("{}", char);
            current_decrypted.push(replace_char(char, &decryption_pairs));
        }
        println!();
        println!("{}", current_decrypted);
        println!();
    }
}

fn replace_char(input: char, decryption_pairs: &HashMap<char, char>) -> char {
    return if decryption_pairs.contains_key(&input) {
        *decryption_pairs.get(&input).unwrap()
    } else {
        ' '
    }
}
