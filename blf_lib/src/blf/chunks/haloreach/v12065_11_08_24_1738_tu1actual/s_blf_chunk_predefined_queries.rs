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

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("fpre", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_predefined_queries
{
    #[bw(try_calc(u32::try_from(queries.len())))]
    query_count: u32,
    #[br(count = query_count)]
    pub queries: Vec<s_blf_chunk_predefined_query>,
}

impl BlfChunkHooks for s_blf_chunk_predefined_queries {}
#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_blf_chunk_predefined_query {
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown3: u32,
    pub name: StaticWcharString<32>,
    pub description: StaticWcharString<256>,
}
