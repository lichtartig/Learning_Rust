/// Letters and Numbers: Given an array filled with letters and numbers, find the longest subarray
/// with an equal number of letters and numbers
use std::collections::HashMap;

fn main() {
    // example
    let arr = ["0", "5", "6", "s", "t", "v", "3", "2", "a", "1", "5", "v", "x", "y"];
    // get the longest subarray
    let (s, e) = find_longest_subarray(&arr);

    // print it
    let mut out = String::from("[");
    for t in arr[s..e].iter() {
        out += t;
        out +=  ", ";
    }
    out += "]";
    println!("Longest subarray: {}", out);
}


/// This is my implementation using a hashmpa meaning that I have to go only once through the entire
/// array.
fn find_longest_subarray(arr: &[&str]) -> (usize, usize) {
    let mut hm = HashMap::new();
    let (mut s, mut e) = (0, 0);

    let mut current = 0;
    for i in 0..arr.len() {
        match arr[i] {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => current += 1,
            _ => current -= 1
        }
        if let Some(w) = hm.get(&current) {
            if i - *w > e - s {
                s = *w;
                e = i;
            }
        } else {
            hm.insert(current, i);
        }
    }
    (s+1, e+1)
}