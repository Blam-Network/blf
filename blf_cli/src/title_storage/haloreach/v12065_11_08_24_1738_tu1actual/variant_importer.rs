
use std::fs::File;
use std::path::Path;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_variant::c_game_variant;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::scenario_map_variant::{c_map_variant, s_variant_object_datum, s_variant_quota};
use blf_lib::blf::chunks::search_for_chunk_in_file;
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::{s_blf_chunk_game_variant, s_blf_chunk_map_variant, s_blf_chunk_matchmaking_game_variant};
use crate::build_path;
use crate::console::console_task;

pub fn import_variant(hoppers_config_path: &String, variant_path: &String) {
    let mut task = console_task::start("Importing Variant");

    let mut game_variant: Option<c_game_variant> = None;
    let game_variant_chunk = search_for_chunk_in_file::<s_blf_chunk_game_variant>(variant_path).unwrap_or(None);
    if game_variant_chunk.is_some() {
        game_variant = Some(game_variant_chunk.unwrap().game_variant);
    }
    let packed_game_variant_chunk = search_for_chunk_in_file::<s_blf_chunk_matchmaking_game_variant>(variant_path).unwrap_or(None);
    if packed_game_variant_chunk.is_some() {
        game_variant = Some(packed_game_variant_chunk.unwrap().game_variant);
    }
    if game_variant.is_some() {
        let game_variant = game_variant.unwrap();
        let metadata = game_variant.get_metadata().unwrap();
        // let output_file_name = format!("{}.json", metadata.name.get_string()
        //     .replace(" ", "_")
        //     .to_lowercase());
        let output_file_name = Path::new(variant_path).file_name().unwrap().to_str().unwrap().to_string().replace(".bin", ".json");
        let output_file = File::create(build_path!(
            hoppers_config_path,
            "game_variants",
            &output_file_name
        )).unwrap();
        serde_json::to_writer_pretty(output_file, &game_variant.clone()).unwrap();

        task.add_message(format!("Added game variant: {output_file_name}"));
        task.complete();
        return;
    }

    let mut map_variant: Option<c_map_variant> = None;
    let map_variant_chunk = search_for_chunk_in_file::<s_blf_chunk_map_variant>(variant_path).unwrap_or(None);
    if map_variant_chunk.is_some() {
        map_variant = Some(map_variant_chunk.unwrap().map_variant);
    }
    if map_variant.is_some() {
        let mut map_variant = map_variant.unwrap().clone();

        let output_file_name = format!("{}.json", map_variant.m_metadata.name.get_string()
            .replace(" ", "_")
            .to_lowercase());

        let output_file = File::create(build_path!(
            hoppers_config_path,
            "map_variants",
            &output_file_name
        )).unwrap();

        serde_json::to_writer_pretty(output_file, &map_variant.clone()).unwrap();

        task.add_message(format!("Added map variant: {output_file_name}"));
        task.complete();
        return;
    }

    task.fail_with_error(format!("Unable to parse variant file. {}", variant_path));
}
