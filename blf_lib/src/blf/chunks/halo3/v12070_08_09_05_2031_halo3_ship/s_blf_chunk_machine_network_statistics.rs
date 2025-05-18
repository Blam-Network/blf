use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
#[cfg(feature = "napi")]
use napi_derive::napi;

#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("funs", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_machine_network_statistics {
    // need to map this whole ass struct
    pub data: StaticArray<u8, 0xC0>,
}

impl BlfChunkHooks for s_blf_chunk_machine_network_statistics {}

