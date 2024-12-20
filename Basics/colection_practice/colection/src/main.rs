use colection_library::lib::{
    models::model_options::ReturnOption,
    solver::solutions::solve,
    solver::user_input::{read_input_option, write_input_to_vector},
};

fn main() {
    let input_number_value: String = String::new();
    let mut input_vector: Vec<u32> = Vec::new();

    let user_option: ReturnOption = read_input_option();
    write_input_to_vector(&mut input_vector, input_number_value);

    solve(&input_vector, &user_option);
    let input_number_value: String = String::new();
    let mut input_vector: Vec<u32> = Vec::new();

    let user_option: ReturnOption = read_input_option();
    write_input_to_vector(&mut input_vector, input_number_value);

    solve(&input_vector, &user_option);
}
