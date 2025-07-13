use std::error::Error;
use std::fs;
use std::io::Write;
use serde::{Deserialize, Serialize};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_message_of_the_day_popup, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::{v12065_11_08_24_1738_tu1actual, v12070_08_09_05_2031_halo3_ship};
use blf_lib::blf_file;
use blf_lib::io::read_json_file;
use blf_lib::types::byte_order_mark::byte_order_mark;
use crate::build_path;
use crate::io::create_parent_folders;
use crate::title_storage::check_file_exists;
use std::path::Path;
use filesize::PathExt;
use std::fs::{exists, File};
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_nag_message;

pub const k_nag_message_file_name: &str = "dynamic_global_nag.bin";
pub const k_cea_nag_message_file_name: &str = "tangerine_dynamic_global_nag.bin";
pub const k_nag_message_image_file_name: &str = "dynamic_global_nag_image.jpg";
pub const k_cea_nag_message_image_file_name: &str = "tangerine_dynamic_global_nag_image.jpg";
pub const k_nag_message_config_folder: &str = "nag_message";
pub const k_cea_nag_message_config_folder: &str = "nag_message_cea";
const k_nag_message_image_width: u16 = 442;
const k_nag_message_image_height: u16 = 540;


blf_file! {
    pub struct nag_message {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        nagm: s_blf_chunk_nag_message,
        _eof: s_blf_chunk_end_of_file,
    }
}

#[derive(Serialize, Deserialize)]
pub struct motd_popup_config {
    pub motdIdentifier: u32,
    pub acceptWaitMilliseconds: u32,
    pub title: String,
    pub heading: String,
    pub accept: String,
    pub wait: String,
    pub body: String,
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown3: u32,
}

impl nag_message {
    pub fn create(nagm: s_blf_chunk_nag_message) -> Self {
        Self {
            _blf: s_blf_chunk_start_of_file::new("reach nag message", byte_order_mark::little_endian),
            athr: s_blf_chunk_author::for_build::<v12065_11_08_24_1738_tu1actual>(),
            nagm,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }

    pub fn read_from_config(
        hoppers_config_folder: impl Into<String>,
        language_code: &str,
        tangerine: bool
    ) -> Result<Self, Box<dyn Error>> {
        let motd_popup_json_path = build_path!(
            hoppers_config_folder,
            { if tangerine { k_cea_nag_message_config_folder } else { k_nag_message_config_folder } },
            format!("{language_code}.json")
        );

        let config = read_json_file::<motd_popup_config>(&motd_popup_json_path)?;

        Ok(Self::create(s_blf_chunk_nag_message::create(
            config.motdIdentifier,
            config.acceptWaitMilliseconds,
            config.title,
            config.heading,
            config.accept,
            config.wait,
            config.body,
            config.unknown1,
            config.unknown2,
            config.unknown3,
        )?))
    }

    pub fn write_to_config(&self, hoppers_config_path: &String, language_code: &str, mythic: bool) -> Result<(), Box<dyn Error>> {
        let config_file_path = build_path!(
            hoppers_config_path,
            { if mythic { k_cea_nag_message_config_folder} else { k_nag_message_config_folder } },
            format!("{language_code}.json")
        );

        let config = motd_popup_config {
            motdIdentifier: self.nagm.title_index_identifier,
            acceptWaitMilliseconds: self.nagm.button_key_wait_time_ms,
            title: self.nagm.title.get_string(),
            heading: self.nagm.header.get_string(),
            accept: self.nagm.button_key.get_string(),
            wait: self.nagm.button_key_wait.get_string(),
            body: self.nagm.message.get_string(),
            unknown1: self.nagm.unknown1,
            unknown2: self.nagm.unknown2,
            unknown3: self.nagm.unknown3,
        };

        create_parent_folders(&config_file_path)?;
        let motd_json = serde_json::to_string_pretty(&config).unwrap();
        let mut text_file = File::create(config_file_path).unwrap();
        text_file.write_all(motd_json.as_bytes())?;

        Ok(())
    }

    pub fn validate_image(path: &String) -> Result<(), Box<dyn Error>> {
        if !exists(path)? {
            return Err(Box::from("No image file was found"));
        }

        let path = Path::new(&path);
        let image_filesize = path.size_on_disk()?;
        // TODO: Work out the max size.
        // if image_filesize > k_max_popup_image_size as u64 {
        //     return Err(Box::from(format!("Image file size is too large ({}B > {}B)", image_filesize, k_max_popup_image_size)));
        // }

        let jpeg_data = fs::read(path)?;
        let mut decoder = jpeg_decoder::Decoder::new(jpeg_data.as_slice());
        decoder.read_info()?;
        let header = decoder.info().unwrap();
        if header.width != k_nag_message_image_width {
            return Err(Box::from(format!("Invalid image width ({}px != {}px)", header.width, k_nag_message_image_width)));
        }
        if header.height != k_nag_message_image_height {
            return Err(Box::from(format!("Invalid image width ({}px != {}px)", header.height, k_nag_message_image_height)));
        }

        Ok(())
    }
}