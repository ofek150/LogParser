use log_parser::parse_log_file;

#[test]
fn parses_sample_file() {
    let result = parse_log_file("tests/data/sample.log").unwrap();
    assert!(result.log_entries.len() > 0);
}