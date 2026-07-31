#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub vertices: Vec<Point>,
}

impl Polygon {
    pub fn new(vertices: Vec<Point>) -> Self {
        Polygon { vertices }
    }

    // TODO 3: Implement `perimeter(&self) -> f64`
    pub fn perimeter(&self) -> f64 {
        if self.vertices.len() < 2 {
            return 0.0;
        }

        let mut total = 0.0;
        let len = self.vertices.len();

        for i in 0..len {
            let current = &self.vertices[i];
            let next = &self.vertices[(i + 1) % len];
            total += current.distance(next);
        }

        total
    }

    // TODO 4: Implement `is_closed(&self) -> bool`
    pub fn is_closed(&self) -> bool {
        self.vertices.len() >= 3
    }
}