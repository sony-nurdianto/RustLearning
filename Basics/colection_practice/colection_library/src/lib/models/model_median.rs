use std::collections::HashMap;

pub struct EvenMedianReturn {
    pub left_median: u32,
    pub right_median: u32,
    pub median: u32,
}

pub struct FindModeReturn {
    pub map: HashMap<u32, u32>,
    pub most_ocurs: u32,
}
