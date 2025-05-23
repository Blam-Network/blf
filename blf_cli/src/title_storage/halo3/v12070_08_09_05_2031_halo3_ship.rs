mod blf_files;
pub mod variant_importer;
pub mod variant_exporter;

use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::fs::{create_dir_all, exists, remove_file, File};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;
use crate::io::{get_directories_in_folder, get_files_in_folder, FILE_SEPARATOR};
use crate::{build_path, debug_log, title_converter, やった};
use crate::title_storage::{check_file_exists, TitleConverter};
use inline_colorization::*;
use lazy_static::lazy_static;
use blf_lib::blam::common::cseries::language::{get_language_string, k_language_suffix_chinese_traditional, k_language_suffix_english, k_language_suffix_french, k_language_suffix_german, k_language_suffix_italian, k_language_suffix_japanese, k_language_suffix_korean, k_language_suffix_mexican, k_language_suffix_portuguese, k_language_suffix_spanish};
use blf_lib::blf::{get_blf_file_hash, BlfFile};
use blf_lib::blf::chunks::{find_chunk_in_file, BlfChunk};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_hopper_description_table, s_blf_chunk_network_configuration, s_blf_chunk_packed_game_variant, s_blf_chunk_packed_map_variant};
use crate::console::console_task;
use crate::title_storage::halo3::release::blf_files::motd_popup::{k_motd_popup_config_folder, k_motd_popup_file_name, k_motd_popup_image_file_name, k_mythic_motd_popup_config_folder, k_mythic_motd_popup_file_name, k_mythic_motd_popup_image_file_name, motd_popup};
use crate::title_storage::halo3::release::blf_files::matchmaking_banhammer_messages::{k_matchmaking_banhammer_messages_file_name, matchmaking_banhammer_messages};
use crate::title_storage::halo3::release::blf_files::matchmaking_tips::{k_matchmaking_tips_file_name, matchmaking_tips};
use regex::Regex;
use tempdir::TempDir;
use tokio::runtime;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use blf_lib::blam::common::memory::crc::crc32;
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blam::halo_3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blam::halo_3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_hopper_configuration_table;
use crate::title_storage::halo3::release::blf_files::game_variant::{game_variant, k_game_variants_config_folder_name};
use crate::title_storage::halo3::release::blf_files::manifest::{k_manifest_file_name, manifest};
use crate::title_storage::halo3::release::blf_files::map_variant::{k_map_variants_blf_folder_name, k_map_variants_config_folder_name, map_variant};
use crate::title_storage::halo3::release::blf_files::matchmaking_hopper::{k_active_hoppers_config_file_name, k_categories_config_file_name, k_hopper_config_file_name, k_hoppers_config_folder_name, k_matchmaking_hopper_file_name, matchmaking_hopper, matchmaking_hopper_categories_config, matchmaking_hopper_category_configuration_and_descriptions, matchmaking_hopper_config, read_active_hoppers};
use crate::title_storage::halo3::release::blf_files::matchmaking_hopper_descriptions::{k_matchmaking_hopper_descriptions_file_name, matchmaking_hopper_descriptions};
use blf_files::network_configuration::network_configuration;
use crate::title_storage::halo3::release::blf_files::game_set::{k_game_set_blf_file_name, game_set_config, game_set, k_game_set_config_file_name};
use crate::title_storage::halo3::release::blf_files::{k_hopper_directory_name_max_length};
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::blf_files::network_configuration::k_network_configuration_file_name;
use crate::title_storage::halo3::release::blf_files::motd::{k_motd_config_folder, k_motd_file_name, k_motd_image_file_name, k_mythic_motd_config_folder, k_mythic_motd_file_name, k_mythic_motd_image_file_name, motd};
use crate::title_storage::halo3::release::blf_files::rsa_manifest::{k_rsa_manifest_file_name, k_rsa_signatures_config_folder_name, rsa_manifest};

pub const k_build_string_halo3_ship_12070: &str = "12070.08.09.05.2031.halo3_ship";

title_converter! (
    #[Title("Halo 3")]
    #[Build("12070.08.09.05.2031.halo3_ship")]
    pub struct v12070_08_09_05_2031_halo3_ship {}
);

// Halo 3's xex supports 12 languages, but only 10 were released.
pub const k_language_suffixes: [&str; 10] = [
    k_language_suffix_english,
    k_language_suffix_japanese,
    k_language_suffix_german,
    k_language_suffix_french,
    k_language_suffix_spanish,
    k_language_suffix_mexican,
    k_language_suffix_italian,
    k_language_suffix_korean,
    k_language_suffix_chinese_traditional,
    // k_language_suffix_chinese_simplified,
    k_language_suffix_portuguese,
    // k_language_suffix_polish,
];

lazy_static! {
    static ref hopper_folder_regex: Regex = Regex::new(r"^[0-9]{5}.*").unwrap();
    static ref config_hopper_folder_identifier_regex: Regex = Regex::new(r"^[0-9]{1,5}").unwrap();
    static ref map_variant_file_regex: Regex = Regex::new(&format!("_{:0>3}.bin$", s_blf_chunk_packed_map_variant::get_version().major)).unwrap();
    static ref game_variant_file_regex: Regex = Regex::new(&format!("_{:0>3}.bin$", s_blf_chunk_packed_game_variant::get_version().major)).unwrap();
    static ref config_rsa_signature_file_map_id_regex: Regex = Regex::new(r"^[0-9]{1,}").unwrap();
}

const k_build_temp_maps_folder_name: &str = "map_variants";
const k_build_temp_games_folder_name: &str = "game_variants";

