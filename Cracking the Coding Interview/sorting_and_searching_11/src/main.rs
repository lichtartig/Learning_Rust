/// Peaks and Valleys: In an array of integers, a "peak" is an element which is greater than or
/// equal to the adjacent integers and a "valley" is an element which is less than or equal to the
/// adjacent integers. For example, in the array {5, 8, 6, 2, 3, 4, 6}, {8, 6} are peaks and {5, 2}
/// are valleys. Given an array of integers, sort the array into an alternating sequence of peaks
/// and valleys.
///
/// EXAMPLE
/// Input: {5, 3, 1, 2, 3}
/// Output: {5, 1, 3, 2, 3}

fn main() {
    let data = vec!(5, 3, 1, 2, 3);
    let out = alternating_series(&data);
    for i in &out {
        println!("{}", i);
    }
}

fn alternating_series(data: &Vec<i32>) -> Vec<i32> {
    let mut a = data.clone();
    a.sort();

    let mut ret: Vec<i32> = Vec::new();
    loop {
        match a.pop() {
            None => break,
            Some(i) => ret.push(i),
        };
        if a.len() > 0 {
            ret.push(a[0]);
            a.remove(0);
        }
    }

    return ret;
}
