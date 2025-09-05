use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::c_bitstream_reader;
use crate::types::c_string::StaticString;
use crate::types::c_string::StaticWcharString;
use blf_lib::types::time::time64_t;
use crate::types::bool::Bool;
use crate::types::u64::Unsigned64;
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::result::BLFLibResult;
use crate::types::time::filetime;
use serde_hex::{SerHex, StrictCap};
use crate::io::bitstream::c_bitstream_writer;
use crate::OPTION_TO_RESULT;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_content_item_metadata_film_data {
    pub seconds: i32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_content_item_metadata_game_variant_data {
    pub icon_index: i8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_content_item_metadata_matchmaking_data {
    pub hopper_identifier: u16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_content_item_metadata_campaign_data {
    pub campaign_id: i32,
    pub campaign_difficulty: i16,
    pub campaign_metagame_scoring: i16,
    pub campaign_insertion_point: i32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize,)]
pub struct s_content_item_metadata_firefight_data {
    pub firefight_difficulty: i16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_content_item_metadata {
    pub file_type: i8,
    pub size_in_bytes: u32,
    pub unique_id: Unsigned64,
    pub parent_unique_id: Unsigned64,
    pub root_unique_id: Unsigned64,
    pub game_id: Unsigned64,
    pub activity: i8,
    pub game_mode: u8,
    pub game_engine_type: u8,
    pub map_id: i32,
    pub unknown1: u64,
    pub creation_time: time64_t,
    #[serde(with = "SerHex::<StrictCap>")]
    pub creator_xuid: Unsigned64,
    pub creator_name: StaticString<16>,
    pub creator_xuid_is_online: Bool,
    pub modification_time: time64_t,
    #[serde(with = "SerHex::<StrictCap>")]
    pub modifier_xuid: Unsigned64,
    pub modifier_name: StaticString<16>,
    pub modifier_xuid_is_online: Bool,
    pub name: StaticWcharString<0x80>,
    pub description: StaticWcharString<0x80>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub film_data: Option<s_content_item_metadata_film_data>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchmaking_data: Option<s_content_item_metadata_matchmaking_data>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_data: Option<s_content_item_metadata_campaign_data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firefight_data: Option<s_content_item_metadata_firefight_data>,

}

impl s_content_item_metadata {
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.file_type = bitstream.read_integer::<i8>(4)? - 1;
        self.size_in_bytes = bitstream.read_integer(32)?;
        self.unique_id = bitstream.read_qword(64)?;
        self.parent_unique_id = bitstream.read_qword(64)?;
        self.root_unique_id = bitstream.read_qword(64)?;
        self.game_id = bitstream.read_qword(64)?;
        self.activity = bitstream.read_integer::<i8>(3)? - 1;
        self.game_mode = bitstream.read_integer(3)?;
        self.game_engine_type = bitstream.read_integer(3)?;
        self.map_id = bitstream.read_signed_integer(32)?;
        self.unknown1 = bitstream.read_qword(64)?;
        self.creation_time = bitstream.read_qword(64)?;
        self.creator_xuid = bitstream.read_qword(64)?;
        self.creator_name = StaticString::from_string(bitstream.read_string_utf8(16).unwrap_or_default())?;
        self.creator_xuid_is_online = bitstream.read_bool()?;
        self.modification_time = bitstream.read_qword(64)?;
        self.modifier_xuid = bitstream.read_qword(64)?;
        self.modifier_name = StaticString::from_string(bitstream.read_string_utf8(16).unwrap_or_default())?;
        self.modifier_xuid_is_online = bitstream.read_bool()?;
        self.name = StaticWcharString::from_string(bitstream.read_string_whar(128)?)?;
        self.description = StaticWcharString::from_string(bitstream.read_string_whar(128)?)?;

        match self.file_type {
            3 | 4 => {
                self.film_data = Some(s_content_item_metadata_film_data {
                    seconds: bitstream.read_signed_integer(32)?
                })
            }
            _ => {}
        }

        match self.activity {
            2 => {
                self.matchmaking_data = Some(s_content_item_metadata_matchmaking_data {
                    hopper_identifier: bitstream.read_integer(16)?,
                })
            }
            _ => {}
        }

        match self.game_mode {
            1 => {
                self.campaign_data = Some(s_content_item_metadata_campaign_data {
                    campaign_id: bitstream.read_integer(8)?,
                    campaign_difficulty: bitstream.read_integer(2)?,
                    campaign_metagame_scoring: bitstream.read_integer(2)?,
                    campaign_insertion_point: bitstream.read_integer(2)?,
                })
            }
            2 => {
                self.firefight_data = Some(s_content_item_metadata_firefight_data {
                    firefight_difficulty: bitstream.read_integer(2)?,
                })
            }
            _ => {}
        }

        Ok(())
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer((self.file_type + 1) as u32, 4)?;
        bitstream.write_integer(self.size_in_bytes, 32)?;
        bitstream.write_qword( self.unique_id, 64)?;
        bitstream.write_qword(self.parent_unique_id, 64)?;
        bitstream.write_qword(self.root_unique_id, 64)?;
        bitstream.write_qword(self.game_id, 64)?;
        bitstream.write_integer((self.activity + 1) as u32, 3)?;
        bitstream.write_integer(self.game_mode, 3)?;
        bitstream.write_integer(self.game_engine_type, 3)?;
        bitstream.write_signed_integer(self.map_id, 32)?;
        bitstream.write_qword(self.unknown1,  64)?;
        bitstream.write_qword(self.creation_time, 64)?;
        bitstream.write_qword(self.creator_xuid, 64)?;
        bitstream.write_string_utf8(&self.creator_name.get_string()?, 16)?;
        bitstream.write_bool(self.creator_xuid_is_online)?;
        bitstream.write_qword(self.modification_time, 64)?;
        bitstream.write_qword(self.modifier_xuid, 64)?;
        bitstream.write_string_utf8(&self.modifier_name.get_string()?, 16)?;
        bitstream.write_bool(self.modifier_xuid_is_online)?;
        bitstream.write_string_wchar(&self.name.get_string(), 128)?;
        bitstream.write_string_wchar(&self.description.get_string(), 128)?;

        match self.file_type {
            3 | 4 => {
                bitstream.write_signed_integer(
                    OPTION_TO_RESULT!(
                        &self.film_data,
                        "Tried to serialize film with no film data."
                    )?.seconds,
                    32
                )?;
            }
            _ => {}
        }

        match self.activity {
            2 => {
                bitstream.write_signed_integer(
                    OPTION_TO_RESULT!(
                        &self.matchmaking_data,
                        "Tried to serialize a file from matchmaking with no matchmaking data."
                    )?.hopper_identifier,
                    16
                )?;
            }
            _ => {}
        }

        match self.activity {
            1 => {
                let campaign_data = OPTION_TO_RESULT!(
                    &self.campaign_data,
                    "Tried to serialize campaign file with no campaign data."
                )?;

                bitstream.write_integer(campaign_data.campaign_id as u32, 8)?;
                bitstream.write_integer(campaign_data.campaign_difficulty as u32, 2)?;
                bitstream.write_integer(campaign_data.campaign_metagame_scoring as u32, 2)?;
                bitstream.write_integer(campaign_data.campaign_insertion_point as u32, 2)?;
            }
            2 => {
                let firefight_data = OPTION_TO_RESULT!(
                    &self.firefight_data,
                    "Tried to serialize firefight file with no firefight data."
                )?;

                bitstream.write_integer(firefight_data.firefight_difficulty as u32, 2)?;
            }
            _ => {}
        }

        Ok(())

    }
}