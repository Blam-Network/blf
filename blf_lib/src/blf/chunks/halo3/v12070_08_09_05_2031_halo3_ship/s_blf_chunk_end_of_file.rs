use binrw::{binrw, BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("_eof", 1.1)]
#[Size(0x5)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_blf_chunk_end_of_file
{
    pub file_size: u32,
    pub authentication_type: e_blf_file_authentication_type,
}

impl BlfChunkHooks for s_blf_chunk_end_of_file {
    fn before_write(&mut self, previously_written: &Vec<u8>) -> BLFLibResult {
        self.file_size = previously_written.len() as u32;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, BinRead, BinWrite, Serialize, Deserialize, Default)]
#[brw(repr = u8)]
#[cfg_attr(feature = "napi", napi(namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
#[repr(u8)]
pub enum e_blf_file_authentication_type {
    #[default]
    none = 0,
    crc = 1,
    sha1 = 2,
    rsa = 3
}

impl s_blf_chunk_end_of_file {
    pub fn new(file_size: u32, authentication_type: e_blf_file_authentication_type) -> s_blf_chunk_end_of_file {
        s_blf_chunk_end_of_file {
            file_size,
            authentication_type,
        }
    }
}