/// An animal shelter, which holds only dogs and cats, operates on a strictly "first in, first out"
/// basis. People must adopt either the "oldest" (based on arrival time) of all animals at the
/// shelter, or they can select whether they would prefer a dog or a cat (and will receive the
/// oldest animal of that type). They cannot select which specific animal they would like. Create
/// the data structures to maintain this system and implement operations such as enqueue,
/// dequeueAny, dequeueDog, and dequeueCat. You may use the built-in Linked list data structure.
use std::collections::LinkedList;

fn main() {
    let mut my_shelter = AnimalShelter::new();

    my_shelter.enqueue(String::from("Rex"), Animal::Dog);
    my_shelter.enqueue(String::from("Fuffi"), Animal::Dog);
    my_shelter.enqueue(String::from("Ramses"), Animal::Cat);
    my_shelter.enqueue(String::from("Frodo"), Animal::Dog);
    my_shelter.enqueue(String::from("Minca"), Animal::Cat);

    println!("Popping a cat: {}", my_shelter.dequeue_cat().unwrap()); // Ramses
    println!("Popping a dog: {}", my_shelter.dequeue_dog().unwrap()); // Rex
    println!("Popping any: {}", my_shelter.dequeue_any().unwrap()); // Fuffi
    println!("Popping any: {}", my_shelter.dequeue_any().unwrap()); // Frodo
    println!("Popping any: {}", my_shelter.dequeue_any().unwrap()); // Minca
}

/// The AnimalShelter data structure implementing above requirements using two LinkedLists. One for
/// cats, one for dogs. The elements are tuples of the form (name, number). The number allows us to
/// decide if the oldest cat or the oldest dog has been longer at the shelter.
struct AnimalShelter {
    cats: LinkedList<(String, usize)>,
    dogs: LinkedList<(String, usize)>,
    count: usize,
}

impl AnimalShelter {
    /// Standard constructor.
    pub fn new() -> AnimalShelter {
        AnimalShelter {
            cats: LinkedList::new(),
            dogs: LinkedList::new(),
            count: 0,
        }
    }

    /// enqueue in the dog- or the cat-queue depending on the Animal enum defined below
    pub fn enqueue(&mut self, name: String, animal: Animal) {
        match animal {
            Animal::Cat => {
                self.cats.push_back((name, self.count));
            }
            Animal::Dog => {
                self.dogs.push_back((name, self.count));
            }
        }
        self.count += 1
    }

    /// (i) Check if there are dogs left. If not, pop from cats (if there is no cat, this returns None)
    /// (ii) If there are dogs left, also check if there are cats left. If not, pop from the dogs
    /// (iii) If there are both cats and dogs, check which one has been at the shelter longer.
    pub fn dequeue_any(&mut self) -> Option<String> {
        match self.dogs.front() {
            None => self.cats.pop_front().map(|cat| cat.0),
            Some(d) => match self.cats.front() {
                None => self.dogs.pop_front().map(|dog| dog.0),
                Some(c) => {
                    if c.1 > d.1 {
                        self.dogs.pop_front().map(|dog| dog.0)
                    } else {
                        self.cats.pop_front().map(|cat| cat.0)
                    }
                }
            },
        }
    }

    pub fn dequeue_dog(&mut self) -> Option<String> {
        self.dogs.pop_front().map(|dog| dog.0)
    }

    pub fn dequeue_cat(&mut self) -> Option<String> {
        self.cats.pop_front().map(|cat| cat.0)
    }
}

/// Just a standard enum
enum Animal {
    Cat,
    Dog,
}
