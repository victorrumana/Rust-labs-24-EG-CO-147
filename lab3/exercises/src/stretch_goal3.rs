pub struct Student {
    pub name: String,
    pub scores: Vec<f64>,
}

pub trait GradeReport {
    fn average(&self) -> f64;
    fn highest(&self) -> f64;
    fn lowest(&self) -> f64;
    fn letter_grade(&self) -> char;
}

impl GradeReport for Student {
    fn average(&self) -> f64 {
        if self.scores.is_empty() { return 0.0; }
        self.scores.iter().sum::<f64>() / self.scores.len() as f64
    }

    fn highest(&self) -> f64 {
        self.scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }

    fn lowest(&self) -> f64 {
        self.scores.iter().cloned().fold(f64::INFINITY, f64::min)
    }

    fn letter_grade(&self) -> char {
        match self.average() {
            avg if avg >= 70.0 => 'A',
            avg if avg >= 60.0 => 'B',
            avg if avg >= 50.0 => 'C',
            avg if avg >= 45.0 => 'D',
            _ => 'F',
        }
    }
}

pub fn run() {
    println!("=== LAB 3 STRETCH GOAL ===");
    let gradebook = vec![
        Student { name: "Anika".to_string(), scores: vec![85.0, 92.0, 78.0, 90.0] },
        Student { name: "Alex".to_string(), scores: vec![62.0, 58.0, 65.0] },
    ];

    for student in &gradebook {
        println!("--- Grade Report for {} ---", student.name);
        println!("Average: {:.2}", student.average());
        println!("Highest: {:.1}", student.highest());
        println!("Lowest: {:.1}", student.lowest());
        println!("Letter Grade: {}\n", student.letter_grade());
    }
}