use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::blam::common::memory::secure_signature::s_network_http_request_hash;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_variant::c_game_variant;

#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("mpvr", 54.1)]
// Not sure what this chunk is called
pub struct s_blf_chunk_game_variant
{
    pub hash: s_network_http_request_hash,
    pub unknown04: u32,
    pub unknown_length: u32,
    pub game_variant: c_game_variant,
}

impl BinRead for s_blf_chunk_game_variant {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut data: Vec<u8> = Vec::new();
        reader.read_to_end(&mut data)?;

        let mut bitstream = c_bitstream_reader::new(data.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();

        let hash = bitstream.read_raw(0x14 * 8)?;
        let unknown04 = bitstream.read_integer("unknown04", 32)?;
        let unknown_length = bitstream.read_integer("unknown_length", 32)?;


        let mut game_variant = c_game_variant::default();
        game_variant.decode(&mut bitstream)?;

        Ok(Self {
            hash,
            unknown04,
            unknown_length,
            game_variant,
        })
    }
}

impl BinWrite for s_blf_chunk_game_variant {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        unimplemented!();
    }
}


impl BlfChunkHooks for s_blf_chunk_game_variant {}