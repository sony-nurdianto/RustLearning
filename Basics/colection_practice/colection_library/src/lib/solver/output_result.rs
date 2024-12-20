use crate::lib::{
    models::model_median::{EvenMedianReturn, FindModeReturn},
    solver::logic::{median, mode, odd_median},
};

pub fn write_median(vec: &[u32]) {
    let is_odd: bool = vec.len() / 2 == 0;

    if is_odd {
        let odd_median_data: EvenMedianReturn = odd_median(vec);
        println!("The data you provide is even");
        println!("The median from left is: {}", odd_median_data.left_median);
        println!("The median from right is: {}", odd_median_data.right_median);
        println!(
            "The median is: ({} + {}) / 2 = {}",
            odd_median_data.left_median, odd_median_data.right_median, odd_median_data.median
        );
    } else {
        println!("The Median of data you provide is: {}", median(vec));
    }
}

pub fn write_mode(vec: &[u32]) {
    let m: FindModeReturn = mode(vec);
    println!(
        "Data Information: \n{:#?}\nMost Frequent element: {}",
        m.map, m.most_ocurs
    );
}

pub fn write_median_mode(vec: &[u32]) {
    write_median(vec);
    write_mode(vec);
}
