use std::io::{Read, Seek, Write};
use binrw::{binrw, BinRead, BinReaderExt, BinResult, BinWrite, BinWriterExt, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blam::haloreach::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mvar", 31.1)]
#[derive(Default)]
pub struct s_blf_chunk_map_variant
{
    pub hash: s_network_http_request_hash,
    // in the file, this is a length followed by packed data
    pub map_variant: c_map_variant,
}


impl BlfChunkHooks for s_blf_chunk_map_variant {}

impl BinRead for s_blf_chunk_map_variant {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut packed_map_variant = Self::default();
        packed_map_variant.hash = s_network_http_request_hash::read_options(reader, endian, ())?;
        let packed_variant_length = u32::read_options(reader, Endian::Big, ())? as usize;

        let mut buffer = Vec::<u8>::with_capacity(packed_variant_length);
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
        let mut bitstream = c_bitstream_writer::new(0xE0A0, e_bitstream_byte_order::_bitstream_byte_order_big_endian);
        bitstream.begin_writing();

        self.map_variant.encode(&mut bitstream)?;

        bitstream.finish_writing();
        let packed_data = bitstream.get_data()?;
        let packed_data_length = packed_data.len() as u32;

        writer.write_ne(&self.hash)?;
        packed_data_length.write_options(writer, Endian::Big, ())?;
        writer.write_ne(&bitstream.get_data()?)
    }
}
