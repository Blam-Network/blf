use blf_lib_derive::TitleAndBuild;
use crate::blf::chunks::halo3;
use crate::blf::chunks::haloreach;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file::*;
pub use haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_content_header::*;

#[derive(TitleAndBuild)]
#[Title("Destiny")]
#[Build("36735.13.12.02.1953.alpha")]
pub struct v36735_13_12_02_1953_alpha {}
