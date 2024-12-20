use crate::lib::models::model_options::ReturnOption;
use crate::lib::solver::output_result::{write_median, write_median_mode, write_mode};

pub fn solve(data: &[u32], return_option: &ReturnOption) {
    match return_option {
        ReturnOption::Median => write_median(data),
        ReturnOption::Mode => write_mode(data),
        ReturnOption::MedianAndMode => write_median_mode(data),
    }
}
