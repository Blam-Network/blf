use std::io::{Read, Seek};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,BinWrite)]
#[Header("gvar", 34.1)]
#[brw(big)]
// Not sure what this chunk is called
#[derive(Default)]
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


impl BlfChunkHooks for s_blf_chunk_matchmaking_game_variant {}