//another way to express conditional
fn another_conditional() {
    let condition = true;

    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn multiple_conditional() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divided by 4");
    } else if number % 3 == 0 {
        println!("number is divided by 3");
    } else if number % 2 == 0 {
        println!("number is divided by 2");
    } else {
        println!("number is not divisible by 4,3 or 2");
    }
}

fn main() {
    let number = 32;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    multiple_conditional();
    another_conditional();
}
