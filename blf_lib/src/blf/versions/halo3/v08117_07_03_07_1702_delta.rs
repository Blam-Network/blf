use blf_lib_derive::TitleAndBuild;
use crate::blf::chunks::halo3;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::*;
pub use halo3::v08117_07_03_07_1702_delta::s_blf_chunk_author::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file_with_crc::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file_with_sha1::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file_with_rsa::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_message_of_the_day::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_banhammer_messages::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_matchmaking_tips::*;
pub use halo3::v08117_07_03_07_1702_delta::s_blf_chunk_network_configuration::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_map_manifest::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_hopper_description_table::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_online_file_manifest::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_content_header::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_user_recent_players::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_player_data::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_compressed_data::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_matchmaking_hopper_statistics::*;

#[derive(TitleAndBuild)]
#[Title("Halo 3")]
#[Build("08117.07.03.07.1702.delta")]
pub struct v08117_07_03_07_1702_delta {}