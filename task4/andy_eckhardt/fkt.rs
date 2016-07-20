use std::ops::Add;

fn funktion<T: Add>(n: T, x: T) -> <T as Add>::Output {
    n+x
}

fn main() {
    println!("{}", funktion(3, 3));
}
