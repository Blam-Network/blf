use blf_lib_derive::TitleAndBuild;
use crate::blf::chunks::halo3;
use crate::blf::chunks::halo4;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file_with_crc::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file_with_sha1::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file_with_rsa::*;
pub use halo4::v20810_12_09_22_1647_main::s_blf_chunk_packed_game_variant::*;
pub use halo4::v20810_12_09_22_1647_main::s_blf_chunk_game_variant::*;

#[derive(TitleAndBuild)]
#[Title("Halo 4")]
#[Build("20810.12.09.22.1647.main")]
pub struct v20810_12_09_22_1647_main {}
