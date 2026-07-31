use std::fmt;

pub trait Describable {
    fn describe(&self) -> String;

    // Default method
    fn short_name(&self) -> String {
        format!("[{}]", &self.describe()[..20.min(self.describe().len())])
    }
}

pub trait Area {
    fn area(&self) -> f64;
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Describable for Circle {
    fn describe(&self) -> String {
        format!("Circle with radius {:.2}", self.radius)
    }
}

// TODO 2: Implement Describable for Rectangle
impl Describable for Rectangle {
    fn describe(&self) -> String {
        format!("Rectangle with width {:.2} and height {:.2}", self.width, self.height)
    }
}

// TODO 3: Implement fmt::Display for both Circle and Rectangle
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle(r={})", self.radius)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle(w={}, h={})", self.width, self.height)
    }
}

// Trait object - dynamic dispatch
pub fn print_area(shape: &dyn Area) {
    println!("Area = {:.4}", shape.area());
}

pub fn run() {
    let c = Circle { radius: 3.0 };
    let r = Rectangle {
        width: 4.0,
        height: 5.0,
    };

    print_area(&c);
    print_area(&r);

    println!("{}", c.describe());
    println!("{}", r.describe());
}

fn main() {
    run();
}