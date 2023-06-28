use actix_web::{http::StatusCode, ResponseError};

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP server error: {0}")]
    Actix(#[from] actix_web::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Resource not found")]
    NotFound,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
