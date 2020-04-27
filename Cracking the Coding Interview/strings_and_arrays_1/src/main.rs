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

/// This function performs the actual check if a string has only unique characters. It does so
/// by using a for loop over the characters and then checks if the string-slice preceding it
/// contained this character already.
fn all_unique(input: &str) -> bool {
    for i in 0..input.len() {
        if input[0..i].contains(&input[i..i+1]) {
            return false;
        }
    }
    true
}
