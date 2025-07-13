use std::io::{Read, Seek};
use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use crate::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib_derive::BlfChunk;

#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,BinWrite)]
#[Header("gvar", 54.1)]
#[brw(big)]
// Not sure what this chunk is called
pub struct s_blf_chunk_matchmaking_game_variant
{
    // these two fields are in MPVR
    // maybe a hash and something, maybe some unique IDs?
    // pub unknown: StaticArray<u8, 24>,
    // pub data_size: u32,

    // Not yet mapped.
    pub data: Vec<u8>,
}

impl BinRead for s_blf_chunk_matchmaking_game_variant {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut data: Vec<u8> = Vec::new();
        reader.read_to_end(&mut data)?;
        Ok(Self {
            data,
        })
    }
}

impl Default for s_blf_chunk_matchmaking_game_variant {
    fn default() -> Self {
        Self {
            data: Vec::new()
        }
    }
}

impl BlfChunkHooks for s_blf_chunk_matchmaking_game_variant {}