use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),

    #[error("migration error: {0}")]
    Migration(#[from] rusqlite_migration::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("xlsx error: {0}")]
    Xlsx(#[from] rust_xlsxwriter::XlsxError),

    #[error("csv error: {0}")]
    Csv(#[from] csv::Error),

    #[error("tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("invalid argument: {0}")]
    InvalidArg(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("firebase error: {0}")]
    Firebase(String),

    #[error("{0}")]
    Other(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_str(self.to_string().as_ref())
    }
}

pub type AppResult<T> = std::result::Result<T, AppError>;
