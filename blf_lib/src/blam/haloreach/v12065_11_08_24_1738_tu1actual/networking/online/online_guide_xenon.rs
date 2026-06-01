use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use blf_lib::TestSize;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[Size(4)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_network_session_privacy_mode {
    pub network_session_privacy: u8,
    pub network_session_closed_status: u8,
    pub unknown2: u8,
    pub unknown3: u8,
}
