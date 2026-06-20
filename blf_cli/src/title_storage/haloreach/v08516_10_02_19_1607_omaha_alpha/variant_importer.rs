use std::fs::File;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::game_variant::{c_game_variant, e_game_mode};
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::chunks::search_for_chunk_in_file;
use blf_lib::blf::versions::haloreach::v08516_10_02_19_1607_omaha_alpha::s_blf_chunk_matchmaking_game_variant;
use blf_lib::blf::versions::haloreach::v09730_10_04_09_1309_omaha_delta::s_blf_chunk_map_variant;
use blf_lib::result::BLFLibResult;
use crate::build_path;
use crate::console::console_task;
use crate::title_storage::haloreach::variant_io::variant_json_output_name;
use crate::title_storage::remove_invalid_characters;

fn game_variant_metadata_name(game_variant: &c_game_variant) -> BLFLibResult<String> {
    let metadata = match game_variant.m_game_engine {
        e_game_mode::sandbox => {
            &game_variant
                .m_sandbox_variant
                .as_ref()
                .ok_or("m_sandbox_variant does not exist.")?
                .m_custom_variant
                .m_base_variant
                .m_metadata
        }
        e_game_mode::megalogamengine => {
            &game_variant
                .m_custom_variant
                .as_ref()
                .ok_or("m_custom_variant does not exist.")?
                .m_base_variant
                .m_metadata
        }
        e_game_mode::campaign => {
            &game_variant
                .m_campaign_variant
                .as_ref()
                .ok_or("m_campaign_variant does not exist.")?
                .m_metadata
        }
        e_game_mode::survival => {
            &game_variant
                .m_survival_variant
                .as_ref()
                .ok_or("m_survival_variant does not exist.")?
                .m_base_variant
                .m_metadata
        }
    };
    Ok(metadata.name.get_string())
}

pub fn import_variant(hoppers_config_path: &String, variant_path: &String) {
    let mut task = console_task::start("Importing Variant");

    let game_variant_chunk =
        search_for_chunk_in_file::<s_blf_chunk_matchmaking_game_variant>(variant_path)
            .unwrap_or(None);
    if let Some(chunk) = game_variant_chunk {
        let game_variant = chunk.game_variant;
        let output_file_name = variant_json_output_name(
            variant_path,
            &game_variant_metadata_name(&game_variant).unwrap(),
        );

        let output_file = File::create(build_path!(
            hoppers_config_path,
            "game_variants",
            remove_invalid_characters(&output_file_name)
        ))
        .unwrap();
        serde_json::to_writer_pretty(output_file, &game_variant).unwrap();

        task.add_message(format!("Added game variant: {output_file_name}"));
        task.complete();
        return;
    }

    let map_variant_chunk =
        search_for_chunk_in_file::<s_blf_chunk_map_variant>(variant_path).unwrap_or(None);
    if let Some(chunk) = map_variant_chunk {
        let map_variant = chunk.map_variant;

        let output_file_name = variant_json_output_name(
            variant_path,
            &map_variant.m_metadata.name.get_string(),
        );

        let output_file = File::create(build_path!(
            hoppers_config_path,
            "map_variants",
            remove_invalid_characters(&output_file_name)
        ))
        .unwrap();
        serde_json::to_writer_pretty(output_file, &map_variant).unwrap();

        task.add_message(format!("Added map variant: {output_file_name}"));
        task.complete();
        return;
    }

    task.fail_with_error(format!("Unable to parse variant file. {}", variant_path));
}
