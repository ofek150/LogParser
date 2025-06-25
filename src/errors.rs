use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ParseFileError {
    #[error("Failed to extract file name for {path:?}: {source}")]
    FileNameError {
        path: PathBuf,
        #[source]
        source: anyhow::Error,
    },

    #[error("Failed to parse log file {path:?}: {source}")]
    ParseError {
        path: PathBuf,
        #[source]
        source: anyhow::Error,
    },
}

impl From<(std::io::Error, PathBuf)> for ParseFileError {
    fn from((err, path): (std::io::Error, PathBuf)) -> Self {
        ParseFileError::ParseError {
            path,
            source: anyhow::Error::from(err),
        }
    }
}