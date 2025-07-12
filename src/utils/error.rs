use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Repository already exists at {0}")]
    AlreadyExists(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("TOML deserialization error: {0}")]
    TomlDe(#[from] toml::de::Error),
    #[error("JSON serialization error: {0}")]
    JsonSer(#[from] serde_json::Error),
    #[error("Repository configuration not found in the current directory or parent directories.")]
    ConfigNotFound,
    #[error("Package not found: {0}")]
    PackageNotFound(String),
    #[error("Package {0} version {1} already exists.")]
    PackageAlreadyExists(String, String),
}

pub type Result<T> = std::result::Result<T, RepositoryError>;