use derive_more::From;
use simd_json;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Custom(String),

    #[from]
    Io(std::io::Error),

    #[from]
    Json(simd_json::Error),

    #[from]
    TryType(simd_json::TryTypeError),
}

impl Error {
    fn custom(value: String) -> Self {
        Error::Custom(value.into())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::custom(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
