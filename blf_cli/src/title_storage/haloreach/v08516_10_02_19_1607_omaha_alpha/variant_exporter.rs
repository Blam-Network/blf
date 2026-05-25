use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::game_variant::c_game_variant;
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::BlfFileBuilder;
use blf_lib::blf::versions::haloreach::v08516_10_02_19_1607_omaha_alpha::{
    s_blf_chunk_end_of_file, s_blf_chunk_matchmaking_game_variant, s_blf_chunk_start_of_file,
};
use blf_lib::blf::versions::haloreach::v09730_10_04_09_1309_omaha_delta::s_blf_chunk_map_variant;
use blf_lib::io::read_json_file;
use crate::console::console_task;

pub fn export_variant(source_json_path: &str, desination_path: &str) {
    let mut task = console_task::start("Exporting Variant");

    let mut blf_file = BlfFileBuilder::new();
    blf_file.add_chunk(s_blf_chunk_start_of_file::default());

    let map_variant = read_json_file::<c_map_variant>(source_json_path);
    let game_variant = read_json_file::<c_game_variant>(source_json_path);

    if map_variant.is_ok() {
        blf_file.add_chunk(s_blf_chunk_map_variant {
            map_variant: map_variant.unwrap(),
        });
    } else if game_variant.is_ok() {
        blf_file.add_chunk(s_blf_chunk_matchmaking_game_variant {
            game_variant: game_variant.unwrap(),
        });
    } else {
        task.fail_with_error("Unrecognized variant file type.");
    }

    blf_file.add_chunk(s_blf_chunk_end_of_file::default());

    blf_file.write_file(desination_path).unwrap();

    task.complete();
}
