use std::io::{Read, Seek, Write};
use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derive::BlfChunk;
use crate::assert_ok;
use crate::types::array::StaticArray;

#[binrw]
#[brw(big)]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("_eof", 1.1)]
#[Size(0x105)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_blf_chunk_end_of_file_with_rsa
{
    pub file_size: u32,
    #[brw(magic(3u8))]
    pub rsa: StaticArray<u8, 256>,
}

impl BlfChunkHooks for s_blf_chunk_end_of_file_with_rsa {
    fn before_write(&mut self, previously_written: &Vec<u8>) -> BLFLibResult {
        self.file_size = previously_written.len() as u32;

        Err("s_blf_chunk_end_of_file_with_sha1 does not support write yet.".into())
    }

    fn after_read(&mut self, previously_read: &Vec<u8>) -> BLFLibResult {
        // TODO: Validate
        assert_ok!(self.file_size == previously_read.len() as u32, "_eof has an invalid size");
        Ok(())
    }
}

impl s_blf_chunk_end_of_file_with_rsa {
    pub fn new(file_size: u32) -> s_blf_chunk_end_of_file_with_rsa {
        s_blf_chunk_end_of_file_with_rsa {
            file_size,
            rsa: StaticArray::default(),
        }
    }
}