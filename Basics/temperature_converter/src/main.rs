use std::io;
use std::time;

const CALC_NUMBER: f32 = 32.0;

enum Conversion {
    FahrenhitToCelcius,
    CelciusToFahrenhit,
}

fn print_instruction(option: &Conversion, input_number: Option<f32>) {
    const FARENHEIT_TEXT: &str = "Farenheit";
    const CELCIUS_TEXT: &str = "Celcius";

    match input_number {
        Some(input_number) => match option {
            Conversion::FahrenhitToCelcius => {
                let result: f32 = (input_number - CALC_NUMBER) * 5.0 / 9.0;
                println!("\nConvert Result From Farenheit To Celcius: {result}°");
            }
            Conversion::CelciusToFahrenhit => {
                let result: f32 = (input_number * 9.0 / 5.0) + CALC_NUMBER;
                println!("\nConvert Result From Celcius To Farenheit: {result}°");
            }
        },
        None => match option {
            Conversion::FahrenhitToCelcius => {
                println!("Input {FARENHEIT_TEXT} To Convert to {CELCIUS_TEXT}");
            }
            Conversion::CelciusToFahrenhit => {
                println!("Input {CELCIUS_TEXT} to Convert to {FARENHEIT_TEXT}");
            }
        },
    }
}

fn main() {
    let start = time::Instant::now();
    println!("Plese Select Converter:");
    println!("1.Farenheit to Celcius");
    println!("2.Celcius to Farenheit");

    let mut option: Conversion = Conversion::FahrenhitToCelcius;
    let input_number_value: f32;

    loop {
        let mut option_input = String::new();

        io::stdin()
            .read_line(&mut option_input)
            .expect("Failed To Read Line");

        match option_input.trim().parse() {
            Ok(1) => {
                break;
            }
            Ok(2) => {
                option = Conversion::CelciusToFahrenhit;
                break;
            }
            _ => {
                println!("Please Input an option Provided");
                option_input.clear();
                continue;
            }
        };
    }

    print_instruction(&option, None);

    loop {
        let mut input_number: String = String::new();

        io::stdin()
            .read_line(&mut input_number)
            .expect("Failed To Read Line");

        match input_number.trim().parse() {
            Ok(num) => {
                input_number_value = num;
                break;
            }
            Err(_) => {
                println!("Please Input A Valid Number");
                continue;
            }
        };
    }

    print_instruction(&option, Some(input_number_value));

    let elapsed = start.elapsed();
    println!("\nProgram Execution Time: {:?}", elapsed);
}

//fn apply_function<F>(function: F, option: i8,)
//where
//    F: Fn(i8) -> i8,
//{
//    function(option)
//}
