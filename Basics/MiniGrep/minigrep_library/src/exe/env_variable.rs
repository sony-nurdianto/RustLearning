use std::env;

pub fn get_ignore_case_env() -> bool {
    env::var("IGNORE_CASE").is_ok()
}
