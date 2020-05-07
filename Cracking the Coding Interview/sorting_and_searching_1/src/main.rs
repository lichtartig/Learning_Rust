/// Sorted Merge: You are given two sorted arrays, A and B, where A has a large enough buffer at the
/// end to hold B. Write a method to merge B into A in sorted order.
fn main() {
    // consider two sorted vectors
    let mut a = vec![1,2,2,3,7,8,8,12];
    let mut b = vec![5,7,6,9];

    // Memory-wise it would be the same to create a third vector and just pop elements off a and b
    // depending on their ordering, pushing them back on the new vector.
    // But as I understand the exercise, the goal is to do this in 'a'
    for i in 0..(a.len()+b.len()) {
        match b.first() {
            None => break,
            Some(&x) if x < a[i] => {
                a.insert(i, x);
                b.remove(0);
            },
            Some(_) => {},
        }
    }

    for i in a.iter() {
        println!("{}", i);
    }
}
