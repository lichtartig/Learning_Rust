mod single_linked_list;

fn main() {
    let mut sll = single_linked_list::SingleLinkedList::new();


    println!("Pushing 5.");
    sll.push_front(5);
    println!("Pushing 7.");
    sll.push_front(7);
    println!("Pushing 3.");
    sll.push_front(3);
    println!("Pushing 9.");
    sll.push_front(9);

    println!("\nGetting elements:");
    for i in 0..5 {
        match sll.get(i) {
            None => println!("No element at {}", i),
            Some(x) => println!("{} at {}", x, i),
        }
    }

    println!("\nInserting elements:");
    match sll.insert(11, 2) {
        Ok(_) => println!("{} inserted at index {}.", 11, 2),
        _ => println!("Error encountered while attempting to insert {} at index {}: Index out of bounds.", 11, 2),
    }
    match sll.insert(1, 17) {
        Ok(_) => println!("{} inserted at index {}.", 11, 2),
        _ => println!("Error encountered while attempting to insert {} at index {}: Index out of bounds.", 11, 2),
    }

    println!("\nDeleting the element at index 4.");
    match sll.delete(4) {
        Ok(x) => println!("Deleted {} at index {}.", x, 4),
        _ => println!("Error encountered while attempting to delete the element at index {}: Index out of bounds.", 4),
    }

    println!("\nPopping elements:");
    while let Some(i) = sll.pop_front() {
        println!("{}", i);
    }
}
