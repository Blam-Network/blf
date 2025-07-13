use std::io::{Cursor, Read, Seek, Write};
use binrw::{binrw, BinRead, BinResult, BinWrite, BinWriterExt, Endian};
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

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fmca", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_megalo_categories
{
    #[bw(try_calc(u32::try_from(categories.len())))]
    category_count: u32,
    #[br(count = category_count)]
    pub categories: Vec<s_blf_chunk_megalo_category>,
}

impl BlfChunkHooks for s_blf_chunk_megalo_categories {}
#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_blf_chunk_megalo_category {
    pub category_id: u32,
    pub category_name: StaticWcharString<32>,
}
