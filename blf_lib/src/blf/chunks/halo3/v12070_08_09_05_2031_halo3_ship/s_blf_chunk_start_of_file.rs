use binrw::binrw;
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::byte_order_mark::byte_order_mark;
use crate::types::c_string::StaticString;

pub const k_tag_string_length: usize = 32;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("_blf", 1.2)]
#[Size(0x24)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct s_blf_chunk_start_of_file
{
    pub byte_order_mark: byte_order_mark,
    #[brw(pad_after = 2)]
    pub name: StaticString<k_tag_string_length>,
}

impl BlfChunkHooks for s_blf_chunk_start_of_file {}

impl s_blf_chunk_start_of_file {
    pub fn new(name: &str, byte_order_mark: byte_order_mark) -> BLFLibResult<s_blf_chunk_start_of_file> {
        Ok(s_blf_chunk_start_of_file {
            byte_order_mark,
            name: StaticString::from_string(name.to_string())?,
        })
    }
}