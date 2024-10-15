use std::env::args;

use num::abs;
use num::integer::mod_floor;

fn main() {
    let args: Vec<String> = args().collect();
    let input_string: String = args[1].clone();

    let mut encrypted = String::new();
    for char in input_string.chars() {
        encrypted.push(rotation_cipher(char, 13));
    }
    println!("Input String: {}\nEncrypted Output: {}", input_string, encrypted);

}

fn rotation_cipher(letter: char, rotate_by: i32) -> char {
    let alphabet_lowercase: Vec<char> = ('a'..='z').collect();
    let alphabet_uppercase: Vec<char> = ('A'..='Z').collect();

    let letter_pos: usize = {
        let mut iterator = 0; 
        while iterator != alphabet_lowercase.len() {
            if alphabet_lowercase[iterator] == letter || alphabet_uppercase[iterator] == letter {
                break;
            }
            iterator += 1;
        }
        
        if iterator > alphabet_lowercase.len() - 1 {
            return letter;
        }

        iterator
    };
    
    // mod_floor is the actual modulo operation while the % symbol gives the remainder
    let new_pos = mod_floor(letter_pos as i32 + rotate_by, alphabet_lowercase.len() as i32);

    // Make it positive so it can be converted to usize properly; then return
    let new_pos = abs(new_pos);
    return if letter.is_uppercase() { 
        alphabet_uppercase[new_pos as usize]
    } else {
        alphabet_lowercase[new_pos as usize]
    };
}
