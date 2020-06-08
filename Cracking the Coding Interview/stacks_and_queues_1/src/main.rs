/// Three in One: Describe how you could use a single array to implement three stacks.

fn main() {
    let mut stack = TripleStack::new();

    // fill the stack with example data
    stack.push_a(0).unwrap();
    stack.push_a(1).unwrap();
    for i in 5..12 {
        stack.push_b(i).unwrap();
    }
    stack.push_c(21).unwrap();
    stack.push_b(12).unwrap();
    for i in 22..29 {
        stack.push_c(i).unwrap();
    }

    println!("\nA:");
    while let Some(x) = stack.pop_a() {
        println!("{}", x);
    }

    println!("\nB:");
    while let Some(x) = stack.pop_b() {
        println!("{}", x);
    }

    println!("\nC:");
    while let Some(x) = stack.pop_c() {
        println!("{}", x);
    }
}

struct TripleStack {
    array: [i32; 18],
    head_a: i32,
    head_b: i32,
    head_c: i32,
    tail_a: i32,
    tail_b: i32,
    tail_c: i32
}

impl TripleStack {
    fn new() -> TripleStack {
        TripleStack {
            array: [0; 18],
            head_a: 0,
            head_b: 6,
            head_c: 12,
            tail_a: 0,
            tail_b: 6,
            tail_c: 12,
        }
    }

    fn size(&self) -> i32 {
        self.head_a + self.head_b + self.head_c - self.tail_a - self.tail_b - self.tail_c
    }

    fn push_a(&mut self, a: i32) -> Result<i32, i32> {
        if self.size() >= 18 {
            return Err(0);
        } else if self.head_a == self.tail_b {
            self.shift_b();
        }
        self.array[self.head_a as usize] = a;
        self.head_a = (self.head_a + 1) % 18;
        return Ok(0);
    }

    fn push_b(&mut self, a: i32) -> Result<i32, i32> {
        if self.size() >= 18 {
            return Err(0);
        } else if self.head_b == self.tail_c {
            self.shift_c();
        }
        self.array[self.head_b as usize] = a;
        self.head_b = (self.head_b + 1) % 18;
        return Ok(0);
    }

    fn push_c(&mut self, a: i32) -> Result<i32, i32> {
        if self.size() >= 18 {
            return Err(0);
        } else if self.head_c == self.tail_a {
            self.shift_a();
        }
        self.array[self.head_c as usize] = a;
        self.head_c = (self.head_c + 1) % 18;
        return Ok(0);
    }

    fn pop_a(&mut self) -> Option<i32> {
        if self.head_a == self.tail_a {
            return None
        } else {
            self.head_a = (self.head_a + 17) % 18;
            return Some(self.array[self.head_a as usize])
        }
    }

    fn pop_b(&mut self) -> Option<i32> {
        if self.head_b == self.tail_b {
            return None
        } else {
            self.head_b = (self.head_b + 17) % 18;
            return Some(self.array[self.head_b as usize])
        }
    }

    fn pop_c(&mut self) -> Option<i32> {
        if self.head_c == self.tail_c {
            return None
        } else {
            self.head_c = (self.head_c + 17) % 18;
            return Some(self.array[self.head_c as usize])
        }
    }

    fn shift_a(&mut self) {
        if self.head_a == self.tail_b {
            self.shift_b();
        }
        for i in 0..(self.head_a - self.tail_a) {
            self.array[((self.head_a + 18 - i) % 18) as usize] = self.array[((self.head_a + 17 - i) % 18) as usize];
        }
        self.head_a = (self.head_a + 1) % 18;
        self.tail_a = (self.tail_a + 1) % 18;
    }

    fn shift_b(&mut self) {
        if self.head_b == self.tail_c {
            self.shift_c();
        }
        for i in 0..(self.head_b - self.tail_b) {
            self.array[((self.head_b + 18 - i) % 18) as usize] = self.array[((self.head_b + 17 - i) % 18) as usize];
        }
        self.head_b = (self.head_b + 1) % 18;
        self.tail_b = (self.tail_b + 1) % 18;
    }

    fn shift_c(&mut self) {
        if self.head_c == self.tail_a {
            self.shift_a();
        }
        for i in 0..(self.head_c - self.tail_c) {
            self.array[((self.head_c + 18 - i) % 18) as usize] = self.array[((self.head_c + 17 - i) % 18) as usize];
        }
        self.head_c = (self.head_c + 1) % 18;
        self.tail_c = (self.tail_c + 1) % 18;
    }
}