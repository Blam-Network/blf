use binrw::{binrw, BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::{BlfChunk, TestSize};
#[cfg(feature = "napi")]
use napi_derive::napi;

#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("funs", 1.1)]
#[Size(0xC0)]
#[brw(big)]
pub struct s_blf_chunk_user_network_statistics {
    pub session: StaticArray<s_network_quality_session_statistics, 2>,
    pub connection_history: StaticArray<u64, 2>,
    #[brw(pad_after = 4)]
    pub bandwidth_data: s_network_bandwidth_persistent_data,
}

#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite,TestSize)]
#[Size(0x20)]
pub struct s_network_quality_session_statistics {
    pub client_badness_history: StaticArray<u64, 2>,
    pub host_badness_history: StaticArray<u64, 2>,
}

#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[derive(Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite,TestSize)]
#[Size(0x6C)]
pub struct s_network_bandwidth_persistent_data {
    pub qos_sample_count: i32,
    pub qos_sample_bps: StaticArray<i32, 8>,
    pub bandwidth_measurement_count: i32,
    pub bandwidth_measurement_successful_bps: StaticArray<i32, 8>,
    pub bandwidth_measurement_unsafe_bps: StaticArray<i32, 8>,
    pub bandwidth_dispute_count: i32,
}

impl BlfChunkHooks for s_blf_chunk_user_network_statistics {}

