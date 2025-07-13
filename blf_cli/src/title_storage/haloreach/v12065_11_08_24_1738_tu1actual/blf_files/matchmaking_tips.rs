use std::error::Error;
use std::io::{Read, Write};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_matchmaking_tips, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::{v12065_11_08_24_1738_tu1actual, v12070_08_09_05_2031_halo3_ship};
use blf_lib::blf_file;
use crate::build_path;
use crate::io::create_parent_folders;
use std::fs::File;
use blf_lib::result::BLFLibResult;

pub const k_matchmaking_tips_file_name: &str = "matchmaking_tips.bin";
pub const k_tangerine_matchmaking_tips_file_name: &str = "tangerine_matchmaking_tips.bin";
pub const m_matchmaking_tips_config_folder_name: &str = "matchmaking_tips";
pub const m_tangerine_matchmaking_tips_config_folder_name: &str = "matchmaking_tips_cea";

blf_file! {
    pub struct matchmaking_tips {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mmtp: s_blf_chunk_matchmaking_tips,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl matchmaking_tips {
    fn create(tips: Vec<String>) -> BLFLibResult<matchmaking_tips> {
        Ok(matchmaking_tips {
            _blf: s_blf_chunk_start_of_file::default(),
            athr: s_blf_chunk_author::for_build::<v12065_11_08_24_1738_tu1actual>(),
            mmtp: s_blf_chunk_matchmaking_tips::create(tips)?,
            _eof: s_blf_chunk_end_of_file::default(),
        })
    }

    pub fn read_from_config(
        hoppers_config_path: &String,
        language_code: &str,
        tangerine: bool,
    ) -> BLFLibResult<matchmaking_tips> {
        let config_file_path = build_path!(
            hoppers_config_path,
            if tangerine { m_tangerine_matchmaking_tips_config_folder_name }
            else { m_matchmaking_tips_config_folder_name },
            &format!("{language_code}.txt")
        );

        let mut config_file = File::open(config_file_path)?;
        let mut matchmaking_tips: String = String::new();
        config_file.read_to_string(&mut matchmaking_tips).unwrap();
        let matchmaking_tips = matchmaking_tips
            .lines()
            .map(String::from)
            .collect();

        matchmaking_tips::create(matchmaking_tips)
    }

    pub fn write_to_config(&self, hoppers_config_path: &String, language_code: &str, tangerine: bool) -> BLFLibResult {
        let config_file_path = build_path!(
            hoppers_config_path,
            if tangerine { m_tangerine_matchmaking_tips_config_folder_name }
            else { m_matchmaking_tips_config_folder_name },
            format!("{language_code}.txt")
        );

        let messages_text = self.mmtp.get_tips()?
            .join("\r\n");

        create_parent_folders(&config_file_path)?;

        let mut text_file = File::create(config_file_path).unwrap();

        text_file.write_all(messages_text.as_bytes())?;

        Ok(())
    }
}