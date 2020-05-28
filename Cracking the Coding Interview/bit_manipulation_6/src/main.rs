/// Conversion: Write a function to determine the number of bits you would need to flip to convert
/// integer A to integer B
use std::io;

fn main() {
    loop {
        let mut inp_a = String::new();
        let mut inp_b = String::new();
        io::stdin().read_line(&mut inp_a).expect("Failed to read line.");
        io::stdin().read_line(&mut inp_b).expect("Failed to read line.");
        if let (Ok(a), Ok(b)) = (inp_a.trim().parse(), inp_b.trim().parse()) {
            println!("Number of bits to flip for {:0b} and {:0b}: {}", a, b, no_of_flip_bits(a, b));
        } else {
            println!("Not an integer. Go again!");
        }
    }
}


/// This function performs the task. We take the XOR operator on the two numbers. This way there's
/// a '1' at every bit that's different and a '0' otherwise. We then return the number of 1's.
fn no_of_flip_bits(a: i32, b: i32) -> i32 {
    let diff = a ^ b;
    let mut counter = 0;
    for i in 0..32 {
        if get_bit(diff, i) {
            counter += 1;
        }
    }
    counter
}

pub fn get_bit(num: i32, i: i32) -> bool { (num & (1 << i)) != 0 }