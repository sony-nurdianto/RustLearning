use std::fmt;
// This function bellow is for purpose of learning of struct and write clean code
#[derive(Debug)] //adding derive debug trait so we can print Struct
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub fn area_rectangle(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

pub fn area(width: u32, hight: u32) -> u32 {
    width * hight
}

pub fn dimension(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
// This function above is for purpose of learning of struct and write clean code

// This function or method bellow is what i got when reserch and learn about rust

// This how to create method .
//
//Method is like function both have a purpose of encapsulating a block of code that performs
//a specific task . however a method is a function Attached to a type like struct , enum , trait
//object etc

impl Rectangle {
    pub fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    pub fn new(dimension: (u32, u32)) -> Self {
        Self {
            width: dimension.0,
            height: dimension.1,
        }
    }
}

// This associated function is to impl Display format to struct
// note Rather than use this associated function bellow we can use #[derive(Debug)] after that we can use
// println!("{:?}");
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Rectangle {{ width: {} , height: {}",
            self.width, self.height
        )
    }
}
