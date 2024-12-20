use std::io;
use std::time::Instant;

fn modify_word(input: &str, vowels: &str) -> String {
    let first_input_char = input.chars().nth(0).unwrap();
    for c in vowels.chars() {
        if c == first_input_char {
            return format!("{}-hay ", input);
        }
    }

    return format!(
        "{}-{}ay ",
        input.chars().skip(1).collect::<String>(),
        first_input_char
    );
}

fn main() {
    println!("Turn Word into pig-latin !");
    loop {
        println!("Input Your Text or Word:");
        let vowels = "aeiou";
        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed To Read Input");

        if input.is_empty() {
            println!("No Input Provided");
            continue;
        }

        let start_me = Instant::now();

        input = input
            .split_whitespace()
            .map(|word| modify_word(word, vowels))
            .collect::<String>();
        println!("{}", input);

        let end = Instant::now();
        let duration = end.duration_since(start_me);
        println!("Time elapsed: {:?}", duration);
    }
}
