use std::error::Error;
use std::fs;
use std::fs::{exists, File};
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;
use crate::io::{get_directories_in_folder, get_files_in_folder, read_text_file_lines, write_text_file, write_text_file_lines, FILE_SEPARATOR};
use crate::{build_path, debug_log, title_converter, やった};
use crate::title_storage::{check_file_exists, validate_jpeg, TitleConverter};
use inline_colorization::*;
use lazy_static::lazy_static;
use blf_lib::blam::halo3::release::cseries::language::{get_language_string, k_language_suffix_chinese_traditional, k_language_suffix_english, k_language_suffix_french, k_language_suffix_german, k_language_suffix_italian, k_language_suffix_japanese, k_language_suffix_korean, k_language_suffix_mexican, k_language_suffix_portuguese, k_language_suffix_spanish};
use blf_lib::blf::{get_blf_file_hash, BlfFile, BlfFileBuilder};
use blf_lib::blf::chunks::find_chunk_in_file;
use crate::console::console_task;
use regex::Regex;
use blf_lib::blf::versions::ares::v_untracked_ares::{s_blf_chunk_end_of_file, s_blf_chunk_map_manifest, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::halo3odst::v13895_09_04_27_2201_atlas_release::{s_blf_chunk_banhammer_messages, s_blf_chunk_message_of_the_day, s_blf_chunk_message_of_the_day_popup, s_blf_chunk_online_file_manifest, s_blf_chunk_network_configuration};
use blf_lib::blf::versions::haloreach::v09730_10_04_09_1309_omaha_delta::s_blf_chunk_author;
use blf_lib::io::{read_file_to_string, read_json_file, write_json_file};
use blf_lib::result::BLFLibResult;
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::v12070_08_09_05_2031_halo3_ship;
use crate::title_storage::halo3odst::v13895_09_04_27_2201_atlas_release::title_storage_output::hopper_directory_name_max_length;

title_converter! (
    #[Title("Halo 3: ODST")]
    #[Build("13895.09.04.27.2201.atlas_release")]
    pub struct v13895_09_04_27_2201_atlas_release {}
);

// Halo 3: ODST's xex supports 12 languages, but only 10 were released.
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
    static ref config_rsa_signature_file_map_id_regex: Regex = Regex::new(r"^[0-9]{1,}").unwrap();
}

mod title_storage_output {
    use blf_lib::blf::chunks::BlfChunk;
    use blf_lib::blf::versions::halo3odst::v13895_09_04_27_2201_atlas_release::{s_blf_chunk_network_configuration, s_blf_chunk_online_file_manifest};
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

    pub const motd_file_name: &str = "black_motd.bin";
    pub fn motd_file_path(hoppers_path: &String, language_code: &str) -> String {
        build_path!(
            hoppers_path,
            language_code,
            motd_file_name
        )
    }

    pub const motd_image_file_name: &str = "black_motd_image.jpg";
    pub fn motd_image_file_path(hoppers_path: &String, language_code: &str) -> String {
        build_path!(
            hoppers_path,
            language_code,
            motd_image_file_name
        )
    }

    pub fn motd_popup_file_name(vidmaster: bool) -> String {
        if vidmaster { "blue_vidmaster_popup.bin".to_string() }
        else { "black_motd_popup.bin".to_string() }
    }
    pub fn motd_popup_file_path(hoppers_path: &String, language_code: &str, vidmaster: bool) -> String {
        build_path!(
            hoppers_path,
            language_code,
            motd_popup_file_name(vidmaster)
        )
    }
    pub fn motd_popup_image_file_name(vidmaster: bool) -> String {
        if vidmaster { "blue_vidmaster_image.jpg".to_string() }
        else { "black_motd_popup_image.jpg".to_string() }
    }
    pub fn motd_popup_image_file_path(hoppers_path: &String, language_code: &str, vidmaster: bool) -> String {
        build_path!(
            hoppers_path,
            language_code,
            motd_popup_image_file_name(vidmaster)
        )
    }
}

