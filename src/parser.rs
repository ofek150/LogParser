use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Result, anyhow};
use chrono::NaiveDateTime;

use crate::constants::DATE_STRING_LEN;
use crate::model::{LogEntry, LogLevel, LogParseResults};

pub fn parse_log_file(file_path: &str) -> Result<LogParseResults> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut log_entries = Vec::new();
    let mut unreadable_log_lines_count = 0;
    let mut unparseable_log_lines_count = 0;

    for line in reader.lines() {
        let log_line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Unreadable line: {}", e);
                unreadable_log_lines_count += 1;
                continue;
            }
        };

        let date = match extract_date(&log_line) {
            Ok(date) => date,
            Err(_) => {
                unparseable_log_lines_count += 1;
                continue;
            }
        };

        let log_level = match extract_log_level(&log_line) {
            Ok(level) => level,
            Err(_) => {
                unparseable_log_lines_count += 1;
                continue;
            }
        };

        log_entries.push(LogEntry { log_level, date });
    }

    Ok(LogParseResults {
        unreadable_log_lines_count,
        unparseable_log_lines_count,
        log_entries,
    })
}

fn extract_date(log_line: &str) -> Result<NaiveDateTime> {
    if log_line.len() < DATE_STRING_LEN {
        return Err(anyhow!("Line too short to contain a date"));
    }
    let date_part = &log_line[..DATE_STRING_LEN];
    let dt = NaiveDateTime::parse_from_str(date_part, "%Y-%m-%d %H:%M:%S,%f")?;
    Ok(dt)
}

fn extract_log_level(log_line: &str) -> Result<LogLevel> {
    let content = &log_line[DATE_STRING_LEN..];

    if content.contains("TRACE") {
        Ok(LogLevel::Trace)
    } else if content.contains("DEBUG") {
        Ok(LogLevel::Debug)
    } else if content.contains("INFO") {
        Ok(LogLevel::Info)
    } else if content.contains("WARNING") {
        Ok(LogLevel::Warning)
    } else if content.contains("ERROR") {
        Ok(LogLevel::Error)
    } else {
        Err(anyhow!("Unrecognized log level"))
    }
}
