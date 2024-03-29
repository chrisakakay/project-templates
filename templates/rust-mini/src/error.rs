//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String), // For beginning only. Remove later.

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
