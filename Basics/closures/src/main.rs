mod example;
mod models;
use std::{thread, time::Duration};

use example::example::example_one;

fn main() {
    example_one();

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating expensive closure..");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("{}", expensive_closure(30));
}
