use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::fs::{create_dir_all, exists, remove_file, File};
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;
use colored::Colorize;
use csv::{ReaderBuilder, WriterBuilder};
use crate::io::{create_parent_folders, get_directories_in_folder, get_files_in_folder, read_text_file_lines, write_text_file, write_text_file_lines, FILE_SEPARATOR};
use crate::{build_path, debug_log, title_converter, やった};
use crate::title_storage::{check_file_exists, validate_jpeg, TitleConverter};
use lazy_static::lazy_static;
use blf_lib::blam::halo3::release::cseries::language::{get_language_string, k_language_suffix_chinese_traditional, k_language_suffix_english, k_language_suffix_french, k_language_suffix_german, k_language_suffix_italian, k_language_suffix_japanese, k_language_suffix_korean, k_language_suffix_mexican, k_language_suffix_portuguese, k_language_suffix_spanish};
use blf_lib::blf::{get_blf_file_hash, BlfFile, BlfFileBuilder};
use blf_lib::blf::chunks::{find_chunk_in_file, BlfChunk};
use blf_lib::blf::versions::ares::v_untracked_ares::{s_blf_chunk_hopper_description_table, s_blf_chunk_matchmaking_tips, s_blf_chunk_message_of_the_day, s_blf_chunk_message_of_the_day_popup, s_blf_chunk_network_configuration, s_blf_chunk_packed_game_variant, s_blf_chunk_packed_map_variant, s_blf_chunk_game_set, s_blf_chunk_author};
use crate::console::console_task;
use regex::Regex;
use tempdir::TempDir;
use tokio::runtime;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use blf_lib::blam::common::memory::crc::crc32;
use blf_lib::blam::common::memory::secure_signature::s_network_http_request_hash;
use blf_lib::blam::halo3::release::game::game_engine_variant::c_game_variant;
use blf_lib::blam::halo3::release::saved_games::scenario_map_variant::c_map_variant;
use blf_lib::blf::versions::ares::v_untracked_ares::s_blf_chunk_hopper_configuration_table;
use blf_lib::blf::versions::ares::v_untracked_ares::{s_blf_chunk_banhammer_messages, s_blf_chunk_online_file_manifest};
use blf_lib::blf::versions::ares::v_untracked_ares::{s_blf_chunk_end_of_file, s_blf_chunk_game_set_entry, s_blf_chunk_map_manifest, s_blf_chunk_start_of_file};
use blf_lib::io::{read_file_to_string, read_json_file, write_json_file};
use blf_lib::result::{BLFLibError, BLFLibResult};
use blf_lib::types::c_string::StaticString;
use crate::title_storage::ares::v_untracked_ares::title_storage_config::{get_hopper_id_from_hopper_folder_name, matchmaking_hopper_category_configuration_and_descriptions};
use crate::title_storage::ares::v_untracked_ares::title_storage_output::hopper_directory_name_max_length;

title_converter! (
    #[Title("Ares")]
    #[Build("untracked")]
    pub struct v_untracked_ares {}
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
    static ref map_variant_file_regex: Regex = Regex::new(&format!("_{:0>3}.bin$", s_blf_chunk_packed_map_variant::get_version().major)).unwrap();
    static ref game_variant_file_regex: Regex = Regex::new(&format!("_{:0>3}.bin$", s_blf_chunk_packed_game_variant::get_version().major)).unwrap();
    static ref config_rsa_signature_file_map_id_regex: Regex = Regex::new(r"^[0-9]{1,}").unwrap();
}

mod title_storage_output {
    use blf_lib::blf::chunks::BlfChunk;
    use blf_lib::blf::versions::ares::v_untracked_ares::{s_blf_chunk_game_set, s_blf_chunk_hopper_configuration_table, s_blf_chunk_hopper_description_table, s_blf_chunk_network_configuration, s_blf_chunk_online_file_manifest, s_blf_chunk_packed_game_variant, s_blf_chunk_packed_map_variant};
    use crate::build_path;

    // applies to the root folder, eg "default_hoppers"
    pub const hopper_directory_name_max_length: usize = 64;

    pub const motd_image_max_size: usize = 61440;
    pub const motd_image_width: usize = 476;
    pub const motd_image_height: usize = 190;

    pub const motd_popup_image_max_size: usize = 61440;
    pub const motd_popup_image_width: usize = 308;
    pub const motd_popup_image_height: usize = 466;

    // Root Directory
    // storage/title/tracked/12070/default_hoppers/
    pub fn network_configuration_file_name() -> String {
        format!("network_configuration_{:0>3}.bin", s_blf_chunk_network_configuration::get_version().major)
    }
    pub fn network_configuration_file_path(hoppers_path: &String) -> String {
        build_path!(
            hoppers_path,
            network_configuration_file_name()
        )
    }
    pub fn manifest_file_name() -> String {
        format!("manifest_{:0>3}.bin", s_blf_chunk_online_file_manifest::get_version().major)
    }

    pub fn manifest_file_path(hoppers_path: &String) -> String {
        build_path!(
            hoppers_path,
            manifest_file_name()
        )
    }

    pub fn matchmaking_hopper_file_name() -> String {
        format!("matchmaking_hopper_{:0>3}.bin", s_blf_chunk_hopper_configuration_table::get_version().major)
    }
    pub fn matchmaking_hopper_file_path(hoppers_path: &String) -> String {
        build_path!(
            hoppers_path,
            matchmaking_hopper_file_name()
        )
    }

    // Languages
    // storage/title/4d53085bf0/12065/default_hoppers/en/
    pub const rsa_manifest_file_name: &str = "rsa_manifest.bin";
    pub fn rsa_manifest_file_path(hoppers_path: &String) -> String {
        build_path!(
            hoppers_path,
            rsa_manifest_file_name
        )
    }
    pub const banhammer_messages_file_name: &str = "matchmaking_banhammer_messages.bin";
    pub fn banhammer_messages_file_path(hoppers_path: &String, language_code: &str) -> String {
        build_path!(
            hoppers_path,
            language_code,
            banhammer_messages_file_name
        )
    }

    pub const matchmaking_tips_file_name: &str = "matchmaking_tips.bin";
    pub fn matchmaking_tips_file_path(hoppers_path: &String, language_code: &str) -> String {
        build_path!(
            hoppers_path,
            language_code,
            matchmaking_tips_file_name
        )
    }

    pub fn motd_file_name(blue: bool) -> String {
        if blue { "blue_motd.bin".to_string() }
        else { "motd.bin".to_string() }
    }
    pub fn motd_file_path(hoppers_path: &String, language_code: &str, blue: bool) -> String {
        build_path!(
            hoppers_path,
            language_code,
            motd_file_name(blue)
        )
    }

    pub fn motd_image_file_name(blue: bool) -> String {
        if blue { "blue_motd_image.jpg".to_string() }
        else { "motd_image.jpg".to_string() }
    }
    pub fn motd_image_file_path(hoppers_path: &String, language_code: &str, blue: bool) -> String {
        build_path!(
            hoppers_path,
            language_code,
            motd_image_file_name(blue)
        )
    }

