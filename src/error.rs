// in src/error.rs
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use thiserror::Error as thiser;

#[derive(thiser, Debug)]
pub enum Error {
    #[error("URL generation wrapped around to the original")]
    Urlinvalid,

    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, body) = match &self {
            Self::Urlinvalid => (StatusCode::CONFLICT, self.to_string()),
            Self::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()),
        };
        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
