//use std::fmt::Display;
//use std::fmt::Formatter;
//use std::fmt::Result;
use std::fmt::{Display, Formatter, Result};

struct Swagger<T: Display> {
    dis: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", "#swag", self.dis, "#yolo")
    }
}

fn main() {
    let origin  = Swagger::<String> {dis: "lalala".to_string()};
    println!("{}", origin);
}
    
