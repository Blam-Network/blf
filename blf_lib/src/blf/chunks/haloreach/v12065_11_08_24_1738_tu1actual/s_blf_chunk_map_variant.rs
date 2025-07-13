use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use crate::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mvar", 31.1)]
#[brw(big)]
pub struct s_blf_chunk_map_variant
{
    pub hash: s_network_http_request_hash,
    pub data_size: u32,
    #[br(count = data_size)]
    pub data: Vec<u8>,
}

impl Default for s_blf_chunk_map_variant {
    fn default() -> Self {
        Self {
            hash: Default::default(),
            data_size: 0,
            data: Vec::new()
        }
    }
}

impl BlfChunkHooks for s_blf_chunk_map_variant {}