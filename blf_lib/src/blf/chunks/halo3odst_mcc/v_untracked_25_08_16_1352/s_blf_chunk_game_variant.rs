use binrw::binrw;
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3odst_mcc::v_untracked_25_08_16_1352::game::game_engine_variant::c_game_variant;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mpvr", 3.1)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3odst_mcc_v_untracked_25_08_16_1352"))]
pub struct s_blf_chunk_game_variant
{
    pub game_variant: c_game_variant,
}

impl BlfChunkHooks for s_blf_chunk_game_variant {}

impl s_blf_chunk_game_variant {
    pub fn create(game_variant: c_game_variant) -> s_blf_chunk_game_variant {
        s_blf_chunk_game_variant { game_variant }
    }
}