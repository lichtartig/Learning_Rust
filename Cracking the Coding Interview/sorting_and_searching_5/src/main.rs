/// Sparse Search: Given a sorted array of strings that is interspersed with empty strings, write a
/// method to find the location of a given string.

fn main() {
    let data = vec![
        "abc", "", "", "", "bac", "", "", "", "", "", "Madrid", "", "", "Ze",
    ];

    for word in vec!["abc", "bac", "Madrid", "Ze", "nothing"] {
        match modified_binary_search(&data, word) {
            None => println!("'{}' is not in the array.", word),
            Some(i) => println!("'{}' is at position {}", word, i),
        }
    }
}

fn modified_binary_search(data: &[&str], search: &str) -> Option<usize> {
    // start at middle of data and check if contains a non-empty string. If not decrement middle
    // until we find one. Then do standard binary search.
    // Caveat: If we arrive at position 0 without finding a non-empty string, perform binary search
    // on the interval [middle;end]
    let middle = data.len() / 2;
    for i in 0..(middle + 1) {
        if data[middle - i] == "" {
            continue;
        } else if data[middle - i] == search {
            return Some(middle - i);
        } else if data[middle - i].to_lowercase() < search.to_lowercase() {
            return modified_binary_search(&data[middle..], search);
        } else {
            return modified_binary_search(&data[..(middle - i)], search);
        }
    }
    None
}
