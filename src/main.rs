use std::collections::HashMap;
use adventofcode_lmh01_lib::read_file;

/// This program can decipher the input text if the correct mapping is provided in the file cipher_coding.txt
///
/// The encoding of the file is as followed:
/// I|O
///
/// So if lets say a gets mapped to z the entry in the `cipher_coding.txt` file should be `a|z`.

fn main() {
    let input = read_file("input/main.txt").unwrap();

    let decryption_pairs = read_cipher_coding();
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

/// Returns the deciphered character for the input character.
fn replace_char(input: char, decryption_pairs: &HashMap<char, char>) -> char {
    return if decryption_pairs.contains_key(&input) {
        *decryption_pairs.get(&input).unwrap()
    } else {
        ' '
    }
}

/// Reads the cipher mapping from the `cipher_coding.txt` file in the `input` directory.
fn read_cipher_coding() -> HashMap<char, char>{
    let mut map = HashMap::new();
    let cipher_coding = read_file("input/cipher_coding.txt").unwrap();
    for string in cipher_coding {
        let mut key = ' ';
        let mut value= ' ';
        for (i, char) in string.to_lowercase().chars().enumerate() {
            match i {
                0 => {key = char},
                2 => {value = char},
                _ => {}
            }
        }
        map.insert(key, value);
    }
    map
}
