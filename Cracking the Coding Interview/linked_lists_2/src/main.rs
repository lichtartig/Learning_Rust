#![feature(linked_list_cursors)]
/// The object of this exercise is to find the k-th to last element of a linked list.
/// Since we have a double linked list at our disposal, we might as well start from the
/// back.
/// In a single linked list, we would need to iterate to the k-th element before starting to read
/// out the elements.
/// (more memory usages vs. more computation time)
/// Note: This has to be compiled with 'nightly'
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

    // get the k-th to n-th elements
    println!("Please provide the index k from which we should start reading out.");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    match buffer.trim().parse() {
        Err(_) => println!("Not a valid number!"),
        Ok(k) => {
            let elements = k_to_last(&mut example_list, k);
            for i in elements.iter() {
                println!("{}", i);
            }
        }
    }
}

/// This function returns the k-th to last element as a LinkedList as described above.
fn k_to_last(list: &mut LinkedList<i32>, k: usize) -> LinkedList<i32> {
    // This computes in O(1)
    let max = list.len().clone() - k;
    let mut ret: LinkedList<i32> = LinkedList::new();
    let mut cursor = list.cursor_back_mut();
    for _i in 0..max {
        match cursor.current() {
            None => break,
            Some(x) => ret.push_front(*x),
        }
        cursor.move_prev();
    }
    ret
}
