use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3odst::release::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
#[cfg(feature = "napi")]
use napi_derive::napi;

#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3odst_13895_09_04_27_2201_atlas_release"))]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("chdr", 9.3)]
#[brw(big)]
pub struct s_blf_chunk_content_header
{
    pub build_number: u16,
    pub map_minor_version: u16,
    pub metadata: s_content_item_metadata,
}

impl BlfChunkHooks for s_blf_chunk_content_header {}
