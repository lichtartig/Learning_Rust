use rand::prelude::*;


fn main() {
    // Make a vector filled with a range of numbers and shuffle it
    let mut gen = rand::thread_rng();
    let mut sample: Vec<i32> = (0..100).collect();
    sample.shuffle(&mut gen);

    // we wrap the sort function into a struct, because Rust does not support recursion together
    // with mutable references as function parameters and this allows to change the array in place.
    let mut my_sort = Sorter {
        sample: &mut sample,
    };

    my_sort.quick_sort(0, 99);

    for i in 0..100 {
        match sample.get(i) {
            Some(x) => println!("{}, {}", i, x),
            None => println!("{}, None", i),
        }
    }
}

struct Sorter<'life> {
    sample: &'life mut Vec<i32>,
}

impl Sorter<'_> {
    // This implements the Quick-Sort algorithm
    fn quick_sort(&mut self, left: i32, right: i32) {
        // check if the part of the vector is longer than zero
        if left < right {
            // get the divisor index from the following closure.
            // This is where the meat of the function is
            let divisor = {
                let mut i = left as usize;
                let mut j = right as usize;
                let pivot = self.sample[right as usize];
                while i < j {
                    while i < (right as usize) && self.sample[i] < pivot { i = i + 1; }
                    while (left as usize) < j && pivot <= self.sample[j] { j = j - 1; }
                    if i < j { self.sample.swap(i,j); }
                }
                if pivot < self.sample[i] { self.sample.swap(i, right as usize); }
                i
            };
            // run merge_sort recursively on the two parts of the vector
            self.quick_sort(left, divisor as i32 -1);
            self.quick_sort(divisor as i32 +1, right);
        }
    }
}
