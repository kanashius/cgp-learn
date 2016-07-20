struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { curr: 1, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let next2 = self.curr + self.next;
        self.curr = self.next;
        self.next = next2;
        Some(self.curr)
    }
}

fn main() {
    for i in Fibonacci::new().take(20) {
        println!("{}", i);
    }
}
