/// Missing Number: An array A contains all the integers from Oto n, except for one number which
/// is missing. In this problem, we cannot access an entire integer in A with a single operation.
/// The elements of A are represented in binary, and the only operation we can use to access them is
/// "fetch the jth bit of A[i] ,"which takes constant time. Write code to find the missing integer.
/// Can you do it in O(n) time?
const N: usize = 10;

fn main() {
    // initialize array as described above
    let skip = 6;
    let mut arr = [0 as u8; N-1];
    for i in 0..N-1 {
        if i < skip {
            arr[i] = i as u8;
        } else {
            arr[i] = (i+1) as u8;
        }
    }

    println!("Missing number: {}", find_missing_number(&arr, 0));
}

/// find missing number
fn find_missing_number(arr: &[u8], offset: usize) -> u8 {
    if arr.len() == 0 {
        return offset as u8;
    }

    let index = arr.len() / 2;
    for i in 0..8 {
        match (get_bit(arr[index], 7-i), get_bit((offset + index) as u8, 7-i)) {
            (true, true) | (false, false) => {},
            (_, _) => {
                return find_missing_number(&arr[..index], offset);
            },
        }
    }
    return find_missing_number(&arr[index+1..], index+offset+1);
}

/// The standard get-bit function
fn get_bit(a: u8, i: u8) -> bool {
    a & (1 << i) > 0
}