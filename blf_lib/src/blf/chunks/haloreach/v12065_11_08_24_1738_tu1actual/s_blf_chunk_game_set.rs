use std::io::{Cursor, Read, Seek, Write};
use binrw::{binrw, binwrite, BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::io::bitstream::{c_bitstream_reader, close_bitstream_writer, create_bitstream_writer, e_bitstream_byte_order};
use blf_lib::types::array::StaticArray;
use crate::types::c_string::StaticString;
use blf_lib::types::time::{filetime};
use serde_hex::{SerHex,StrictCap};
use blf_lib::types::bool::Bool;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derive::{BlfChunk, TestSize};
use crate::types::numbers::Float32;

pub const k_maximum_game_entries: usize = 128;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("gset", 15.1)]
pub struct s_blf_chunk_game_set
{
    pub entries: Vec<s_game_set_entry>,
}

impl BlfChunkHooks for s_blf_chunk_game_set {
    fn before_write(&mut self, _previously_written: &Vec<u8>) -> BLFLibResult {
        for entry in self.entries.iter_mut() {
            entry.has_game_variant = Bool::from(entry.game_variant_file_name.get_string()?.len() > 0);
            entry.has_map_variant = Bool::from(entry.map_variant_file_name.get_string()?.len() > 0);
        }

        Ok(())
    }
}

impl BinRead for s_blf_chunk_game_set {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        // this chunk in this version is BE
        let endian = Endian::Big;

        // Hopper file is packed AND compressed. First create the bitstream for unpacking.
        let mut packed_buffer = Vec::<u8>::new();
        reader.read_to_end(&mut packed_buffer)?;
        let mut bitstream = c_bitstream_reader::new(packed_buffer.as_slice(), e_bitstream_byte_order::from_binrw_endian(endian));

        // Now decompress.
        let compressed_length= bitstream.read_integer(14)? - 4; // this -4 is necessary, but idk why
        let decompressed_length = bitstream.read_integer(32)?;
        let compressed_hopper_table_data: Vec<u8> = bitstream.read_raw_data((compressed_length * 8) as usize)?;
        let mut decompressed_hopper_table_data: Vec<u8> = Vec::with_capacity(decompressed_length as usize);
        let mut decoder = ZlibDecoder::new(Cursor::new(compressed_hopper_table_data));
        decoder.read_to_end(&mut decompressed_hopper_table_data)?;

        // Read the unpacked, decompressed chunk.
        let mut decompressed_hopper_reader = Cursor::new(decompressed_hopper_table_data);
        let mut game_set = Self::default();
        let game_entry_count: u32 = BinRead::read_options(&mut decompressed_hopper_reader, endian, args)?;

        for i in 0..game_entry_count {
            game_set.entries.push(BinRead::read_options(&mut decompressed_hopper_reader, endian, args)?);
        }

        Ok(game_set)
    }
}

impl BinWrite for s_blf_chunk_game_set {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        // 2. Deflate via zlib
        // 3. Write packed chunk

        // this chunk in this version is BE
        let endian = Endian::Big;

        // 1. Encode chunk
        let mut encoded_chunk = Vec::<u8>::new();
        let mut encoded_writer = Cursor::new(&mut encoded_chunk);

        let entries_count = self.entries.len() as u32;
        let game_set_entries: StaticArray<s_game_set_entry, k_maximum_game_entries>
            = StaticArray::from_vec(&self.entries)?;

        entries_count.write_options(&mut encoded_writer, endian, args)?;
        game_set_entries.write_options(&mut encoded_writer, endian, args)?;

        // 2. Deflate
        let mut e = ZlibEncoder::new(Vec::new(), Compression::new(9));
        e.write_all(encoded_chunk.as_slice())?;
        let compressed_data = e.finish()?;

        // 3. Pack
        let compressed_length: u16 = compressed_data.len() as u16;
        let uncompressed_length: u32 = encoded_chunk.len() as u32;
        let mut packed_writer = create_bitstream_writer(0xD404, e_bitstream_byte_order::from_binrw_endian(endian));
        packed_writer.write_integer((compressed_length + 4) as u32, 14)?;
        packed_writer.write_integer(uncompressed_length, 32)?;
        packed_writer.write_raw_data(&compressed_data, (compressed_length * 8) as usize)?;
        writer.write_ne(&close_bitstream_writer(&mut packed_writer)?)?;

        Ok(())
    }
}


#[derive(Clone, Default, PartialEq, Debug, Copy, Serialize, Deserialize)]
#[binrw]
pub struct s_game_set_entry {
    pub dword0: u32,
    pub dword4: u32,
    pub dword8: u32,
    pub dwordc: u32,
    pub dword10: u32,
    pub dword14: u32,
    pub dword18: u32,
    pub dword1c: u32,
    pub dword20: u32,
    pub dword24: u32,
    pub dword28: u32,
    pub gap2c: u16,
    pub word2e: u16,
    pub dword30: u32,
    pub dword34: u32,
    pub float38: Float32,
    pub float3c: Float32,
    #[brw(pad_after = 3)]
    pub byte40: u8,
    // #[serde(skip_serializing,skip_deserializing)]
    pub map_id: u32,
    #[serde(skip_serializing,skip_deserializing)]
    has_game_variant: Bool,  // set before write via hook
    #[serde(skip_serializing_if = "StaticString::is_empty", default)]
    pub game_name: StaticString<16>,
    #[serde(skip_serializing_if = "StaticString::is_empty", default)]
    pub game_variant_file_name: StaticString<32>,
    #[serde(skip_serializing,skip_deserializing)]
    pub game_variant_hash: s_network_http_request_hash,
    #[serde(skip_serializing,skip_deserializing)]
    has_map_variant: Bool, // set before write via hook
    #[serde(skip_serializing_if = "StaticString::is_empty", default)]
    pub map_name: StaticString<16>,
    #[serde(skip_serializing_if = "StaticString::is_empty", default)]
    pub map_variant_file_name: StaticString<32>,
    #[serde(skip_serializing,skip_deserializing)]
    pub map_variant_hash: s_network_http_request_hash,
    pub unknown3: u16,
}
