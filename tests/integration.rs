use std::path::{Path, PathBuf};
use log_parser::{parse_log_file, parse_logs_folder};

#[test]
fn parses_sample_file() {
    let file_path = PathBuf::from("tests/logs/sample.log");
    let (_file_name, log_parse_results) = parse_log_file(file_path).expect("Failed to parse log file");
    assert!(log_parse_results.log_entries.len() > 0);
}

#[test]
fn parses_sample_folder() {
    let folder_path = Path::new("tests/logs");
    let result = parse_logs_folder(folder_path).expect("Failed to parse logs folder");
    assert!(result.logs_by_file.len() > 0);
}