use std::io::{Cursor, Read, Seek, Write};
use binrw::{BinRead, BinResult, BinWrite, BinWriterExt, Endian};
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
use blf_lib::types::c_string::StaticWcharString;
use blf_lib::types::u64::Unsigned64;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::{BlfChunk, TestSize};
use crate::types::numbers::Float32;

pub const k_manifest_maximum_maps_count: usize = 16;

#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize,BinRead,BinWrite)]
#[Header("dlcd", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_dlc_map_manifest
{
    pub map_count: u32,
    pub maps: StaticArray<s_blf_chunk_dlc_map_manifest_entry, k_manifest_maximum_maps_count>,
}

impl BlfChunkHooks for s_blf_chunk_dlc_map_manifest {}
#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_blf_chunk_dlc_map_manifest_entry {
    pub map_type: u32, // 2 = mp, 3 // ff
    pub map_id: u32,
    pub name_en: StaticWcharString<0x20>,
    pub name_jpn: StaticWcharString<0x20>,
    pub name_de: StaticWcharString<0x20>,
    pub name_fr: StaticWcharString<0x20>,
    pub name_sp: StaticWcharString<0x20>,
    pub name_mx: StaticWcharString<0x20>,
    pub name_it: StaticWcharString<0x20>,
    pub name_ko: StaticWcharString<0x20>,
    pub name_cht: StaticWcharString<0x40>,
    pub name_pt: StaticWcharString<0x40>,
    pub description_en: StaticWcharString<0x80>,
    pub description_jpn: StaticWcharString<0x80>,
    pub description_de: StaticWcharString<0x80>,
    pub description_fr: StaticWcharString<0x80>,
    pub description_sp: StaticWcharString<0x80>,
    pub description_mx: StaticWcharString<0x80>,
    pub description_it: StaticWcharString<0x80>,
    pub description_ko: StaticWcharString<0x80>,
    pub description_cht: StaticWcharString<0x100>,
    pub description_pt: StaticWcharString<0x100>,
    pub small_image_file_name: StaticString<32>,
    pub large_image_file_name: StaticString<32>,
    pub dlc_offer_id: Unsigned64,
}
