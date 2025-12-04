use crate::database::error::{BackendError, DbError};
use crate::templates::error::GenericError;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use std::env;
use std::error::Error;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::ParseBoolError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Clone)]
pub enum AppError {
    #[error("DbError -> {0}")]
    DbError(DbError),
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Templating error: {0}")]
    TemplatingError(String),
    #[error("Identity error: {0}")]
    IdentityError(String),
    #[error("Session error: {0}")]
    SessionError(String),
    #[error("Cookie error: {0}")]
    CookieError(String),
    #[error("File error: {0}")]
    FileError(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Device discovery request denied: {0}")]
    DeviceTransmitRequestDenied(String),
    #[error("AP database error: {0}")]
    ApDatabaseError(String),
    #[error("OIDC error: {0}")]
    OidcError(String),
    #[error("DNS packet manipulation error: {0}")]
    DnsPacketManipulationError(String),
    #[error("Could not load the env var: {0}")]
    EnvVarError(String),
}

impl Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self)
    }
}

impl From<DbError> for AppError {
    fn from(value: DbError) -> Self {
        Self::DbError(value)
    }
}

impl From<JoinError> for AppError {
    fn from(value: JoinError) -> Self {
        Self::InternalServerError(value.to_string())
    }
}

impl From<askama::Error> for AppError {
    fn from(error: askama::Error) -> Self {
        Self::TemplatingError(error.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::FileError(value.to_string())
    }
}

impl From<ParseIntError> for AppError {
    fn from(_: ParseIntError) -> Self {
        Self::IdentityError("Invalid User ID".to_string())
    }
}

impl From<ParseBoolError> for AppError {
    fn from(value: ParseBoolError) -> Self {
        Self::ParseError(value.to_string())
    }
}

impl From<ipnetwork::IpNetworkError> for AppError {
    fn from(value: ipnetwork::IpNetworkError) -> Self {
        Self::ParseError(value.to_string())
    }
}

impl From<env::VarError> for AppError {
    fn from(value: env::VarError) -> Self {
        Self::EnvVarError(value.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::BadRequest(_) | AppError::ParseError(_) => StatusCode::BAD_REQUEST,
            AppError::DeviceTransmitRequestDenied(_) => StatusCode::FORBIDDEN,
            AppError::DbError(ref db_e) => match db_e {
                DbError::BackendError(be_e) => match &be_e {
                    BackendError::DoesNotExist(_) => StatusCode::NOT_FOUND,
                    BackendError::Deleted => StatusCode::BAD_REQUEST,
                    BackendError::UpdateParametersEmpty => StatusCode::BAD_REQUEST,
                    BackendError::UserPasswordDoesNotMatch => StatusCode::UNAUTHORIZED,
                    BackendError::UserPasswordVerificationFailed(_) => StatusCode::BAD_REQUEST,
                    BackendError::PermissionDenied(_) => StatusCode::FORBIDDEN,
                },
                DbError::ForeignKeyError(_)
                | DbError::UniqueConstraintError(_)
                | DbError::NotNullError(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::TemplatingError(_)
            | AppError::InternalServerError(_)
            | AppError::IdentityError(_)
            | AppError::SessionError(_)
            | AppError::CookieError(_)
            | AppError::FileError(_)
            | AppError::ApDatabaseError(_)
            | AppError::DnsPacketManipulationError(_)
            | AppError::EnvVarError(_)
            | AppError::OidcError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        };
        let template = GenericError {
            code: status_code.as_u16(),
            status_code: status_code.to_string(),
            description: self.to_string(),
        };

        match template.render() {
            Ok(body) => (status_code, Html(body)).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    }
}
