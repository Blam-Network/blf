use std::fmt::{Debug, Display};

pub struct BLFLibError(Box<dyn std::error::Error>);

impl From<String> for BLFLibError {
    fn from(e: String) -> BLFLibError {
        BLFLibError(e.into())
    }
}

impl From<&str> for BLFLibError {
    fn from(e: &str) -> BLFLibError {
        BLFLibError(e.into())
    }
}

impl From<std::io::Error> for BLFLibError {
    fn from(e: std::io::Error) -> BLFLibError {
        BLFLibError(e.into())
    }
}

impl From<std::array::TryFromSliceError> for BLFLibError {
    fn from(e: std::array::TryFromSliceError) -> BLFLibError {
        BLFLibError(e.into())
    }
}

impl From<std::string::FromUtf8Error> for BLFLibError {
    fn from(e: std::string::FromUtf8Error) -> BLFLibError {
        BLFLibError(e.into())
    }
}

impl From<std::string::FromUtf16Error> for BLFLibError {
    fn from(e: std::string::FromUtf16Error) -> BLFLibError {
        BLFLibError(e.into())
    }
}

impl From<Box<dyn std::error::Error>> for BLFLibError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        BLFLibError(err)
    }
}

impl From<std::fmt::Error> for BLFLibError {
    fn from(e: std::fmt::Error) -> Self {
        BLFLibError(e.into())
    }
}

impl From<binrw::Error> for BLFLibError {
    fn from(e: binrw::Error) -> BLFLibError {
        BLFLibError(e.into())
    }
}

impl From<csv::Error> for BLFLibError {
    fn from(e: csv::Error) -> BLFLibError {
        BLFLibError(e.into())
    }
}

impl From<serde_json::Error> for BLFLibError {
    fn from(e: serde_json::Error) -> BLFLibError {
        BLFLibError(e.into())
    }
}

impl From<BLFLibError> for binrw::Error {
    fn from(err: BLFLibError) -> Self {
        binrw::error::Error::Custom {
            pos: u64::MAX,
            err: Box::new(err.0.to_string()),
        }
    }
}


impl Display for BLFLibError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl Debug for BLFLibError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for BLFLibError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}
pub type BLFLibResult<T = ()> = Result<T, BLFLibError>;
