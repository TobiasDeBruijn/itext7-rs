use thiserror::Error;

pub type PdfResult<T> = Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Jni(#[from] jni::errors::Error),
}