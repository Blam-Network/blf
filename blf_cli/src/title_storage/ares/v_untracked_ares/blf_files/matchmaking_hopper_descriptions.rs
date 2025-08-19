use blf_lib::blf::versions::ares::v_untracked_ares::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_hopper_description_table, s_blf_chunk_start_of_file};
use crate::title_storage::ares::v_untracked_ares::v_untracked_ares;
use blf_lib::blf_file;

pub const k_matchmaking_hopper_descriptions_file_name: &str = "matchmaking_hopper_descriptions_003.bin";

blf_file! {
    pub struct matchmaking_hopper_descriptions {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        mhdf: s_blf_chunk_hopper_description_table,
        _eof: s_blf_chunk_end_of_file,
    }
}

impl matchmaking_hopper_descriptions {
    pub fn create(descriptions_table: s_blf_chunk_hopper_description_table) -> matchmaking_hopper_descriptions {
        matchmaking_hopper_descriptions {
            _blf: s_blf_chunk_start_of_file::default(),
            athr: s_blf_chunk_author::for_build::<v_untracked_ares>(),
            mhdf: descriptions_table,
            _eof: s_blf_chunk_end_of_file::default(),
        }
    }
}