/// Group Anagrams: Write a method to sort an array of strings so that all the anagrams are next to
/// each other.
///
/// IDEA: We use a standard sorting algorithm, but when we pass each string to it, we always compare
/// the first and the last letter: Depending on which comes first in the alphabet we pass the string
/// itself or its reverse.
/// This way anagrams automatically end up next to each other.
///
/// I can see no reason why either MergeSort or QuickSort would perform better here, as no info is
/// given about the data size and memory usage.

fn main() {
    // A data sample to test the algorithm
    let data = vec![
        "Madrid",
        "Heidelberg",
        "dirdaM",
        "znatsnoK",
        "grebledieH",
        "nnoB",
        "Konstanz",
        "hcruhctsirhC",
        "Southampton",
        "Ittenbach",
        "hcabnettI",
        "Bonn",
        "Christchurch",
        "notpmahtuoS",
    ];

    // use the custom-merge sort algorithm to sort it as described above
    let data = mergesort(&data);

    for &s in data.iter() {
        println!("{}", s);
    }
}

/// The following to functions implement the standard merge-sort algorithm
fn mergesort<'a>(data_slice: &'a [&str]) -> Vec<&'a str> {
    if data_slice.len() <= 1 {
        data_slice.to_vec()
    } else {
        let mut left = mergesort(&data_slice[..(data_slice.len() / 2)]);
        let mut right = mergesort(&data_slice[(data_slice.len() / 2)..]);
        merge(&mut left, &mut right)
    }
}

fn merge<'a>(left: &mut Vec<&'a str>, right: &mut Vec<&'a str>) -> Vec<&'a str> {
    let mut merged_vec = Vec::new();
    loop {
        let l = left.first();
        let r = right.first();
        if l == None && r == None {
            break;
        } else if r == None {
            merged_vec.push(*l.unwrap());
            left.remove(0);
        } else if l == None || is_smaller(*r.unwrap(), *l.unwrap()) {
            merged_vec.push(*r.unwrap());
            right.remove(0);
        } else {
            merged_vec.push(*l.unwrap());
            left.remove(0);
        }
    }
    merged_vec
}

/// This is just the custom "<" operator, but Rust does not support operator overloading for
/// std-datastructures like &str.
/// This is where the essential Anagram-logic is implemented as explained at the beginning of this
/// file.
fn is_smaller(first: &str, second: &str) -> bool {
    let mut left = String::from(first);
    if first[..1] > first[first.len() - 1..] {
        left = left.chars().rev().collect();
    }

    let mut right = String::from(second);
    if second[..1] > second[second.len() - 1..] {
        right = right.chars().rev().collect();
    }

    left < right
}
