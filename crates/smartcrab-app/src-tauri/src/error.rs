/// Application-level errors for the `SmartCrab` desktop app.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Adapter not found: {id}")]
    AdapterNotFound { id: String },

    #[error("Adapter error ({adapter}): {message}")]
    AdapterError { adapter: String, message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
