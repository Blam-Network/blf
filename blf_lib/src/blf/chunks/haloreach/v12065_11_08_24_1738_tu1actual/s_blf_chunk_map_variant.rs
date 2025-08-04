use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mvar", 31.1)]
#[brw(big)]
#[derive(Default)]
pub struct s_blf_chunk_map_variant
{
    pub hash: s_network_http_request_hash,
    pub data_size: u32,
    #[br(count = data_size)]
    pub data: Vec<u8>,
}


impl BlfChunkHooks for s_blf_chunk_map_variant {}