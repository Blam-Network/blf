use std::io::{Cursor, Read, Seek, Write};
use binrw::{binrw, BinRead, BinResult, BinWrite, BinWriterExt, Endian};
use flate2::{Compress, Compression};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::data_compress::runtime_data_compress;
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer, e_bitstream_byte_order};
use blf_lib::types::array::StaticArray;
use crate::types::c_string::StaticString;
use blf_lib::types::bool::Bool;
use blf_lib::types::c_string::StaticWcharString;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derive::BlfChunk;
use crate::blam::common::memory::data_compress::runtime_data_decompress;
use crate::types::numbers::Float32;

pub const k_maximum_game_entries: usize = 256;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("gset", 15.1)]
pub struct s_blf_chunk_game_set
{
    pub entries: Vec<s_game_set_entry>,
}

impl BlfChunkHooks for s_blf_chunk_game_set {
    fn before_write(&mut self, _previously_written: &Vec<u8>) -> BLFLibResult {
        for entry in self.entries.iter_mut() {
            entry.game_variant_file.exists = Bool::from(!entry.game_variant_file.file_name.get_string()?.is_empty());
            entry.map_variant_file.exists = Bool::from(!entry.map_variant_file.file_name.get_string()?.is_empty());
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
        let compressed_length = bitstream.read_unnamed_integer::<usize>(14)?; // this -4 is necessary, but idk why
        let compressed_data = bitstream.read_raw_data(compressed_length * 8)?;
        let mut decompressed_data = Vec::<u8>::new();
        runtime_data_decompress(&compressed_data, &mut decompressed_data, e_bitstream_byte_order::_bitstream_byte_order_big_endian)?;

        // Read the unpacked, decompressed chunk.
        let mut decompressed_hopper_reader = Cursor::new(decompressed_data);
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
        let mut compressed_data = Vec::new();
        runtime_data_compress(&encoded_chunk, &mut compressed_data, e_bitstream_byte_order::_bitstream_byte_order_big_endian)?;

        // 3. Pack
        let compressed_length: u16 = compressed_data.len() as u16;
        let mut packed_writer = c_bitstream_writer::new(0xD404, e_bitstream_byte_order::from_binrw_endian(endian));
        packed_writer.begin_writing();

        packed_writer.write_integer(compressed_length as u32, 14)?;
        packed_writer.write_raw_data(&compressed_data, (compressed_length * 8) as usize)?;

        packed_writer.finish_writing();
        writer.write_ne(&packed_writer.get_data()?)?;

        Ok(())
    }
}


#[derive(Clone, Default, PartialEq, Debug, Copy, Serialize, Deserialize)]
#[binrw]
pub struct s_game_set_entry_campaign_and_survival_data {
    #[brw(pad_after = 3)]
    pub unknown00: Bool,
    pub unknown04: u32,
    pub unknown08: u32,
    pub unknown0C: u32,
}

#[derive(Clone, Default, PartialEq, Debug, Copy, Serialize, Deserialize)]
#[binrw]
pub struct s_game_set_entry_replicated_data {
    #[brw(pad_after = 1)]
    pub unknown00: Bool,
    pub unknown02: u16,
    pub unknown04: u32,
    pub unknown08: Float32,
    pub unknown0C: Float32,
    pub unknown10: Float32,
    #[brw(pad_after = 3)]
    pub unknown14: Bool,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
#[binrw]
pub struct s_game_set_entry {
    pub weight: u32,
    pub minimum_player_count: u32,
    pub maximum_player_count: u32,
    pub voting_max_fails: u32,
    pub voting_round: u32,
    pub min_skill: u32,
    pub max_skill: u32,
    pub campaign_and_survival_data: s_game_set_entry_campaign_and_survival_data,
    pub replicated_data:s_game_set_entry_replicated_data,
    pub map_id: u32,
    #[serde(skip_serializing_if = "s_game_set_file::is_empty", default)]
    pub game_variant_file: s_game_set_file,
    #[serde(skip_serializing_if = "s_game_set_file::is_empty", default)]
    #[brw(pad_after = 2)]
    pub map_variant_file: s_game_set_file,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
#[binrw]
pub struct s_game_set_file {
    #[serde(skip_serializing,skip_deserializing)]
    exists: Bool,  // set before write via hook
    #[serde(skip_serializing_if = "StaticString::is_empty", default)]
    pub name: StaticString<16>,
    #[serde(skip_serializing_if = "StaticString::is_empty", default)]
    pub file_name: StaticString<32>,
    #[serde(skip_serializing,skip_deserializing)]
    pub hash: s_network_http_request_hash,
}

impl s_game_set_file {
    pub fn is_empty(&self) -> bool {
        !self.exists.0
    }
}