use log_parser::{parse_log_file, parse_logs_folder, LogLevel};
use std::path::{Path, PathBuf};

const SAMPLE_LOG_PATH: &str = "tests/logs/sample.log";
const EXPECTED_FILE_NAME: &str = "sample.log";
const EXPECTED_UNREADABLE_LINE_COUNT: usize = 0;
const EXPECTED_UNPARSEABLE_LINE_COUNT: usize = 3;
const EXPECTED_PARSED_LOG_COUNT: usize = 20;
const EXPECTED_FIRST_LOG_TIMESTAMP: &str = "2025-06-25 13:00:01,001";

#[test]
fn parses_sample_file() {
    let file_path = PathBuf::from(SAMPLE_LOG_PATH);
    let (file_name, log_parse_results) =
        parse_log_file(file_path).expect("Failed to parse log file");

    assert_eq!(file_name, EXPECTED_FILE_NAME);
    assert_eq!(
        log_parse_results.statistics.unreadable_log_line_count,
        EXPECTED_UNREADABLE_LINE_COUNT
    );
    assert_eq!(
        log_parse_results.statistics.unparseable_log_line_count,
        EXPECTED_UNPARSEABLE_LINE_COUNT
    );
    assert_eq!(
        log_parse_results.statistics.parsed_log_count,
        EXPECTED_PARSED_LOG_COUNT
    );
    assert_eq!(
        log_parse_results.log_entries.len(),
        EXPECTED_PARSED_LOG_COUNT
    );

    let first_entry = &log_parse_results.log_entries[0];
    assert!(matches!(first_entry.log_level, LogLevel::Info));
    assert_eq!(
        first_entry.date.format("%Y-%m-%d %H:%M:%S,%3f").to_string(),
        EXPECTED_FIRST_LOG_TIMESTAMP
    );
}

const SAMPLE_FOLDER_PATH: &str = "tests/logs";
const SAMPLE_LOG_FILE_NAME: &str = "sample.log";
const EXPECTED_TOTAL_PARSED_LOG_COUNT: usize = 60;
const EXPECTED_UNREADABLE_LOG_FILES_COUNT: usize = 0;
const EXPECTED_TOTAL_UNREADABLE_LOG_LINE_COUNT: usize = 0;
const EXPECTED_UNPARSEABLE_LOG_LINE_COUNT: usize = 10;

#[test]
fn parses_sample_folder() {
    let folder_path = Path::new(SAMPLE_FOLDER_PATH);
    let result = parse_logs_folder(folder_path).expect("Failed to parse logs folder");

    assert!(!result.logs_by_file.is_empty(), "No log files parsed");

    assert_eq!(
        result.statistics.total_parsed_log_count,
        EXPECTED_TOTAL_PARSED_LOG_COUNT
    );
    assert_eq!(
        result.statistics.unreadable_log_files_count,
        EXPECTED_UNREADABLE_LOG_FILES_COUNT
    );

    assert_eq!(
        result.statistics.total_unreadable_log_line_count,
        EXPECTED_TOTAL_UNREADABLE_LOG_LINE_COUNT
    );

    assert_eq!(
        result.statistics.total_unparseable_log_line_count,
        EXPECTED_UNPARSEABLE_LOG_LINE_COUNT
    );

    if let Some(sample_log) = result.logs_by_file.get(SAMPLE_LOG_FILE_NAME) {
        assert_eq!(
            sample_log.statistics.parsed_log_count,
            EXPECTED_PARSED_LOG_COUNT
        );
        assert_eq!(
            sample_log.statistics.unreadable_log_line_count,
            EXPECTED_UNREADABLE_LINE_COUNT
        );
        assert_eq!(
            sample_log.statistics.unparseable_log_line_count,
            EXPECTED_UNPARSEABLE_LINE_COUNT
        );
        assert_eq!(sample_log.log_entries.len(), EXPECTED_PARSED_LOG_COUNT);
    } else {
        panic!("Expected sample.log not found in logs_by_file");
    }
}
