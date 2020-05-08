/// Search in Rotated Array: Given a sorted array of n integers that has been rotated an unknown
/// number of times, write code to find an element in the array. You may assume that the array was
/// originally sorted in increasing order.
///
/// IDEA: One of the to halves has to be correctly ordered in itself, while the other contains the
/// inflection point. First find out which half is ordered and decide whether I need to look at the
/// other half at all. Otherwise repeat procedure with the non-ordered half.
fn main() {
    let numbers = vec![15, 16, 19, 20, 25, 1, 3, 4, 5, 7, 10, 14];

    for i in 0..30 {
        match modified_binary_search(&numbers, i) {
            None => println!("{}: Not in array.", i),
            Some(x) => println!("{}: at index {}", i, x),
        }
    }
}

/// The modified binary search as described above.
fn modified_binary_search(numbers: &[i32], search: i32) -> Option<i32> {
    let n = numbers.len();
    if n == 0 {
        None
    } else if numbers[n / 2] == search {
        Some((n / 2) as i32)
    } else if (numbers[0] <= search && search <= numbers[n / 2])
        || (numbers[n / 2] <= numbers[n - 1]
            && (search < numbers[n / 2] || numbers[n - 1] < search))
    {
        modified_binary_search(&numbers[..(n / 2)], search)
    } else {
        match modified_binary_search(&numbers[(n / 2)..], search) {
            None => None,
            Some(x) => Some(x + (n / 2) as i32),
        }
    }
}
