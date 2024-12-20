use std::fmt::{Display, Formatter, Result as FmtResult};

//example of using generic in function
fn first_element<T>(list: &[T]) -> Option<&T> {
    let first = &list.first().unwrap();
    Some(first)
}

fn last_element<T>(list: &[T]) -> Option<&T> {
    match &list.last() {
        Some(val) => return Some(val),
        None => None,
    }
}
//example of using generic in function

// example using generic on Struct
struct Point<T> {
    x: T,
    y: T,
}

struct MixPointType<T, U> {
    x: T,
    y: U,
}

//example of using generic in struct

//example of using generic in method definitions
impl<T: Display, U: Display> Display for MixPointType<T, U> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl<T: Display> Display for Point<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
//example of using generic in method definitions

//example of using generic in method definitions with constraints
impl Point<f32> {
    fn distance_from(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
//example of using generic in method definitions with constraints

//example of another mixup generic
struct AnotherMixupPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> AnotherMixupPoint<X1, Y1> {
    fn mixvalue<X2, Y2>(self, other: AnotherMixupPoint<X2, Y2>) -> AnotherMixupPoint<X1, Y2> {
        AnotherMixupPoint {
            x: self.x,
            y: other.y,
        }
    }
}

impl<X1: Display, Y1: Display> Display for AnotherMixupPoint<X1, Y1> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Another Display {{ x: {} , y: {} }}", &self.x, &self.y)
    }
}
//example of another mixup generic

fn main() {
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let first_slice: Option<&i32> = first_element(&arr);
    if let Some(val) = first_slice {
        println!("{val}");
    }

    let last_slice: Option<&i32> = last_element(&arr);
    if let Some(last) = last_slice {
        println!("{last}");
    }

    let integer_point: Point<i32> = Point { x: 5, y: 9 };
    let floating_point: Point<f32> = Point { x: 3.4, y: 8.9 };
    let mix_point: MixPointType<i32, f32> = MixPointType { x: 6, y: 9.5 };
    println!("{}", integer_point);
    println!("{}", mix_point);

    println!("{}", integer_point.x());
    println!("{}", integer_point.y());
    println!("{}", floating_point.distance_from());

    let another_mixup_point: AnotherMixupPoint<i32, i32> = AnotherMixupPoint { x: 5, y: 9 };
    let another_mixup_chare: AnotherMixupPoint<&str, &str> = AnotherMixupPoint {
        x: "Mother",
        y: "Father",
    };

    println!("{}", another_mixup_point.mixvalue(another_mixup_chare))
}
