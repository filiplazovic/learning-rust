#[derive(Debug)]
pub struct Structure(pub i32);

#[derive(Debug)]
pub struct Deep(pub Structure);

#[derive(Debug)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u8
}

use std::fmt;

#[derive(Debug)]
pub struct MinMax(pub i64, pub i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
