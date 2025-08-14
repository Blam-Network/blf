use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

#[cfg(feature = "napi")]
use napi_derive::napi;

#[derive(Clone, Copy, Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize, Default)]
#[brw(repr = u16)]
#[cfg_attr(feature = "napi", napi(namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[repr(u16)]
pub enum byte_order_mark {
    little_endian = 0xFFFE,
    #[default]
    big_endian = 0xFEFF
}