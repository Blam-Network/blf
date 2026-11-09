use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

/// Reach Spartan render upload (`auiu` 1.2): 12-byte BE header + RGBA8 pixels.
#[binrw]
#[derive(BlfChunk, Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[Header("auiu", 1.2)]
#[brw(big)]
pub struct s_blf_chunk_auth_upload_image {
    pub width: u16,
    pub height: u16,
    pub stride: u16,
    pub reserved: u16,
    pub data_size: u32,
    #[br(count = data_size)]
    pub pixels: Vec<u8>,
}

impl BlfChunkHooks for s_blf_chunk_auth_upload_image {}
