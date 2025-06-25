use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use log::{error, warn};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::thread;
use tracing::instrument;

use crate::constants::DATE_STRING_LEN;
use crate::errors::ParseFileError;
use crate::model::{
    FolderParseResults, FolderParseStatistics, LogEntry, LogLevel, LogParseResults,
    LogParseStatistics,
};
use crate::utils::{extract_file_name_from_path, list_files_in_folder};

#[instrument]
fn process_log_line(log_line: &str) -> Option<LogEntry> {
    let date = extract_date(log_line)
        .map_err(|e| error!("Invalid date: {}", e))
        .ok()?;
    let log_level = extract_log_level(log_line)
        .map_err(|e| error!("Invalid log level: {}", e))
        .ok()?;
    Some(LogEntry { log_level, date })
}

#[instrument]
fn spawn_file_parsing_threads(
    file_paths: Vec<PathBuf>,
) -> Vec<thread::JoinHandle<Result<(String, LogParseResults), ParseFileError>>> {
    file_paths
        .into_iter()
        .map(|file_path| thread::spawn(move || parse_log_file(file_path)))
        .collect()
}

#[instrument]
fn collect_results(
    handles: Vec<thread::JoinHandle<Result<(String, LogParseResults), ParseFileError>>>,
) -> FolderParseResults {
    let mut logs_by_file = HashMap::new();
    let mut unreadable_log_files_count = 0;
    let mut total_parsed_log_count = 0;
    let mut total_unreadable_log_line_count = 0;
    let mut total_unparseable_log_line_count = 0;

    for handle in handles {
        match handle.join() {
            Ok(Ok((file_name, log_parse_result))) => {
                total_parsed_log_count += log_parse_result.statistics.parsed_log_count;
                total_unreadable_log_line_count +=
                    log_parse_result.statistics.unreadable_log_line_count;
                total_unparseable_log_line_count +=
                    log_parse_result.statistics.unparseable_log_line_count;
                logs_by_file.insert(file_name, log_parse_result);
            }
            Ok(Err(e)) => {
                match &e {
                    ParseFileError::FileNameError { path, .. }
                    | ParseFileError::ParseError { path, .. } => {
                        error!("Failed to parse '{}': {}", path.display(), e);
                    }
                }
                unreadable_log_files_count += 1;
            }
            Err(e) => {
                error!("Thread panicked: {:?}", e);
                unreadable_log_files_count += 1;
            }
        }
    }

    let statistics = FolderParseStatistics {
        unreadable_log_files_count,
        total_parsed_log_count,
        total_unreadable_log_line_count,
        total_unparseable_log_line_count,
    };

    FolderParseResults {
        statistics,
        logs_by_file,
    }
}

#[instrument]
fn extract_date(log_line: &str) -> Result<NaiveDateTime> {
    if log_line.len() < DATE_STRING_LEN {
        return Err(anyhow!("Line too short to contain a date"));
    }
    let date_part = &log_line[..DATE_STRING_LEN];
    let dt = NaiveDateTime::parse_from_str(date_part, "%Y-%m-%d %H:%M:%S,%f")?;
    Ok(dt)
}

#[instrument]
fn extract_log_level(log_line: &str) -> Result<LogLevel> {
    let content = log_line[DATE_STRING_LEN..].trim_start();

    let closing_bracket = content.find(']').ok_or_else(|| anyhow!("Missing ']'"))?;
    let token = &content[1..closing_bracket];

    match token {
        "TRACE" => Ok(LogLevel::Trace),
        "DEBUG" => Ok(LogLevel::Debug),
        "INFO" => Ok(LogLevel::Info),
        "WARNING" => Ok(LogLevel::Warning),
        "ERROR" => Ok(LogLevel::Error),
        _ => Err(anyhow!("Unrecognized log level: {}", token)),
    }
}

#[instrument]
pub fn parse_log_file(file_path: PathBuf) -> Result<(String, LogParseResults), ParseFileError> {
    let file_name =
        extract_file_name_from_path(&file_path).map_err(|e| ParseFileError::FileNameError {
            path: PathBuf::from(file_path.clone()),
            source: anyhow::anyhow!(e),
        })?;
    let file = File::open(&file_path).map_err(|e| (e, file_path.to_path_buf()))?;
    let reader = BufReader::new(file);

    let mut log_entries = Vec::new();
    let mut unreadable_log_line_count = 0;
    let mut unparseable_log_line_count = 0;
    let mut parsed_log_count = 0;

    for line in reader.lines() {
        match line {
            Ok(log_line) => match process_log_line(&log_line) {
                Some(entry) => {
                    parsed_log_count += 1;
                    log_entries.push(entry);
                }
                None => unparseable_log_line_count += 1,
            },
            Err(e) => {
                error!("Unreadable line: {}", e);
                unreadable_log_line_count += 1;
            }
        }
    }

    let statistics = LogParseStatistics {
        parsed_log_count,
        unreadable_log_line_count,
        unparseable_log_line_count,
    };

    Ok((
        file_name,
        LogParseResults {
            statistics,
            log_entries,
        },
    ))
}

#[instrument]
pub fn parse_logs_folder(folder_path: &Path) -> Result<FolderParseResults> {
    let file_paths = list_files_in_folder(folder_path)
        .map_err(|e| anyhow!("Failed to list log files: {}", e))?;

    let handles = spawn_file_parsing_threads(file_paths);
    let folder_parse_results = collect_results(handles);

    Ok(folder_parse_results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn test_extract_date_valid() {
        let line = "2025-06-25 13:00:03,887 [INFO] something happened";
        let result = extract_date(line).unwrap();
        let expected =
            NaiveDateTime::parse_from_str("2025-06-25 13:00:03,887", "%Y-%m-%d %H:%M:%S,%f")
                .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_log_level_info() {
        let line = "2025-06-25 13:00:03,887 [INFO] something happened";
        let result = extract_log_level(line).unwrap();
        assert!(matches!(result, LogLevel::Info));
    }
}
