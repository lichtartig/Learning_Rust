/// Magic Index: A magic index in an array A [0, ..., n-1] is defined to be an index such that
/// A[i] = i. Given a sorted array of distinct integers, write a method to find a magic index, if
/// one exists, in array A
///
/// What if the values are not distinct?

fn main() {
    let arr = [0, 1, 2, 5, 7];
    match find_magic_index(&arr, 0, 5) {
        None => println!("No magic number exists."),
        Some(x) => println!("{}", x),
    }

    let arr = [2, 3, 3, 4, 4];
    match find_magic_index(&arr, 0, 5) {
        None => println!("No magic number exists."),
        Some(x) => println!("{}", x),
    }
}

fn find_magic_index(arr: &[usize; 5], start: usize, end: usize) -> Option<usize> {
    let ind = start + (end - start) / 2;
    if end - start <= 1 {
        return None;
    } else if arr[ind] == ind {
        return Some(ind);
    } else if arr[ind] < ind {
        return find_magic_index(&arr, start, ind);
    } else {
        return find_magic_index(&arr, ind, end);
    }
}