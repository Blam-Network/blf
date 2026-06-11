use binrw::{BinRead, BinWrite};
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::c_bitstream_reader;
use crate::types::c_string::StaticString;
use crate::types::c_string::StaticWcharString;
use blf_lib::types::time::time64_t;
use crate::types::bool::Bool;
use crate::types::u64::Unsigned64;
use blf_lib_derivable::result::BLFLibResult;
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
    pub campaign_skull_flags: u32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_content_item_metadata_firefight_data {
    pub firefight_difficulty: i16,
    pub firefight_skull_flags: u32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_content_item_history {
    pub timestamp: time64_t,
    #[serde(with = "SerHex::<StrictCap>")]
    pub xuid: Unsigned64,
    pub name: StaticString<16>,
    pub is_online: Bool,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_content_item_general_metadata {
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
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_content_item_display_metadata {
    pub unknown1: u64,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize)]
pub struct s_content_item_metadata {
    pub general: s_content_item_general_metadata,
    pub display: s_content_item_display_metadata,
    pub creation_history: s_content_item_history,
    pub modification_history: s_content_item_history,
    pub name: StaticWcharString<0x80>,
    pub description: StaticWcharString<0x80>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub film_data: Option<s_content_item_metadata_film_data>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_variant_data: Option<s_content_item_metadata_game_variant_data>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchmaking_data: Option<s_content_item_metadata_matchmaking_data>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_data: Option<s_content_item_metadata_campaign_data>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub firefight_data: Option<s_content_item_metadata_firefight_data>,
}

#[derive(Deserialize)]
struct s_content_item_metadata_nested_json {
    general: s_content_item_general_metadata,
    display: s_content_item_display_metadata,
    creation_history: s_content_item_history,
    modification_history: s_content_item_history,
    name: StaticWcharString<0x80>,
    description: StaticWcharString<0x80>,
    #[serde(default)]
    film_data: Option<s_content_item_metadata_film_data>,
    #[serde(default)]
    game_variant_data: Option<s_content_item_metadata_game_variant_data>,
    #[serde(default)]
    matchmaking_data: Option<s_content_item_metadata_matchmaking_data>,
    #[serde(default)]
    campaign_data: Option<s_content_item_metadata_campaign_data>,
    #[serde(default)]
    firefight_data: Option<s_content_item_metadata_firefight_data>,
}

#[derive(Deserialize)]
struct s_content_item_metadata_flat_json {
    file_type: i8,
    size_in_bytes: u32,
    unique_id: Unsigned64,
    parent_unique_id: Unsigned64,
    root_unique_id: Unsigned64,
    game_id: Unsigned64,
    activity: i8,
    game_mode: u8,
    game_engine_type: u8,
    map_id: i32,
    unknown1: u64,
    creation_time: time64_t,
    #[serde(with = "SerHex::<StrictCap>")]
    creator_xuid: Unsigned64,
    creator_name: StaticString<16>,
    creator_xuid_is_online: Bool,
    modification_time: time64_t,
    #[serde(with = "SerHex::<StrictCap>")]
    modifier_xuid: Unsigned64,
    modifier_name: StaticString<16>,
    modifier_xuid_is_online: Bool,
    name: StaticWcharString<0x80>,
    description: StaticWcharString<0x80>,
    #[serde(default)]
    film_data: Option<s_content_item_metadata_film_data>,
    #[serde(default)]
    game_variant_data: Option<s_content_item_metadata_game_variant_data>,
    #[serde(default)]
    matchmaking_data: Option<s_content_item_metadata_matchmaking_data>,
    #[serde(default)]
    campaign_data: Option<s_content_item_metadata_campaign_data>,
    #[serde(default)]
    firefight_data: Option<s_content_item_metadata_firefight_data>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum s_content_item_metadata_json {
    Nested(s_content_item_metadata_nested_json),
    Flat(s_content_item_metadata_flat_json),
}

impl From<s_content_item_metadata_nested_json> for s_content_item_metadata {
    fn from(value: s_content_item_metadata_nested_json) -> Self {
        Self {
            general: value.general,
            display: value.display,
            creation_history: value.creation_history,
            modification_history: value.modification_history,
            name: value.name,
            description: value.description,
            film_data: value.film_data,
            game_variant_data: value.game_variant_data,
            matchmaking_data: value.matchmaking_data,
            campaign_data: value.campaign_data,
            firefight_data: value.firefight_data,
        }
    }
}

impl From<s_content_item_metadata_flat_json> for s_content_item_metadata {
    fn from(value: s_content_item_metadata_flat_json) -> Self {
        Self {
            general: s_content_item_general_metadata {
                file_type: value.file_type,
                size_in_bytes: value.size_in_bytes,
                unique_id: value.unique_id,
                parent_unique_id: value.parent_unique_id,
                root_unique_id: value.root_unique_id,
                game_id: value.game_id,
                activity: value.activity,
                game_mode: value.game_mode,
                game_engine_type: value.game_engine_type,
                map_id: value.map_id,
            },
            display: s_content_item_display_metadata {
                unknown1: value.unknown1,
            },
            creation_history: s_content_item_history {
                timestamp: value.creation_time,
                xuid: value.creator_xuid,
                name: value.creator_name,
                is_online: value.creator_xuid_is_online,
            },
            modification_history: s_content_item_history {
                timestamp: value.modification_time,
                xuid: value.modifier_xuid,
                name: value.modifier_name,
                is_online: value.modifier_xuid_is_online,
            },
            name: value.name,
            description: value.description,
            film_data: value.film_data,
            game_variant_data: value.game_variant_data,
            matchmaking_data: value.matchmaking_data,
            campaign_data: value.campaign_data,
            firefight_data: value.firefight_data,
        }
    }
}

impl From<s_content_item_metadata_json> for s_content_item_metadata {
    fn from(value: s_content_item_metadata_json) -> Self {
        match value {
            s_content_item_metadata_json::Nested(nested) => nested.into(),
            s_content_item_metadata_json::Flat(flat) => flat.into(),
        }
    }
}

impl<'de> Deserialize<'de> for s_content_item_metadata {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        s_content_item_metadata_json::deserialize(deserializer)
            .map(Into::into)
            .map_err(de::Error::custom)
    }
}

impl s_content_item_metadata {
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.general.file_type = bitstream.read_unnamed_integer::<i8>(4)? - 1;
        self.general.size_in_bytes = bitstream.read_unnamed_integer(32)?;
        self.general.unique_id = bitstream.read_qword(64)?;
        self.general.parent_unique_id = bitstream.read_qword(64)?;
        self.general.root_unique_id = bitstream.read_qword(64)?;
        self.general.game_id = bitstream.read_qword(64)?;
        self.general.activity = bitstream.read_unnamed_integer::<i8>(3)? - 1;
        self.general.game_mode = bitstream.read_unnamed_integer(3)?;
        self.general.game_engine_type = bitstream.read_unnamed_integer(3)?;
        self.general.map_id = bitstream.read_unnamed_signed_integer(32)?;
        self.display.unknown1 = bitstream.read_qword(64)?;
        self.creation_history.timestamp = bitstream.read_qword(64)?;
        self.creation_history.xuid = bitstream.read_qword(64)?;
        self.creation_history.name = StaticString::from_string(bitstream.read_string_utf8(16).unwrap_or_default())?;
        self.creation_history.is_online = bitstream.read_unnamed_bool()?;
        self.modification_history.timestamp = bitstream.read_qword(64)?;
        self.modification_history.xuid = bitstream.read_qword(64)?;
        self.modification_history.name = StaticString::from_string(bitstream.read_string_utf8(16).unwrap_or_default())?;
        self.modification_history.is_online = bitstream.read_unnamed_bool()?;
        self.name = StaticWcharString::from_string(bitstream.read_string_wchar(128)?)?;
        self.description = StaticWcharString::from_string(bitstream.read_string_wchar(128)?)?;

        match self.general.file_type {
            3 | 4 => {
                self.film_data = Some(s_content_item_metadata_film_data {
                    seconds: bitstream.read_unnamed_signed_integer(32)?
                })
            }
            _ => {}
        }

        match self.general.activity {
            2 => {
                self.matchmaking_data = Some(s_content_item_metadata_matchmaking_data {
                    hopper_identifier: bitstream.read_unnamed_integer(16)?,
                })
            }
            _ => {}
        }

        match self.general.game_mode {
            1 => {
                self.campaign_data = Some(s_content_item_metadata_campaign_data {
                    campaign_id: bitstream.read_unnamed_integer(8)?,
                    campaign_difficulty: bitstream.read_unnamed_integer(2)?,
                    campaign_metagame_scoring: bitstream.read_unnamed_integer(2)?,
                    campaign_insertion_point: bitstream.read_unnamed_integer(2)?,
                    campaign_skull_flags: bitstream.read_unnamed_integer(32)?,
                })
            }
            2 => {
                self.firefight_data = Some(s_content_item_metadata_firefight_data {
                    firefight_difficulty: bitstream.read_unnamed_integer(2)?,
                    firefight_skull_flags: bitstream.read_unnamed_integer(32)?,
                })
            }
            _ => {}
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
        bitstream.write_qword(self.display.unknown1, 64)?;
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
                    )?.seconds,
                    32
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
                    )?.hopper_identifier,
                    16
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
