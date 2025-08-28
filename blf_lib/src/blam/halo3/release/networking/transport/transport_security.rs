#[cfg(feature = "napi")]
use napi_derive::napi;

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
use blf_lib::types::net::Ipv4Addr;
use blf_lib_derive::TestSize;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite, Default, TestSize)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[Size(0x24)]
pub struct s_transport_secure_address {
    pub ina: Ipv4Addr,
    pub ina_online: Ipv4Addr,
    pub data: StaticArray<u8, 28>,
}
