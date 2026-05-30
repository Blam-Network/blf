use std::io::{Read, Seek, Write};
use binrw::{binrw, BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, e_bitstream_byte_order};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::blf::get_buffer_hash;

/// Reach map variant chunk packed bitstream storage (bytes).
pub const MAP_VARIANT_STORAGE_CAPACITY: usize = 0x7000;

#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mvar", 31.1)]
#[derive(Default)]
pub struct s_blf_chunk_map_variant
{
    // Hash + BE packed_length + 0x7000 storage + 4-byte pad; hash is over length + packed bytes only.
    pub map_variant: c_map_variant,
}

impl BlfChunkHooks for s_blf_chunk_map_variant {}

impl BinRead for s_blf_chunk_map_variant {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut packed_map_variant = Self::default();
        s_network_http_request_hash::read_options(reader, endian, ())?;
        let packed_variant_length = u32::read_options(reader, Endian::Big, ())? as usize;

        let mut buffer = Vec::<u8>::new();
        reader.read_to_end(&mut buffer)?;
        let decode_length = if packed_variant_length > 0 {
            packed_variant_length
                .min(MAP_VARIANT_STORAGE_CAPACITY)
                .min(buffer.len())
        } else {
            MAP_VARIANT_STORAGE_CAPACITY.min(buffer.len())
        };

        let mut bitstream = c_bitstream_reader::new(
            &buffer[..decode_length],
            e_bitstream_byte_order::_bitstream_byte_order_big_endian,
        );
        bitstream.begin_reading();

        packed_map_variant.map_variant.decode(&mut bitstream)?;

        Ok(packed_map_variant)
    }
}

impl BinWrite for s_blf_chunk_map_variant {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        let mut bitstream = c_bitstream_writer::new(
            MAP_VARIANT_STORAGE_CAPACITY,
            e_bitstream_byte_order::_bitstream_byte_order_big_endian,
        );
        bitstream.begin_writing();

        self.map_variant.encode(&mut bitstream)?;

        bitstream.finish_writing();
        let packed_data = bitstream.get_data()?;
        let packed_data_length = packed_data.len() as u32;

        let mut hash_buffer = packed_data_length.to_be_bytes().to_vec();
        hash_buffer.extend_from_slice(&packed_data);

        let mut packed_storage = vec![0u8; MAP_VARIANT_STORAGE_CAPACITY];
        packed_storage[..packed_data.len()].copy_from_slice(&packed_data);

        writer.write_ne(&get_buffer_hash(&hash_buffer)?)?;
        packed_data_length.write_options(writer, Endian::Big, ())?;
        writer.write_all(&packed_storage)?;
        writer.write_all(&[0u8; 4])?;

        Ok(())
    }
}

impl s_blf_chunk_map_variant {
    pub fn create(map_variant: c_map_variant) -> Self {
        Self {
            map_variant,
        }
    }
}