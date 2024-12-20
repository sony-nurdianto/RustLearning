fn main() {
    let mut data = String::from("original data");

    let closure1 = || {
        println!("Value of data in closure1: {}", data); // Borrowing data (default)
    };

    closure1(); // This works because data is still valid

    let mut closure2 = move || {
        data.push_str(" (modified)"); // Modify data using move
        println!("Value of data in closure2: {}", data);
    };

    closure2(); // This now works because closure2 owns data

    // println!("Original data: {}", data); this will be error because we move the ownership of
    // data to closure2
}
