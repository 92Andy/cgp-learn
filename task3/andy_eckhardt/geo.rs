struct Point {
   x: f32,
   y: f32,
}

impl Point {
    pub fn new(a: f32, b: f32) -> Point {
        Point { x: a, y: b }
    }

    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn is_origin(&self) -> bool {
        if self.x == 0.0 && self.y == 0.0 {
            true
        } else {   
            false
        }
    }

    pub fn distance(p: Point, p2: Point) -> f32 {
        let x = p2.x - p.x;
        let y = p2.y - p.y;
        (x*x + y*y).sqrt()
    }
}

struct Rectangle {
    ///bottom left
    p1: Point,
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rectangle {
        Rectangle { p1: Point::new(x,y), width: w, height: h }
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn contains(&self, p: Point) -> bool {
        if self.p1.x < p.x && self.p1.x + self.width > p.x && self.p1.y < p.y && self.p1.y + self.height > p.y {
            true 
        } else {
            false
        }
    }
}

fn main() {
    let n = Rectangle::new(1.0, 1.0, 5.0, 5.0);
    println!("{}", n.contains(Point::new(2.0, 2.0)));
}
        


    


