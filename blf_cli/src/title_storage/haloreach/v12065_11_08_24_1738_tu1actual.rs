mod blf_files;

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
use blf_lib::blf::{get_blf_file_hash, BlfFile, BlfFileBuilder};
use blf_lib::blf::chunks::{find_chunk_in_file, BlfChunk};
use crate::console::console_task;
use regex::Regex;
use tempdir::TempDir;
use tokio::runtime;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use blf_files::{k_rsa_manifest_file_name, k_rsa_signatures_config_folder_name};
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blf::versions::halo3::v11855_07_08_20_2317_halo3_ship::s_blf_chunk_map_manifest;
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_game_set, s_blf_chunk_hopper_configuration_table, s_blf_chunk_hopper_description_table, s_blf_chunk_map_variant, s_blf_chunk_matchmaking_game_variant, s_blf_chunk_megalo_categories, s_blf_chunk_network_configuration, s_blf_chunk_online_file_manifest, s_blf_chunk_predefined_queries, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_dlc_map_manifest;
use blf_lib::io::{read_json_file, write_json_file};
use blf_lib::result::BLFLibResult;
use blf_lib::types::byte_order_mark::byte_order_mark;
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::blf_files::{k_game_set_blf_file_name, k_game_set_config_file_name};
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::blf_files::k_hopper_directory_name_max_length;
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::blf_files::k_game_variants_config_folder_name;
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::blf_files::{k_map_variants_blf_folder_name, k_map_variants_config_folder_name};
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::blf_files::matchmaking_banhammer_messages::{k_matchmaking_banhammer_messages_file_name, matchmaking_banhammer_messages};
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::blf_files::matchmaking_hopper::{k_active_hoppers_config_file_name, k_categories_config_file_name, k_hopper_config_file_name, k_hoppers_config_folder_name, k_matchmaking_hopper_file_name, k_matchmaking_hopper_image_filename, k_matchmaking_hopper_images_folder, matchmaking_hopper, matchmaking_hopper_categories_config, matchmaking_hopper_category_configuration_and_descriptions, matchmaking_hopper_config, read_active_hoppers};
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::blf_files::matchmaking_hopper_descriptions::{k_matchmaking_hopper_descriptions_file_name, matchmaking_hopper_descriptions};
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::blf_files::matchmaking_tips::{k_matchmaking_tips_file_name, k_tangerine_matchmaking_tips_file_name, matchmaking_tips};
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::blf_files::nag_message::{k_cea_nag_message_config_folder, k_cea_nag_message_file_name, k_cea_nag_message_image_file_name, k_nag_message_config_folder, k_nag_message_file_name, k_nag_message_image_file_name, nag_message};
use crate::title_storage::haloreach::v12065_11_08_24_1738_tu1actual::blf_files::k_network_configuration_file_name;
title_converter! (
    #[Title("Halo: Reach")]
    #[Build("12065.11.08.24.1738.tu1actual")]
    pub struct v12065_11_08_24_1738_tu1actual {}
);

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
    static ref map_variant_file_regex: Regex = Regex::new(&format!("_{:0>3}.bin$", s_blf_chunk_map_variant::get_version().major)).unwrap();
    static ref game_variant_file_regex: Regex = Regex::new(&format!("_{:0>3}.bin$", s_blf_chunk_matchmaking_game_variant::get_version().major)).unwrap();
    static ref config_rsa_signature_file_map_id_regex: Regex = Regex::new(r"^[0-9]{1,}").unwrap();
}

const k_build_temp_maps_folder_name: &str = "map_variants";
const k_build_temp_games_folder_name: &str = "game_variants";

