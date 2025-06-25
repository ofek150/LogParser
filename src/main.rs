use clap::Parser;
use log_parser::{parse_log_file, parse_logs_folder};
use std::path::PathBuf;
use tracing::{debug, error, info, instrument};
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(
    name = "log-parser",
    version,
    about = "A tool to parse and analyze log files or directories containing logs, \
             providing detailed statistics on log entries, errors, and warnings."
)]
struct Cli {
    path: PathBuf,
}

#[instrument]
fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_line_number(true)
        .init();

    let cli = Cli::parse();
    debug!("Input path: {:?}", cli.path);

    if cli.path.is_file() {
        parse_file(cli.path);
    } else if cli.path.is_dir() {
        parse_folder(cli.path);
    } else {
        error!("The provided path is neither a file nor a folder.");
        std::process::exit(1);
    }
}

#[instrument]
fn parse_file(path: PathBuf) {
    match parse_log_file(path) {
        Ok(results) => {
            let (file_name, parse_log_result) = results;
            info!("Parsed logs from file: {}", file_name);
            info!("Parsed {} entries.", parse_log_result.log_entries.len());
            info!(
                "Unreadable lines: {}",
                parse_log_result.statistics.unreadable_log_line_count
            );
            info!(
                "Unparseable lines: {}",
                parse_log_result.statistics.unparseable_log_line_count
            );
        }
        Err(e) => {
            error!("Error while parsing file: {}", e);
            std::process::exit(2);
        }
    }
}

#[instrument]
fn parse_folder(path: PathBuf) {
    match parse_logs_folder(&path) {
        Ok(results) => {
            info!("Parsed logs from folder: {}", path.display());
            info!(
                "Total parsed log count: {}",
                results.statistics.total_parsed_log_count
            );
            info!(
                "Unreadable log files count: {}",
                results.statistics.unreadable_log_files_count
            );
            info!(
                "Total unreadable log line count: {}",
                results.statistics.total_unreadable_log_line_count
            );
            info!(
                "Total unparseable log line count: {}",
                results.statistics.total_unparseable_log_line_count
            );
        }
        Err(e) => {
            error!("Error while parsing folder: {}", e);
            std::process::exit(2);
        }
    }
}
