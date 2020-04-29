/// Write code to partition a linked list around a value x, such that all nodes less than x come
/// before all nodes greater than or equal to x. If x is contained within the list, the values of x
/// only need to be after the elements less than x (see below). The partition element x can appear
/// anywhere in the "right partition"; it does not need to appear between the left and right partitions.
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

    // get the pivot number
    println!("Please provide the pivot number.");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    match buffer.trim().parse() {
        Err(_) => println!("Not a valid number!"),
        Ok(k) => {
            let elements = partion_list(&mut example_list, k);
            for i in elements.iter() {
                println!("{}", i);
            }
        }
    }
}

/// This is the function performing the manipulation. We make use of the properties of the double
/// linked list: We iteratively pop all elements from the current list and push them to a new one.
/// If the value is smaller than the pivot value, we push it to the front, otherwise to the back.
fn partion_list(list: &mut LinkedList<i32>, pivot: i32) -> LinkedList<i32> {
    let mut ret: LinkedList<i32> = LinkedList::new();

    loop {
        match list.pop_front() {
            None => break,
            Some(x) => {
                if x < pivot {
                    ret.push_front(x);
                } else {
                    ret.push_back(x);
                }
            }
        }
    }

    ret
}
