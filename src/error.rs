use std::fmt;

/// An error which can be returned when parsing `&str` or `u32`.
#[derive(Debug, PartialEq, Clone)]
pub enum ParseCodeError {
    Alpha(String),
    Num(u32),
}

impl fmt::Display for ParseCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseCodeError::*;
        match self {
            Alpha(v) => write!(f, "`{}` is no match any alphabetic code.", v),
            Num(v) => write!(f, "{} is no match numeric code.", v),
        }
    }
}

impl std::error::Error for ParseCodeError {}
