/// The object of this exercise is to encode a string by writing repeating characters as numbers.
/// That is 'aaabbccccd' becomes 'a3b2c5d1'. If the encoded string is not shorter than the original
/// one, it should not be replaced
use std::io;

fn main() {
    loop {
        // Get a string from the user
        println!("Provide a string to encode!");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line.");

        println!("The encoded string is: {}", encoder(&input));
    }
}

/// This is the function where we do the encoding. We start with an empty string and then append to
/// it by looping over the input characters
fn encoder(input: &str) -> String {
    // ret is the string to be returned. chars is an iterator over the input characters
    let mut ret = String::new();
    let mut chars = input.chars();
    // counter holds the number of repetitions of the current character, which is stored in previous
    let mut counter = 1;
    let mut previous: char;
    match chars.next() {
        None => return String::from(""),
        Some(x) => previous = x,
    }

    // loop over the characters and update counter/previous. Every time we encounter a break,
    // we append to the return-string.
    for c in chars {
        if c == previous {
            counter += 1;
        } else {
            ret.push(previous);
            ret += &counter.to_string();
            previous = c;
            counter = 1;
        }
    }

    // check if the encoded string is shorter than the input string
    if ret.len() < input.len() {
        return ret;
    } else {
        return String::from(input);
    }
}
