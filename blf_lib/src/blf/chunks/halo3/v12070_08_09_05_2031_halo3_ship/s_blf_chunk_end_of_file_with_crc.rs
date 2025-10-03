use std::io::{Read, Seek, Write};
use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::blam::common::memory::crc::crc_checksum_buffer;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derive::BlfChunk;
use crate::assert_ok;

#[binrw]
#[brw(big)]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("_eof", 1.1)]
#[Size(0x9)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_blf_chunk_end_of_file_with_crc
{
    pub file_size: u32,
    #[brw(magic(1u8))]
    pub crc: u32,
}

impl BlfChunkHooks for s_blf_chunk_end_of_file_with_crc {
    fn before_write(&mut self, previously_written: &Vec<u8>) -> BLFLibResult {
        self.file_size = previously_written.len() as u32;
        self.crc = crc_checksum_buffer(0xFFFFFFFF, previously_written);

        Ok(())
    }

    fn after_read(&mut self, previously_read: &Vec<u8>) -> BLFLibResult {
        let expected_crc = self.crc;
        let actual_crc = crc_checksum_buffer(0xFFFFFFFF, previously_read);
        assert_ok!(expected_crc == actual_crc, "s_blf_chunk_end_of_file_with_crc has an invalid crc");
        assert_ok!(self.file_size == previously_read.len() as u32, "_eof has an invalid size");

        Ok(())
    }
}

impl s_blf_chunk_end_of_file_with_crc {
    pub fn new(file_size: u32) -> s_blf_chunk_end_of_file_with_crc {
        s_blf_chunk_end_of_file_with_crc {
            file_size,
            crc: 0,
        }
    }
}