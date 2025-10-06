use std::u32;
use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticWcharString;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("mtdp", 4.1)]
#[brw(big)]
pub struct s_blf_chunk_message_of_the_day_popup
{
    pub title_index_identifier: u32,
    pub button_key_wait_time_ms: u32,
    #[bw(try_calc(u32::try_from(title.get_string().len() * 2)))]
    #[br(temp)]
    title_size: u32,
    pub title: StaticWcharString<k_motd_popup_title_max_length>,
    #[bw(try_calc(u32::try_from(header.get_string().len() * 2)))]
    #[br(temp)]
    header_size: u32,
    pub header: StaticWcharString<k_motd_popup_header_max_length>,
    #[bw(try_calc(u32::try_from(button_key.get_string().len() * 2)))]
    #[br(temp)]
    button_key_size: u32,
    pub button_key: StaticWcharString<k_motd_popup_button_key_max_length>,
    #[bw(try_calc(u32::try_from(button_key_wait.get_string().len() * 2)))]
    #[br(temp)]
    button_key_wait_size: u32,
    pub button_key_wait: StaticWcharString<k_motd_popup_button_key_max_length>,
    #[bw(try_calc(u32::try_from(message.get_string().len() * 2)))]
    #[br(temp)]
    message_size: u32,
    pub message: StaticWcharString<k_motd_popup_message_max_length>,
}

impl BlfChunkHooks for s_blf_chunk_message_of_the_day_popup {}

const k_motd_popup_title_max_length: usize = 48;
const k_motd_popup_header_max_length: usize = 48;
const k_motd_popup_button_key_max_length: usize = 48;
const k_motd_popup_button_key_wait_max_length: usize = 48;
const k_motd_popup_message_max_length: usize = 1024;
