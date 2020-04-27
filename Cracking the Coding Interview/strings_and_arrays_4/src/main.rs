use std::collections::HashMap;
/// The object of this exercise is to see if a string provided by the user is a permutation of a
/// palindrome, where the palindrome does not have to be an actual word.
/// (E.g. 'abcba' is considered a palindrome.)
use std::io;

fn main() {
    loop {
        // Get a string from the user and check it.
        println!("Please provide a string to check!");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match permutation_of_palindrome(&input) {
            false => println!("The string is not a permutation of a palindrome."),
            true => println!("PERMUTATION OF PALINDROME"),
        }
    }
}

/// This is the function that checks if a string is a permutation of a palindrome.
/// It returns None, if it is not a perm. of a palindrome and Some(palindrome) otherwise.
/// In a permutation of a palindrome all but one character have to occur in even numbers.
/// This is what we check for in this function.
fn permutation_of_palindrome(input: &str) -> bool {
    // Make a hash map of counters and loop over the input characters to count
    let mut lookup: HashMap<char, usize> = HashMap::new();
    for c in input.chars() {
        // Get the reference of the counter for this char or initialize it with 0 then increment.
        if c != '\n' {
            let counter = lookup.entry(c).or_insert(0);
            *counter += 1;
        }
    }

    // now check if there is more than one key that as an odd count
    let mut odd_numbers = 0;
    for (_, value) in lookup {
        if value % 2 != 0 {
            odd_numbers += 1;
        }
        if odd_numbers > 1 {
            return false;
        }
    }

    true
}
