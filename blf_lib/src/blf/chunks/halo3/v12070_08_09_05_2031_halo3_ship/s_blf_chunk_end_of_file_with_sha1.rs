use std::io::{Read, Seek, Write};
use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blf::get_buffer_hash;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derive::BlfChunk;
use crate::assert_ok;

#[binrw]
#[brw(big)]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("_eof", 1.1)]
#[Size(0x19)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_blf_chunk_end_of_file_with_sha1
{
    pub file_size: u32,
    #[brw(magic(2u8))]
    pub sha1: s_network_http_request_hash,
}

impl BlfChunkHooks for s_blf_chunk_end_of_file_with_sha1 {
    fn before_write(&mut self, previously_written: &Vec<u8>) -> BLFLibResult {
        self.file_size = previously_written.len() as u32;
        self.sha1 = get_buffer_hash(previously_written)?;

        Ok(())
    }

    fn after_read(&mut self, previously_read: &[u8]) -> BLFLibResult {
        let expected_hash = &self.sha1;
        let actual_hash = &get_buffer_hash(&previously_read)?;
        assert_ok!(actual_hash == expected_hash, "s_blf_chunk_end_of_file_with_sha1 has an invalid sha1");
        assert_ok!(self.file_size == previously_read.len() as u32, "_eof has an invalid size");

        Ok(())
    }
}

impl s_blf_chunk_end_of_file_with_sha1 {
    pub fn new(file_size: u32) -> s_blf_chunk_end_of_file_with_sha1 {
        s_blf_chunk_end_of_file_with_sha1 {
            file_size,
            sha1: s_network_http_request_hash::default(),
        }
    }
}