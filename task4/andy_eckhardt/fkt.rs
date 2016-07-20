use std::ops::Add;
use std::ops::Mul;

fn funktion<T: Add + Mul + Copy>(n: T, x: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (n+x, n*x)
}

fn main() {
    println!("{:?}", funktion(3, 3));
}