impl TitleConverter for v12065_11_08_24_1738_tu1actual {
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
                Self::build_blf_matchmaking_tips(&hopper_config_path, &hopper_blfs_path, false)?;
                Self::build_blf_matchmaking_tips(&hopper_config_path, &hopper_blfs_path, true)?;
                Self::build_blf_nag_messages(&hopper_config_path, &hopper_blfs_path, false)?;
                Self::build_blf_nag_messages(&hopper_config_path, &hopper_blfs_path, true)?;
                Self::build_blf_map_manifest(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_megalo_categories(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_predefined_queries(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_dlc_manifest(&hopper_config_path, &hopper_blfs_path)?;

                let active_hoppers = Self::read_active_hopper_configuration(&hopper_config_path);
                let game_sets = Self::read_game_set_configuration(&hopper_config_path, &active_hoppers)?;
                let mut game_variant_hashes = HashMap::<String, s_network_http_request_hash>::new();
                let mut map_variant_hashes = HashMap::<String, s_network_http_request_hash>::new();
                let mut map_variant_map_ids = HashMap::<String, u32>::new();

                Self::build_blf_game_variants(&hopper_config_path, &hopper_blfs_path, &build_temp_dir_path, &game_sets, &mut game_variant_hashes);
                Self::build_blf_map_variants(&hopper_config_path, &hopper_blfs_path, &build_temp_dir_path, &game_sets, &mut map_variant_hashes);
                Self::build_blf_game_sets(&hopper_blfs_path, game_sets, &game_variant_hashes, &map_variant_hashes, &build_temp_dir_path)?;
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
                Self::build_config_megalo_categories(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_predefined_queries(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_matchmaking_tips(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_dlc_manifest(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_motd_popups(&hoppers_blf_path, &hoppers_config_path, false)?;
                Self::build_config_motd_popups(&hoppers_blf_path, &hoppers_config_path, true)?;
                Self::build_config_map_variants(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_game_variants(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_game_sets(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_hoppers(&hoppers_blf_path, &hoppers_config_path)?;
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

impl v12065_11_08_24_1738_tu1actual {
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

    fn build_config_megalo_categories(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting Megalo Categories");

        for language_code in k_language_suffixes {
            let blf_file_path = build_path!(hoppers_blf_path, language_code, "file_megalo_categories.bin");

            if !exists(&blf_file_path)? {
                task.add_warning(format!(
                    "No {} megalo categories are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            let megalo_categories = find_chunk_in_file::<s_blf_chunk_megalo_categories>(blf_file_path)?;
            write_json_file(&megalo_categories, build_path!(
                hoppers_config_path,
                "megalo_categories",
                format!("{}.json", language_code)
            ))?;
        }

        やった!(task)
    }

    fn build_config_predefined_queries(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting Predefined Queries");

        for language_code in k_language_suffixes {
            let blf_file_path = build_path!(hoppers_blf_path, language_code, "file_predefined_queries.bin");

            if !exists(&blf_file_path)? {
                task.add_warning(format!(
                    "No {} predefined queries are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            let megalo_categories = find_chunk_in_file::<s_blf_chunk_predefined_queries>(blf_file_path)?;
            write_json_file(&megalo_categories, build_path!(
                hoppers_config_path,
                "predefined_queries",
                format!("{}.json", language_code)
            ))?;
        }

        task.complete();

        let mut task = console_task::start("Converting CEA Predefined Queries");

        for language_code in k_language_suffixes {
            let blf_file_path = build_path!(hoppers_blf_path, language_code, "tangerine_file_predefined_queries.bin");

            if !exists(&blf_file_path)? {
                task.add_warning(format!(
                    "No {} CEA predefined queries are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            let megalo_categories = find_chunk_in_file::<s_blf_chunk_predefined_queries>(blf_file_path)?;
            write_json_file(&megalo_categories, build_path!(
                hoppers_config_path,
                "predefined_queries_cea",
                format!("{}.json", language_code)
            ))?;
        }

        やった!(task)
    }

    fn build_config_matchmaking_tips(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting Matchmaking Tips");

        for language_code in k_language_suffixes {
            let blf_file_path = build_path!(hoppers_blf_path, language_code, k_matchmaking_tips_file_name);

            if !exists(&blf_file_path)? {
                task.add_warning(format!(
                    "No {} matchmaking tips are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            matchmaking_tips::read_file(&blf_file_path)?.write_to_config(hoppers_config_path, language_code, false)?;
        }

        task.complete();

        let mut task = console_task::start("Converting CEA Matchmaking Tips");

        for language_code in k_language_suffixes {
            let blf_file_path = build_path!(hoppers_blf_path, language_code, k_tangerine_matchmaking_tips_file_name);

            if !exists(&blf_file_path)? {
                task.add_warning(format!(
                    "No {} CEA matchmaking tips are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            matchmaking_tips::read_file(&blf_file_path)?.write_to_config(hoppers_config_path, language_code, true)?;
        }

        やった!(task)
    }

    fn build_config_dlc_manifest(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting DLC Manifest");

        let blf_file_path = build_path!(hoppers_blf_path, "dlc_map_manifest.bin");
        let dlc_manifest = find_chunk_in_file::<s_blf_chunk_dlc_map_manifest>(blf_file_path)?;
        let dlc_config_folder = build_path!(hoppers_config_path, "dlc");
        let config_file_path = build_path!(&dlc_config_folder, "manifest.json");
        write_json_file(&dlc_manifest, config_file_path)?;
        let dlc_images_folder = build_path!(hoppers_blf_path, "dlc");

        for map in dlc_manifest.maps.get() {
            if !map.small_image_file_name.is_empty() {
                let small_image_path = build_path!(&dlc_images_folder, map.small_image_file_name.get_string()?);
                let small_image_destination = build_path!(&dlc_config_folder, map.small_image_file_name.get_string()?);
                if exists(&small_image_path)? {
                    fs::copy(&small_image_path, small_image_destination)?;
                } else {
                    task.add_warning(format!("No small image was found for map {}", map.name_en.get_string()))
                }
            }

            if !map.large_image_file_name.is_empty() {
                let large_image_path = build_path!(&dlc_images_folder, map.large_image_file_name.get_string()?);
                let large_image_destination = build_path!(&dlc_config_folder, map.large_image_file_name.get_string()?);
                if exists(&large_image_path)? {
                    fs::copy(&large_image_path, large_image_destination)?;
                } else {
                    task.add_warning(format!("No large image was found for map {}", map.name_en.get_string()))
                }
            }
        }

        やった!(task)
    }

    fn build_config_motd_popups(hoppers_blf_path: &String, hoppers_config_path: &String, cea: bool) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start(
            if cea { "Converting CEA MOTD Popups" }
            else { "Converting MOTD Popups" }
        );

        // BLFs
        for language_code in k_language_suffixes {
            let file_path = build_path!(
                hoppers_blf_path,
                language_code,
                if cea { k_cea_nag_message_file_name } else { k_nag_message_file_name }
            );

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD Popup is present.",
                    get_language_string(language_code),
                    if cea { "CEA " } else { "" }
                ));

                continue;
            }

            nag_message::read_file(&file_path)?.write_to_config(hoppers_config_path, language_code, cea)?;
        }

        // JPEGs
        for language_code in k_language_suffixes {
            let relative_file_path = format!("{language_code}{FILE_SEPARATOR}{}motd_popup_image.jpg", if cea { "blue_" } else { "" });
            let file_path = format!("{hoppers_blf_path}{FILE_SEPARATOR}{relative_file_path}");
            let output_path = build_path!(
                hoppers_blf_path,
                language_code,
                format!("{language_code}.jpg")
            );

            let file_path = build_path!(
                hoppers_blf_path,
                language_code,
                if cea { k_cea_nag_message_image_file_name } else { k_nag_message_image_file_name }
            );

            let output_path = build_path!(
                hoppers_config_path,
                if cea { k_cea_nag_message_config_folder } else { k_nag_message_config_folder },
                format!("{language_code}.jpg")
            );

            if !check_file_exists(&file_path) {
                task.add_warning(format!(
                    "No {} {}MOTD Popup image is present.",
                    get_language_string(language_code),
                    if cea { "CEA " } else { "" }
                ));

                continue;
            }

            fs::copy(file_path, output_path).unwrap();
        }

        やった!(task)
    }

    fn build_config_map_variants(hoppers_blf_path: &String, hoppers_config_path: &String) -> BLFLibResult {
        let mut task = console_task::start("Converting Map Variants");

        // Iterate through hopper folders. eg default_hoppers/00101
        let hopper_directory_subfolders = get_directories_in_folder(hoppers_blf_path)?;

        create_dir_all(build_path!(
            hoppers_config_path,
            k_map_variants_config_folder_name
        ))?;

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
                    &format!("_{:0>3}.bin", s_blf_chunk_map_variant::get_version().major),
                    ""
                );

                let map_variant_config_file_name = format!("{map_variant_file_name}.bin");

                let map_variant_blf_file_path = build_path!(
                    &map_variant_blfs_folder,
                    &map_variant_blf_file_name
                );

                let map_variant_config_file_path = build_path!(
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
                if exists(&map_variant_config_file_path)? {
                    remove_file(&map_variant_config_file_path)?
                }

                let mvar = find_chunk_in_file::<s_blf_chunk_map_variant>(map_variant_blf_file_path)?;

                BlfFileBuilder::new()
                    .add_chunk(s_blf_chunk_start_of_file::default())
                    .add_chunk(mvar)
                    .add_chunk(s_blf_chunk_end_of_file::default())
                    .write_file(map_variant_config_file_path)?;

                // map_variant::read_file(&map_variant_blf_file_path)?
                //     .write_to_config(hoppers_config_path, &map_variant_file_name)?;
            }
        }

        task.add_message(format!("Converted {} map variants.", converted_maps.len()));

        やった!(task)
    }

    fn build_config_game_variants(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting Game Variants");

        // Iterate through hopper folders. eg default_hoppers/00101
        let hopper_directory_subfolders = get_directories_in_folder(hoppers_blf_path)?;

        create_dir_all(build_path!(
            hoppers_config_path,
            k_game_variants_config_folder_name
        ))?;

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
                    &format!("_{:0>3}.bin", s_blf_chunk_matchmaking_game_variant::get_version().major),
                    ""
                );

                let game_variant_config_file_name = format!("{game_variant_file_name}.bin");

                let game_variant_blf_file_path = build_path!(
                    &game_variant_blfs_folder,
                    &game_variant_blf_file_name
                );

                let game_variant_config_file_path = build_path!(
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
                if exists(&game_variant_config_file_path)? {
                    remove_file(&game_variant_config_file_path)?;
                }

                let gvar = find_chunk_in_file::<s_blf_chunk_matchmaking_game_variant>(game_variant_blf_file_path)?;

                BlfFileBuilder::new()
                    .add_chunk(s_blf_chunk_start_of_file::default())
                    .add_chunk(gvar)
                    .add_chunk(s_blf_chunk_end_of_file::default())
                    .write_file(game_variant_config_file_path)?;

                // game_variant::read_file(&game_variant_blf_file_path)?
                //     .write_to_config(hoppers_config_path, &game_variant_file_name)?;
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

            let game_set = find_chunk_in_file::<s_blf_chunk_game_set>(&game_set_blf_path)?;
            let game_set_config_path = build_path!(
                hoppers_config_path,
                k_hoppers_config_folder_name,
                &hopper_folder,
                k_game_set_config_file_name
            );

            write_json_file(&game_set, &game_set_config_path)?;

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
    ) -> BLFLibResult<HashMap<String, HashMap<u16, String>>>
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

            let hopper_description_table = find_chunk_in_file::<s_blf_chunk_hopper_description_table>(&hopper_descriptions_path)
                .expect("Could not find hopper description table");

            let mut hoppers_description_map = HashMap::<u16, String>::new();

            for hopper_description in hopper_description_table.get_descriptions() {
                hoppers_description_map.insert(hopper_description.identifier, hopper_description.description.get_string()?);
            }

            language_descriptions_map.insert(String::from(language_code), hoppers_description_map);
        }

        Ok(language_descriptions_map)
    }

    fn build_config_hoppers(hoppers_blfs_path: &String, hoppers_config_path: &String) -> BLFLibResult {
        let mut task = console_task::start("Converting Hopper Configuration...");

        let language_hopper_descriptions
            = Self::read_hopper_description_blfs(hoppers_blfs_path, &mut task)?;

        let hopper_configuration_blf_path = build_path!(
            hoppers_blfs_path,
            k_matchmaking_hopper_file_name
        );

        let mut hopper_configuration_table = find_chunk_in_file::<s_blf_chunk_hopper_configuration_table>(&hopper_configuration_blf_path)?;
        let mut hopper_configurations = &mut hopper_configuration_table.hopper_configurations;
        let mut category_configurations = &mut hopper_configuration_table.hopper_categories;

        // Generate active_hoppers.txt
        let active_hopper_ids = hopper_configurations.iter().map(|config|config.identifier);
        let active_hoppers_txt_path = build_path!(
            hoppers_config_path,
            k_active_hoppers_config_file_name
        );
        let mut active_hoppers_txt_file = File::create(active_hoppers_txt_path)?;
        active_hoppers_txt_file.write_all(
            active_hopper_ids.map(|id|format!("{id:0>5}")).collect::<Vec<_>>().join("\r\n").as_bytes()
        )?;

        // Build hopper configuration json
        for hopper_configuration in hopper_configurations.iter_mut() {
            let mut hopper_configuration_json = matchmaking_hopper_config {
                descriptions: HashMap::new(),
                configuration: hopper_configuration.clone(),
            };

            for language_code in k_language_suffixes {
                if language_hopper_descriptions.contains_key(language_code)
                    && language_hopper_descriptions.get(language_code).unwrap().contains_key(&hopper_configuration_json.configuration.identifier)
                {
                    hopper_configuration_json.descriptions.insert(
                        String::from(language_code),
                        language_hopper_descriptions.get(language_code).unwrap().get(&hopper_configuration_json.configuration.identifier).unwrap().clone()
                    );
                }
                else {
                    hopper_configuration_json.descriptions.insert(String::from(language_code), String::new());
                }
            }

            let hopper_configuration_json_folder = build_path!(
                hoppers_config_path,
                k_hoppers_config_folder_name,
                format!("{:0>5}", hopper_configuration_json.configuration.identifier)
            );
            create_dir_all(&hopper_configuration_json_folder)?;

            let hopper_configuration_json_file = build_path!(
                &hopper_configuration_json_folder,
                k_hopper_config_file_name
            );
            let mut hopper_configuration_json_file = File::create(hopper_configuration_json_file)?;
            serde_json::to_writer_pretty(&mut hopper_configuration_json_file, &hopper_configuration_json)?;

            let hopper_image_file_path = build_path!(
                hoppers_blfs_path,
                format!("{:0>5}", &hopper_configuration.identifier),
                k_matchmaking_hopper_images_folder,
                k_matchmaking_hopper_image_filename
            );

            let hopper_image_file_path = build_path!(
                hoppers_blfs_path,
                format!("{:0>5}", &hopper_configuration.identifier),
                k_matchmaking_hopper_images_folder,
                k_matchmaking_hopper_image_filename
            );

            let hopper_image_file_destination_path = build_path!(
                hopper_configuration_json_folder,
                k_matchmaking_hopper_image_filename
            );

            if !exists(&hopper_image_file_path)? {
                task.add_warning(format!("No hopper image was found for {} ({})", &hopper_configuration.hopper_name.get_string()?, &hopper_configuration.identifier))
            } else {
                fs::copy(hopper_image_file_path, hopper_image_file_destination_path).unwrap();
            }
        }

        // Build categories json
        let mut categories_config = matchmaking_hopper_categories_config::default();

        for &mut category_configuration in category_configurations.iter_mut() {
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

        task.add_message(format!("Converted {} hopper configurations.", hopper_configuration_table.hopper_configurations.len()));

        やった!(task)
    }

    fn build_config_network_configuration(hoppers_blfs_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        // For now we just copy it as is. But we do check that it contains a netc.
        let mut task = console_task::start("Converting Network Configuration");

        let network_configuration_source_path = build_path!(
            hoppers_blfs_path,
            k_network_configuration_file_name
        );

        let network_configuration_dest_path = build_path!(
            hoppers_config_path,
            k_network_configuration_file_name
        );

        let netc = find_chunk_in_file::<s_blf_chunk_network_configuration>(network_configuration_source_path)?;
        BlfFileBuilder::new()
            .add_chunk(s_blf_chunk_start_of_file::new("reach net config", byte_order_mark::big_endian))
            .add_chunk(s_blf_chunk_author::for_build::<v12065_11_08_24_1738_tu1actual>())
            .add_chunk(netc)
            .add_chunk(s_blf_chunk_end_of_file::default())
            .write_file(network_configuration_dest_path)?;

        やった!(task)
    }

    fn build_blf_banhammer_messages(hoppers_config_folder: &String, hoppers_blf_folder: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Banhammer Messages");

        for language_code in k_language_suffixes {
            let matchmaking_banhammer_messages = matchmaking_banhammer_messages::read_from_config(
                hoppers_config_folder,
                language_code,
            );

            if matchmaking_banhammer_messages.is_err() {
                task.add_warning(format!("Failed to build {} banhammer messages.", get_language_string(language_code)));
                continue;
            }

            matchmaking_banhammer_messages?.write_file(build_path!(
                hoppers_blf_folder,
                language_code,
                k_matchmaking_banhammer_messages_file_name
            ))?;
        }

        やった!(task)
    }

    fn build_blf_dlc_manifest(hoppers_config_folder: &String, hoppers_blf_folder: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building DLC Manifest");

        let dlc_manifest = read_json_file::<s_blf_chunk_dlc_map_manifest>(build_path!(
            hoppers_config_folder,
            "dlc",
            "manifest.json"
        ))?;

        let dlc_config_folder = build_path!(hoppers_config_folder, "dlc");
        let dlc_images_folder = build_path!(hoppers_blf_folder, "dlc");

        create_dir_all(&dlc_images_folder)?;

        for map in dlc_manifest.maps.get() {
            if !map.small_image_file_name.is_empty() {
                let small_image_path = build_path!(&dlc_config_folder, map.small_image_file_name.get_string()?);
                let small_image_destination = build_path!(&dlc_images_folder, map.small_image_file_name.get_string()?);
                if exists(&small_image_path)? {
                    fs::copy(&small_image_path, small_image_destination)?;
                } else {
                    task.add_warning(format!("No small image was found for map {}", map.name_en.get_string()))
                }
            }

            if !map.large_image_file_name.is_empty() {
                let large_image_path = build_path!(&dlc_config_folder, map.large_image_file_name.get_string()?);
                let large_image_destination = build_path!(&dlc_images_folder, map.large_image_file_name.get_string()?);
                if exists(&large_image_path)? {
                    fs::copy(&large_image_path, large_image_destination)?;
                } else {
                    task.add_warning(format!("No large image was found for map {}", map.name_en.get_string()))
                }
            }
        }

        BlfFileBuilder::new()
            .add_chunk(s_blf_chunk_start_of_file::default())
            .add_chunk(dlc_manifest)
            .add_chunk(s_blf_chunk_end_of_file::default())
            .write_file(build_path!(
                hoppers_blf_folder,
                "dlc_map_manifest.bin"
            ))?;

        やった!(task)
    }

    fn build_blf_megalo_categories(hoppers_config_folder: &String, hoppers_blf_folder: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Megalo Categories");

        for language_code in k_language_suffixes {
            let megalo_categories = read_json_file::<s_blf_chunk_megalo_categories>(build_path!(
                hoppers_config_folder,
                "megalo_categories",
                format!("{}.json", language_code)
            ))?;

            BlfFileBuilder::new()
                .add_chunk(s_blf_chunk_start_of_file::default())
                .add_chunk(megalo_categories)
                .add_chunk(s_blf_chunk_end_of_file::default())
                .write_file(build_path!(
                    hoppers_blf_folder,
                    language_code,
                    "file_megalo_categories.bin"
                ))?;
        }

        やった!(task)
    }

    fn build_blf_predefined_queries(hoppers_config_folder: &String, hoppers_blf_folder: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Predefined Queries");

        for language_code in k_language_suffixes {
            let predefined_queries = read_json_file::<s_blf_chunk_predefined_queries>(build_path!(
                hoppers_config_folder,
                "predefined_queries",
                format!("{}.json", language_code)
            ))?;

            BlfFileBuilder::new()
                .add_chunk(s_blf_chunk_start_of_file::default())
                .add_chunk(predefined_queries)
                .add_chunk(s_blf_chunk_end_of_file::default())
                .write_file(build_path!(
                    hoppers_blf_folder,
                    language_code,
                    "file_predefined_queries.bin"
                ))?;
        }

        task.complete();

        let mut task = console_task::start("Building CEA Predefined Queries");

        for language_code in k_language_suffixes {
            let predefined_queries = read_json_file::<s_blf_chunk_predefined_queries>(build_path!(
                hoppers_config_folder,
                "predefined_queries_cea",
                format!("{}.json", language_code)
            ))?;

            BlfFileBuilder::new()
                .add_chunk(s_blf_chunk_start_of_file::default())
                .add_chunk(predefined_queries)
                .add_chunk(s_blf_chunk_end_of_file::default())
                .write_file(build_path!(
                    hoppers_blf_folder,
                    language_code,
                    "tangerine_file_predefined_queries.bin"
                ))?;
        }

        やった!(task)
    }

    fn build_blf_matchmaking_tips(hoppers_config_folder: &String, hoppers_blf_folder: &String, tangerine: bool) -> BLFLibResult {
        let mut task = console_task::start(
            if tangerine { "Building CEA Matchmaking Tips" }
            else { "Building Matchmaking Tips" }
        );

        for language_code in k_language_suffixes {
            let mut matchmaking_tips = matchmaking_tips::read_from_config(
                hoppers_config_folder,
                language_code,
                tangerine
            ).inspect_err(|err|{
                task.fail();
            })?;

            matchmaking_tips.write_file(build_path!(
                hoppers_blf_folder,
                language_code,
                if tangerine { k_tangerine_matchmaking_tips_file_name }
                else { k_matchmaking_tips_file_name }
            ))?;
        }

        やった!(task)
    }

    fn build_blf_nag_messages(hoppers_config_folder: &String, hoppers_blf_folder: &String, tangerine: bool) -> BLFLibResult {
        let mut task = console_task::start(format!(
            "Building {}Nag Messages",
            if tangerine { "CEA " } else { "" }
        ));

        for language_code in k_language_suffixes {
            let nag_message = nag_message::read_from_config(hoppers_config_folder, language_code, tangerine);

            if nag_message.is_err() {
                task.add_warning(format!(
                    "Failed to build {} {}Nag Message: {}",
                    get_language_string(language_code),
                    if tangerine { "CEA " } else { "" },
                    nag_message.unwrap_err()
                ));

                continue;
            }

            nag_message?.write_file(build_path!(
                hoppers_blf_folder,
                language_code,
                if tangerine { k_cea_nag_message_file_name } else { k_nag_message_file_name }
            ))?;
        }

        for language_code in k_language_suffixes {
            let jpeg_file_path = build_path!(
                hoppers_config_folder,
                if tangerine { k_cea_nag_message_config_folder } else { k_nag_message_config_folder },
                format!("{}.jpg", language_code)
            );

            let destination_path = build_path!(
                hoppers_blf_folder,
                language_code,
                if tangerine { k_cea_nag_message_image_file_name } else { k_nag_message_image_file_name }
            );

            let validated = nag_message::validate_image(&jpeg_file_path);

            if validated.is_err() {
                task.add_warning(format!(
                    "Failed to convert {}{} Nag Message Image: {}",
                    get_language_string(language_code),
                    if tangerine { "CEA " } else { "" },
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

        let rsa_folder = build_path!(hoppers_config_path, k_rsa_signatures_config_folder_name);
        let rsa_files = get_files_in_folder(&rsa_folder)?;

        if rsa_files.is_empty() {
            task.add_error("No RSA signatures were found.")
        }

        let mut map_manifest = s_blf_chunk_map_manifest::default();

        for rsa_file_name in rsa_files {
            let rsa_file_path = build_path!(&rsa_folder, &rsa_file_name);
            let mut rsa_file = File::open(&rsa_file_path)?;
            let mut rsa_signature = Vec::<u8>::with_capacity(0x100);
            rsa_file.read_to_end(&mut rsa_signature).unwrap();

            map_manifest.add_rsa_signature(rsa_signature.as_slice())?;
        }

        BlfFileBuilder::new()
            .add_chunk(s_blf_chunk_author::for_build::<v12065_11_08_24_1738_tu1actual>())
            .add_chunk(s_blf_chunk_start_of_file::new("rsa manifest", byte_order_mark::big_endian))
            .add_chunk(map_manifest)
            .add_chunk(s_blf_chunk_end_of_file::default())
            .write_file(build_path!(
                hoppers_blf_path,
                k_rsa_manifest_file_name
            ))?;

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

    fn read_game_set_configuration(hoppers_config_path: &String, active_hopper_folders: &Vec<String>) -> BLFLibResult<HashMap<u16, s_blf_chunk_game_set>>
    {
        let mut task = console_task::start("Reading Game Set Config");

        let mut game_sets = HashMap::<u16, s_blf_chunk_game_set>::new();

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

            let game_set_json_path = build_path!(
                &hopper_tables_config_path,
                subfolder,
                k_game_set_config_file_name
            );

            if !exists(&game_set_json_path)? {
                task.fail_with_error(format!("No game set was found for hopper \"{subfolder}\""));
                panic!();
            }

            let game_set = read_json_file::<s_blf_chunk_game_set>(game_set_json_path)?;

            game_sets.insert(hopper_id, game_set);
        }

        task.complete();

        Ok(game_sets)
    }

    fn build_blf_game_variants(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
        build_temp_dir: &String,
        game_sets: &HashMap<u16, s_blf_chunk_game_set>,
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
            game_set.entries.iter()
                .filter(|entry|!entry.game_variant_file_name.is_empty())
                .map(|entry|entry.game_variant_file_name.get_string().unwrap().clone()).collect::<Vec<String>>()
        ).collect();

        let game_variants_to_convert: HashSet<String> = HashSet::from_iter(game_variants_to_convert.iter().cloned());

        let mut json_queue: Vec<(String, s_blf_chunk_matchmaking_game_variant)> = Vec::new();
        for game_variant in game_variants_to_convert {
            let game_variant_json_path = build_path!(
                &game_variants_config_path,
                format!("{game_variant}.bin")
            );

            if !Path::new(&game_variant_json_path).exists() {
                task.fail_with_error(format!("Game variant \"{}\" could not be found.", game_variant));
                panic!();
            }

            // let mut file = File::open(&map_variant_json_path).unwrap();
            // let mut game_variant_json = String::new();
            // file.read_to_string(&mut game_variant_json).unwrap();
            let chunk = find_chunk_in_file::<s_blf_chunk_matchmaking_game_variant>(game_variant_json_path).unwrap();

            json_queue.push((game_variant, chunk));
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
                                    s_blf_chunk_matchmaking_game_variant::get_version().major
                                )
                            );

                            // let game_variant_json: c_game_variant = serde_json::from_str(&json).unwrap();
                            //
                            // let mut game_variant_blf_file = game_variant::create(game_variant_json);
                            // game_variant_blf_file.write_file(&game_variant_blf_path).expect(&format!("Failed to write game variant {}", game_variant_file_name));

                            BlfFileBuilder::new()
                                .add_chunk(s_blf_chunk_start_of_file::new("game var", byte_order_mark::big_endian))
                                .add_chunk(s_blf_chunk_author::for_build::<v12065_11_08_24_1738_tu1actual>())
                                .add_chunk(json)
                                .add_chunk(s_blf_chunk_end_of_file::default())
                                .write_file(&game_variant_blf_path)
                                .unwrap();

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

    fn build_blf_map_variants(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
        build_temp_dir: &String,
        game_sets: &HashMap<u16, s_blf_chunk_game_set>,
        variant_hashes: &mut HashMap<String, s_network_http_request_hash>,
    )
    {
        let mut task = console_task::start("Building Map Variants");

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
            game_set.entries.iter()
                .filter(|entry| !entry.map_variant_file_name.is_empty())
                .map(|entry| entry.map_variant_file_name.get_string().unwrap().clone()).collect::<Vec<String>>()
        ).collect();
        let map_variants_to_convert: HashSet<String> = HashSet::from_iter(map_variants_to_convert.iter().cloned());

        let mut json_queue: Vec<(String, s_blf_chunk_map_variant)> = Vec::new();
        for map_variant in map_variants_to_convert {
            let map_variant_json_path = build_path!(
                &map_variants_config_path,
                format!("{map_variant}.bin")
            );

            if !Path::new(&map_variant_json_path).exists() {
                task.fail_with_error(format!("Map variant \"{}\" could not be found.", map_variant));
                panic!();
            }

            // let mut file = File::open(&map_variant_json_path).unwrap();
            // let mut map_variant_json = String::new();
            // file.read_to_string(&mut map_variant_json).unwrap();
            let chunk = find_chunk_in_file::<s_blf_chunk_map_variant>(map_variant_json_path).unwrap();

            json_queue.push((map_variant, chunk));
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
                let map_variants_config_path = map_variants_config_path.clone();
                let map_variants_temp_build_path = map_variants_temp_build_path.clone();
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
                                    s_blf_chunk_map_variant::get_version().major
                                )
                            );

                            // let mut map_variant_json: c_map_variant = serde_json::from_str(&json).unwrap();
                            //
                            // let mut map_variant_blf_file = map_variant::create(map_variant_json);
                            // map_variant_blf_file.write_file(&map_variant_blf_path).unwrap();
                            BlfFileBuilder::new()
                                .add_chunk(s_blf_chunk_start_of_file::default())
                                .add_chunk(json)
                                .add_chunk(s_blf_chunk_end_of_file::default())
                                .write_file(&map_variant_blf_path).unwrap();

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

            let mut task = task.lock().await;
            task.add_message(format!("Built {} variants.", variant_hashes.len()));
            task.complete();
        });
    }

    fn build_blf_game_sets(
        hoppers_blf_path: &String,
        active_game_sets: HashMap<u16, s_blf_chunk_game_set>,
        game_variant_hashes: &HashMap<String, s_network_http_request_hash>,
        map_variant_hashes: &HashMap<String, s_network_http_request_hash>,
        build_temp_dir_path: &String,
    ) -> Result<(), Box<dyn Error>>
    {
        let mut task = console_task::start("Building Game Sets");

        for (hopper_id, mut game_set_config) in active_game_sets {
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
                if !game_set_row.game_variant_file_name.is_empty()
                    && !copied_games.contains(&game_set_row.game_variant_file_name.get_string()?)
                {
                    let game_variant_file_name = format!(
                        "{}_{:0>3}.bin",
                        game_set_row.game_variant_file_name.get_string()?,
                        s_blf_chunk_matchmaking_game_variant::get_version().major,
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

                if !game_set_row.map_variant_file_name.is_empty()
                    && !copied_maps.contains(&game_set_row.map_variant_file_name.get_string()?) {
                    let map_variant_file_name = format!(
                        "{}_{:0>3}.bin",
                        game_set_row.map_variant_file_name.get_string()?,
                        s_blf_chunk_map_variant::get_version().major,
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

            for mut entry in game_set_config.entries.iter_mut() {
                if !entry.map_variant_file_name.is_empty() {
                    entry.map_variant_hash = map_variant_hashes.get(&entry.map_variant_file_name.get_string()?).unwrap().clone();
                }
                if !entry.game_variant_file_name.is_empty() {
                    entry.game_variant_hash = game_variant_hashes.get(&entry.game_variant_file_name.get_string()?).unwrap().clone();
                }
            }

            BlfFileBuilder::new()
                .add_chunk(s_blf_chunk_start_of_file::new("game set", byte_order_mark::big_endian))
                .add_chunk(s_blf_chunk_author::for_build::<v12065_11_08_24_1738_tu1actual>())
                .add_chunk(game_set_config)
                .add_chunk(s_blf_chunk_end_of_file::default())
                .write_file(build_path!(
                    &hopper_folder_path,
                    k_game_set_blf_file_name
                ))?;
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
            hopper_config.game_set_hash = get_blf_file_hash(game_set_blf_file_path)?;
            hopper_configuration_table.hopper_configurations.push(hopper_config);
        }

        // Load category configuration
        let categories_configuration = matchmaking_hopper_categories_config::read(hoppers_config_path)?;

        let active_hopper_categories = hopper_configuration_table
            .hopper_configurations
            .iter().map(|hopper|hopper.category_identifier)
            .collect::<HashSet<_>>();
        let active_hopper_category_configurations = categories_configuration.categories
            .iter().filter(|category_configuration|active_hopper_categories.contains(&category_configuration.configuration.category_identifier))
            .cloned()
            .collect::<Vec<matchmaking_hopper_category_configuration_and_descriptions>>();

        for active_hopper_category in &active_hopper_category_configurations {
            hopper_configuration_table.hopper_categories.push(active_hopper_category.configuration);
        }

        // Initialize language_hopper_descriptions
        for language_code in k_language_suffixes {
            let mut language_descriptions = s_blf_chunk_hopper_description_table::default();

            for (hopper_identifier, hopper_configuration_json) in &hopper_configuration_jsons {
                if !hopper_configuration_json.descriptions.contains_key(&language_code.to_string()) {
                    task.add_warning(format!(
                        "No {} description was found for hopper {hopper_identifier} ({})",
                        get_language_string(language_code),
                        hopper_configuration_json.configuration.hopper_name.get_string()?,
                    ));
                    continue;
                }

                let description = hopper_configuration_json.descriptions.get(&language_code.to_string()).unwrap();
                if description.is_empty() {
                    task.add_warning(format!(
                        "No {} description was found for hopper {hopper_identifier} ({})",
                        get_language_string(language_code),
                        hopper_configuration_json.configuration.hopper_name.get_string()?,
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
                        active_hopper_category.configuration.category_name.get_string()?,
                    ));
                    continue;
                }

                let description = active_hopper_category.descriptions.get(&language_code.to_string()).unwrap();
                if description.is_empty() {
                    task.add_warning(format!(
                        "No {} description was found for category {} ({})",
                        get_language_string(language_code),
                        active_hopper_category.configuration.category_identifier,
                        active_hopper_category.configuration.category_name.get_string()?,
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
            matchmaking_hopper_descriptions.write_file(&descriptions_blf_path)?;
        }

        // Write the hopper config file.
        let mut matchmaking_hopper_blf = matchmaking_hopper::create(hopper_configuration_table);
        matchmaking_hopper_blf.write_file(build_path!(
            hoppers_blfs_path,
            k_matchmaking_hopper_file_name
        ))?;

        やった!(task)
    }

    fn build_blf_network_configuration(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
    ) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Network Configuration");
        let netc = find_chunk_in_file::<s_blf_chunk_network_configuration>(build_path!(
            hoppers_config_path,
            k_network_configuration_file_name
        ))?;

        BlfFileBuilder::new()
            .add_chunk(s_blf_chunk_start_of_file::new("reach net config", byte_order_mark::big_endian))
            .add_chunk(s_blf_chunk_author::for_build::<v12065_11_08_24_1738_tu1actual>())
            .add_chunk(netc)
            .add_chunk(s_blf_chunk_end_of_file::default())
            .write_file(build_path!(
                hoppers_blfs_path,
                k_network_configuration_file_name
            ))?;

        やった!(task)
    }

    fn build_blf_manifest(
        hoppers_blfs_path: &String,
    ) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Building Manifest File");

        let mut manifest_chunk = s_blf_chunk_online_file_manifest::default();
        let hopper_directory_name = Path::new(hoppers_blfs_path).file_name().unwrap().to_str().unwrap();

        let dlc_manifest_file_hash = get_blf_file_hash(build_path!(
            hoppers_blfs_path,
            "dlc_map_manifest.bin"
        ))?;

        let hopper_config_file_hash = get_blf_file_hash(build_path!(
            hoppers_blfs_path,
            k_matchmaking_hopper_file_name
        ))?;

        let network_config_file_hash = get_blf_file_hash(build_path!(
            hoppers_blfs_path,
            format!("network_configuration_{:0>3}.bin", s_blf_chunk_network_configuration::get_version().major)
        ))?;

        let rsa_manifest_file_hash = get_blf_file_hash(build_path!(
            hoppers_blfs_path,
            k_rsa_manifest_file_name
        ))?;

        manifest_chunk.add_file_hash(
            format!("/title/{hopper_directory_name}/dlc_map_manifest.bin"),
            dlc_manifest_file_hash,
        )?;

        manifest_chunk.add_file_hash(
            format!("/title/{hopper_directory_name}/{k_matchmaking_hopper_file_name}"),
            hopper_config_file_hash,
        )?;

        manifest_chunk.add_file_hash(
            format!("/title/{hopper_directory_name}/network_configuration_{:0>3}.bin", s_blf_chunk_network_configuration::get_version().major),
            network_config_file_hash,
        )?;

        manifest_chunk.add_file_hash(
            format!("/title/{hopper_directory_name}/{k_rsa_manifest_file_name}"),
            rsa_manifest_file_hash,
        )?;

        for language_code in crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::k_language_suffixes {
            let megalo_categories_file_hash = get_blf_file_hash(build_path!(
                hoppers_blfs_path,
                language_code,
                "file_megalo_categories.bin"
            ))?;

            let predefined_queries_file_hash = get_blf_file_hash(build_path!(
                hoppers_blfs_path,
                language_code,
                "file_predefined_queries.bin"
            ))?;

            let tangerine_predefined_queries_file_hash = get_blf_file_hash(build_path!(
                hoppers_blfs_path,
                language_code,
                "tangerine_file_predefined_queries.bin"
            ))?;

            let banhammer_messages_file_hash = get_blf_file_hash(build_path!(
                hoppers_blfs_path,
                language_code,
                k_matchmaking_banhammer_messages_file_name
            ))?;

            let hopper_descriptions_file_hash = get_blf_file_hash(build_path!(
                hoppers_blfs_path,
                language_code,
                k_matchmaking_hopper_descriptions_file_name.clone()
            ))?;

            let matchmaking_tips_file_hash = get_blf_file_hash(build_path!(
                hoppers_blfs_path,
                language_code,
                k_matchmaking_tips_file_name
            ))?;

            let tangerine_matchmaking_tips_file_hash = get_blf_file_hash(build_path!(
                hoppers_blfs_path,
                language_code,
                k_tangerine_matchmaking_tips_file_name
            ))?;

            manifest_chunk.add_file_hash(
                format!("/title/{hopper_directory_name}/{language_code}/file_megalo_categories.bin"),
                megalo_categories_file_hash,
            )?;

            manifest_chunk.add_file_hash(
                format!("/title/{hopper_directory_name}/{language_code}/file_predefined_queries.bin"),
                predefined_queries_file_hash,
            )?;

            manifest_chunk.add_file_hash(
                format!("/title/{hopper_directory_name}/{language_code}/tangerine_file_predefined_queries.bin"),
                tangerine_predefined_queries_file_hash,
            )?;

            manifest_chunk.add_file_hash(
                format!("/title/{hopper_directory_name}/{language_code}/{k_matchmaking_banhammer_messages_file_name}"),
                banhammer_messages_file_hash,
            )?;

            manifest_chunk.add_file_hash(
                format!("/title/{hopper_directory_name}/{language_code}/{k_matchmaking_hopper_descriptions_file_name}"),
                hopper_descriptions_file_hash,
            )?;

            manifest_chunk.add_file_hash(
                format!("/title/{hopper_directory_name}/{language_code}/{k_matchmaking_tips_file_name}"),
                matchmaking_tips_file_hash,
            )?;

            manifest_chunk.add_file_hash(
                format!("/title/{hopper_directory_name}/{language_code}/{k_tangerine_matchmaking_tips_file_name}"),
                tangerine_matchmaking_tips_file_hash,
            )?;
        }

        やった!(task)
    }
}