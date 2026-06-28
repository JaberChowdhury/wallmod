//! Error handling definitions for the backend service layer.

use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    Io(String),
    Image(String),
    Processing(String),
    Config(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "I/O Error: {}", err),
            AppError::Image(err) => write!(f, "Image Processing Error: {}", err),
            AppError::Processing(err) => write!(f, "Grading Pipeline Error: {}", err),
            AppError::Config(err) => write!(f, "Config Exporter Error: {}", err),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

impl From<image::ImageError> for AppError {
    fn from(err: image::ImageError) -> Self {
        AppError::Image(err.to_string())
    }
}
