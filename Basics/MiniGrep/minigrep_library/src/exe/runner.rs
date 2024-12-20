use std::{error::Error, fs};

use crate::{
    models::config::Config,
    utils::helper::{search, search_insensitive},
};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in result {
        print!("{line}\n");
    }
    print!("\n");

    Ok(())
}
