use binrw::binrw;
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_variant::c_game_variant;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::scenario_map_variant::c_map_variant;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derive::BlfChunk;

#[binrw]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("chdr", 10.2)]
#[brw(big)]
pub struct s_blf_chunk_content_header
{
    pub build_number: u16,
    pub map_minor_version: u16,
    pub metadata: s_content_item_metadata,
}

impl BlfChunkHooks for s_blf_chunk_content_header {}

impl s_blf_chunk_content_header {
    pub fn create_for_game_variant(game_variant: &c_game_variant) -> BLFLibResult<s_blf_chunk_content_header> {
        Ok(s_blf_chunk_content_header {
            build_number: 12065,
            map_minor_version: 0,
            metadata: game_variant.get_metadata()?.clone(),
        })
    }

    pub fn create_for_map_variant(map_variant: &c_map_variant) -> s_blf_chunk_content_header {
        s_blf_chunk_content_header {
            build_number: 12065,
            map_minor_version: 0,
            metadata: map_variant.m_metadata.clone(),
        }
    }
}