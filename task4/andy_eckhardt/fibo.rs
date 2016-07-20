use std::iter::Iterator;

struct Fibo {
    x: i32,
    y: i32,
}
impl Fibo {
    fn new() -> Self {
        Fibo { x: 1, y: 0 }
    }
}

impl Iterator for Fibo{
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.x + self.y;
        self.y = self.x;
        self.x = n;
        Some(n)
    }
}

fn main() {
    let f = Fibo::new();
    for i in f.take(20) {
        println!("{}", i);
    }
}

        
        
    
