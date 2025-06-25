use chrono::NaiveDateTime;
use std::collections::HashMap;

#[derive(Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct LogEntry {
    pub log_level: LogLevel,
    pub date: NaiveDateTime,
}

#[derive(Debug)]
pub struct LogParseResults {
    pub statistics: LogParseStatistics,
    pub log_entries: Vec<LogEntry>,
}

#[derive(Debug)]
pub struct LogParseStatistics {
    pub parsed_log_count: usize,
    pub unreadable_log_line_count: usize,
    pub unparseable_log_line_count: usize,
}

#[derive(Debug)]
pub struct FolderParseResults {
    pub statistics: FolderParseStatistics,
    pub logs_by_file: HashMap<String, LogParseResults>,
}

#[derive(Debug)]
pub struct FolderParseStatistics {
    pub unreadable_log_files_count: usize,
    pub total_parsed_log_count: usize,
    pub total_unreadable_log_line_count: usize,
    pub total_unparseable_log_line_count: usize,
}
