/// Flip Bit to Win: You have an integer and you can flip exactly one bit from a 0 to a 1. Write
/// code to find the length of the longest sequence of ls you could create.

fn main() {
    let a = 0b11011101111;
    println!("{}", best_place(a));

    let a = 0b11110111011;
    println!("{}", best_place(a));
}

/// get a single bit from num
fn get_bit(num: i32, i: i32) -> bool {
    num & (1 << i) != 0
}

/// turn number into count-array
fn num_to_bin_array(num: i32) -> Vec<i32> {
    let mut current = true;
    let mut counter = 0;
    let mut ret: Vec<i32> = Vec::new();

    for i in 0..32 {
        if get_bit(num, i) == current {
            counter += 1;
        } else {
            ret.push(counter);
            counter = 1;
            current = !current;
        }
    }
    ret.push(counter);
    if current {
        ret.push(1);
    }

    ret
}

/// find best place to insert 1
fn best_place(num: i32) -> i32 {
    let arr = num_to_bin_array(num);
    let mut longest = 0;
    let mut index = -1;

    for i in (1..arr.len()).step_by(2) {
        let mut length = arr[i-1];
        if arr[i] == 1 && i+1 < arr.len() {
            length += arr[i+1];
        }
        if length > longest {
            longest = length;
            index = arr[..i].iter().sum();
        }
    }
    index
}