use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("netc", 245.1)]
#[brw(big)]
pub struct s_blf_chunk_network_configuration
{
    // TODO: Map
    data: StaticArray<u8, 0x25B0>,
}

impl BlfChunkHooks for s_blf_chunk_network_configuration {}