    pub fn motd_popup_file_name(blue: bool) -> String {
        if blue { "blue_motd_popup.bin".to_string() }
        else { "motd_popup.bin".to_string() }
    }
    pub fn motd_popup_file_path(hoppers_path: &String, language_code: &str, blue: bool) -> String {
        build_path!(
            hoppers_path,
            language_code,
            motd_popup_file_name(blue)
        )
    }
    pub fn motd_popup_image_file_name(blue: bool) -> String {
        if blue { "blue_motd_popup_image.jpg".to_string() }
        else { "motd_popup_image.jpg".to_string() }
    }
    pub fn motd_popup_image_file_path(hoppers_path: &String, language_code: &str, blue: bool) -> String {
        build_path!(
            hoppers_path,
            language_code,
            motd_popup_image_file_name(blue)
        )
    }

    pub fn hopper_descriptions_file_name() -> String {
        format!("matchmaking_hopper_descriptions_{:0>3}.bin", s_blf_chunk_hopper_description_table::get_version().major)
    }

    pub fn hopper_descriptions_file_path(hoppers_path: &String, language_code: &str) -> String {
        build_path!(
            hoppers_path,
            language_code,
            hopper_descriptions_file_name()
        )
    }

    // Hoppers
    // storage/title/4d53085bf0/12065/default_hoppers/00101/
    pub fn game_set_file_name() -> String {
        format!("game_set_{:0>3}.bin", s_blf_chunk_game_set::get_version().major)
    }
    pub fn game_set_file_path(hoppers_path: &String, hopper_identifier: u16) -> String {
        build_path!(
            hoppers_path,
            format!("{:0>5}", hopper_identifier),
            game_set_file_name()
        )
    }

    pub fn game_variant_file_path(hoppers_path: &String, hopper_identifier: u16, game_variant_file_name: &String) -> String {
        build_path!(
            hoppers_path,
            format!("{:0>5}", hopper_identifier),
            format!(
                "{game_variant_file_name}_{:0>3}.bin",
                s_blf_chunk_packed_game_variant::get_version().major
            )
        )
    }

    pub const map_variants_folder_name: &str = "map_variants";
    pub fn map_variant_file_path(hoppers_path: &String, hopper_identifier: u16, map_variant_file_name: &String) -> String {
        build_path!(
            hoppers_path,
            format!("{:0>5}", hopper_identifier),
            map_variants_folder_name,
            format!(
                "{map_variant_file_name}_{:0>3}.bin",
                s_blf_chunk_packed_map_variant::get_version().major
            )
        )
    }
}

mod title_storage_config {
    use std::collections::HashMap;
    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use blf_lib::OPTION_TO_RESULT;
    use blf_lib::result::BLFLibResult;
    use crate::build_path;
    use crate::io::ordered_map;
    use blf_lib::blf::versions::ares::v_untracked_ares::{c_hopper_configuration, s_game_hopper_custom_category};

    pub const banhammer_messages_folder_name: &str = "banhammer_messages";
    pub fn banhammer_messages_file_path(config_folder: &String, language_code: &str) -> String {
        build_path!(
            config_folder,
            banhammer_messages_folder_name,
            format!("{language_code}.txt")
        )
    }

    pub const rsa_signatures_folder_name: &str = "rsa_signatures";
    pub fn rsa_signatures_folder_path(config_folder: &String) -> String {
        build_path!(
            config_folder,
            rsa_signatures_folder_name
        )
    }

    pub const matchmaking_tips_folder_name: &str = "matchmaking_tips";

    pub fn matchmaking_tips_file_path(config_folder: &String, language_code: &str) -> String {
        build_path!(
            config_folder,
            matchmaking_tips_folder_name,
            format!("{language_code}.txt")
        )
    }

    pub fn motd_folder_name(blue: bool) -> String {
        if blue { "motd_mythic".into() }
        else { "motd".into() }
    }

    pub fn motd_file_path(config_folder: &String, language_code: &str, blue: bool) -> String {
        build_path!(
            config_folder,
            motd_folder_name(blue),
            format!("{language_code}.txt")
        )
    }

    pub fn motd_image_file_path(config_folder: &String, language_code: &str, blue: bool) -> String {
        build_path!(
            config_folder,
            motd_folder_name(blue),
            format!("{language_code}.jpg")
        )
    }

    pub fn motd_popup_folder_name(blue: bool) -> String {
        if blue { "popup_mythic".into() }
        else { "popup".into() }
    }

    pub fn motd_popup_file_path(config_folder: &String, language_code: &str, blue: bool) -> String {
        build_path!(
            config_folder,
            motd_popup_folder_name(blue),
            format!("{language_code}.json")
        )
    }

    pub fn motd_popup_image_file_path(config_folder: &String, language_code: &str, blue: bool) -> String {
        build_path!(
            config_folder,
            motd_popup_folder_name(blue),
            format!("{language_code}.jpg")
        )
    }

    pub const active_hoppers_file_name: &str = "active_hoppers.txt";
    pub fn active_hoppers_file_path(config_folder: &String) -> String {
        build_path!(
            config_folder,
            active_hoppers_file_name
        )
    }

    pub const game_set_file_name: &str = "game_set.csv";
    pub fn game_set_file_path(config_folder: &String, hopper_folder: &String) -> String {
        build_path!(
            config_folder,
            matchmaking_hoppers_folder_name,
            hopper_folder,
            game_set_file_name
        )
    }

    pub fn get_hopper_id_from_hopper_folder_name(hopper_folder: &String) -> BLFLibResult<u16> {
        OPTION_TO_RESULT!(
            OPTION_TO_RESULT!(
                Regex::new(r"^[0-9]{1,5}")?.captures(hopper_folder),
                format!("No hopper ID found in folder {hopper_folder}")
            )?.get(0),
            format!("No hopper ID found in folder {hopper_folder}")
        )?.as_str().parse::<u16>().map_err(From::from)
    }

    pub const game_variants_folder_name: &str = "game_variants";
    pub fn game_variant_file_path(config_folder: &String, variant_file_name: &String) -> String {
        build_path!(
            config_folder,
            game_variants_folder_name,
            format!("{variant_file_name}.json")
        )
    }

    pub const map_variants_folder_name: &str = "map_variants";
    pub fn map_variant_file_path(config_folder: &String, variant_file_name: &String) -> String {
        build_path!(
            config_folder,
            map_variants_folder_name,
            format!("{variant_file_name}.json")
        )
    }


    pub const matchmaking_hoppers_folder_name: &str = "hoppers";
    pub const matchmaking_hopper_configuration_file_name: &str = "configuration.json";
    pub fn matchmaking_hopper_configuration_file_path(config_folder: &String, hopper_folder: &String) -> String {
        build_path!(
            config_folder,
            matchmaking_hoppers_folder_name,
            hopper_folder,
            matchmaking_hopper_configuration_file_name
        )
    }

