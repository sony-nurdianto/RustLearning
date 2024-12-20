use crate::exe::env_variable::get_ignore_case_env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case: get_ignore_case_env(),
        })
    }

    pub fn new(query: &str, file_path: &str) -> Config {
        Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case: get_ignore_case_env(),
        }
    }
}