impl TitleConverter for v12070_08_09_05_2031_halo3_ship {
    fn build_blfs(&mut self, config_path: &String, blfs_path: &String) {
        let start_time = SystemTime::now();

        println!("{style_bold}Writing Title Storage BLFs to {blfs_path} {style_reset}");

        let hopper_directories = get_directories_in_folder(config_path).unwrap_or_else(|err|{
            println!("{}", err);
            panic!()
        });

        for hopper_directory in hopper_directories {
            let result = || -> Result<(), Box<dyn Error>> {
                if hopper_directory.len() > k_hopper_directory_name_max_length {
                    return Err(Box::from(format!(
                        "Hoppers folder \"{hopper_directory}\" is too long and will be skipped. ({} > {} characters)",
                        hopper_directory.len(),
                        k_hopper_directory_name_max_length
                    )))
                }

                let build_temp_dir = TempDir::new("blf_cli")?;
                let build_temp_dir_path = String::from(build_temp_dir.path().to_str().unwrap());

                debug_log!("Using temp directory: {build_temp_dir_path}");

                let hopper_config_path = build_path!(
                    config_path,
                    &hopper_directory
                );

                let hopper_blfs_path = build_path!(
                    blfs_path,
                    &hopper_directory
                );

                println!("{style_bold}Converting {color_bright_white}{}{style_reset}...", hopper_directory);
                Self::build_blf_banhammer_messages(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_matchmaking_tips(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_motds(&hopper_config_path, &hopper_blfs_path, false)?;
                Self::build_blf_motds(&hopper_config_path, &hopper_blfs_path, true)?;
                Self::build_blf_motd_popups(&hopper_config_path, &hopper_blfs_path, false)?;
                Self::build_blf_motd_popups(&hopper_config_path, &hopper_blfs_path, true)?;
                Self::build_blf_map_manifest(&hopper_config_path, &hopper_blfs_path)?;

                let active_hoppers = Self::read_active_hopper_configuration(&hopper_config_path);
                let game_sets = Self::read_game_set_configuration(&hopper_config_path, &active_hoppers)?;
                let mut game_variant_hashes = HashMap::<String, s_network_http_request_hash>::new();
                let mut map_variant_hashes = HashMap::<String, s_network_http_request_hash>::new();
                let mut map_variant_map_ids = HashMap::<String, u32>::new();

                Self::build_blf_game_variants(&hopper_config_path, &hopper_blfs_path, &build_temp_dir_path, &game_sets, &mut game_variant_hashes);
                Self::build_blf_map_variants(&hopper_config_path, &hopper_blfs_path, &build_temp_dir_path, &game_sets, &mut map_variant_hashes, &mut map_variant_map_ids);
                Self::build_blf_game_sets(&hopper_blfs_path, game_sets, &game_variant_hashes, &map_variant_hashes, &map_variant_map_ids, &build_temp_dir_path)?;
                Self::build_blf_hoppers(&hopper_config_path, &hopper_blfs_path, &active_hoppers)?;
                Self::build_blf_network_configuration(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_manifest(&hopper_blfs_path)?;
                Ok(())
            }();

            if result.is_err() {
                println!("{color_red}Failed to build title storage for hoppers {hopper_directory}{style_reset}");
                println!("{color_red}{}{style_reset}", result.err().unwrap());
            }
        }

        let seconds = start_time.elapsed().unwrap().as_secs_f32();
        println!("Finished conversion in {seconds:.2} seconds.");
    }

    fn build_config(&mut self, blfs_path: &String, config_path: &String) {
        println!("{style_bold}Writing Title Storage config to {config_path} {style_reset}");

        let hopper_directories = get_directories_in_folder(blfs_path).unwrap_or_else(|err|{
            println!("{}", err);
            panic!();
        });

        for hopper_directory in hopper_directories {
            let result = || -> Result<(), Box<dyn Error>> {
                if hopper_directory.len() > k_hopper_directory_name_max_length {
                    return Err(Box::<dyn Error>::from(format!("Skipping \"{hopper_directory}\" as it's name is too long. ({k_hopper_directory_name_max_length} characters MAX)")))
                }

                let hoppers_config_path = build_path!(
                    config_path,
                    &hopper_directory
                );

                let hoppers_blf_path = build_path!(
                    blfs_path,
                    &hopper_directory
                );

                println!("{style_bold}Converting {color_bright_white}{}{style_reset}...", hopper_directory);
                Self::build_config_banhammer_messages(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_matchmaking_tips(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_motds(&hoppers_blf_path, &hoppers_config_path, false)?;
                Self::build_config_motds(&hoppers_blf_path, &hoppers_config_path, true)?;
                Self::build_config_motd_popups(&hoppers_blf_path, &hoppers_config_path, false)?;
                Self::build_config_motd_popups(&hoppers_blf_path, &hoppers_config_path, true)?;
                Self::build_config_map_variants(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_game_variants(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_game_sets(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_hoppers(&hoppers_blf_path, &hoppers_config_path);
                Self::build_config_network_configuration(&hoppers_blf_path, &hoppers_config_path)?;
                Ok(())
            }();

            if result.is_err() {
                println!("{color_red}Failed to build title storage for hoppers {hopper_directory}{style_reset}");
                println!("{color_red}{}{style_reset}", result.err().unwrap());
            }
        }
    }
}

impl v12070_08_09_05_2031_halo3_ship {
    fn build_config_banhammer_messages(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting Banhammer Messages");

        for language_code in k_language_suffixes {
            let blf_file_path = build_path!(hoppers_blf_path, language_code, k_matchmaking_banhammer_messages_file_name);

            if !check_file_exists(&blf_file_path) {
                task.add_warning(format!(
                    "No {} banhammer messages are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            matchmaking_banhammer_messages::read_file(&blf_file_path)?.write_to_config(hoppers_config_path, language_code)?;
        }

        やった!(task)
    }

    fn build_config_matchmaking_tips(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting Matchmaking Tips");

        for language_code in k_language_suffixes {
            let blf_file_path = build_path!(hoppers_blf_path, language_code, k_matchmaking_tips_file_name);

            if !check_file_exists(&blf_file_path) {
                task.add_warning(format!(
                    "No {} matchmaking tips are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            matchmaking_tips::read_file(&blf_file_path)?.write_to_config(hoppers_config_path, language_code)?;
        }

        やった!(task)
    }

    fn build_config_motds(hoppers_blf_path: &String, hoppers_config_path: &String, mythic: bool) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start(
            if mythic { "Converting Mythic MOTDs" }
            else { "Converting MOTDs" }
        );

        // BLFs
        for language_code in k_language_suffixes {
            let file_path = build_path!(
                hoppers_blf_path,
                language_code,
                if mythic { k_mythic_motd_file_name } else { k_motd_file_name }
            );


            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            motd::read_file(&file_path)?.write_to_config(hoppers_config_path, language_code, mythic)?;
        }

        // JPEGs
        for language_code in k_language_suffixes {
            let file_path = build_path!(
                hoppers_blf_path,
                language_code,
                if mythic { k_mythic_motd_image_file_name } else { k_motd_image_file_name }
            );

            let output_path = build_path!(
                hoppers_config_path,
                if mythic { k_mythic_motd_config_folder } else { k_motd_config_folder },
                format!("{language_code}.jpg")
            );

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD image is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            fs::copy(file_path, output_path)?;
        }

        やった!(task)
    }

    fn build_config_motd_popups(hoppers_blf_path: &String, hoppers_config_path: &String, mythic: bool) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start(
            if mythic { "Converting Mythic MOTD Popups" }
            else { "Converting MOTD Popups" }
        );

        // BLFs
        for language_code in k_language_suffixes {
            let file_path = build_path!(
                hoppers_blf_path,
                language_code,
                if mythic { k_mythic_motd_popup_file_name } else { k_motd_popup_file_name }
            );

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD Popup is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            motd_popup::read_file(&file_path)?.write_to_config(hoppers_config_path, language_code, mythic)?;
        }

        // JPEGs
        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}{FILE_SEPARATOR}{}motd_popup_image.jpg", if mythic { "blue_" } else { "" });
            let file_path = format!("{hoppers_blf_path}{FILE_SEPARATOR}{relative_file_path}");
            let output_path = build_path!(
                hoppers_blf_path,
                language_code,
                format!("{language_code}.jpg")
            );

            let file_path = build_path!(
                hoppers_blf_path,
                language_code,
                if mythic { k_mythic_motd_popup_image_file_name } else { k_motd_popup_image_file_name }
            );

            let output_path = build_path!(
                hoppers_config_path,
                if mythic { k_mythic_motd_popup_config_folder } else { k_motd_popup_config_folder },
                format!("{language_code}.jpg")
            );

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD Popup image is present.",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" }
                ));

                continue;
            }

            fs::copy(file_path, output_path).unwrap();
        }

        やった!(task)
    }

    fn build_config_map_variants(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting Map Variants");

        // Iterate through hopper folders. eg default_hoppers/00101
        let hopper_directory_subfolders = get_directories_in_folder(hoppers_blf_path)?;

        // Keep track of maps we've converted to avoid duplication between different hoppers.
        let mut converted_maps = Vec::<String>::new();

        for subfolder in hopper_directory_subfolders {
            if !hopper_folder_regex.is_match(&subfolder) {
                continue;
            }

            let map_variant_blfs_folder = build_path!(
                hoppers_blf_path,
                &subfolder,
                k_map_variants_blf_folder_name
            );

            // if this hoppers folder has no maps (perhaps incomplete), skip it.
            if !exists(&map_variant_blfs_folder)? {
                continue;
            }

            let map_variant_files = get_files_in_folder(&map_variant_blfs_folder)?;

            for map_variant_blf_file_name in map_variant_files {
                if !map_variant_file_regex.is_match(&map_variant_blf_file_name) {
                    continue;
                }

                let map_variant_file_name = map_variant_blf_file_name.replace(
                    &format!("_{:0>3}.bin", s_blf_chunk_packed_map_variant::get_version().major),
                    ""
                );

                let map_variant_config_file_name = format!("{map_variant_file_name}.json");

                let map_variant_blf_file_path = build_path!(
                    &map_variant_blfs_folder,
                    &map_variant_blf_file_name
                );

                let map_variant_json_file_path = build_path!(
                    hoppers_config_path,
                    k_map_variants_config_folder_name,
                    map_variant_config_file_name
                );

                // If we've already converted this map from a different hopper folder, we skip it.
                if converted_maps.contains(&map_variant_blf_file_name) {
                    continue;
                }
                else {
                    converted_maps.push(map_variant_blf_file_name.clone());
                }

                // If this map already exists in the config folder from an older convert, we delete it to rewrite.
                if exists(&map_variant_json_file_path)? {
                    remove_file(&map_variant_json_file_path)?
                }

                map_variant::read_file(&map_variant_blf_file_path)?
                    .write_to_config(hoppers_config_path, &map_variant_file_name)?;
            }
        }

        task.add_message(format!("Converted {} map variants.", converted_maps.len()));

        やった!(task)
    }

    fn build_config_game_variants(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting Game Variants");

        // Iterate through hopper folders. eg default_hoppers/00101
        let hopper_directory_subfolders = get_directories_in_folder(hoppers_blf_path)?;

        // Keep track of games we've converted to avoid duplication between different hoppers.
        let mut converted_games = Vec::<String>::new();

        for subfolder in hopper_directory_subfolders {
            if !hopper_folder_regex.is_match(&subfolder) {
                continue;
            }

            let game_variant_blfs_folder = build_path!(
                hoppers_blf_path,
                &subfolder
            );

            if !exists(&game_variant_blfs_folder)? {
                continue;
            }

            let game_variant_files = get_files_in_folder(&game_variant_blfs_folder)?;

            for game_variant_blf_file_name in game_variant_files {
                if !game_variant_file_regex.is_match(&game_variant_blf_file_name) {
                    continue;
                }

                let game_variant_file_name = game_variant_blf_file_name.replace(
                    &format!("_{:0>3}.bin", s_blf_chunk_packed_game_variant::get_version().major),
                    ""
                );

                let game_variant_config_file_name = format!("{game_variant_file_name}.json");

                let game_variant_blf_file_path = build_path!(
                    &game_variant_blfs_folder,
                    &game_variant_blf_file_name
                );

                let game_variant_json_file_path = build_path!(
                    hoppers_config_path,
                    k_game_variants_config_folder_name,
                    &game_variant_config_file_name
                );

                // If we've already converted this game from a different hopper folder, we skip it.
                if converted_games.contains(&game_variant_blf_file_name) {
                    continue;
                }
                else {
                    converted_games.push(game_variant_blf_file_name.clone());
                }

                // If this game already exists in the config folder from an older convert, we delete it to rewrite.
                if exists(&game_variant_json_file_path)? {
                    remove_file(&game_variant_json_file_path)?;
                }

                game_variant::read_file(&game_variant_blf_file_path)?
                    .write_to_config(hoppers_config_path, &game_variant_file_name)?;
            }
        }

        task.add_message(format!("Converted {} game variants.", converted_games.len()));

        やった!(task)
    }

    fn build_config_game_sets(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting Game Sets");

        // Iterate through hopper folders. eg default_hoppers/00101
        let hopper_directory_subfolders = get_directories_in_folder(hoppers_blf_path)?;

        let mut game_sets_count = 0;

        for hopper_folder in hopper_directory_subfolders {
            if !hopper_folder_regex.is_match(&hopper_folder) {
                continue;
            }

            let game_set_blf_path = build_path!(
                hoppers_blf_path,
                &hopper_folder,
                k_game_set_blf_file_name
            );

            if !exists(&game_set_blf_path).unwrap() {
                task.add_warning(format!("No game set was found for hopper \"{hopper_folder}\""));
                continue;
            }

            let game_set = game_set::read_file(&game_set_blf_path)?;

            let output_folder_path = build_path!(
                hoppers_config_path,
                k_hoppers_config_folder_name,
                &hopper_folder
            );

            game_set.write_to_config(&output_folder_path)?;

            game_sets_count += 1;
        }

        task.add_message(format!("Converted {game_sets_count} game sets."));

        やった!(task)
    }

    // Ideally, we'd separate hopper and category descriptions separately to avoid ID conflicts...
    // But Foreunner doesn't seem to make this distinction, so why should I?
    fn read_hopper_description_blfs(
        hoppers_blfs_folder: &String,
        task: &mut console_task
    ) -> HashMap<String, HashMap<u16, String>>
    {
        let mut language_descriptions_map = HashMap::<String, HashMap<u16, String>>::new();

        for language_code in k_language_suffixes {
            let hopper_descriptions_path = build_path!(
                hoppers_blfs_folder,
                language_code,
                k_matchmaking_hopper_descriptions_file_name
            );


            if !check_file_exists(&hopper_descriptions_path) {
                task.add_warning(format!(
                    "No {} hopper descriptions are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            let hopper_description_table =
                find_chunk_in_file::<s_blf_chunk_hopper_description_table>(&hopper_descriptions_path);

            if hopper_description_table.is_err() {
                task.fail_with_error(format!("Failed to read hopper descriptions file at: {hopper_descriptions_path}"));
                panic!()
            }

            let mut hoppers_description_map = HashMap::<u16, String>::new();

            hopper_description_table.unwrap().get_descriptions().iter().for_each(|hopper_description| {
                hoppers_description_map.insert(hopper_description.identifier, hopper_description.description.get_string());
            });

            language_descriptions_map.insert(String::from(language_code), hoppers_description_map);
        }

        language_descriptions_map
    }

    fn build_config_hoppers(hoppers_blfs_path: &String, hoppers_config_path: &String) {
        let mut task = console_task::start("Converting Hopper Configuration...");

        let language_hopper_descriptions
            = Self::read_hopper_description_blfs(hoppers_blfs_path, &mut task);

        let hopper_configuration_blf_path = build_path!(
            hoppers_blfs_path,
            k_matchmaking_hopper_file_name
        );

        let hopper_configuration_table = find_chunk_in_file::<s_blf_chunk_hopper_configuration_table>(&hopper_configuration_blf_path).unwrap();
        let hopper_configurations = hopper_configuration_table.get_hopper_configurations();
        let category_configurations = hopper_configuration_table.get_hopper_categories();

        // Generate active_hoppers.txt
        let active_hopper_ids = hopper_configurations.iter().map(|config|config.hopper_identifier);
        let active_hoppers_txt_path = build_path!(
            hoppers_config_path,
            k_active_hoppers_config_file_name
        );
        let mut active_hoppers_txt_file = File::create(active_hoppers_txt_path).unwrap();
        active_hoppers_txt_file.write_all(
            active_hopper_ids.map(|id|format!("{id:0>5}")).collect::<Vec<_>>().join("\r\n").as_bytes()
        ).unwrap();

        // Build hopper configuration json
        for hopper_configuration in hopper_configurations {
            let mut hopper_configuration_json = matchmaking_hopper_config {
                descriptions: HashMap::new(),
                configuration: hopper_configuration,
            };

            for language_code in k_language_suffixes {
                if language_hopper_descriptions.contains_key(language_code)
                    && language_hopper_descriptions.get(language_code).unwrap().contains_key(&hopper_configuration_json.configuration.hopper_identifier)
                {
                    hopper_configuration_json.descriptions.insert(
                        String::from(language_code),
                        language_hopper_descriptions.get(language_code).unwrap().get(&hopper_configuration_json.configuration.hopper_identifier).unwrap().clone()
                    );
                }
                else {
                    hopper_configuration_json.descriptions.insert(String::from(language_code), String::new());
                }
            }

            let hopper_configuration_json_folder = build_path!(
                hoppers_config_path,
                k_hoppers_config_folder_name,
                format!("{:0>5}", hopper_configuration_json.configuration.hopper_identifier)
            );
            create_dir_all(&hopper_configuration_json_folder).unwrap();

            let hopper_configuration_json_file = build_path!(
                &hopper_configuration_json_folder,
                k_hopper_config_file_name
            );
            let mut hopper_configuration_json_file = File::create(hopper_configuration_json_file).unwrap();
            serde_json::to_writer_pretty(&mut hopper_configuration_json_file, &hopper_configuration_json).unwrap();
        }

        // Build categories json
        let mut categories_config = matchmaking_hopper_categories_config::default();

        for category_configuration in category_configurations {
            let mut category_configuration_and_description = matchmaking_hopper_category_configuration_and_descriptions {
                descriptions: HashMap::new(),
                configuration: category_configuration,
            };

            for language_code in k_language_suffixes {
                if language_hopper_descriptions.contains_key(language_code)
                    && language_hopper_descriptions.get(language_code).unwrap().contains_key(&category_configuration_and_description.configuration.category_identifier)
                {
                    category_configuration_and_description.descriptions.insert(
                        String::from(language_code),
                        language_hopper_descriptions.get(language_code).unwrap().get(&category_configuration_and_description.configuration.category_identifier).unwrap().clone()
                    );
                }
                else {
                    category_configuration_and_description.descriptions.insert(String::from(language_code), String::new());
                }
            }

            categories_config.categories.push(category_configuration_and_description);
        }


        let categories_json_file = build_path!(
            hoppers_config_path,
            k_categories_config_file_name
        );

        let mut categories_json_file = File::create(categories_json_file).unwrap();
        serde_json::to_writer_pretty(&mut categories_json_file, &categories_config).unwrap();

        task.add_message(format!("Converted {} hopper configurations.", hopper_configuration_table.hopper_configuration_count()));

        task.complete();
    }

    fn build_config_network_configuration(hoppers_blfs_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        // For now we just copy it as is. But we do check that it contains a netc.
        let mut task = console_task::start("Converting Network Configuration");

        let network_configuration_source_path = build_path!(
            hoppers_blfs_path,
            k_network_configuration_file_name
        );

        let network_config = network_configuration::read_file(&network_configuration_source_path)?;
        network_config.write_to_config(hoppers_config_path)?;

        やった!(task)
    }

    fn build_blf_banhammer_messages(hoppers_config_folder: &String, hoppers_blf_folder: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Banhammer Messages");

        for language_code in k_language_suffixes {
            let mut matchmaking_banhammer_messages = matchmaking_banhammer_messages::read_from_config(
                hoppers_config_folder,
                language_code,
            ).inspect_err(|err|{
                task.fail();
            })?;

            matchmaking_banhammer_messages.write_file(build_path!(
                hoppers_blf_folder,
                language_code,
                k_matchmaking_banhammer_messages_file_name
            ));
        }

        やった!(task)
    }

    fn build_blf_matchmaking_tips(hoppers_config_folder: &String, hoppers_blf_folder: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Matchmaking Tips");

        for language_code in k_language_suffixes {
            let mut matchmaking_tips = matchmaking_tips::read_from_config(
                hoppers_config_folder,
                language_code,
            ).inspect_err(|err|{
                task.fail();
            })?;

            matchmaking_tips.write_file(build_path!(
                hoppers_blf_folder,
                language_code,
                k_matchmaking_tips_file_name
            ));
        }

        やった!(task)
    }

    fn build_blf_motds(
        hoppers_config_path: &String,
        hoppers_blf_path: &String,
        mythic: bool
    ) -> Result<(), Box<dyn Error>>
    {
        let mut task = console_task::start(
            if mythic { "Building Mythic MOTDs" } else { "Building MOTDs" }
        );

        for language_code in k_language_suffixes {
            let motd = motd::read_from_config(
                hoppers_config_path,
                language_code,
                mythic
            );

            if motd.is_err() {
                task.add_warning(format!(
                    "Failed to build {} {}MOTD: {}",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" },
                    motd.unwrap_err()
                ));

                continue;
            }

            motd?.write_file(build_path!(
                hoppers_blf_path,
                language_code,
                if mythic { k_mythic_motd_file_name } else { k_mythic_motd_file_name }
            ));
        }

        for language_code in k_language_suffixes {
            let jpeg_file_path = build_path!(
                hoppers_config_path,
                if mythic { k_mythic_motd_config_folder } else { k_motd_config_folder },
                format!("{}.jpg", language_code)
            );

            let destination_path = build_path!(
                hoppers_blf_path,
                language_code,
                if mythic { k_mythic_motd_image_file_name } else { k_motd_image_file_name }
            );

            let validated = motd::validate_image(&jpeg_file_path);

            if validated.is_err() {
                task.add_warning(format!(
                    "Failed to convert {} {} MOTD Image: {}",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" },
                    validated.unwrap_err()
                ));

                continue;
            }

            fs::copy(jpeg_file_path, destination_path).unwrap();
        }

        やった!(task)
    }

    fn build_blf_motd_popups(hoppers_config_folder: &String, hoppers_blf_folder: &String, mythic: bool) -> Result<(), Box<dyn Error>>{
        let mut task = console_task::start(format!(
            "Building {}MOTD Popups",
            if mythic { "Mythic " } else { "" }
        ));

        for language_code in k_language_suffixes {
            let motd_popup = motd_popup::read_from_config(hoppers_config_folder, language_code, mythic);

            if motd_popup.is_err() {
                task.add_warning(format!(
                    "Failed to build {} {}MOTD Popup: {}",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" },
                    motd_popup.unwrap_err()
                ));

                continue;
            }

            motd_popup?.write_file(build_path!(
                hoppers_blf_folder,
                language_code,
                if mythic { k_mythic_motd_popup_file_name } else { k_motd_popup_file_name }
            ));
        }

        for language_code in k_language_suffixes {
            let jpeg_file_path = build_path!(
                hoppers_config_folder,
                if mythic { k_mythic_motd_popup_config_folder } else { k_motd_popup_config_folder },
                format!("{}.jpg", language_code)
            );

            let destination_path = build_path!(
                hoppers_blf_folder,
                language_code,
                if mythic { k_mythic_motd_popup_image_file_name } else { k_motd_popup_image_file_name }
            );

            let validated = motd_popup::validate_image(&jpeg_file_path);

            if validated.is_err() {
                task.add_warning(format!(
                    "Failed to convert {} {} MOTD Popup Image: {}",
                    get_language_string(language_code),
                    if mythic { "Mythic " } else { "" },
                    validated.unwrap_err()
                ));

                continue;
            }

            fs::copy(jpeg_file_path, destination_path).unwrap();
        }

        やった!(task)
    }

    fn build_blf_map_manifest(hoppers_config_path: &String, hoppers_blf_path: &String) -> Result<(), Box<dyn Error>>
    {
        let mut task = console_task::start("Building Map Manifest");

        let mut rsa_manifest = rsa_manifest::build_for_hoppers(hoppers_config_path)
            .inspect_err(|_| { task.fail() })?;

        rsa_manifest.write_file(build_path!(
            hoppers_blf_path,
            k_rsa_manifest_file_name
        ));

        やった!(task)
    }

    fn read_active_hopper_configuration(hoppers_config_path: &String) -> Vec<String> {
        let mut task = console_task::start("Reading Active Hoppers");

        let active_hoppers_folders = read_active_hoppers(hoppers_config_path).unwrap_or_else(|err| {
            task.fail_with_error(err);
            panic!();
        });

        task.complete();

        active_hoppers_folders
    }

    fn read_game_set_configuration(hoppers_config_path: &String, active_hopper_folders: &Vec<String>) -> Result<HashMap<u16, game_set_config>, Box<dyn Error>>
    {
        let mut task = console_task::start("Reading Game Set Config");

        let mut game_sets = HashMap::<u16, game_set_config>::new();

        let hopper_tables_config_path = build_path!(
            hoppers_config_path,
            k_hoppers_config_folder_name
        );

        for subfolder in active_hopper_folders {
            let hopper_id= config_hopper_folder_identifier_regex.captures(subfolder);
            if !&hopper_id.is_some() {
                continue;
            }
            let hopper_id = hopper_id.unwrap();
            if hopper_id.get(0).is_none() {
                continue;
            }
            let hopper_id = hopper_id.get(0).unwrap().as_str();
            let hopper_id = u16::from_str(hopper_id)?;

            let game_set_csv_path = build_path!(
                &hopper_tables_config_path,
                subfolder,
                k_game_set_config_file_name
            );

            if !exists(&game_set_csv_path).unwrap() {
                task.fail_with_error(format!("No game set was found for hopper \"{subfolder}\""));
                panic!();
            }

            let game_set = game_set_config::read(game_set_csv_path)?;

            game_sets.insert(hopper_id, game_set);
        }

        task.complete();

        Ok(game_sets)
    }

    fn build_blf_game_variants(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
        build_temp_dir: &String,
        game_sets: &HashMap<u16, game_set_config>,
        variant_hashes: &mut HashMap<String, s_network_http_request_hash>
    )
    {
        let mut task = console_task::start("Building Game Variants");

        let game_variants_config_path = build_path!(
            hoppers_config_path,
            k_game_variants_config_folder_name
        );

        let game_variants_temp_build_path = build_path!(
            build_temp_dir,
            k_build_temp_games_folder_name
        );

        create_dir_all(&game_variants_temp_build_path).unwrap();

        let game_variants_to_convert: Vec<String> = game_sets.iter().flat_map(|(_, game_set)|
            game_set.entries.iter().map(|entry|entry.game_variant_file_name.clone()).collect::<Vec<String>>()
        ).collect();

        let game_variants_to_convert: HashSet<String> = HashSet::from_iter(game_variants_to_convert.iter().cloned());

        let mut json_queue: Vec<(String, String)> = Vec::new();
        for game_variant in game_variants_to_convert {
            let map_variant_json_path = build_path!(
                &game_variants_config_path,
                format!("{game_variant}.json")
            );

            if !Path::new(&map_variant_json_path).exists() {
                task.fail_with_error(format!("Game variant \"{}\" could not be found.", game_variant));
                panic!();
            }

            let mut file = File::open(&map_variant_json_path).unwrap();
            let mut game_variant_json = String::new();
            file.read_to_string(&mut game_variant_json).unwrap();

            json_queue.push((game_variant, game_variant_json));
        }


        let rt = runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .build()
            .unwrap();

        let task = Arc::new(Mutex::new(task));
        let json_queue = Arc::new(Mutex::new(VecDeque::from(json_queue)));
        let shared_variant_hashes = Arc::new(Mutex::new(HashMap::new()));

        let cpu_cores = num_cpus::get();
        rt.block_on(async {
            let mut thread_handles = Vec::<JoinHandle<()>>::with_capacity(cpu_cores);

            for n in 0..cpu_cores {
                let shared_variant_hashes = Arc::clone(&shared_variant_hashes);
                let game_variants_config_path = game_variants_config_path.clone();
                let game_variants_temp_build_path = game_variants_temp_build_path.clone();
                let task = Arc::clone(&task);
                let json_queue = Arc::clone(&json_queue);

                thread_handles.push(rt.spawn(async move {
                    loop {
                        let mut json_queue = json_queue.lock().await;

                        if let Some((game_variant_file_name, json)) = json_queue.pop_front() {
                            let remaining = json_queue.len();
                            drop(json_queue);

                            // debug_log!("[GAMES] Thread {n} got {game_variant_file_name} ({remaining} remaining)");

                            let game_variant_blf_path = build_path!(
                                &game_variants_temp_build_path,
                                format!("{game_variant_file_name}_{:0>3}.bin",
                                    s_blf_chunk_packed_game_variant::get_version().major
                                )
                            );

                            let game_variant_json: c_game_variant = serde_json::from_str(&json).unwrap();

                            let mut map_variant_blf_file = game_variant::create(game_variant_json);
                            map_variant_blf_file.write_file(&game_variant_blf_path);

                            let hash = get_blf_file_hash(game_variant_blf_path).unwrap();
                            let mut hashes = shared_variant_hashes.lock().await;
                            hashes.insert(game_variant_file_name.clone(), hash);
                        } else {
                            break;
                        }
                    }
                }));
            }

            for thread_handle in thread_handles {
                thread_handle.await.unwrap();
            }

            let final_hashes = shared_variant_hashes.lock().await;
            variant_hashes.extend(final_hashes.clone());

            let mut task = task.lock().await;
            task.add_message(format!("Built {} variants.", variant_hashes.len()));
            task.complete();
        });
    }

    pub fn get_scenario_rsa_crc32s(hoppers_config_path: &String) -> HashMap<u32, u32> {
        let mut result = HashMap::<u32, u32>::new();

        let rsa_folder = build_path!(
            hoppers_config_path,
            k_rsa_signatures_config_folder_name
        );

        if !exists(&rsa_folder).unwrap() {
            return result;
        }

        let rsa_files = get_files_in_folder(&rsa_folder).unwrap_or_else(|err|{
            panic!();
        });

        for rsa_file_name in rsa_files {
            let rsa_file_path = build_path!(
                &rsa_folder,
                &rsa_file_name
            );
            let rsa_file = File::open(&rsa_file_path);
            if rsa_file.is_err() {
                continue;
            }
            let mut rsa_file = rsa_file.unwrap();
            let mut rsa_signature = Vec::<u8>::with_capacity(0x100);
            rsa_file.read_to_end(&mut rsa_signature).unwrap();

            let map_id = config_rsa_signature_file_map_id_regex.captures(rsa_file_name.as_str()).unwrap();
            let map_id = map_id.get(0).unwrap();
            let map_id = u32::from_str(map_id.as_str()).unwrap();
            let crc32 = crc32(0xFFFFFFFF, &rsa_signature);

            result.insert(map_id, crc32);
        }

        result
    }

    fn build_blf_map_variants(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
        build_temp_dir: &String,
        game_sets: &HashMap<u16, game_set_config>,
        variant_hashes: &mut HashMap<String, s_network_http_request_hash>,
        variant_map_ids: &mut HashMap<String, u32>
    )
    {
        let mut task = console_task::start("Building Map Variants");

        let scenario_crc32s = Arc::new(Self::get_scenario_rsa_crc32s(hoppers_config_path));

        let map_variants_config_path = build_path!(
            hoppers_config_path,
            k_map_variants_config_folder_name
        );

        let map_variants_temp_build_path = build_path!(
            build_temp_dir,
            k_build_temp_maps_folder_name
        );

        create_dir_all(&map_variants_temp_build_path).unwrap();

        let map_variants_to_convert: Vec<String> = game_sets.iter().flat_map(|(_, game_set)|
            game_set.entries.iter().map(|entry| entry.map_variant_file_name.clone()).collect::<Vec<String>>()
        ).collect();
        let map_variants_to_convert: HashSet<String> = HashSet::from_iter(map_variants_to_convert.iter().cloned());

        let mut json_queue: Vec<(String, String)> = Vec::new();
        for map_variant in map_variants_to_convert {
            let map_variant_json_path = build_path!(
                &map_variants_config_path,
                format!("{map_variant}.json")
            );

            if !Path::new(&map_variant_json_path).exists() {
                task.fail_with_error(format!("Map variant \"{}\" could not be found.", map_variant));
                panic!();
            }

            let mut file = File::open(&map_variant_json_path).unwrap();
            let mut map_variant_json = String::new();
            file.read_to_string(&mut map_variant_json).unwrap();

            json_queue.push((map_variant, map_variant_json));
        }

        let rt = runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .build()
            .unwrap();

        let task = Arc::new(Mutex::new(task));
        let json_queue = Arc::new(Mutex::new(VecDeque::from(json_queue)));
        let shared_variant_hashes = Arc::new(Mutex::new(HashMap::new()));
        let shared_variant_map_ids = Arc::new(Mutex::new(HashMap::new()));

        let cpu_cores = num_cpus::get();
        rt.block_on(async {
            let mut thread_handles = Vec::<JoinHandle<()>>::with_capacity(cpu_cores);

            for n in 0..cpu_cores {
                let shared_variant_hashes = Arc::clone(&shared_variant_hashes);
                let shared_variant_map_ids = Arc::clone(&shared_variant_map_ids);
                let map_variants_config_path = map_variants_config_path.clone();
                let map_variants_temp_build_path = map_variants_temp_build_path.clone();
                let scenario_crc32s = Arc::clone(&scenario_crc32s);
                let task = Arc::clone(&task);
                let json_queue = Arc::clone(&json_queue);

                thread_handles.push(rt.spawn(async move {
                    loop {
                        let mut json_queue = json_queue.lock().await;

                        if let Some((map_variant_file_name, json)) = json_queue.pop_front() {
                            let remaining = json_queue.len();
                            drop(json_queue);

                            // debug_log!("[MAPS] Thread {n} got {map_variant_file_name} ({remaining} remaining)");

                            let map_variant_blf_path = build_path!(
                                &map_variants_temp_build_path,
                                format!("{map_variant_file_name}_{:0>3}.bin",
                                    s_blf_chunk_packed_map_variant::get_version().major
                                )
                            );

                            let mut map_variant_json: c_map_variant = serde_json::from_str(&json).unwrap();

                            // Check the scenario crc
                            let expected_scenario_crc = scenario_crc32s.get(&map_variant_json.m_map_id);
                            if expected_scenario_crc.is_none() {
                                let mut task = task.lock().await;
                                task.add_error(format!("Map Variant {map_variant_file_name} could not be validated due to missing RSA signature!"))
                            }
                            else {
                                let expected_scenario_crc = expected_scenario_crc.unwrap();
                                if expected_scenario_crc != &map_variant_json.m_original_map_rsa_signature_hash {
                                    let mut task = task.lock().await;
                                    task.add_error(format!("Map Variant \"{map_variant_file_name}\" has a bad checksum and may not load properly! (got {:08X}, expected {:08X})", &map_variant_json.m_original_map_rsa_signature_hash, expected_scenario_crc));
                                    map_variant_json.m_original_map_rsa_signature_hash = *expected_scenario_crc;
                                }
                            }

                            let mut map_ids = shared_variant_map_ids.lock().await;
                            map_ids.insert(map_variant_file_name.clone(), map_variant_json.m_map_id);

                            let mut map_variant_blf_file = map_variant::create(map_variant_json);
                            map_variant_blf_file.write_file(&map_variant_blf_path);

                            let hash = get_blf_file_hash(map_variant_blf_path).unwrap();
                            let mut hashes = shared_variant_hashes.lock().await;
                            hashes.insert(map_variant_file_name.clone(), hash);
                        } else {
                            break;
                        }
                    }
                }));
            }

            for thread_handle in thread_handles {
                thread_handle.await.unwrap();
            }

            let final_hashes = shared_variant_hashes.lock().await;
            variant_hashes.extend(final_hashes.clone());

            let final_map_ids = shared_variant_map_ids.lock().await;
            variant_map_ids.extend(final_map_ids.clone());

            let mut task = task.lock().await;
            task.add_message(format!("Built {} variants.", variant_hashes.len()));
            task.complete();
        });
    }

    fn build_blf_game_sets(
        hoppers_blf_path: &String,
        active_game_sets: HashMap<u16, game_set_config>,
        game_variant_hashes: &HashMap<String, s_network_http_request_hash>,
        map_variant_hashes: &HashMap<String, s_network_http_request_hash>,
        map_variant_map_ids: &HashMap<String, u32>,
        build_temp_dir_path: &String,
    ) -> Result<(), Box<dyn Error>>
    {
        let mut task = console_task::start("Building Game Sets");

        for (hopper_id, game_set_config) in active_game_sets {
            let hopper_folder_path = build_path!(
                hoppers_blf_path,
                format!("{hopper_id:0>5}")
            );

            let hopper_folder_map_variants_path = build_path!(
                &hopper_folder_path,
                k_map_variants_blf_folder_name
            );

            let game_variants_temp_build_path = build_path!(
                build_temp_dir_path,
                k_build_temp_games_folder_name
            );

            let map_variants_temp_build_path = build_path!(
                build_temp_dir_path,
                k_build_temp_maps_folder_name
            );

            create_dir_all(&hopper_folder_map_variants_path).unwrap();

            let copied_maps = HashSet::<String>::new();
            let copied_games = HashSet::<String>::new();

            for game_set_row in &game_set_config.entries {
                // Copy the game and map variants over...
                if !copied_games.contains(&game_set_row.game_variant_file_name) {
                    let game_variant_file_name = format!(
                        "{}_{:0>3}.bin",
                        game_set_row.game_variant_file_name,
                        s_blf_chunk_packed_game_variant::get_version().major,
                    );
                    fs::copy(
                        build_path!(
                            &game_variants_temp_build_path,
                            &game_variant_file_name
                        ),
                        build_path!(
                            &hopper_folder_path,
                            &game_variant_file_name
                        )
                    )?;
                }

                if !copied_maps.contains(&game_set_row.map_variant_file_name) {
                    let map_variant_file_name = format!(
                        "{}_{:0>3}.bin",
                        game_set_row.map_variant_file_name,
                        s_blf_chunk_packed_map_variant::get_version().major,
                    );
                    fs::copy(
                        build_path!(
                            &map_variants_temp_build_path,
                            &map_variant_file_name
                        ),
                        build_path!(
                            &hopper_folder_map_variants_path,
                            &map_variant_file_name
                        )
                    )?;
                }
            }


            let mut game_set = game_set::create_from_config(
                &game_set_config,
                game_variant_hashes,
                map_variant_hashes,
                map_variant_map_ids
            )?;

            // Write the game set file
            game_set.write_file(build_path!(
                &hopper_folder_path,
                k_game_set_blf_file_name
            ))
        }

        やった!(task)
    }

    fn build_blf_hoppers(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
        active_hopper_folders: &Vec<String>,
    ) -> Result<(), Box<dyn Error>>
    {
        let mut task = console_task::start("Building Hopper Configuration");

        let mut hopper_configuration_table = s_blf_chunk_hopper_configuration_table::default();

        // Load the configuration.json files for each hopper
        let mut hopper_configuration_jsons = Vec::<(u16, matchmaking_hopper_config)>::new();
        for active_hopper_folder in active_hopper_folders {
            let configuration_path = build_path!(
                hoppers_config_path,
                k_hoppers_config_folder_name,
                active_hopper_folder,
                k_hopper_config_file_name
            );

            if !exists(&configuration_path).unwrap() {
                task.fail_with_error(format!("Couldn't find a configuration file for hopper {active_hopper_folder}!"));
                panic!();
            }

            let mut configuration_file = File::open(&configuration_path).unwrap();

            // TODO: Refactor out
            let hopper_id= config_hopper_folder_identifier_regex.captures(active_hopper_folder);
            if !&hopper_id.is_some() {
                continue;
            }
            let hopper_id = hopper_id.unwrap();
            if hopper_id.get(0).is_none() {
                continue;
            }
            let hopper_id = hopper_id.get(0).unwrap().as_str();
            let hopper_id = u16::from_str(hopper_id).unwrap();

            hopper_configuration_jsons.push((
                hopper_id,
                serde_json::from_reader(&mut configuration_file).unwrap()
            ));
        }

        for (hopper_identifier, hopper_configuration_json) in &hopper_configuration_jsons {
            let mut hopper_config = hopper_configuration_json.configuration.clone();
            let game_set_blf_file_path = build_path!(
                hoppers_blfs_path,
                format!("{hopper_identifier:0>5}"),
                k_game_set_blf_file_name
            );
            hopper_config.game_set_hash = get_blf_file_hash(game_set_blf_file_path).unwrap();
            hopper_configuration_table.add_hopper_configuration(hopper_config).unwrap()
        }

        // Load category configuration
        let categories_configuration = matchmaking_hopper_categories_config::read(hoppers_config_path)?;

        let active_hopper_categories = hopper_configuration_table
            .get_hopper_configurations()
            .iter().map(|hopper|hopper.hopper_category)
            .collect::<HashSet<_>>();
        let active_hopper_category_configurations = categories_configuration.categories
            .iter().filter(|category_configuration|active_hopper_categories.contains(&category_configuration.configuration.category_identifier))
            .cloned()
            .collect::<Vec<matchmaking_hopper_category_configuration_and_descriptions>>();

        for active_hopper_category in &active_hopper_category_configurations {
            hopper_configuration_table.add_category_configuration(active_hopper_category.configuration)?;
        }

        // Initialize language_hopper_descriptions
        for language_code in k_language_suffixes {
            let mut language_descriptions = s_blf_chunk_hopper_description_table::default();

            for (hopper_identifier, hopper_configuration_json) in &hopper_configuration_jsons {
                if !hopper_configuration_json.descriptions.contains_key(&language_code.to_string()) {
                    task.add_warning(format!(
                        "No {} description was found for hopper {hopper_identifier} ({})",
                        get_language_string(language_code),
                        hopper_configuration_json.configuration.hopper_name.get_string(),
                    ));
                    continue;
                }

                let description = hopper_configuration_json.descriptions.get(&language_code.to_string()).unwrap();
                if description.is_empty() {
                    task.add_warning(format!(
                        "No {} description was found for hopper {hopper_identifier} ({})",
                        get_language_string(language_code),
                        hopper_configuration_json.configuration.hopper_name.get_string(),
                    ));
                    continue;
                }

                language_descriptions.add_description((
                    *hopper_identifier,
                    &description.to_string()
                ))?;
            }

            for active_hopper_category in &active_hopper_category_configurations {
                if !active_hopper_category.descriptions.contains_key(&language_code.to_string()) {
                    task.add_warning(format!(
                        "No {} description was found for category {} ({})",
                        get_language_string(language_code),
                        active_hopper_category.configuration.category_identifier,
                        active_hopper_category.configuration.category_name.get_string(),
                    ));
                    continue;
                }

                let description = active_hopper_category.descriptions.get(&language_code.to_string()).unwrap();
                if description.is_empty() {
                    task.add_warning(format!(
                        "No {} description was found for category {} ({})",
                        get_language_string(language_code),
                        active_hopper_category.configuration.category_identifier,
                        active_hopper_category.configuration.category_name.get_string(),
                    ));
                    continue;
                }

                language_descriptions.add_description((
                    active_hopper_category.configuration.category_identifier,
                    description
                ))?;
            }

            // Write description file
            let descriptions_blf_path = build_path!(
                hoppers_blfs_path,
                language_code,
                k_matchmaking_hopper_descriptions_file_name
            );

            let mut matchmaking_hopper_descriptions = matchmaking_hopper_descriptions::create(language_descriptions);
            matchmaking_hopper_descriptions.write_file(&descriptions_blf_path);
        }

        // Write the hopper config file.
        let mut matchmaking_hopper_blf = matchmaking_hopper::create(hopper_configuration_table);
        matchmaking_hopper_blf.write_file(build_path!(
            hoppers_blfs_path,
            k_matchmaking_hopper_file_name
        ));

        やった!(task)
    }

    fn build_blf_network_configuration(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
    ) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Network Configuration");

        let mut network_configuration_blf_file = network_configuration::read_from_config(hoppers_config_path)?;
        network_configuration_blf_file.write_file(
            build_path!(
                hoppers_blfs_path,
                k_network_configuration_file_name
            )
        );

        やった!(task)
    }

    fn build_blf_manifest(
        hoppers_blfs_path: &String,
    ) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Manifest File");

        let mut manifest_blf_file = manifest::build_for_hoppers::<s_blf_chunk_network_configuration>(hoppers_blfs_path).inspect_err(|err|{
            task.fail();
        })?;

        manifest_blf_file.write_file(build_path!(
            hoppers_blfs_path,
            k_manifest_file_name
        ));

        やった!(task)
    }
}