use std::fmt::Display;

mod geometry;
mod utils;
mod stretch_goal5;

use geometry::shapes::Polygon;
use geometry::Point;

// --- 5.2 Exercise A ---
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

struct Important<'a> {
    content: &'a str,
}

impl<'a> Important<'a> {
    fn summarise(&self) -> &str {
        &self.content[..self.content.len().min(80)]
    }
}

// TODO 1: `first_sentence`
fn first_sentence<'a>(text: &'a str) -> &'a str {
    match text.find('.') {
        Some(index) => &text[..index],
        None => text,
    }
}

// --- 5.3 Exercise B ---
fn print_largest<T: PartialOrd + Display>(list: &[T]) {
    if list.is_empty() {
        return;
    }
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    println!("The largest is {}", largest);
}

#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("First is larger: {}", self.first);
        } else {
            println!("Second is larger: {}", self.second);
        }
    }
}

// TODO 2: `zip_with`
fn zip_with<A, B, C, F>(a: &[A], b: &[B], f: F) -> Vec<C>
where
    F: Fn(&A, &B) -> C,
{
    let min_len = a.len().min(b.len());
    let mut result = Vec::with_capacity(min_len);
    for i in 0..min_len {
        result.push(f(&a[i], &b[i]));
    }
    result
}

// --- Main Execution ---
fn main() {
    // --- Exercise A ---
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("Longest: {}", result);
    }

    let article = String::from("Rust 2024 edition brings many improvements... Full release notes coming soon.");
    let imp = Important {
        content: &article,
    };
    println!("Summary: {}", imp.summarise());

    // TODO 1 output
    let sentence = first_sentence(&article);
    println!("First sentence: \"{}\"", sentence);

    // --- Exercise B ---
    print_largest(&[34, 50, 25, 100, 65]);
    print_largest(&["mango", "apple", "banana"]);

    let p = Pair::new(5, 10);
    p.cmp_display();

    // TODO 2 output
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![10, 20, 30];
    let sums = zip_with(&nums1, &nums2, |x, y| x + y);
    println!("Zipped sums: {:?}", sums);

    // --- Exercise C ---
    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.0, 4.0);
    println!("Distance a-b: {:.2}", a.distance(&b));

    let square = Polygon {
        vertices: vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ],
    };

    // TODO 3 & TODO 4 outputs
    println!("Perimeter: {:.2}", square.perimeter());
    println!("Is closed: {}", square.is_closed());

    //Stretch_goal5---------
    println!("Printing stretch goal-------");
    stretch_goal5::run();
}
