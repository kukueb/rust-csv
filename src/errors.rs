use std::fmt;

#[derive(Debug)]
pub enum AppError {
    FileReadingError(String),
    FileParsingError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::FileReadingError(file_name) => {
                write!(f, "Cannot read the file \"{}\"", file_name)
            }
            AppError::FileParsingError(string) => {
                write!(f, "Cannot parse a NaiveDateTime from the \"{}\"", string)
            }
        }
    }
}
