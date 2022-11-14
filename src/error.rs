use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    #[error(transparent)]
    TomlDeError(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
