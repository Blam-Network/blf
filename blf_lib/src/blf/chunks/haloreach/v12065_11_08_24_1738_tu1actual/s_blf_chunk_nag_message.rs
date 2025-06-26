use std::u32;
use binrw::binrw;
use serde::{Deserialize, Serialize};
use blf_lib_derivable::result::BLFLibResult;
use blf_lib_derivable::blf::chunks::BlfChunkHooks;
use blf_lib_derive::BlfChunk;
use crate::types::c_string::StaticWcharString;

#[binrw]
#[derive(BlfChunk,Default,PartialEq,Debug,Clone,Serialize,Deserialize)]
#[Header("nagm", 1.1)]
#[brw(big)]
pub struct s_blf_chunk_nag_message
{
    pub title_index_identifier: u32,
    pub button_key_wait_time_ms: u32,
    title_size: u32,
    pub title: StaticWcharString<k_nag_message_title_max_length>,
    header_size: u32,
    pub header: StaticWcharString<k_nag_message_header_max_length>,
    button_key_size: u32,
    pub button_key: StaticWcharString<k_nag_message_button_key_max_length>,
    button_key_wait_size: u32,
    pub button_key_wait: StaticWcharString<k_nag_message_button_key_max_length>,
    message_size: u32,
    pub message: StaticWcharString<k_nag_message_body_max_length>,
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown3: u32,
}

impl BlfChunkHooks for s_blf_chunk_nag_message {
    fn before_write(&mut self, _previously_written: &Vec<u8>) -> BLFLibResult {
        self.title_size = self.title.get_string().len() as u32 * 2;
        self.header_size = self.title.get_string().len() as u32 * 2;
        self.button_key_size = self.title.get_string().len() as u32 * 2;
        self.button_key_wait_size = self.title.get_string().len() as u32 * 2;
        self.message_size = self.title.get_string().len() as u32 * 2;

        Ok(())
    }
}

const k_nag_message_title_max_length: usize = 0x30;
const k_nag_message_header_max_length: usize = 0x30;
const k_nag_message_button_key_max_length: usize = 0x30;
const k_nag_message_button_key_wait_max_length: usize = 0x30;
const k_nag_message_body_max_length: usize = 0x400;


impl s_blf_chunk_nag_message {
    pub fn create(
        title_index_identifier: u32,
        button_key_wait_time_ms: u32,
        title: String,
        header: String,
        button_key: String,
        button_key_wait: String,
        message: String,
    ) -> BLFLibResult<Self> {
        Ok(s_blf_chunk_nag_message {
            title_index_identifier,
            button_key_wait_time_ms,
            title_size: 0,
            title: StaticWcharString::from_string(&title)?,
            header_size: 0,
            header: StaticWcharString::from_string(&header)?,
            button_key_size: 0,
            button_key: StaticWcharString::from_string(&button_key)?,
            button_key_wait_size: 0,
            button_key_wait: StaticWcharString::from_string(&button_key_wait)?,
            message_size: 0,
            message: StaticWcharString::from_string(&message)?,
            unknown1: 0,
            unknown2: 0,
            unknown3: 0,
        })
    }
}