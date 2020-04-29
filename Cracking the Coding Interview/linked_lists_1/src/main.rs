/// The objective of this exercise is to eliminate duplicates in an unsorted linked list without
/// keeping an extra buffer of values.
/// We implement this with the doubly linked list included in the standard library.
/// In principle a single linked would be enough here and the code would be unchanged, but I prefer
/// using the standard library for the sake of the exercise.
use std::collections::LinkedList;
use std::io;

fn main() {
    // get the items of a linked list from the user
    let mut example_list: LinkedList<i32> = LinkedList::new();
    println!("Provide as many list elements (i32) as you want!");
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");
        match buffer.trim().parse() {
            Ok(x) => example_list.push_back(x),
            Err(_) => break,
        }
    }

    // eliminate duplicates and print out results
    example_list = eliminate_duplicates(&mut example_list);

    for i in &example_list {
        println!("{}", i);
    }
}

/// This is the function that eliminates the duplicates from the list
/// We implement this using two loops which is of O(n^2) in the worst case.
///
/// Keeping a buffer with all previous characters would be slightly less expensive, but needs extra
/// memory.
fn eliminate_duplicates(list: &mut LinkedList<i32>) -> LinkedList<i32> {
    let mut ret: LinkedList<i32> = LinkedList::new();
    loop {
        match list.pop_front() {
            None => break,
            Some(x) => {
                if !ret.contains(&x) {
                    ret.push_back(x)
                };
            }
        }
    }
    ret
}
