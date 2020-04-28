/// The exercise is:
/// String Rotation: Assume you have a method is Substring which checks if one word is a substring
/// of another. Given two strings, sl and s2, write code to check if s2 is a rotation of sl using
/// only one call to isSubstring (e.g., "waterbottle" is a rotation of "erbottlewat").
use std::io;

/// This can be easily solved by constructing a string that is s1 appended to s1. Then we check if
/// s2 is a substring of it.
fn main() {
    loop {
        println!("Please provide to input strings!");
        let mut s1 = String::new();
        let mut s2 = String::new();
        io::stdin()
            .read_line(&mut s1)
            .expect("Failed to read input.");
        io::stdin()
            .read_line(&mut s2)
            .expect("Failed to read input.");

        let doubled: String = format!("{}{}", &s1.trim(), &s1.trim());
        match doubled.find(&s2.trim()) {
            None => println!("s2 is not a rotation of s1."),
            Some(_) => println!("s2 is a rotation of s1."),
        }
    }
}
