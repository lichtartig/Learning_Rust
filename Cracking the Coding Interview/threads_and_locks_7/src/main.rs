/// In the classic problem FizzBuzz, you are told to print the numbers from 1 to n. However,
/// when the number is divisible by 3, print "Fizz". When it is divisible by 5, print "Buzz".
/// When it is divisible by 3 and 5, print "FizzBuzz". In this problem, you are asked to do this in
/// a multithreaded way. Implement a multithreaded version of FizzBuzz with four threads. One thread
/// checks for divisibility of 3 and prints "Fizz". Another thread is responsible for divisibility
/// of 5 and prints "Buzz". A third thread is responsible for divisibility of 3 and 5 and prints
/// "FizzBuzz". A fourth thread does the numbers.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Wrapt the counter in a Mutex and an Arc-pointer to implement shared ownership.
    let counter_ptr = Arc::new(Mutex::new(1));
    let mut handles = vec![];
    // this is the maximum number we will reach.
    let max = 20;

    // Start up the first thread. All following threads follow the exact same pattern.
    // We loop until 'max' is reached: Get the lock on the counter and see if it matches the
    // fizzbuzz-pattern. If so, print and increment the counter.
    let ptr_copy = Arc::clone(&counter_ptr);
    let handle = thread::spawn(move || loop {
        let mut counter = ptr_copy.lock().unwrap();
        if *counter > max {
            break;
        }
        if fizzbuzz(&counter) {
            *counter += 1;
        }
    });
    handles.push(handle);

    // Start up 2nd thread in the same way as the first.
    let ptr_copy = Arc::clone(&counter_ptr);
    let handle = thread::spawn(move || loop {
        let mut counter = ptr_copy.lock().unwrap();
        if *counter > max {
            break;
        }
        if fizz(&counter) {
            *counter += 1;
        }
    });
    handles.push(handle);

    // Start up 3rd thread in the same way as the first.
    let ptr_copy = Arc::clone(&counter_ptr);
    let handle = thread::spawn(move || loop {
        let mut counter = ptr_copy.lock().unwrap();
        if *counter > max {
            break;
        }
        if buzz(&counter) {
            *counter += 1;
        }
    });
    handles.push(handle);

    // Start up 4thd thread in the same way as the first.
    let ptr_copy = Arc::clone(&counter_ptr);
    let handle = thread::spawn(move || loop {
        let mut counter = ptr_copy.lock().unwrap();
        if *counter > max {
            break;
        }
        if plain_number(&counter) {
            *counter += 1;
        }
    });
    handles.push(handle);

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}

/// The following three functions check if the given number corresponds to their case. If so, they
/// print their sentence and return true. Otherwise they return false.
fn fizzbuzz(number: &i32) -> bool {
    if number % 3 == 0 && number % 5 == 0 {
        println!("FizzBuzz!");
        return true;
    } else {
        return false;
    }
}

fn fizz(number: &i32) -> bool {
    if number % 3 == 0 && number % 5 != 0 {
        println!("Fizz!");
        return true;
    } else {
        return false;
    }
}

fn buzz(number: &i32) -> bool {
    if number % 5 == 0 && number % 3 != 0 {
        println!("Buzz!");
        return true;
    } else {
        return false;
    }
}

fn plain_number(number: &i32) -> bool {
    if number % 3 != 0 && number % 5 != 0 {
        println!("{}", number);
        return true;
    } else {
        return false;
    }
}
