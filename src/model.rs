use chrono::NaiveDateTime;

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
    pub unreadable_log_lines_count: usize,
    pub unparseable_log_lines_count: usize,
    pub log_entries: Vec<LogEntry>,
}
