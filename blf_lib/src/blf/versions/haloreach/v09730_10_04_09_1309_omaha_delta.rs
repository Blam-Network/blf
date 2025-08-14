use blf_lib_derive::TitleAndBuild;
use crate::blf::chunks::halo3;
use crate::blf::chunks::haloreach;
use crate::blf::chunks::halo3odst;

pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file::*;
pub use halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file::*;
pub use halo3odst::v13895_09_04_27_2201_atlas_release::s_blf_chunk_author::*;
pub use haloreach::v09730_10_04_09_1309_omaha_delta::s_blf_chunk_hopper_configuration_table::*;

#[derive(TitleAndBuild)]
#[Title("Halo: Reach")]
#[Build("09730.10.04.09.1309.omaha_delta")]
pub struct v09730_10_04_09_1309_omaha_delta {}