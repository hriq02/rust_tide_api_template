use std::sync::PoisonError;
use crate::sql_service::sql_error::SqlServErr;

pub enum ServerError{
    DatabaseError(String),
    TideError(String),
    IoError(String),
    OtherError(String),
    MutexError(String),
    LogWritingError(String),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::DatabaseError(err) => {
                write!(f, "Database Error: {}", err)
            }
            ServerError::TideError(err) => {
                write!(f, "Tide Error: {}", err)
            }
            ServerError::IoError(err) => {
                write!(f, "IO Error: {}", err)
            }
            ServerError::OtherError(err) => {
                write!(f, "Other Error: {}", err)
            }
            ServerError::MutexError(err) => {
                write!(f, "Mutex Error: {}", err)
            }
            ServerError::LogWritingError(err) => {
                write!(f, "Log Writing Error: {}", err)
            }
        }
    }
}



impl From<tide::Error> for ServerError {
    fn from(err: tide::Error) -> Self {
        ServerError::TideError(err.to_string())
    }
}


impl From<sqlx::Error> for ServerError {
    fn from(err: sqlx::Error) -> Self {
        ServerError::DatabaseError(err.to_string())
    }
}

impl From<SqlServErr> for ServerError {
    fn from(err: SqlServErr) -> Self {
        ServerError::DatabaseError(err.to_string())
    }
}

impl From <std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::IoError(err.to_string())
    }
}

impl From <String> for ServerError {

    fn from(err: String) -> Self {
        ServerError::OtherError(err)
    }
}


impl<T> From<PoisonError<T>> for ServerError {
    fn from(err: PoisonError<T>) -> Self {
        ServerError::MutexError(err.to_string())
    }
}


impl From<std::fmt::Error> for ServerError {
    fn from(err: std::fmt::Error) -> Self {
        ServerError::LogWritingError(err.to_string())
    }
}