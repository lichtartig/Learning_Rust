/// The object of this exercise is to replace all spaces in a string with '%20'.
use std::io;

fn main() {
    loop {
        println!("Please provide an input string!");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line!");

        let urlified = replace_spaces(&mut input);
        println!("{}", urlified);
    }
}

/// This function handles the replcaement.
fn replace_spaces(input: &String) -> String {
    input.replace(" ", "%20")
}