    pub const matchmaking_hopper_categories_file_name: &str = "hopper_categories.json";
    pub fn matchmaking_hopper_categories_file_path(config_folder: &String) -> String {
        build_path!(
            config_folder,
            matchmaking_hopper_categories_file_name
        )
    }

    pub fn network_configuration_file_name() -> String {
        format!("network_configuration.json")
    }
    pub fn network_configuration_file_path(config_folder: &String) -> String {
        build_path!(
            config_folder,
            network_configuration_file_name()
        )
    }


    #[derive(Serialize, Deserialize)]
    pub struct matchmaking_hopper {
        #[serde(serialize_with = "ordered_map")]
        pub descriptions: HashMap<String, String>,
        pub configuration: c_hopper_configuration,
    }

    #[derive(Serialize, Deserialize, Default, Clone)]
    pub struct matchmaking_hopper_category_configuration_and_descriptions {
        pub configuration: s_game_hopper_custom_category,
        #[serde(serialize_with = "ordered_map")]
        pub descriptions: HashMap<String, String>,
    }

    #[derive(Serialize, Deserialize, Default)]
    pub struct matchmaking_hopper_categories {
        pub categories: Vec<matchmaking_hopper_category_configuration_and_descriptions>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct game_set_config_row {
        pub map_variant_file_name: String,
        pub game_variant_file_name: String,
        pub weight: u32,
        pub minimum_player_count: u8,
        pub skip_after_veto: bool,
        pub optional: bool,
    }

