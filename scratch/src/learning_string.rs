pub fn learnin_string() {
    let s: String = String::from("hello world");
    let ss: &str = &s[..5];

    match ss.chars().nth(10) {
        Some(n) => println!("{n}"),
        None => (),
    }
}
