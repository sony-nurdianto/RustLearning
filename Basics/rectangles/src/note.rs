/* This Note is for Learning purpose .
 *
 * In this note i learn about how to export a function , variable , struct and impl struct
 * from another file.
 *
 * The Example of import module at the main.rs file
 */

// my_utils.rs

use std::fmt;

pub fn greet(name: &str) {
    println!("\nHello, {}!", name);
}

pub const PI: f64 = 3.14159;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {} , y: {} }}", self.x, self.y)
    }
}
