use blf_lib_derive::TitleAndBuild;
use crate::blf::chunks::halo3odst_mcc;

pub use halo3odst_mcc::v_untracked_25_08_16_1402::s_blf_chunk_game_variant::*;
pub use halo3odst_mcc::v_untracked_25_08_16_1402::s_blf_chunk_packed_game_variant::*;

#[derive(TitleAndBuild)]
#[Title("Halo 3: ODST")]
#[Build("untracked version")]
pub struct v_untracked_25_08_16_1402 {}
