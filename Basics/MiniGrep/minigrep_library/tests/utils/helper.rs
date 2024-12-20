use minigrep_library::utils::helper::{search, search_insensitive};

#[test]
fn sensitive_search() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn insensitive_search() {
    let query = "RuSt";
    let content = "\
Rust:
safe, fast ,productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_insensitive(query, content)
    );
}
