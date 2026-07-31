#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

pub fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

pub fn describe_direction(d: &Direction) {
    match d {
        Direction::North => println!("Heading North — towards the mountains"),
        Direction::South => println!("Heading South — towards the coast"),
        Direction::East => println!("Heading East — towards the city"),
        Direction::West => println!("Heading West — towards the sunset"),
    }
}

pub fn run() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for s in &shapes {
        println!("{:?} => area = {:.2}", s, area(s));
    }

    describe_direction(&Direction::North);
}