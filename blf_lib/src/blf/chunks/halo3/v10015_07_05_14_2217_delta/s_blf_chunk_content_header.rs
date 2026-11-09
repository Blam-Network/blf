use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::saved_games::saved_game_files::s_content_item_metadata;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Size(0xFC)]
#[Header("chdr", 9.1)]
#[brw(big)]
pub struct s_blf_chunk_content_header
{
    pub build_number: u16,
    pub build_sequence_number: u16,
    pub metadata: s_content_item_metadata,
}

impl BlfChunkHooks for s_blf_chunk_content_header {}