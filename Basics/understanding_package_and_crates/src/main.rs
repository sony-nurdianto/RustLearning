use crate::garden::vegetables::{some_test, Asparagus};
use grouping_related_code_in_modules::front_house::{hosting, office::help_desk, serving};

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'am growing {plant:?}!");
    some_test();

    hosting::add_to_writelist();
    hosting::seat_to_available();
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
    help_desk::write_issue();
}

//note if you want to use this structure rather at folder rectangle
//make sure the folder name must same as module name example /garden and garden.rs
//folder /garden is for we store sub module and garden.rs at root of crate for define a module
