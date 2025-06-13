use std::fmt;
use axum::http::StatusCode;
use zip::result::ZipError;

#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    ConfigError(String),
    CodegenError(String),
    NotFound(String),
    ZipError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(err) => write!(f, "Erreur I/O: {}", err),
            AppError::ConfigError(msg) => write!(f, "Erreur de configuration: {}", msg),
            AppError::CodegenError(msg) => write!(f, "Erreur de génération de code: {}", msg),
            AppError::NotFound(msg) => write!(f, "Non trouvé: {}", msg),
            AppError::ZipError(msg) => write!(f, "Erreur de compression: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::CodegenError(err.to_string())
    }
}

impl From<ZipError> for AppError {
    fn from(err: ZipError) -> Self {
        AppError::ZipError(err.to_string())
    }
}

impl From<AppError> for StatusCode {
    fn from(error: AppError) -> Self {
        match error {
            AppError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::CodegenError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ZipError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
} 