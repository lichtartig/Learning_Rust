/// Sorted Search, No Size: You are given an array-like data structure Listy which lacks a size
/// method. It does, however, have an elementAt(i) method that returns the element at index i in
/// O(1) time. If i is beyond the bounds of the data structure, it returns -1.
/// (For this reason, the data structure only supports positive integers.)
/// Given a Listy which contains sorted, positive integers, find the index at which an element x
/// occurs. If x occurs multiple times, you may return any index.
use std::u32;

const INF: i64 = u32::MAX as i64 + 1;

fn main() {
    let listy = Listy::new(vec![1, 5, 7, 12, 19, 25, 36, 37, 38, 41, 45]);

    //test the search for the numbers between 0 and 50
    for i in 0..50 {
        match modified_binary_search(&listy, i, 0, 2048) {
            None => println!("{}: The number is not in Listy.", i),
            Some(x) => println!("{}: The number is located at {}", i, x),
        }
    }
}

/// This is the modified binary search used to solve the problem. The trick is that we consider -1
/// as infinity, such that it is greater than any number we might search for.
/// We then consider the interval of indices 0-N, if the number we are searching lies in this
/// interval, we proceed with standard binary search. If not, we consider the interval N-2N and
/// start at the top. This grows exponentially with 2 and is therefore sufficiently fast.
fn modified_binary_search(data: &Listy, search: u32, min: usize, max: usize) -> Option<usize> {
    // if the element is smaller than max, enlarge the searching window. This grows exponentially
    // with 2 and is therefore sufficiently fast
    if max_utility(data.element_at(max)) < search as i64 {
        return modified_binary_search(data, search, max, 2 * max);
    }

    let middle = min + (max - min) / 2;
    let middle_element = max_utility(data.element_at(middle));
    if (search as i64) == middle_element {
        return Some(middle as usize);
    } else if (search as i64) < middle_element && max > middle {
        return modified_binary_search(data, search, min, middle);
    } else if middle_element < (search as i64) && min < middle {
        return modified_binary_search(data, search, middle, max);
    } else {
        return None;
    }
}

/// This function simply converts -1 to a number higher than any of the permitted numbers in the
/// Listy datastructures and therefore allows easy comparison
fn max_utility(i: i64) -> i64 {
    match i {
        -1 => INF,
        x => x,
    }
}

/// The Listy data structure explained above
struct Listy {
    data: Vec<u32>,
}

impl Listy {
    pub fn new(data: Vec<u32>) -> Listy {
        Listy { data: data }
    }

    pub fn element_at(&self, i: usize) -> i64 {
        match self.data.get(i) {
            None => -1,
            Some(&x) => x as i64,
        }
    }
}
