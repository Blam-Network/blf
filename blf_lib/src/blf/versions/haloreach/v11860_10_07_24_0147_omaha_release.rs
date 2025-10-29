use blf_lib_derive::TitleAndBuild;
use crate::blf::chunks::halo3;
use crate::blf::chunks::haloreach;
use crate::blf::chunks::halo3odst;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file_with_crc::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file_with_sha1::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file_with_rsa::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_map_manifest::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_hopper_description_table::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_online_file_manifest::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_banhammer_messages::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_matchmaking_tips::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_compressed_data::*;
pub use halo3odst::v13895_09_04_27_2201_atlas_release::s_blf_chunk_author::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_player_data::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_arena_hopper_stats::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_player_heartbeat_response::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_challenge_state::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_rewards_persistence::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_service_record::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_hopper_configuration_table::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_game_set::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_nag_message::*;
pub use haloreach::v11860_10_07_24_0147_omaha_release::s_blf_chunk_network_configuration::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_map_variant::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_content_header::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_matchmaking_game_variant::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_dlc_map_manifest::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_megalo_categories::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_predefined_queries::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_matchmaking_hopper_statistics::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_challenge_progress::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_reward_persistence_upload_to_lsp::*;

#[derive(TitleAndBuild)]
#[Title("Halo: Reach")]
#[Build("11860.10.07.24.0147.omaha_release")]
pub struct v11860_10_07_24_0147_omaha_release {}