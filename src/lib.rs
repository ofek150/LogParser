mod constants;
mod model;
mod parser;
mod utils;
mod errors;

pub use model::LogLevel;
pub use parser::parse_log_file;
pub use parser::parse_logs_folder;
