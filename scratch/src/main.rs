use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

trait Soming {
    type Point;

    fn area(&self) -> Self::Point;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Soming for Rectangle {
    type Point = f64;

    fn area(&self) -> Self::Point {
        self.width * self.height
    }
}

fn main() {
    let kotak: Rectangle = Rectangle {
        width: 9.0,
        height: 9.0,
    };

    println!("{:?}", kotak.area());

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    let mut arr: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let r = &mut arr[..];
    let check: (&mut [i32], &mut [i32]) = split_at_mut(r, 3);
    println!("{:?}", check.0);
    println!("{:?}", check.1);

    let mut num: i32 = 5;

    let x = &num as *const i32;
    let y = &mut num as *mut i32;

    unsafe {
        println!("x is: {}", *x);
        println!("y is: {}", *y);
    }

    let rs: &i32 = &num;

    println!("{:p}", rs);
}
// use std::sync::mpsc;
// use std::thread;
// // use std::time::Duration;
//
// fn main() {
//     // --snip--
//
//     let (tx, rx) = mpsc::channel();
//
//     let tx1 = tx.clone();
//
//     let mut check: &str = "soming";
//     println!("{check}");
//     check = "to someing";
//     println!("{check}");
//
//     let mut tvec: Vec<&str> = vec!["some", "one", "you", "love"];
//     for i in tvec.iter_mut() {
//         *i = "someing";
//         // tvec.push("soming");
//         println!("{i}");
//     }
//
//     thread::spawn(move || {
//         let vals: [&str; 2] = ["halo ", "from "];
//
//         for item in vals {
//             if let Err(err) = tx1.send(item) {
//                 eprintln!("{err}")
//             }
//         }
//     });
//
//     thread::spawn(move || {
//         if let Err(err) = tx.send("the other side") {
//             eprintln!("{err}");
//         }
//     });
//
//     let mut msg: String = String::new();
//
//     for receiver in rx {
//         msg.push_str(receiver)
//     }
//
//     println!("{msg}");
// }
// use std::fmt::Write;

// enum MixValue {
//     Int(i32),
//     Float(f32),
//     Text(String),
// }

// impl MixValue {
//     fn call(&self) -> Box<dyn std::fmt::Display> {
//         match self {
//             Self::Int(int) => Box::new(*int),
//             Self::Float(float) => Box::new(*float),
//             Self::Text(text) => Box::new(text.clone()),
//         }
//     }
// }
//
// pub enum MessagesTypes {
//     Text(String),
//     Template(String),
//     Image(String),
//     Audio(String),
//     Document(String),
// }
//
// impl MessagesTypes {
//     pub fn get_value(&self) -> String {
//         match self {
//             Self::Text(v) => String::from(v),
//             Self::Template(v) => String::from(v),
//             Self::Image(v) => String::from(v),
//             Self::Audio(v) => String::from(v),
//             Self::Document(v) => String::from(v),
//         }
//     }
//
//     pub fn text() -> String {
//         let m: Self = Self::Text(String::from("text"));
//         m.get_value()
//     }
// }
//
// trait Somei {
//     type SomeingString: ToString;
//
//     fn write_somei(&self) -> Self::SomeingString;
// }
//
// impl Somei for MixValue {
//     type SomeingString = String;
//
//     fn write_somei(&self) -> Self::SomeingString {
//         "abogobogayeamplow".to_string()
//     }
// }

// fn main() {

// let mut ini_kata = String::from("ini kata");
// ini_kata.clear();
// ini_kata.write_str("kata baru").unwrap();
// println!("{ini_kata}");
// let change_kata = move |mut s: String| -> String {
//     s = "kata ini".to_string();
//     s
// };
// let ini_kata = change_kata(ini_kata);
// println!("{ini_kata}");

// let mut ini_list = vec![1, 2, 3, 4];
// let move_value = move || -> (String, Vec<i32>) {
//     ini_kata = "kata ini".to_string();
//     ini_list.push(5);
//     (ini_kata, ini_list)
// };
//
// let (k, l) = move_value();
// println!("{k}");
// println!("{l:?}");

// let m: String = MessagesTypes::text();
// println!("{m}");
//
// let s1: String = String::from("tic");
// let s2: String = String::from("tac");
// let s3: String = String::from("toe");
//
// let s1: String = format!("{} {} {}", s1, s2, s3);
// println!("{s1}");
//
// let cyrilic: String = String::from("Пожалуйста");
// let cyrilic: String = cyrilic.replace("й", "C");
// println!("{}", cyrilic);

// println!("{:?}", cyrilic.chars().nth(0).unwrap_or_else(|| 'N'));

// for w in cyrilic.chars() {
//     if w == 'й' {
//         println!("Got It")
//     }
// }
//
// let v: Vec<MixValue> = vec![
//     MixValue::Text(String::from("This Is Text")),
//     MixValue::Int(32),
//     MixValue::Float(1.24),
// ];
//
// for i in v {
//     match i {
//         MixValue::Int(int) => println!("{int}"),
//         MixValue::Float(float) => println!("{float}"),
//         MixValue::Text(text) => println!("{text}"),
//     }
// }
// let mut v: Vec<u8> = vec![5, 7, 8];
//
// for i in 0..v.len() {
//     if let Ok(index) = u8::try_from(i) {
//         v.push(index);
//     };
// }
// println!("{:?}", v)
// }
