use crate::vcf::Content;
use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    CLIError(String),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error(transparent)]
    TeraError(#[from] tera::Error),

    #[error("{0}")]
    FileNotFoundError(String),

    #[error("{0}")]
    ConfigurationError(String),

    #[error("Invalid UTF-8 character at {0}")]
    VCFReadUtf8Error(Content),
}
