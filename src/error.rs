// src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{env::VarError, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("URL generation wrapped around to the original")]
    Urlinvalid,

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error("Missing or invalid environment variable: {0}")]
    EnvVar(#[from] VarError),

    #[error("Failed to bind to address: {0}")]
    Io(#[from] io::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Urlinvalid => StatusCode::CONFLICT,
            Self::EnvVar(_) | Self::Io(_) | Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let msg = self.to_string();
        (status, msg).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
