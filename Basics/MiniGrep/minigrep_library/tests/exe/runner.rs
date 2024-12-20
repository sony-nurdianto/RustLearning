use minigrep_library::{exe::runner::run, models::config::Config};

#[test]
fn run_success() {
    let arg: Config = Config::new(
        "query",
        "/home/sonynurdianto/Project/RustLearning/Basics/MiniGrep/poem.txt",
    );

    let res = run(arg);
    assert!(res.is_ok());
}

#[test]
fn run_failed() {
    let arg: Config = Config::new("query", "");

    let res = run(arg);
    assert!(res.is_err());
}
