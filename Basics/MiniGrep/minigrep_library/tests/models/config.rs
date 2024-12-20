use minigrep_library::models::config::Config;

#[test]
fn build_config_success() {
    let config: Vec<String> = vec![
        "filler".to_string(),
        "read".to_string(),
        "/path".to_string(),
    ];

    let res = Config::build(config.into_iter());

    assert!(res.is_ok());
    let args = res.unwrap();
    assert_eq!(args.query, "read".to_string());
    assert_eq!(args.file_path, "/path".to_string());
}

#[test]
fn build_config_failed() {
    let config: Vec<String> = vec!["filler".to_string(), "read".to_string()];

    let res = Config::build(config.into_iter());

    assert!(res.is_err());
    assert_eq!(res.err().unwrap(), "Didn't get a file path");
}

#[test]
fn config_new() {
    let query: String = String::from("query");
    let file_path: String = String::from("file_path");

    let new_config = Config::new(&query, &file_path);

    assert!(matches!(&new_config, Config { .. }));
    assert_eq!(new_config.query, "query");
    assert_eq!(new_config.file_path, "file_path");
}
