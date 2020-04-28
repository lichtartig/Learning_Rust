/// The object of this exercise is to identify if two strings are related by at most one edit, where
/// by edit we mean (i) deleting a character (ii) insert a character (iii) replace a character
use std::io;

fn main() {
    loop {
        // Get two input strings from the user
        println!("Please provide two input strings!");
        let mut input_a = String::new();
        let mut input_b = String::new();
        io::stdin()
            .read_line(&mut input_a)
            .expect("Failed to read line.");
        io::stdin()
            .read_line(&mut input_b)
            .expect("Failed to read line.");

        // check if they are at most one edit away
        match one_edit_away(&input_a, &input_b) {
            true => println!("ONE EDIT AWAY!"),
            false => println!("More than one edit away."),
        }
    }
}

/// This is the function that performs the check. We go about this in the following way:
/// 1) If either of the strings is longer than the other, they need to be related by
/// insertion/deletion depending on perspective. We send those to second fct. ordered by length.
/// 2) If they are of equal length at most one character can be different.
fn one_edit_away(input_a: &str, input_b: &str) -> bool {
    // If one of the strings is longer, send it to the insertion check function
    if input_a.len() < input_b.len() {
        return one_insertion_away(input_b, input_a);
    } else if input_b.len() < input_a.len() {
        return one_insertion_away(input_a, input_b);
    }
    // If the strings are of equal length there can be at most one difference b/c of a replacement.
    let mut no_of_diff = 0;
    for i in 0..input_a.len() {
        if input_a[i..i + 1] != input_b[i..i + 1] {
            no_of_diff += 1;
        }
        if no_of_diff > 1 {
            return false;
        }
    }
    true
}

/// This function checks whether to strings are related by one insertion. We do this as follows:
/// We iterate through the long string and compare the character to the character an additional
/// variable 'pos' points to. If they are the same, we increment 'pos'.
/// If at one point the difference of 'pos' and the for-loop iterator is greater than 1, there is
/// more than 1 insertion necessary.
fn one_insertion_away(long_inp: &str, short_inp: &str) -> bool {
    let mut pos: usize = 0;
    for i in 0..long_inp.len() {
        if i - pos > 1 {
            return false;
        }
        if long_inp[i..i + 1] == short_inp[pos..pos + 1] {
            pos += 1;
        }
    }
    true
}
