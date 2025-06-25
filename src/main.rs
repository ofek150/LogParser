
use clap::Parser;
use log_parser::parse_log_file;

#[derive(Parser, Debug)]
#[command(name = "log-analyzer", version, about = "Parses log files")]
struct Cli {
    log_file_path: std::path::PathBuf,
}

fn main() {
    let cli = Cli::parse();
    println!("Log file path: {:?}", cli.log_file_path);

    match parse_log_file(cli.log_file_path.to_str().unwrap()) {
        Ok(log_parse_results) => {
            dbg!(&log_parse_results.log_entries);
            println!("Parsed {} entries.", log_parse_results.log_entries.len());
            println!(
                "Unreadable lines: {}",
                log_parse_results.unreadable_log_lines_count
            );
            println!(
                "Unparseable lines: {}",
                log_parse_results.unparseable_log_lines_count
            );
        }
        Err(e) => {
            eprintln!("Error while parsing file: {}", e);
            std::process::exit(2);
        }
    }
}
