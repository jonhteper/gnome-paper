use chrono::ParseError;

pub type AppResult<T> = Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("`$HOME` is not defined")]
    NoHomeVar,

    #[error("File `0` don't exist")]
    InvalidLocation(String),

    #[error("Error executing command `0`")]
    CommandError(String),

    #[error("No images loaded")]
    NoImagesLoaded,

    #[error("The limit is 1440 images: 1 for minute")]
    TooImagesLoaded,

    // Third Party Errors
    #[error(transparent)]
    ParseError(#[from] ParseError),
}
