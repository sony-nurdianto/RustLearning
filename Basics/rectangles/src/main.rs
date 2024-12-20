use rectangle::calc::{area, area_rectangle, dimension, Rectangle};

mod note;
mod rectangle;

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "\nThe area of rectangle is {} square pixels",
        area(width1, height1)
    );

    let rect = (width1, height1);

    println!("The area of dimension is {} square pixels", dimension(rect));

    let rctngl = Rectangle {
        width: 30,
        height: 50,
    };

    let scale = 2;
    let rctngl2 = Rectangle {
        width: dbg!(30 * scale),
        ..rctngl
    };

    dbg!(&rctngl2); //This is to print struct with dbg macro

    println!(
        "The area of rectangle using struct {}",
        area_rectangle(&rctngl)
    );

    let rctngl3 = Rectangle::new((32, 70));

    println!("{rctngl3:?}");

    //from this line bellow is my reserch

    println!(
        "The area of rectangle using struct impl {}",
        rctngl.calculate_area()
    );

    println!("Print with derive Debug trait for struct(Rectangle) => {rctngl:?}");

    note::greet("Sony");

    let pi = note::PI;
    let point = note::Point::new(30, 50);
    println!("This is the value of PI(Ⲡ): {pi}");
    println!("This is point struct: {point}");
    println!("Print with explicit impl fmt::Display for struct(Rectangle) => {rctngl}")
}
