use std::error::Error;
use std::path::Path;

use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::game_variant::c_game_variant;
use blf_lib::io::read_json_file;

use crate::title_storage::haloreach::megalo_builder::{
    build_megalo_from_folder, encode_megalo_bitstream,
};

pub fn build_megalo(json_input_folder: &str, mglo_output_folder: &str) -> Result<(), Box<dyn Error>> {
    build_megalo_from_folder(json_input_folder, mglo_output_folder, encode_game_variant_json)
}

fn encode_game_variant_json(json_path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let game_variant: c_game_variant = read_json_file(json_path.to_string_lossy().as_ref())?;
    encode_megalo_bitstream(|bitstream| game_variant.encode(bitstream)).map_err(|err| err.into())
}
