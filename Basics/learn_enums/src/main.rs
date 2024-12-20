#[derive(Debug)]
enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Messages {
    fn get_write(&self) -> String {
        if let Self::Write(x) = self {
            x.to_string()
        } else {
            "".to_string()
        }
    }

    fn call(&self) {
        match self {
            Self::Quit => println!("{:?}", self),
            Self::Move { x, y } => println!("x:{},y:{}", x, y),
            Self::Write(s) => println!("{s}"),
            Self::ChangeColor(r, g, b) => println!("{r},{g},{b}"),
        }
    }
}

fn main() {
    let q: Messages = Messages::Quit;
    let m: Messages = Messages::Move { x: 1, y: 2 };
    let w: Messages = Messages::Write(String::from("Someing"));
    let c: Messages = Messages::ChangeColor(224, 224, 223);

    q.call();
    m.call();
    w.call();
    c.call();
    let getw: String = w.get_write();
    println!("{}", getw);
}
