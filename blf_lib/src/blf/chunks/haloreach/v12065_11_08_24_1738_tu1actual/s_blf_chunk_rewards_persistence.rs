use binrw::binrw;
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::BlfChunk;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
#[cfg(feature = "napi")]
use napi_derive::napi;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("rpdl", 2.1)]
#[Size(0x21B)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_rewards_persistance {
    // TODO: Map
    pub credits: u32,
    pub unknown1: u32,
    pub commendations: StaticArray<u16, 128>, // commendation state structs
    pub purchased_items: StaticArray<u8, 256>,
    pub unknown2: u32,
    pub unknown3: u8, // shows bonus notice
    pub unknown4: u8, // shows bonus notice
    pub unknown5: u8,


}

impl BlfChunkHooks for s_blf_chunk_rewards_persistance {}

