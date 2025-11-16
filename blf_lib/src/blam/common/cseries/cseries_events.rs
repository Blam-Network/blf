use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

#[cfg(feature = "napi")]
use napi_derive::napi;

#[cfg_attr(feature = "napi", napi(namespace = "common"))]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default)]
#[brw(repr = u32)]
pub enum e_event_level
{
    _event_verbose = 0,
    _event_status = 1,
    _event_message = 2,
    _event_warning = 3,
    _event_error = 4,
    _event_critical = 5,
}

#[cfg_attr(feature = "napi", napi(namespace = "common"))]
pub const k_event_level_severity_strings: [&str; 6] = [
    "verbose",
    "status",
    "message",
    "WARNING",
    "-ERROR-",
    "-CRITICAL-",
];