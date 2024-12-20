use crate::lib::models::model_median::{EvenMedianReturn, FindModeReturn};
use std::collections::HashMap;

pub fn odd_median(vec: &[u32]) -> EvenMedianReturn {
    let median_right: u32 = vec[(vec.len() / 2) - 1];
    let median_left: u32 = vec[vec.len() / 2];
    let odd_median: u32 = (median_left + median_right) / 2;

    let median: EvenMedianReturn = EvenMedianReturn {
        left_median: median_left,
        right_median: median_right,
        median: odd_median,
    };

    median
}

pub fn median(vec: &[u32]) -> u32 {
    let m: u32 = vec[vec.len() / 2];
    m
}

pub fn mode(vec: &[u32]) -> FindModeReturn {
    let mut map: HashMap<u32, u32> = HashMap::new();
    for i in vec.iter() {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut most_frequent: u32 = 0;
    let mut max_count: u32 = 0;
    for (element, count) in &map {
        if count > &max_count {
            most_frequent = *element;
            max_count = *count;
        }
    }

    let m: FindModeReturn = FindModeReturn {
        map,
        most_ocurs: most_frequent,
    };
    m
}
