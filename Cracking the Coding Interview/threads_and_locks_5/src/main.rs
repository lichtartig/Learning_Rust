/// Call In Order: Suppose we have the following code:
/// public class Foo {
/// public Foo() { ... }
/// public void first() { ... }
/// public void second() { ... }
/// public void third() { ... }
/// }
/// The same instance of Foo will be passed to three different threads. ThreadA will call first,
/// ThreadB will call second, and threadC will call third. Design a mechanism to ensure that
/// first is called before second and second is called before third.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let foo_ptr: Arc<Mutex<Foo>> = Arc::new(Mutex::new(Foo::new()));

    let mut handles = vec![];

    // Start the first thread and make it sleep for a millisecond to make sure that there is some
    // source of randomness as to which thread starts.
    let foo_copy = Arc::clone(&foo_ptr);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1));
        let mut foo = foo_copy.lock().unwrap();
        foo.first("threadA");
        foo.counter += 1;
    });
    handles.push(handle);

    // Start the second and third thrads. They run in a loop and periodically wait to get the lock
    // on the Foo instance. If they get it, they check foo.counter if the previous steps have been
    // performed yet.
    // The important part ist, that foo.counter is part of the thread-safe mutex.
    let foo_copy = Arc::clone(&foo_ptr);
    let handle = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1));
        let mut foo = foo_copy.lock().unwrap();
        if foo.counter == 1 {
            foo.second("threadB");
            foo.counter += 1;
            break;
        }
    });
    handles.push(handle);

    let foo_copy = Arc::clone(&foo_ptr);
    let handle = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1));
        let foo = foo_copy.lock().unwrap();
        if foo.counter == 2 {
            foo.third("threadC");
            break;
        }
    });
    handles.push(handle);

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}

/// This is just the rusty implementation of the class described in the exercise
struct Foo {
    counter: usize,
}

impl Foo {
    pub fn new() -> Foo {
        Foo { counter: 0 }
    }

    pub fn first(&self, thread_name: &str) {
        println!("{} has called first().", thread_name)
    }

    pub fn second(&self, thread_name: &str) {
        println!("{} has called second().", thread_name)
    }

    pub fn third(&self, thread_name: &str) {
        println!("{} has called third().", thread_name)
    }
}
