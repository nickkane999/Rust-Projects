pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}
impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}


pub fn run () {
    println!("Hello from p1");
}
//////////////////////////////////////////////////////////////////
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/*
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
*/
pub fn run2 () {
    let p1: Point = Point { x: 1, y: 0 };
    let p2: Point = Point { x: 2, y: 3 };
    assert_eq!(p1 + p2, Point { x: 3, y: 3 });
}

//////////////////////////////////////////////////////////////////

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

//////////////////////////////////////////////////////////////////

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

pub fn run3 () {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

//////////////////////////////////////////////////////////////////

trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//////////////////////////////////////////////////////////////////

struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn run4 () {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}