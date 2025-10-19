use binrw::binrw;
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_10015_07_05_14_2217_delta"))]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Size(0xFC)]
#[Header("chdr", 9.1)]
#[brw(big)]
pub struct s_blf_chunk_content_header
{
    pub build_number: u16,
    pub map_minor_version: u16,
    pub metadata: s_content_item_metadata,
}

impl BlfChunkHooks for s_blf_chunk_content_header {}