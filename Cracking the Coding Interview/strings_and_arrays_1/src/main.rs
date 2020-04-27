/// The goal of this exercise is to determine whether a string is comprised of all unique characters.
use std::io;

fn main() {
    loop {
        // Get a test string from the user
        println!("Please type a test string!");
        let mut test_str: String = String::new();
        io::stdin()
            .read_line(&mut test_str)
            .expect("Failed to read line.");

        // Use the test function to check it.
        match all_unique(&test_str) {
            true => println!("ALL UNIQUE!"),
            false => println!("Not all unique."),
        }
    }
}

/// This is the function that performs the actual test for uniqueness of all characters:
/// We create a vector an empty vector and then parse the string character by character.
/// If the character is not contained in the vector, we append it. Otherwise we return false.
fn all_unique(input: &str) -> bool {
    let mut buffer: Vec<char> = Vec::new();
    for c in input.chars() {
        match buffer.contains(&c) {
            true => return false,
            false => buffer.push(c),
        }
    }
    true
}
