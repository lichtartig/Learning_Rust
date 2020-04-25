use rand::prelude::*;

fn main() {
    // Make a vector filled with a range of numbers and shuffle it
    let mut gen = rand::thread_rng();
    let mut sample: Vec<i32> = (0..100).collect();
    sample.shuffle(&mut gen);

    sample = merge_sort(&sample);

    for i in 0..100 {
        match sample.get(i) {
            Some(x) => println!("{}, {}", i, x),
            None => println!("{}, None", i),
        }
    }
}

// This implements the Merge-Sort algorithm
fn merge_sort(sub_vect: &[i32]) -> Vec<i32> {
    // if the list has only one element, it is already sorted.
    if sub_vect.len() <= 1 {
        sub_vect.to_vec()
    } else {
        // split the list into two half-lists and recursively call merge_sort on them.
        let divisor = sub_vect.len() / 2;
        let mut left_vec: Vec<i32> = merge_sort(&sub_vect[..divisor]);
        let mut right_vec: Vec<i32> = merge_sort(&sub_vect[divisor..]);
        // merge the two sub-lists in an ordered way
        merge(&mut left_vec, &mut right_vec)
    }
}

// Run through both vectors and merge them together in a zipper-style
fn merge(left_vec: &mut Vec<i32>, right_vec: &mut Vec<i32>) -> Vec<i32> {
    let mut merged: Vec<i32> = Vec::new();
    loop {
        match left_vec.get(0) {
            Some(x) => match right_vec.get(0) {
                Some(y) => {
                    if *x < *y {
                        merged.push(left_vec.remove(0));
                    } else {
                        merged.push(right_vec.remove(0));
                    }
                }
                None => {
                    merged.push(left_vec.remove(0));
                }
            },
            None => match right_vec.get(0) {
                Some(_) => {
                    merged.push(right_vec.remove(0));
                }
                None => break,
            },
        }
    }

    merged
}
