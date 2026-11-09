use binrw::binrw;
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("clif", 1.1)]
#[Size(12)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_blf_chunk_local_info {
    pub display_mode: i32,
    pub screen_width: i32,
    pub screen_height: i32,
}

impl BlfChunkHooks for s_blf_chunk_local_info {}