pub use blf_lib_derivable::result::*;

#[macro_export]
macro_rules! BINRW_RESULT {
    ($task:expr) => {
        $task.map_err(|e| binrw::error::Error::Custom {
            pos: u64::MAX,
            err: Box::new(e.to_string()),
        })
    };
}

#[macro_export]
macro_rules! BINRW_ERROR {
    ($err:expr) => {
        binrw::error::Error::Custom {
            pos: u64::MAX,
            err: Box::new($err),
        }
    };
}

#[macro_export]
macro_rules! SERDE_DESERIALIZE_RESULT {
    ($task:expr) => {
        $task.map_err(|e|serde::de::Error::custom(e))
    };
}

#[macro_export]
macro_rules! SERDE_SERIALIZE_RESULT {
    ($task:expr) => {
        $task.map_err(|e|serde::ser::Error::custom(e))
    };
}

#[macro_export]
macro_rules! OPTION_TO_RESULT {
    ($opt:expr, $err:expr) => {
        match $opt {
            Some(val) => Ok(val),
            None => Err($err),
        }
    };
}

#[macro_export]
macro_rules! assert_ok {
    ($cond:expr, $arg:expr) => {
        if (!$cond) {
            return Err(blf_lib::result::BLFLibError::from($arg))
        }
    };
    ($($cond:tt)*) => {
        if (!($($cond)*)) {
            return Err(blf_lib::result::BLFLibError::from(format!("{:?}", String::from(stringify!($($cond)*)))));
        }
    };
}

#[cfg(feature = "napi")]
pub fn create_napi_error(error: BLFLibError) -> napi::Error {
    napi::Error::from_reason(error.to_string())
}