    pub struct game_set_config {
        pub entries: Vec<game_set_config_row>,
    }
}

impl TitleConverter for v_untracked_ares {
    fn build_blfs(&mut self, config_path: &String, blfs_path: &String) {
        let start_time = SystemTime::now();

        println!("{}", format!("Writing Title Storage BLFs to {blfs_path}").bold());

        let hopper_directories = get_directories_in_folder(config_path).unwrap_or_else(|err|{
            println!("{}", err);
            panic!()
        });

        for hopper_directory in hopper_directories {
            let result = || -> Result<(), Box<dyn Error>> {
                if hopper_directory.len() > hopper_directory_name_max_length {
                    return Err(Box::from(format!(
                        "Hoppers folder \"{hopper_directory}\" is too long and will be skipped. ({} > {} characters)",
                        hopper_directory.len(),
                        hopper_directory_name_max_length
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

                println!("{} {}...", "Converting".bold(), hopper_directory.bold().bright_white());

                let active_hoppers = Self::read_active_hopper_configuration(&hopper_config_path)?;
                let game_sets = Self::read_game_set_configuration(&hopper_config_path, &active_hoppers)?;
                let mut game_variant_hashes = HashMap::<String, s_network_http_request_hash>::new();
                let mut map_variant_hashes = HashMap::<String, s_network_http_request_hash>::new();
                let mut map_variant_map_ids = HashMap::<String, u32>::new();

                Self::build_blf_banhammer_messages(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_matchmaking_tips(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_motds(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_motd_popups(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_map_manifest(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_game_variants(&hopper_config_path, &hopper_blfs_path, &build_temp_dir_path, &game_sets, &mut game_variant_hashes);
                Self::build_blf_map_variants(&hopper_config_path, &hopper_blfs_path, &build_temp_dir_path, &game_sets, &mut map_variant_hashes, &mut map_variant_map_ids);
                Self::build_blf_game_sets(&hopper_blfs_path, game_sets, &game_variant_hashes, &map_variant_hashes, &map_variant_map_ids, &build_temp_dir_path)?;
                Self::build_blf_hoppers(&hopper_config_path, &hopper_blfs_path, &active_hoppers)?;
                Self::build_blf_network_configuration(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_manifest(&hopper_blfs_path)?;

                Ok(())
            }();

            if result.is_err() {
                println!("{}", "Failed to build title storage for hoppers".bright_white().on_red());
                println!("{}", result.err().unwrap().to_string().on_red());
            }
        }

        let seconds = start_time.elapsed().unwrap().as_secs_f32();
        println!("Finished conversion in {seconds:.2} seconds.");
    }

    fn build_config(&mut self, blfs_path: &String, config_path: &String) {
        println!("{} {}", "Writing Title Storage config to ".bold(), config_path.bold());

        let hopper_directories = get_directories_in_folder(blfs_path).unwrap_or_else(|err|{
            println!("{}", err);
            panic!();
        });

        for hopper_directory in hopper_directories {
            let result = || -> Result<(), Box<dyn Error>> {
                if hopper_directory.len() > hopper_directory_name_max_length {
                    return Err(Box::<dyn Error>::from(format!("Skipping \"{hopper_directory}\" as it's name is too long. ({hopper_directory_name_max_length} characters MAX)")))
                }

                let hoppers_config_path = build_path!(
                    config_path,
                    &hopper_directory
                );

                let hoppers_blf_path = build_path!(
                    blfs_path,
                    &hopper_directory
                );

                println!("{} {}...", "Converting".bold(), hopper_directory.bold().bright_white());
                Self::build_config_banhammer_messages(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_matchmaking_tips(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_motds(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_motd_popups(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_map_variants(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_game_variants(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_game_sets(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_hoppers(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_network_configuration(&hoppers_blf_path, &hoppers_config_path)?;
                Ok(())
            }();

            if result.is_err() {
                println!("{}", "Failed to build title storage config for hoppers".bright_white().on_red());
                println!("{}", result.err().unwrap().to_string().on_red());
            }
        }
    }
}

impl v_untracked_ares {
    fn build_config_banhammer_messages(hoppers_blf_path: &String, hoppers_config_path: &String) -> BLFLibResult {
        let mut task = console_task::start("Converting Banhammer Messages");

        for language_code in k_language_suffixes {
            let blf_file_path = title_storage_output::banhammer_messages_file_path(
                hoppers_blf_path,
                language_code
            );

            if !exists(&blf_file_path)? {
                task.add_warning(format!(
                    "No {} banhammer messages are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            let bhms = find_chunk_in_file::<s_blf_chunk_banhammer_messages>(blf_file_path)?;
            write_text_file_lines(
                title_storage_config::banhammer_messages_file_path(
                    hoppers_config_path,
                    language_code
                ),
                &bhms.get_messages()?
            )?;
        }

        やった!(task)
    }

    fn build_config_matchmaking_tips(hoppers_blf_path: &String, hoppers_config_path: &String) -> BLFLibResult {
        let mut task = console_task::start("Converting Matchmaking Tips");

        for language_code in k_language_suffixes {
            let blf_file_path = title_storage_output::matchmaking_tips_file_path(
                hoppers_blf_path,
                language_code,
            );

            if !exists(&blf_file_path)? {
                task.add_warning(format!(
                    "No {} matchmaking tips are present.",
                    get_language_string(language_code),
                ));

                continue;
            }

            let mmtp = find_chunk_in_file::<s_blf_chunk_matchmaking_tips>(blf_file_path)?;

            write_text_file_lines(
                title_storage_config::matchmaking_tips_file_path(
                    hoppers_config_path,
                    language_code,
                ),
                &mmtp.tips.iter().map(|tip|tip.get_string()).collect::<Result<Vec<String>, BLFLibError>>()?
            )?
        }

        やった!(task)
    }

    fn build_config_motds(hoppers_blf_path: &String, hoppers_config_path: &String) -> BLFLibResult {
        for blue in [false, true] {
            let mut task = console_task::start(
                if blue { "Converting Mythic MOTDs" } else { "Converting MOTDs" }
            );

            // BLFs
            for language_code in k_language_suffixes {
                let blf_file_path = title_storage_output::motd_file_path(
                    hoppers_blf_path,
                    language_code,
                    blue
                );

                if !exists(&blf_file_path)? {
                    task.add_warning(format!(
                        "No {} MOTD is present.",
                        get_language_string(language_code),
                    ));

                    continue;
                }

                let motd = find_chunk_in_file::<s_blf_chunk_message_of_the_day>(
                    title_storage_output::motd_file_path(
                        hoppers_blf_path,
                        language_code,
                        blue,
                    )
                )?;

                write_text_file(
                    title_storage_config::motd_file_path(
                        hoppers_config_path,
                        language_code,
                        blue,
                    ),
                    motd.get_message()
                )?;

                let jpg_file_path = title_storage_output::motd_image_file_path(
                    hoppers_blf_path,
                    language_code,
                    blue
                );

                let output_path = title_storage_config::motd_image_file_path(
                    hoppers_config_path,
                    language_code,
                    blue
                );

                if !exists(&jpg_file_path)? {
                    task.add_warning(format!(
                        "No {} MOTD image is present.",
                        get_language_string(language_code),
                    ));

                    continue;
                }

                fs::copy(jpg_file_path, output_path)?;
            }

            task.complete();
        }

        やった!()

    }

    fn build_config_motd_popups(hoppers_blf_path: &String, hoppers_config_path: &String) -> BLFLibResult {
        for blue in [false, true] {
            let mut task = console_task::start(
                if blue { "Converting Mythic MOTD Popups" } else { "Converting MOTD Popups" }
            );

            // BLFs
            for language_code in k_language_suffixes {
                let blf_file_path = title_storage_output::motd_popup_file_path(
                    hoppers_blf_path,
                    language_code,
                    blue
                );

                if !exists(&blf_file_path)? {
                    task.add_warning(format!(
                        "No {} MOTD Popup is present.",
                        get_language_string(language_code),
                    ));

                    continue;
                }

                write_json_file(
                    &find_chunk_in_file::<s_blf_chunk_message_of_the_day_popup>(blf_file_path)?,
                    title_storage_config::motd_popup_file_path(
                        hoppers_config_path,
                        language_code,
                        blue
                    )
                )?;

                let image_path = title_storage_output::motd_popup_image_file_path(
                    hoppers_blf_path,
                    language_code,
                    blue
                );

                if exists(&image_path)? {
                    fs::copy(&image_path, title_storage_config::motd_popup_image_file_path(
                        hoppers_config_path,
                        language_code,
                        blue
                    ))?;
                }
                else {
                    task.add_warning(format!("No image was found for {} Popup", language_code));
                }
            }

            task.complete();
        }

        やった!()
    }

    fn build_config_map_variants(hoppers_blf_path: &String, hoppers_config_path: &String) -> BLFLibResult {
        let mut task = console_task::start("Converting Map Variants");

        // Iterate through hopper folders. eg default_hoppers/00101
        let hopper_directory_subfolders = get_directories_in_folder(hoppers_blf_path)?;

        create_dir_all(build_path!(
            hoppers_config_path,
            title_storage_config::map_variants_folder_name
        ))?;

        // Keep track of maps we've converted to avoid duplication between different hoppers.
        let mut converted_maps = Vec::<String>::new();

        for subfolder in hopper_directory_subfolders {
            if !hopper_folder_regex.is_match(&subfolder) {
                continue;
            }

            let hopper_id = title_storage_config::get_hopper_id_from_hopper_folder_name(&subfolder)?;

            let map_variant_blfs_folder = build_path!(
                hoppers_blf_path,
                &subfolder,
                title_storage_output::map_variants_folder_name
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

                let map_variant_blf_file_path = title_storage_output::map_variant_file_path(
                    hoppers_blf_path,
                    hopper_id,
                    &map_variant_file_name,
                );

                let map_variant_config_file_path = title_storage_config::map_variant_file_path(
                    hoppers_config_path,
                    &map_variant_file_name,
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

                let mvar = find_chunk_in_file::<s_blf_chunk_packed_map_variant>(
                    &map_variant_blf_file_path,
                )?;

                write_json_file(&mvar.map_variant, map_variant_config_file_path)?;
            }
        }

        task.add_message(format!("Converted {} map variants.", converted_maps.len()));

        やった!(task)
    }

    fn build_config_game_variants(hoppers_blf_path: &String, hoppers_config_path: &String) -> BLFLibResult {
        let mut task = console_task::start("Converting Game Variants");

        // Iterate through hopper folders. eg default_hoppers/00101
        let hopper_directory_subfolders = get_directories_in_folder(hoppers_blf_path)?;

        // Keep track of games we've converted to avoid duplication between different hoppers.
        let mut converted_games = Vec::<String>::new();

        for subfolder in hopper_directory_subfolders {
            if !hopper_folder_regex.is_match(&subfolder) {
                continue;
            }

            let hopper_id = get_hopper_id_from_hopper_folder_name(&subfolder)?;

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

                let game_variant_blf_file_path = title_storage_output::game_variant_file_path(
                    hoppers_blf_path,
                    hopper_id,
                    &game_variant_file_name,
                );

                let game_variant_config_file_path = title_storage_config::game_variant_file_path(
                    hoppers_config_path,
                    &game_variant_file_name
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

                let gvar = find_chunk_in_file::<s_blf_chunk_packed_game_variant>(game_variant_blf_file_path)?;

                write_json_file(&gvar.game_variant, game_variant_config_file_path)?;
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

            let hopper_id = get_hopper_id_from_hopper_folder_name(&hopper_folder)?;

            let game_set_blf_path = title_storage_output::game_set_file_path(
                hoppers_blf_path,
                hopper_id,
            );

            if !exists(&game_set_blf_path).unwrap() {
                task.add_warning(format!("No game set was found for hopper \"{hopper_folder}\""));
                continue;
            }

            let gset = find_chunk_in_file::<s_blf_chunk_game_set>(&game_set_blf_path)?;
            let game_set_config_path = title_storage_config::game_set_file_path(
                hoppers_config_path,
                &hopper_folder,
            );

            let mut writer = WriterBuilder::new().from_writer(vec![]);

            for game_set_entry in gset.get_entries() {
                writer.serialize(title_storage_config::game_set_config_row {
                    map_variant_file_name: game_set_entry.map_variant_file_name.get_string()?,
                    game_variant_file_name: game_set_entry.game_variant_file_name.get_string()?,
                    weight: game_set_entry.weight,
                    minimum_player_count: game_set_entry.minimum_player_count,
                    skip_after_veto: game_set_entry.skip_after_veto,
                    optional: game_set_entry.optional,
                })?
            }

            create_parent_folders(&game_set_config_path)?;

            let mut config_file = File::create(game_set_config_path)?;
            config_file.write_all(&writer.into_inner()?)?;
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
            let hopper_descriptions_path = title_storage_output::hopper_descriptions_file_path(
                hoppers_blfs_folder,
                language_code
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
        let mut task = console_task::start("Converting Hopper Configuration");

        let language_hopper_descriptions
            = Self::read_hopper_description_blfs(hoppers_blfs_path, &mut task)?;

        let hopper_configuration_blf_path = title_storage_output::matchmaking_hopper_file_path(hoppers_blfs_path);

        let hopper_configuration_table = find_chunk_in_file::<s_blf_chunk_hopper_configuration_table>(&hopper_configuration_blf_path).unwrap();
        let hopper_configurations = hopper_configuration_table.get_hopper_configurations();
        let category_configurations = hopper_configuration_table.get_hopper_categories();

        // Generate active_hoppers.txt
        let active_hopper_ids = hopper_configurations.iter().map(|config|config.hopper_identifier);
        let active_hoppers_txt_path = title_storage_config::active_hoppers_file_path(
            hoppers_config_path,
        );

        write_text_file_lines(
            active_hoppers_txt_path,
            &active_hopper_ids.map(|id|format!("{id:0>5}")).collect(),
        )?;

        // Build hopper configuration json
        for hopper_configuration in hopper_configurations {
            let mut hopper_configuration_json = title_storage_config::matchmaking_hopper {
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

            write_json_file(&hopper_configuration_json, title_storage_config::matchmaking_hopper_configuration_file_path(
                hoppers_config_path,
                &format!("{:0>5}", hopper_configuration_json.configuration.hopper_identifier),
            ))?;
        }

        // Build categories json
        let mut categories_config = title_storage_config::matchmaking_hopper_categories::default();

        for category_configuration in category_configurations {
            let mut category_configuration_and_description = title_storage_config::matchmaking_hopper_category_configuration_and_descriptions {
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

        write_json_file(&categories_config, title_storage_config::matchmaking_hopper_categories_file_path(hoppers_config_path))?;

        task.add_message(format!("Converted {} hopper configurations.", hopper_configuration_table.hopper_configuration_count()));

        やった!(task)
    }

    fn build_config_network_configuration(hoppers_blfs_path: &String, hoppers_config_path: &String) -> BLFLibResult {
        // For now we just copy it as is. But we do check that it contains a netc.
        let mut task = console_task::start("Converting Network Configuration");

        let netc = find_chunk_in_file::<s_blf_chunk_network_configuration>(
            title_storage_output::network_configuration_file_path(hoppers_blfs_path)
        )?;

        write_json_file(&netc.config, title_storage_config::network_configuration_file_path(hoppers_config_path))?;

        やった!(task)
    }

    fn build_blf_banhammer_messages(hoppers_config_folder: &String, hoppers_blf_folder: &String) -> BLFLibResult {
        let mut task = console_task::start("Building Banhammer Messages");

        for language_code in k_language_suffixes {
            let config_path = title_storage_config::banhammer_messages_file_path(hoppers_config_folder, language_code);

            if !exists(&config_path)? {
                task.add_warning(format!("{} banhammer messages are missing.", get_language_string(language_code)));
                continue;
            }

            let matchmaking_banhammer_messages = read_text_file_lines(
                config_path,
            )?;

            let bhms = s_blf_chunk_banhammer_messages::create(matchmaking_banhammer_messages)?;

            BlfFileBuilder::new()
                .add_chunk(s_blf_chunk_start_of_file::default())
                .add_chunk(s_blf_chunk_author::for_build::<v_untracked_ares>())
                .add_chunk(bhms)
                .add_chunk(s_blf_chunk_end_of_file::default())
                .write_file(title_storage_output::banhammer_messages_file_path(hoppers_blf_folder, language_code))?;
        }

        やった!(task)
    }

    fn build_blf_matchmaking_tips(hoppers_config_folder: &String, hoppers_blf_folder: &String) -> BLFLibResult {
        let mut task = console_task::start("Building Matchmaking Tips");

        for language_code in k_language_suffixes {
            let config_path = title_storage_config::matchmaking_tips_file_path(hoppers_config_folder, language_code);

            if !exists(&config_path)? {
                task.add_warning(format!("{} matchmaking tips are missing.", get_language_string(language_code)));
                continue;
            }

            let matchmaking_tips = read_text_file_lines(config_path)?;

            BlfFileBuilder::new()
                .add_chunk(s_blf_chunk_start_of_file::default())
                .add_chunk(s_blf_chunk_author::for_build::<v_untracked_ares>())
                .add_chunk(s_blf_chunk_matchmaking_tips::create(matchmaking_tips)?)
                .add_chunk(s_blf_chunk_end_of_file::default())
                .write_file(title_storage_output::matchmaking_tips_file_path(
                    hoppers_blf_folder,
                    language_code
                ))?
        }

        やった!(task)
    }

    fn build_blf_motds(
        hoppers_config_folder: &String,
        hoppers_blf_folder: &String,
    ) -> BLFLibResult
    {
        for blue in [false, true] {
            let mut task = console_task::start(
                if blue { "Building Mythic MOTDs" } else { "Building MOTDs" }
            );

            for language_code in k_language_suffixes {
                let motd_config_path = title_storage_config::motd_file_path(
                    hoppers_config_folder,
                    language_code,
                    blue
                );

                if !exists(&motd_config_path)? {
                    task.add_warning(format!(
                        "No {} MOTD is present.",
                        get_language_string(language_code),
                    ));
                    continue;
                }

                let motd = s_blf_chunk_message_of_the_day::new(read_file_to_string(
                    &motd_config_path
                )?);

                BlfFileBuilder::new()
                    .add_chunk(s_blf_chunk_start_of_file::default())
                    .add_chunk(s_blf_chunk_author::for_build::<v_untracked_ares>())
                    .add_chunk(motd)
                    .add_chunk(s_blf_chunk_end_of_file::default())
                    .write_file(title_storage_output::motd_file_path(hoppers_blf_folder, language_code, blue))?;

                // copy images.
                let image_source = title_storage_config::motd_image_file_path(
                    hoppers_config_folder,
                    language_code,
                    blue
                );

                let image_valid = validate_jpeg(
                    &image_source,
                    title_storage_output::motd_image_width,
                    title_storage_output::motd_image_height,
                    Some(title_storage_output::motd_image_max_size)
                );

                if image_valid.is_err() {
                    task.add_warning(format!(
                        "{} MOTD has an invalid Image: {}",
                        get_language_string(language_code),
                        image_valid.unwrap_err()
                    ));

                    continue;
                }

                fs::copy(image_source, title_storage_output::motd_image_file_path(
                    hoppers_blf_folder,
                    language_code,
                    blue
                ))?;
            }

            task.complete();
        }
        Ok(())
    }

    fn build_blf_motd_popups(
        hoppers_config_folder: &String,
        hoppers_blf_folder: &String,
    ) -> BLFLibResult {
        for blue in [false, true] {
            let mut task = console_task::start(
                if blue { "Building Mythic MOTD Popups" } else { "Building MOTD Popups" }
            );

            for language_code in k_language_suffixes {
                let motd_popup_config_path = title_storage_config::motd_popup_file_path(
                    hoppers_config_folder,
                    language_code,
                    blue
                );

                if !exists(&motd_popup_config_path)? {
                    task.add_warning(format!(
                        "No {} MOTD Popup is present.",
                        get_language_string(language_code),
                    ));
                    continue;
                }

                let mtdp = read_json_file::<s_blf_chunk_message_of_the_day_popup>(
                    &motd_popup_config_path
                )?;

                BlfFileBuilder::new()
                    .add_chunk(s_blf_chunk_start_of_file::default())
                    .add_chunk(s_blf_chunk_author::for_build::<v_untracked_ares>())
                    .add_chunk(mtdp)
                    .add_chunk(s_blf_chunk_end_of_file::default())
                    .write_file(title_storage_output::motd_popup_file_path(hoppers_blf_folder, language_code, blue))?;

                // copy images.
                let image_source = title_storage_config::motd_popup_image_file_path(
                    hoppers_config_folder,
                    language_code,
                    blue
                );

                let image_valid = validate_jpeg(
                    &image_source,
                    title_storage_output::motd_popup_image_width,
                    title_storage_output::motd_popup_image_height,
                    Some(title_storage_output::motd_popup_image_max_size)
                );

                if image_valid.is_err() {
                    task.add_warning(format!(
                        "{} MOTD Popup has an invalid Image: {}",
                        get_language_string(language_code),
                        image_valid.unwrap_err()
                    ));

                    continue;
                }

                fs::copy(image_source, title_storage_output::motd_popup_image_file_path(
                    hoppers_blf_folder,
                    language_code,
                    blue
                ))?;
            }

            task.complete();
        }
        Ok(())
    }

    fn build_blf_map_manifest(hoppers_config_path: &String, hoppers_blf_path: &String) -> BLFLibResult
    {
        let mut task = console_task::start("Building Map Manifest");

        let rsa_folder = title_storage_config::rsa_signatures_folder_path(
            hoppers_config_path,
        );

        let mut rsa_files = Vec::<String>::new();

        if exists(&rsa_folder)? {
            rsa_files = get_files_in_folder(&rsa_folder)?;
        }

        if rsa_files.is_empty() {
            task.add_error(format!("No RSA signatures were found"))
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
            .add_chunk(s_blf_chunk_start_of_file::new("rsa manifest"))
            .add_chunk(s_blf_chunk_author::for_build::<v_untracked_ares>())
            .add_chunk(map_manifest)
            .add_chunk(s_blf_chunk_end_of_file::default())
            .write_file(title_storage_output::rsa_manifest_file_path(
                hoppers_blf_path,
            ))?;

        やった!(task)
    }

    fn read_active_hopper_configuration(hoppers_config_path: &String) -> BLFLibResult<Vec<String>> {
        let mut task = console_task::start("Reading Active Hoppers");

        let active_hoppers_file_path = title_storage_config::active_hoppers_file_path(hoppers_config_path);

        let active_hoppers_file = File::open(&active_hoppers_file_path);
        if active_hoppers_file.is_err() {
            return Err(BLFLibError::from(active_hoppers_file.unwrap_err().to_string()));
        }

        let mut active_hoppers_file = active_hoppers_file.unwrap();
        let mut active_hoppers_string = String::new();
        let read_result = active_hoppers_file.read_to_string(&mut active_hoppers_string);
        if read_result.is_err() {
            return Err(BLFLibError::from(read_result.unwrap_err().to_string()));
        }

        let active_hopper_folders = active_hoppers_string.lines();

        task.complete();

        Ok(active_hopper_folders.map(String::from).collect::<Vec<String>>())
    }

    fn read_game_set_configuration(hoppers_config_path: &String, active_hopper_folders: &Vec<String>) -> BLFLibResult<HashMap<u16, title_storage_config::game_set_config>>
    {
        let mut task = console_task::start("Reading Game Set Config");

        let mut game_sets = HashMap::<u16, title_storage_config::game_set_config>::new();

        for subfolder in active_hopper_folders {
            let hopper_id = get_hopper_id_from_hopper_folder_name(&subfolder)?;

            let game_set_csv_path = title_storage_config::game_set_file_path(
                hoppers_config_path,
                subfolder,
            );

            if !exists(&game_set_csv_path).unwrap() {
                task.fail_with_error(format!("No game set was found for hopper \"{subfolder}\""));
                panic!();
            }

            let mut reader = ReaderBuilder::new().from_path(&game_set_csv_path)?;
            let mut rows = Vec::<title_storage_config::game_set_config_row>::new();
            for row in reader.deserialize() {
                if let Ok(row) = row {
                    let row: title_storage_config::game_set_config_row = row;
                    rows.push(row);
                } else {
                    return Err(format!("Failed to parse game set CSV: {game_set_csv_path}").into());
                }
            }

            game_sets.insert(hopper_id, title_storage_config::game_set_config { entries: rows });
        }

        task.complete();

        Ok(game_sets)
    }

    fn build_blf_game_variants(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
        build_temp_dir: &String,
        game_sets: &HashMap<u16, title_storage_config::game_set_config>,
        variant_hashes: &mut HashMap<String, s_network_http_request_hash>
    )
    {
        let mut task = console_task::start("Building Game Variants");

        let game_variants_temp_build_path = build_path!(
            build_temp_dir,
            title_storage_config::game_variants_folder_name
        );

        create_dir_all(&game_variants_temp_build_path).unwrap();

        let game_variants_to_convert: Vec<String> = game_sets.iter().flat_map(|(_, game_set)|
            game_set.entries.iter().map(|entry|entry.game_variant_file_name.clone()).collect::<Vec<String>>()
        ).collect();

        let game_variants_to_convert: HashSet<String> = HashSet::from_iter(game_variants_to_convert.iter().cloned());

        let mut json_queue: Vec<(String, String)> = Vec::new();
        for game_variant in game_variants_to_convert {
            let game_variant_json_path = title_storage_config::game_variant_file_path(hoppers_config_path, &game_variant);

            if !Path::new(&game_variant_json_path).exists() {
                task.fail_with_error(format!("Game variant \"{}\" could not be found.", game_variant));
                panic!();
            }

            let mut file = File::open(&game_variant_json_path).unwrap();
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

                            BlfFileBuilder::new()
                                .add_chunk(s_blf_chunk_start_of_file::new("game var"))
                                .add_chunk(s_blf_chunk_author::for_build::<v_untracked_ares>())
                                .add_chunk(s_blf_chunk_packed_game_variant::create(game_variant_json))
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

    pub fn get_scenario_rsa_crc32s(hoppers_config_path: &String) -> HashMap<u32, u32> {
        let mut result = HashMap::<u32, u32>::new();

        let rsa_folder = title_storage_config::rsa_signatures_folder_path(
            hoppers_config_path,
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
        game_sets: &HashMap<u16, title_storage_config::game_set_config>,
        variant_hashes: &mut HashMap<String, s_network_http_request_hash>,
        variant_map_ids: &mut HashMap<String, u32>
    )
    {
        let mut task = console_task::start("Building Map Variants");

        let scenario_crc32s = Arc::new(Self::get_scenario_rsa_crc32s(hoppers_config_path));

        let map_variants_temp_build_path = build_path!(
            build_temp_dir,
            title_storage_config::map_variants_folder_name
        );

        create_dir_all(&map_variants_temp_build_path).unwrap();

        let map_variants_to_convert: Vec<String> = game_sets.iter().flat_map(|(_, game_set)|
            game_set.entries.iter().map(|entry| entry.map_variant_file_name.clone()).collect::<Vec<String>>()
        ).collect();
        let map_variants_to_convert: HashSet<String> = HashSet::from_iter(map_variants_to_convert.iter().cloned());

        let mut json_queue: Vec<(String, String)> = Vec::new();
        for map_variant in map_variants_to_convert {
            let map_variant_json_path = title_storage_config::map_variant_file_path(
                hoppers_config_path,
                &map_variant,
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

                            BlfFileBuilder::new()
                                .add_chunk(s_blf_chunk_start_of_file::default())
                                .add_chunk(s_blf_chunk_packed_map_variant {
                                    map_variant: map_variant_json
                                })
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

            let final_map_ids = shared_variant_map_ids.lock().await;
            variant_map_ids.extend(final_map_ids.clone());

            let mut task = task.lock().await;
            task.add_message(format!("Built {} variants.", variant_hashes.len()));
            task.complete();
        });
    }

    fn build_blf_game_sets(
        hoppers_blf_path: &String,
        active_game_sets: HashMap<u16, title_storage_config::game_set_config>,
        game_variant_hashes: &HashMap<String, s_network_http_request_hash>,
        map_variant_hashes: &HashMap<String, s_network_http_request_hash>,
        map_variant_map_ids: &HashMap<String, u32>,
        build_temp_dir_path: &String,
    ) -> BLFLibResult
    {
        let mut task = console_task::start("Building Game Sets");

        for (hopper_id, game_set_config) in active_game_sets {
            let game_variants_temp_build_path = build_path!(
                build_temp_dir_path,
                title_storage_config::game_variants_folder_name
            );

            let map_variants_temp_build_path = build_path!(
                build_temp_dir_path,
                title_storage_config::map_variants_folder_name
            );

            let copied_maps = HashSet::<String>::new();
            let copied_games = HashSet::<String>::new();

            for game_set_row in &game_set_config.entries {
                // Copy the game and map variants over...
                if !copied_games.contains(&game_set_row.game_variant_file_name) {
                    let game_variant_file_name = &game_set_row.game_variant_file_name;
                    let game_variant_dst_path = title_storage_output::game_variant_file_path(
                        hoppers_blf_path,
                        hopper_id,
                        &game_set_row.game_variant_file_name,
                    );
                    create_parent_folders(&game_variant_dst_path)?;

                    fs::copy(
                        build_path!(
                            &game_variants_temp_build_path,
                            format!(
                                "{game_variant_file_name}_{:0>3}.bin",
                                s_blf_chunk_packed_game_variant::get_version().major
                            )
                        ),
                        game_variant_dst_path,
                    )?;
                }

                if !copied_maps.contains(&game_set_row.map_variant_file_name) {
                    let map_variant_file_name = &game_set_row.map_variant_file_name;

                    let map_variant_dst_path = title_storage_output::map_variant_file_path(
                        hoppers_blf_path,
                        hopper_id,
                        &map_variant_file_name
                    );
                    create_parent_folders(&map_variant_dst_path)?;

                    fs::copy(
                        build_path!(
                            &map_variants_temp_build_path,
                            format!(
                                "{map_variant_file_name}_{:0>3}.bin",
                                s_blf_chunk_packed_map_variant::get_version().major
                            )
                        ),
                        map_variant_dst_path,
                    )?;
                }
            }

            let mut gset = s_blf_chunk_game_set::default();
            for row in game_set_config.entries.iter() {
                gset.add_entry(s_blf_chunk_game_set_entry {
                    map_variant_file_name: StaticString::from_string(&row.map_variant_file_name)?,
                    game_variant_file_name: StaticString::from_string(&row.game_variant_file_name)?,
                    weight: row.weight,
                    minimum_player_count: row.minimum_player_count,
                    skip_after_veto: row.skip_after_veto,
                    optional: row.optional,

                    map_variant_file_hash: *map_variant_hashes.get(&row.map_variant_file_name)
                        .unwrap_or_else(|| panic!("No map variant hash found for {}", row.map_variant_file_name)),
                    game_variant_file_hash: *game_variant_hashes.get(&row.game_variant_file_name)
                        .unwrap_or_else(|| panic!("No map variant hash found for {}", row.game_variant_file_name)),
                    map_id: *map_variant_map_ids.get(&row.map_variant_file_name)
                        .unwrap_or_else(|| panic!("No map ID found for {}", row.map_variant_file_name)),
                })?;
            }

            BlfFileBuilder::new()
                .add_chunk(s_blf_chunk_start_of_file::new("game set"))
                .add_chunk(s_blf_chunk_author::for_build::<v_untracked_ares>())
                .add_chunk(gset)
                .add_chunk(s_blf_chunk_end_of_file::default())
                .write_file(title_storage_output::game_set_file_path(
                    hoppers_blf_path,
                    hopper_id,
                ))?;
        }

        やった!(task)
    }

    fn build_blf_hoppers(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
        active_hopper_folders: &Vec<String>,
    ) -> BLFLibResult
    {
        let mut task = console_task::start("Building Hopper Configuration");

        let mut mhcf = s_blf_chunk_hopper_configuration_table::default();

        // Load the configuration.json files for each hopper
        let mut hopper_configuration_jsons = Vec::<(u16, title_storage_config::matchmaking_hopper)>::new();
        for active_hopper_folder in active_hopper_folders {
            let configuration_path = title_storage_config::matchmaking_hopper_configuration_file_path(
                hoppers_config_path,
                active_hopper_folder,
            );

            if !exists(&configuration_path)? {
                task.fail_with_error(format!("Couldn't find a configuration file for hopper {active_hopper_folder}!"));
                return Err(BLFLibError::from("Failed to build hoppers."));
            }

            let mut configuration_file = File::open(&configuration_path).unwrap();

            let hopper_id = get_hopper_id_from_hopper_folder_name(active_hopper_folder)?;

            hopper_configuration_jsons.push((
                hopper_id,
                serde_json::from_reader(&mut configuration_file)?
            ));
        }

        for (hopper_identifier, hopper_configuration_json) in &hopper_configuration_jsons {
            let mut hopper_config = hopper_configuration_json.configuration.clone();
            let game_set_blf_file_path = title_storage_output::game_set_file_path(
                hoppers_blfs_path,
                *hopper_identifier,
            );
            hopper_config.game_set_hash = s_network_http_request_hash::default();
            mhcf.add_hopper_configuration(hopper_config)?
        }

        // Load category configuration
        let categories_configuration = read_json_file::<title_storage_config::matchmaking_hopper_categories>(
            title_storage_config::matchmaking_hopper_categories_file_path(hoppers_config_path)
        )?;

        let active_hopper_categories = mhcf
            .get_hopper_configurations()
            .iter().map(|hopper|hopper.hopper_category)
            .collect::<HashSet<_>>();
        let active_hopper_category_configurations = categories_configuration.categories
            .iter().filter(|category_configuration|active_hopper_categories.contains(&category_configuration.configuration.category_identifier))
            .cloned()
            .collect::<Vec<matchmaking_hopper_category_configuration_and_descriptions>>();

        for active_hopper_category in &active_hopper_category_configurations {
            mhcf.add_category_configuration(active_hopper_category.configuration)?;
        }

        // Initialize language_hopper_descriptions
        for language_code in k_language_suffixes {
            let mut mhdf = s_blf_chunk_hopper_description_table::default();

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

                mhdf.add_description((
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

                mhdf.add_description((
                    active_hopper_category.configuration.category_identifier,
                    description
                ))?;
            }

            // Write description file
            let descriptions_blf_path =title_storage_output::hopper_descriptions_file_path(
                hoppers_blfs_path,
                language_code,
            );

            BlfFileBuilder::new()
                .add_chunk(s_blf_chunk_start_of_file::default())
                .add_chunk(s_blf_chunk_author::for_build::<v_untracked_ares>())
                .add_chunk(mhdf)
                .add_chunk(s_blf_chunk_end_of_file::default())
                .write_file(title_storage_output::hopper_descriptions_file_path(
                    hoppers_blfs_path,
                    language_code,
                ))?;
        }

        BlfFileBuilder::new()
            .add_chunk(s_blf_chunk_start_of_file::new("hopper config"))
            .add_chunk(s_blf_chunk_author::for_build::<v_untracked_ares>())
            .add_chunk(mhcf)
            .add_chunk(s_blf_chunk_end_of_file::default())
            .write_file(title_storage_output::matchmaking_hopper_file_path(
                hoppers_blfs_path,
            ))?;

        やった!(task)
    }

    fn build_blf_network_configuration(
        hoppers_config_path: &String,
        hoppers_blfs_path: &String,
    ) -> BLFLibResult {
        let mut task = console_task::start("Building Network Configuration");

        let netc = s_blf_chunk_network_configuration {
            config: read_json_file(
                title_storage_config::network_configuration_file_path(hoppers_config_path)
            )?,
        };

        BlfFileBuilder::new()
            .add_chunk(s_blf_chunk_start_of_file::new("ares net config"))
            .add_chunk(s_blf_chunk_author::for_build::<v_untracked_ares>())
            .add_chunk(netc)
            .add_chunk(s_blf_chunk_end_of_file::default())
            .write_file(title_storage_output::network_configuration_file_path(
                hoppers_blfs_path,
            ))?;

        やった!(task)
    }

    fn build_blf_manifest(
        hoppers_blfs_path: &String,
    ) -> BLFLibResult {
        let mut task = console_task::start("Building Manifest File");

        let mut manifest_chunk = s_blf_chunk_online_file_manifest::default();
        let hopper_directory_name = Path::new(hoppers_blfs_path).file_name().unwrap().to_str().unwrap();

        let mut add_hash_if_file_exists = |manifest_path: String, file_path: String| -> BLFLibResult {
            if exists(&file_path)? {
                manifest_chunk.add_file_hash(
                    manifest_path,
                    s_network_http_request_hash::default(),
                )?;
            }
            Ok(())
        };

        add_hash_if_file_exists(
            format!(
                "/title/{hopper_directory_name}/{}",
                title_storage_output::matchmaking_hopper_file_name()
            ),
            title_storage_output::matchmaking_hopper_file_path(hoppers_blfs_path)
        )?;

        add_hash_if_file_exists(
            format!(
                "/title/{hopper_directory_name}/{}",
                title_storage_output::network_configuration_file_name()
            ),
            title_storage_output::network_configuration_file_path(hoppers_blfs_path)
        )?;

        add_hash_if_file_exists(
            format!(
                "/title/{hopper_directory_name}/{}",
                title_storage_output::rsa_manifest_file_name
            ),
            title_storage_output::rsa_manifest_file_path(hoppers_blfs_path)
        )?;

        for language_code in k_language_suffixes {
            add_hash_if_file_exists(
                format!(
                    "/title/{hopper_directory_name}/{language_code}/{}",
                    title_storage_output::banhammer_messages_file_name
                ),
                title_storage_output::banhammer_messages_file_path(hoppers_blfs_path, language_code)
            )?;

            add_hash_if_file_exists(
                format!(
                    "/title/{hopper_directory_name}/{language_code}/{}",
                    title_storage_output::hopper_descriptions_file_name()
                ),
                title_storage_output::hopper_descriptions_file_path(hoppers_blfs_path, language_code)
            )?;

            add_hash_if_file_exists(
                format!(
                    "/title/{hopper_directory_name}/{language_code}/{}",
                    title_storage_output::matchmaking_tips_file_name,
                ),
                title_storage_output::matchmaking_tips_file_path(hoppers_blfs_path, language_code)
            )?;
        }

        BlfFileBuilder::new()
            .add_chunk(s_blf_chunk_start_of_file::default())
            .add_chunk(manifest_chunk)
            // OG omaha manifests have an RSA _eof, but we're skipping that
            .add_chunk(s_blf_chunk_end_of_file::default())
            .write_file(title_storage_output::manifest_file_path(
                hoppers_blfs_path,
            ))?;

        やった!(task)
    }
}