mod title_storage_config {
    use blf_lib::blf::chunks::BlfChunk;
    use blf_lib::blf::versions::halo3odst::v13895_09_04_27_2201_atlas_release::s_blf_chunk_network_configuration;
    use crate::build_path;

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

    pub const motd_folder_name: &str = "motd";

    pub fn motd_file_path(config_folder: &String, language_code: &str) -> String {
        build_path!(
            config_folder,
            motd_folder_name,
            format!("{language_code}.txt")
        )
    }

    pub fn motd_image_file_path(config_folder: &String, language_code: &str) -> String {
        build_path!(
            config_folder,
            motd_folder_name,
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

    pub const network_configuration_file_name: &str = "network_configuration.json";

    pub fn network_configuration_file_path(config_folder: &String) -> String {
        build_path!(
            config_folder,
            network_configuration_file_name
        )
    }
}

impl TitleConverter for v13895_09_04_27_2201_atlas_release {
    fn build_blfs(&mut self, config_path: &String, blfs_path: &String) {
        let start_time = SystemTime::now();

        println!("{style_bold}Writing Title Storage BLFs to {blfs_path} {style_reset}");

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
                Self::build_blf_motds(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_motd_popups(&hopper_config_path, &hopper_blfs_path)?;
                Self::build_blf_map_manifest(&hopper_config_path, &hopper_blfs_path)?;
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

                println!("{style_bold}Converting {color_bright_white}{}{style_reset}...", hopper_directory);
                Self::build_config_banhammer_messages(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_motds(&hoppers_blf_path, &hoppers_config_path)?;
                Self::build_config_popups(&hoppers_blf_path, &hoppers_config_path)?;
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

impl v13895_09_04_27_2201_atlas_release {
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

    fn build_config_motds(hoppers_blf_path: &String, hoppers_config_path: &String) -> Result<(), Box<dyn Error>> {
        let mut task = console_task::start("Converting MOTDs");

        // BLFs
        for language_code in k_language_suffixes {
            let blf_file_path = title_storage_output::motd_file_path(
                hoppers_blf_path,
                language_code,
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
                )
            )?;

            write_text_file(
                title_storage_config::motd_file_path(
                    hoppers_config_path,
                    language_code,
                ),
                motd.get_message()
            )?;

            let jpg_file_path = title_storage_output::motd_image_file_path(
                hoppers_blf_path,
                language_code,
            );

            let output_path = title_storage_config::motd_image_file_path(
                hoppers_config_path,
                language_code,
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

        やった!(task)
    }

    fn build_config_popups(hoppers_blf_path: &String, hoppers_config_path: &String) -> BLFLibResult {
        for vidmaster in [false, true] {
            let mut task = console_task::start(
                if vidmaster { "Converting Vidmaster Popups" } else { "Converting MOTD Popups" }
            );

            // BLFs
            for language_code in k_language_suffixes {
                let blf_file_path = title_storage_output::motd_popup_file_path(
                    hoppers_blf_path,
                    language_code,
                    vidmaster
                );

                if !exists(&blf_file_path)? {
                    task.add_warning(format!(
                        "No {} Popup is present.",
                        get_language_string(language_code),
                    ));

                    continue;
                }

                write_json_file(
                    &find_chunk_in_file::<s_blf_chunk_message_of_the_day_popup>(blf_file_path)?,
                    title_storage_config::motd_popup_file_path(
                        hoppers_config_path,
                        language_code,
                        vidmaster
                    )
                )?;

                let image_path = title_storage_output::motd_popup_image_file_path(
                    hoppers_blf_path,
                    language_code,
                    vidmaster
                );

                if exists(&image_path)? {
                    fs::copy(&image_path, title_storage_config::motd_popup_image_file_path(
                        hoppers_config_path,
                        language_code,
                        vidmaster
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

    fn build_config_network_configuration(hoppers_blfs_path: &String, hoppers_config_path: &String) -> BLFLibResult {
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
                .add_chunk(s_blf_chunk_author::for_build::<v13895_09_04_27_2201_atlas_release>())
                .add_chunk(bhms)
                .add_chunk(s_blf_chunk_end_of_file::default())
                .write_file(title_storage_output::banhammer_messages_file_path(hoppers_blf_folder, language_code))?;
        }

        やった!(task)
    }

    fn build_blf_motds(
        hoppers_config_folder: &String,
        hoppers_blf_folder: &String,
    ) -> BLFLibResult
    {
        let mut task = console_task::start("Building MOTDs");

        for language_code in k_language_suffixes {
            let motd_config_path = title_storage_config::motd_file_path(
                hoppers_config_folder,
                language_code,
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
                .add_chunk(s_blf_chunk_author::for_build::<v13895_09_04_27_2201_atlas_release>())
                .add_chunk(motd)
                .add_chunk(s_blf_chunk_end_of_file::default())
                .write_file(title_storage_output::motd_file_path(hoppers_blf_folder, language_code))?;

            // copy images.
            let image_source = title_storage_config::motd_image_file_path(
                hoppers_config_folder,
                language_code,
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
            ))?;
        }

        やった!(task);
    }

    fn build_blf_motd_popups(
        hoppers_config_folder: &String,
        hoppers_blf_folder: &String,
    ) -> BLFLibResult {
        for vidmaster in [false, true] {
            let mut task = console_task::start(
                if vidmaster { "Building Vidmaster Popups" } else { "Building MOTD Popups" }
            );

            for language_code in k_language_suffixes {
                let motd_popup_config_path = title_storage_config::motd_popup_file_path(
                    hoppers_config_folder,
                    language_code,
                    vidmaster
                );

                if !exists(&motd_popup_config_path)? {
                    task.add_warning(format!(
                        "No {} Popup is present.",
                        get_language_string(language_code),
                    ));
                    continue;
                }

                let mtdp = read_json_file::<s_blf_chunk_message_of_the_day_popup>(
                    &motd_popup_config_path
                )?;

                BlfFileBuilder::new()
                    .add_chunk(s_blf_chunk_start_of_file::default())
                    .add_chunk(s_blf_chunk_author::for_build::<v13895_09_04_27_2201_atlas_release>())
                    .add_chunk(mtdp)
                    .add_chunk(s_blf_chunk_end_of_file::default())
                    .write_file(title_storage_output::motd_popup_file_path(hoppers_blf_folder, language_code, vidmaster))?;

                // copy images.
                let image_source = title_storage_config::motd_popup_image_file_path(
                    hoppers_config_folder,
                    language_code,
                    vidmaster
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
                    vidmaster
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
            .add_chunk(s_blf_chunk_author::for_build::<v13895_09_04_27_2201_atlas_release>())
            .add_chunk(map_manifest)
            .add_chunk(s_blf_chunk_end_of_file::default())
            .write_file(title_storage_output::rsa_manifest_file_path(
                hoppers_blf_path,
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
            .add_chunk(s_blf_chunk_start_of_file::new("atlas net config"))
            .add_chunk(s_blf_chunk_author::for_build::<v13895_09_04_27_2201_atlas_release>())
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
                    get_blf_file_hash(file_path)?,
                )?;
            }
            Ok(())
        };

        add_hash_if_file_exists(
            format!(
                "/{}",
                title_storage_output::network_configuration_file_name()
            ),
            title_storage_output::network_configuration_file_path(hoppers_blfs_path)
        )?;

        add_hash_if_file_exists(
            format!(
                "/{}",
                title_storage_output::rsa_manifest_file_name
            ),
            title_storage_output::rsa_manifest_file_path(hoppers_blfs_path)
        )?;

        for language_code in k_language_suffixes {
            add_hash_if_file_exists(
                format!(
                    "/{language_code}/{}",
                    title_storage_output::banhammer_messages_file_name
                ),
                title_storage_output::banhammer_messages_file_path(hoppers_blfs_path, language_code)
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