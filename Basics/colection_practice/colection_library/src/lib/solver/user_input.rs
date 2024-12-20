use crate::lib::models::model_options::ReturnOption;
use std::{io, u32};

pub fn read_input_option() -> ReturnOption {
    println!("\nPlease Select The Options Below\n");
    println!("0 -> Process median for list of number");
    println!("1 -> Count the value occurs most offten");
    println!("2 -> Process Both 0 and 2 options");

    let mut user_opt: String = String::new();
    io::stdin()
        .read_line(&mut user_opt)
        .expect("Failed To Read Input");
    let user_opt: u32 = user_opt.trim().parse().expect("Failed to parse number");
    let option: ReturnOption = match user_opt {
        0 => ReturnOption::Median,
        1 => ReturnOption::Mode,
        2 => ReturnOption::MedianAndMode,
        _ => panic!("Please input options provided"),
    };
    option
}

pub fn write_input_to_vector(vec: &mut Vec<u32>, mut value: String) {
    println!("\nPlease Input List of Number\n");

    io::stdin()
        .read_line(&mut value)
        .expect("Failed To Read User Input");

    for c in value.trim().chars() {
        let num: u32 = match c.to_digit(10) {
            Some(num) => num,
            None => {
                println!("Non number value will be remove: {c}");
                continue;
            }
        };

        vec.push(num);
    }
}
