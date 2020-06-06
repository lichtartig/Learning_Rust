/// Number Swapper: Write a function to swap a number in place (that is, without temporary variables).

fn main() {
    let mut a = 0b10111101;
    let mut b = 0b10000100;
    println!("a before swap: {}", a);
    println!("b before swap: {}", b);
    number_swapper(&mut a, &mut b);
    println!("a after swap:  {}", a);
    println!("b after swap:  {}", b);

    println!("\n");

    let mut a = 17;
    let mut b = 256;
    println!("a before swap: {}", a);
    println!("b before swap: {}", b);
    number_swapper(&mut a, &mut b);
    println!("a after swap:  {}", a);
    println!("b after swap:  {}", b);
}

/// We perform the task by applying the XOR operator 3 times
fn number_swapper(a: &mut i32, b: &mut i32) {
    *a = *a ^ *b;
    *b = *a ^ *b;
    *a = *a ^ *b;
}