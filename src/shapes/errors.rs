use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShapesFileError {
    #[error("reached EOF when reading data")]
    EOF,
    #[error("bad version in unit type: {0}")]
    BadVersion(u32),
    // TODO: Can we add path here?
    #[error("Shapes file not found")]
    FileNotFound,
    #[error("IO Error: {0:?}")]
    IOError(std::io::ErrorKind),
    #[error("bad color table: {0}")]
    BadColorTable(u32),
}

impl From<std::io::Error> for ShapesFileError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => ShapesFileError::FileNotFound,
            std::io::ErrorKind::UnexpectedEof => ShapesFileError::EOF,
            e => ShapesFileError::IOError(e),
        }
    }
}
