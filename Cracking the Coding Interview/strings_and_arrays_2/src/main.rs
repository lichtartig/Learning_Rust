/// In this exercise the goal is to check if two strings are a permutation of each other.
use std::io;

fn main() {
    loop {
        // get two input strings from the user
        println!("Please provide to strings!");
        let mut input_a = String::new();
        let mut input_b = String::new();
        io::stdin()
            .read_line(&mut input_a)
            .expect("Failed to read line.");
        io::stdin()
            .read_line(&mut input_b)
            .expect("Failed.to read line.");

        match is_permutation(&input_a, &input_b) {
            true => println!("PERMUTATIONS!"),
            false => println!("They are not permutations of each other."),
        }
    }
}

/// This is the function that checks if the two strings are permutations of each other.
/// We do this by turning both input strings into vectors of chars, which are then sorted.
/// After sorting them, permutations are equal and we check this.
fn is_permutation(input_a: &str, input_b: &str) -> bool {
    let mut sorted_a: Vec<char> = input_a.chars().collect();
    sorted_a.sort();

    let mut sorted_b: Vec<char> = input_b.chars().collect();
    sorted_b.sort();

    if sorted_a == sorted_b {
        true
    } else {
        false
    }
}