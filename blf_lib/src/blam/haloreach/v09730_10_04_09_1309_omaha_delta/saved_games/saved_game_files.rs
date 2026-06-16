use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::c_bitstream_reader;
use crate::types::string::StaticString;
use crate::types::string::StaticWcharString;
use blf_lib::types::time::time64_t;
use crate::types::bool::Bool;
use crate::types::u64::Unsigned64;
use blf_lib_derivable::result::BLFLibResult;
use serde_hex::{SerHex, StrictCap};
use crate::io::bitstream::c_bitstream_writer;
use crate::OPTION_TO_RESULT;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_film_metadata {
    #[brw(pad_after = 12)]
    pub seconds: i32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_game_variant_metadata {
    #[brw(pad_after = 15)]
    pub icon_index: i8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_matchmaking_metadata {
    #[brw(pad_after = 14)]
    pub hopper_identifier: u16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_campaign_metadata {
    pub campaign_id: i32,
    pub campaign_difficulty: i16,
    pub campaign_metagame_scoring: i16,
    pub campaign_insertion_point: i32,
    pub campaign_skull_flags: u32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_firefight_metadata {
    pub firefight_difficulty: i16,
    pub firefight_skull_flags: u32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_history {
    pub timestamp: time64_t,
    #[serde(with = "SerHex::<StrictCap>")]
    pub xuid: Unsigned64,
    pub name: StaticString<16>,
    #[brw(pad_after = 3)]
    pub is_online: Bool,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_general_metadata {
    #[brw(pad_after = 3)]
    pub file_type: i8,
    pub size_in_bytes: u32,
    pub unique_id: Unsigned64,
    pub parent_unique_id: Unsigned64,
    pub root_unique_id: Unsigned64,
    pub game_id: Unsigned64,
    pub activity: i8,
    pub game_mode: u8,
    #[brw(pad_after = 1)]
    pub game_engine_type: u8,
    pub map_id: i32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_display_metadata {
    pub megalo_category_index: i8,
    pub padding: u64,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_content_item_metadata {
    pub general: s_content_item_general_metadata,
    pub display: s_content_item_display_metadata,
    pub creation_history: s_content_item_history,
    pub modification_history: s_content_item_history,
    pub name: StaticWcharString<0x80>,
    pub description: StaticWcharString<0x80>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub film_data: Option<s_content_item_film_metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchmaking_data: Option<s_content_item_matchmaking_metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_data: Option<s_content_item_campaign_metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firefight_data: Option<s_content_item_firefight_metadata>,
}

impl s_content_item_metadata {
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.general.file_type = bitstream.read_integer::<i8>("type", 4)? - 1;
        self.general.size_in_bytes = bitstream.read_integer("file-size", 32)?;
        self.general.unique_id = bitstream.read_qword(64)?;
        self.general.parent_unique_id = bitstream.read_qword(64)?;
        self.general.root_unique_id = bitstream.read_qword(64)?;
        self.general.game_id = bitstream.read_qword(64)?;
        self.general.activity = bitstream.read_integer::<i8>("activity", 3)? - 1;
        self.general.game_mode = bitstream.read_integer("game-mode", 3)?;
        self.general.game_engine_type = bitstream.read_integer("game-engine-type", 3)?;
        self.general.map_id = bitstream.read_signed_integer("map-id", 32)?;
        self.display.padding = bitstream.read_qword(64)?;
        self.display.megalo_category_index = 0;
        self.creation_history.timestamp = bitstream.read_qword(64)?;
        self.creation_history.xuid = bitstream.read_qword(64)?;
        self.creation_history.name =
            StaticString::from_string(bitstream.read_string_utf8(16).unwrap_or_default())?;
        self.creation_history.is_online = bitstream.read_bool("author-flags")?;
        self.modification_history.timestamp = bitstream.read_qword(64)?;
        self.modification_history.xuid = bitstream.read_qword(64)?;
        self.modification_history.name =
            StaticString::from_string(bitstream.read_string_utf8(16).unwrap_or_default())?;
        self.modification_history.is_online = bitstream.read_bool("author-flags")?;
        self.name = StaticWcharString::from_string(bitstream.read_string_wchar(128)?)?;
        self.description = StaticWcharString::from_string(bitstream.read_string_wchar(128)?)?;

        match self.general.file_type {
            3 | 4 => {
                self.film_data = Some(s_content_item_film_metadata {
                    seconds: bitstream.read_signed_integer("film-seconds", 32)?,
                });
            }
            _ => {
                self.film_data = None;
            }
        }

        match self.general.activity {
            2 => {
                self.matchmaking_data = Some(s_content_item_matchmaking_metadata {
                    hopper_identifier: bitstream.read_integer("hopper-identifier", 16)?,
                });
            }
            _ => {
                self.matchmaking_data = None;
            }
        }

        match self.general.game_mode {
            1 => {
                self.campaign_data = Some(s_content_item_campaign_metadata {
                    campaign_id: bitstream.read_integer("campaign-id", 8)?,
                    campaign_difficulty: bitstream.read_integer("difficulty-level", 2)?,
                    campaign_metagame_scoring: bitstream.read_integer("metagame-scoring", 2)?,
                    campaign_insertion_point: bitstream.read_integer("insertion-point", 2)?,
                    campaign_skull_flags: bitstream.read_integer("skull-flags", 32)?,
                });
                self.firefight_data = None;
            }
            2 => {
                self.firefight_data = Some(s_content_item_firefight_metadata {
                    firefight_difficulty: bitstream.read_integer("difficulty-level", 2)?,
                    firefight_skull_flags: bitstream.read_integer("skull-flags", 32)?,
                });
                self.campaign_data = None;
            }
            _ => {
                self.campaign_data = None;
                self.firefight_data = None;
            }
        }

        Ok(())
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer((self.general.file_type + 1) as u32, 4)?;
        bitstream.write_integer(self.general.size_in_bytes, 32)?;
        bitstream.write_qword(self.general.unique_id, 64)?;
        bitstream.write_qword(self.general.parent_unique_id, 64)?;
        bitstream.write_qword(self.general.root_unique_id, 64)?;
        bitstream.write_qword(self.general.game_id, 64)?;
        bitstream.write_integer((self.general.activity + 1) as u32, 3)?;
        bitstream.write_integer(self.general.game_mode, 3)?;
        bitstream.write_integer(self.general.game_engine_type, 3)?;
        bitstream.write_signed_integer(self.general.map_id, 32)?;
        bitstream.write_qword(self.display.padding, 64)?;
        bitstream.write_qword(self.creation_history.timestamp, 64)?;
        bitstream.write_qword(self.creation_history.xuid, 64)?;
        bitstream.write_string_utf8(&self.creation_history.name.get_string()?, 16)?;
        bitstream.write_bool(self.creation_history.is_online)?;
        bitstream.write_qword(self.modification_history.timestamp, 64)?;
        bitstream.write_qword(self.modification_history.xuid, 64)?;
        bitstream.write_string_utf8(&self.modification_history.name.get_string()?, 16)?;
        bitstream.write_bool(self.modification_history.is_online)?;
        bitstream.write_string_wchar(&self.name.get_string(), 128)?;
        bitstream.write_string_wchar(&self.description.get_string(), 128)?;

        match self.general.file_type {
            3 | 4 => {
                bitstream.write_signed_integer(
                    OPTION_TO_RESULT!(
                        &self.film_data,
                        "Tried to serialize film with no film data."
                    )?
                    .seconds,
                    32,
                )?;
            }
            _ => {}
        }

        match self.general.activity {
            2 => {
                bitstream.write_signed_integer(
                    OPTION_TO_RESULT!(
                        &self.matchmaking_data,
                        "Tried to serialize a file from matchmaking with no matchmaking data."
                    )?
                    .hopper_identifier,
                    16,
                )?;
            }
            _ => {}
        }

        match self.general.game_mode {
            1 => {
                let campaign_data = OPTION_TO_RESULT!(
                    &self.campaign_data,
                    "Tried to serialize campaign file with no campaign data."
                )?;

                bitstream.write_integer(campaign_data.campaign_id as u32, 8)?;
                bitstream.write_integer(campaign_data.campaign_difficulty as u32, 2)?;
                bitstream.write_integer(campaign_data.campaign_metagame_scoring as u32, 2)?;
                bitstream.write_integer(campaign_data.campaign_insertion_point as u32, 2)?;
                bitstream.write_integer(campaign_data.campaign_skull_flags, 32)?;
            }
            2 => {
                let firefight_data = OPTION_TO_RESULT!(
                    &self.firefight_data,
                    "Tried to serialize firefight file with no firefight data."
                )?;

                bitstream.write_integer(firefight_data.firefight_difficulty as u32, 2)?;
                bitstream.write_integer(firefight_data.firefight_skull_flags, 32)?;
            }
            _ => {}
        }

        Ok(())
    }
}
