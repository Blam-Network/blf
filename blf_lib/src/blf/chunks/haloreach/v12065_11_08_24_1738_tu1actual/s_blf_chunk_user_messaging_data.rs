use binrw::{binrw, BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use blf_lib::types::bool::Bool;
use blf_lib::types::time::time64_t;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::array::StaticArray;
use crate::types::c_string::StaticString;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("umsg", 1.0)]
#[brw(big)]
#[Size(0xC)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_user_messaging_data {
    pub unknown0: u64, // might be unused.
    pub message_index: u64,
    pub expires_at: time64_t,
}
