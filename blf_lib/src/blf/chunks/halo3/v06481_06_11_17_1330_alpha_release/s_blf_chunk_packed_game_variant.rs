use std::io::{Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v06481_06_11_17_1330_alpha_release::game::game_engine_variant::c_game_variant;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("gvar", 2.1)]
pub struct s_blf_chunk_packed_game_variant
{
    pub game_variant: c_game_variant,
}

impl BlfChunkHooks for s_blf_chunk_packed_game_variant {}

impl BinRead for s_blf_chunk_packed_game_variant {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer)?;

        let mut bitstream = c_bitstream_reader::new_with_legacy_settings(buffer.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();

        let mut packed_game_variant = Self::default();

        packed_game_variant.game_variant.decode(&mut bitstream)?;

        Ok(packed_game_variant)
    }
}

impl BinWrite for s_blf_chunk_packed_game_variant {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        let mut bitstream = c_bitstream_writer::new_with_legacy_settings(0x264, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_writing();

        self.game_variant.encode(&mut bitstream)?;

        bitstream.finish_writing();
        writer.write_ne(&bitstream.get_data()?)
    }
}
