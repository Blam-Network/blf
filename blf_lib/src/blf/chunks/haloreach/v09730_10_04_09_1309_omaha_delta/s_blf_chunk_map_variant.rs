use std::io::{Read, Seek, Write};
use binrw::{binrw, BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::get_buffer_hash;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mvar", 19.1)]
#[derive(Default)]
pub struct s_blf_chunk_map_variant
{
    // in the file, this is a length followed by packed data
    pub map_variant: c_map_variant,
}


impl BlfChunkHooks for s_blf_chunk_map_variant {}


impl BinRead for s_blf_chunk_map_variant {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut packed_map_variant = Self::default();

        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer)?;

        let mut bitstream = c_bitstream_reader::new(buffer.as_slice(), e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_reading();

        packed_map_variant.map_variant.decode(&mut bitstream)?;

        Ok(packed_map_variant)
    }
}

impl BinWrite for s_blf_chunk_map_variant {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        let mut bitstream = c_bitstream_writer::new(0xD9B0, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_writing();

        self.map_variant.encode(&mut bitstream)?;

        bitstream.finish_writing();
        let packed_data = bitstream.get_data()?;

        writer.write_ne(&bitstream.get_data()?)
    }
}
