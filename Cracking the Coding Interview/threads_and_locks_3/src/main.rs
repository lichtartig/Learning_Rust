/// Dining Philosophers: In the famous dining philosophers problem, a bunch of philosophers are
/// sitting around a circular table with one chopstick between each of them. A philosopher needs
/// both chopsticks to eat, and always picks up the left chopstick before the right one. A deadlock
/// could potentially occur if all the philosophers reached for the left chopstick at the same time.
/// Using threads and locks, implement a simulation of the dining philosophers problem that prevents
/// deadlocks.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let names = vec!["Confucius", "Laozi", "Zhu Xi", "Sokrates", "Kierkegaard"];

    // We create the chopsticks and wrap the into thread-safe smartpointers.
    let mut chopsticks: Vec<Arc<Mutex<Chopstick>>> = Vec::new();
    for i in 0..names.len() {
        chopsticks.push(Arc::new(Mutex::new(Chopstick::new(i as u8))));
    }

    // We initialize the philosophers with their names and their chopsticks
    let mut philosophers: Vec<Philosopher> = Vec::new();
    for i in 0..names.len() {
        philosophers.push(Philosopher::new(
            names[i],
            Arc::clone(&chopsticks[i]),
            Arc::clone(&chopsticks[(i + 1) % names.len()]),
        ));
    }

    // The dinner starts! It's an infinite dinner for demonstration :-)
    // Every philosopher takes 1 ns of break after eating to make sure there's a context switch
    let mut handles = vec![];
    for p in philosophers {
        let handle = thread::spawn(move || loop {
            p.eat();
            thread::sleep(Duration::from_nanos(1));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// This class represents a Philosopher. It has only one method: eat(), that picks up the chopsticks
/// and eats. To ensure that there are no dead-locks, we wait until we can pick up the left
/// chopstick, but then we only try once if we can also pick up the right one. Otherwise we move on.
///
/// We also left the 'naive' approach of two locks in there as a comment. Indeed this produces a
/// dead lock fairly shortly.
struct Philosopher {
    name: String,
    left_chopstick: Arc<Mutex<Chopstick>>,
    right_chopstick: Arc<Mutex<Chopstick>>,
}

impl Philosopher {
    pub fn new(
        name: &str,
        left_chopstick: Arc<Mutex<Chopstick>>,
        right_chopstick: Arc<Mutex<Chopstick>>,
    ) -> Philosopher {
        Philosopher {
            name: String::from(name),
            left_chopstick: left_chopstick,
            right_chopstick: right_chopstick,
        }
    }

    pub fn eat(&self) {
        let left = self.left_chopstick.lock().unwrap();
        left.pick_up(&self.name);

        // The following three lines are the 'naive' implementation. It produces dead locks
        //let right = self.right_chopstick.lock().unwrap();
        //right.pick_up(&self.name);
        //println!("{} eats.", self.name);

        match self.right_chopstick.try_lock() {
            Ok(right) => {
                right.pick_up(&self.name);
                println!("{} eats.", self.name);
            }
            Err(_) => {}
        }
    }
}

/// This class represents a single chopstick
struct Chopstick {
    id: u8,
}

impl Chopstick {
    pub fn new(id: u8) -> Chopstick {
        Chopstick { id: id }
    }

    pub fn pick_up(&self, name: &str) {
        println!("{} picks up chopstick no. {}.", name, self.id);
    }
}
