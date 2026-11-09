use binrw::binrw;
use serde::{Deserialize, Serialize};
use crate::blf::chunks::BlfChunkHooks;
use crate::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("clif", 1.1)]
#[Size(12)]
#[brw(little)]
pub struct s_blf_chunk_local_info {
    pub display_mode: i32,
    pub screen_width: i32,
    pub screen_height: i32,
}

impl BlfChunkHooks for s_blf_chunk_local_info {}