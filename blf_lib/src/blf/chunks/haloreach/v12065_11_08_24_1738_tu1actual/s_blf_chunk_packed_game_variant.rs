use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_variant::c_game_variant;
use blf_lib::blf::get_buffer_hash;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("gvar", 54.1)]
// Not sure what this chunk is called
pub struct s_blf_chunk_matchmaking_game_variant
{
    pub game_variant: c_game_variant,
}

impl BinRead for s_blf_chunk_matchmaking_game_variant {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut data: Vec<u8> = Vec::new();
        reader.read_to_end(&mut data)?;

        let mut bitstream = c_bitstream_reader::new(data.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();

        let mut game_variant = c_game_variant::default();
        game_variant.decode(&mut bitstream)?;

        Ok(Self {
            game_variant,
        })
    }
}

impl BinWrite for s_blf_chunk_matchmaking_game_variant {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        let mut bitstream_writer = c_bitstream_writer::new(0x5028, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream_writer.begin_writing();
        self.game_variant.encode(&mut bitstream_writer)?;
        bitstream_writer.finish_writing();
        let gametype_data = bitstream_writer.get_data()?;
        gametype_data.write_options(writer, Endian::Big, args)?;

        Ok(())
    }
}


impl BlfChunkHooks for s_blf_chunk_matchmaking_game_variant {}