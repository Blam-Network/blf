use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::c_bitstream_reader;
use crate::types::string::StaticString;
use crate::types::string::StaticWcharString;
use blf_lib::types::time::time64_t;
use crate::types::bool::Bool;
use crate::types::u64::Unsigned64;
use blf_lib::types::array::StaticArray;
use blf_lib_derivable::result::BLFLibResult;
use crate::types::time::filetime;
use serde_hex::{SerHex, StrictCap};
use crate::io::bitstream::c_bitstream_writer;
use crate::OPTION_TO_RESULT;

#[cfg(feature = "napi")]
use napi_derive::napi;

/// Halo 4 metadata `activity` is `e_network_game_type` (0..3), 2 bits.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(repr = u8)]
#[cfg_attr(feature = "napi", napi(namespace = "halo4_v20810_12_09_22_1647_main"))]
pub enum e_network_game_type {
    #[default]
    none = 0,
    custom_game = 1,
    matchmaking = 2,
    film = 3,
}

/// Back-compat alias for Reach-shaped callers.
pub type e_gui_game_mode = e_network_game_type;

/// Halo 4 `e_game_mode` (0..6), 3 bits.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(repr = u8)]
#[cfg_attr(feature = "napi", napi(namespace = "halo4_v20810_12_09_22_1647_main"))]
pub enum e_game_mode {
    #[default]
    none = 0,
    campaign = 1,
    /// Firefight / survival.
    survival = 2,
    multiplayer = 3,
    unknown_4 = 4,
    unknown_5 = 5,
    unknown_6 = 6,
}

/// Engine type in metadata (0..5), 3 bits.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize, BinRead, BinWrite)]
#[brw(repr = u8)]
#[cfg_attr(feature = "napi", napi(namespace = "halo4_v20810_12_09_22_1647_main"))]
pub enum e_metadata_game_engine_type {
    #[default]
    none = 0,
    sandbox = 1,
    megalogamengine = 2,
    campaign = 3,
    survival = 4,
    firefight = 5,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_content_item_film_metadata {
    #[brw(pad_after = 12)]
    pub seconds: i32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_content_item_game_variant_metadata {
    #[brw(pad_after = 15)]
    pub icon_index: i8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_content_item_matchmaking_metadata {
    #[brw(pad_after = 14)]
    pub hopper_identifier: u16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_content_item_campaign_metadata {
    pub campaign_id: i32,
    pub campaign_difficulty: i16,
    pub campaign_metagame_scoring: i16,
    pub campaign_insertion_point: i32,
    /// Halo 4 encodes a single 32-bit skull flags field.
    pub skull_flags: u32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo4_v20810_12_09_22_1647_main"))]
pub struct s_content_item_firefight_metadata {
    pub firefight_difficulty: i16,
    /// Halo 4 encodes a single 32-bit skull flags field.
    pub skull_flags: u32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_content_item_history {
    pub timestamp: time64_t,
    #[serde(with = "SerHex::<StrictCap>")]
    pub xuid: Unsigned64,
    pub name: StaticString<16>,
    #[brw(pad_after = 3)]
    pub is_online: Bool,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_content_item_general_metadata {
    #[brw(pad_after = 3)]
    pub file_type: i8,
    pub size_in_bytes: u32,
    pub unique_id: Unsigned64,
    pub parent_unique_id: Unsigned64,
    pub root_unique_id: Unsigned64,
    pub game_id: Unsigned64,
    pub activity: e_network_game_type,
    pub game_mode: e_game_mode,
    #[brw(pad_after = 1)]
    pub game_engine_type: u8,
    pub map_id: i32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_content_item_display_metadata {
    #[brw(pad_after = 7)]
    pub megalo_category_index: i8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct c_content_item_metadata {
    pub general: s_content_item_general_metadata,
    pub display: s_content_item_display_metadata,
    pub creation_history: s_content_item_history,
    pub modification_history: s_content_item_history,
    pub name: StaticWcharString<0x80>,
    pub description: StaticWcharString<0x80>,

    #[br(if(general.file_type == 3))]
    #[bw(if(general.file_type == 3))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub film_data: Option<s_content_item_film_metadata>,
    #[br(if(general.file_type == 6))]
    #[bw(if(general.file_type == 6))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_variant_data: Option<s_content_item_game_variant_metadata>,
    #[br(if(general.file_type != 6 && general.file_type != 3))]
    #[bw(if(general.file_type != 6 && general.file_type != 3))]
    #[serde(skip_serializing,skip_deserializing)]
    pub pad1: StaticArray<u8, 16>,

    #[br(if(general.activity == e_network_game_type::matchmaking))]
    #[bw(if(general.activity == e_network_game_type::matchmaking))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchmaking_data: Option<s_content_item_matchmaking_metadata>,
    #[br(if(general.activity != e_network_game_type::matchmaking))]
    #[bw(if(general.activity != e_network_game_type::matchmaking))]
    #[serde(skip_serializing,skip_deserializing)]
    pub pad2: StaticArray<u8, 16>,

    #[br(if(general.game_engine_type == e_metadata_game_engine_type::campaign as u8))]
    #[bw(if(general.game_engine_type == e_metadata_game_engine_type::campaign as u8))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_data: Option<s_content_item_campaign_metadata>,
    #[br(if(general.game_engine_type == e_metadata_game_engine_type::survival as u8))]
    #[bw(if(general.game_engine_type == e_metadata_game_engine_type::survival as u8))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firefight_data: Option<s_content_item_firefight_metadata>,
    #[br(if(general.game_engine_type != e_metadata_game_engine_type::campaign as u8 && general.game_engine_type != e_metadata_game_engine_type::survival as u8))]
    #[bw(if(general.game_engine_type != e_metadata_game_engine_type::campaign as u8 && general.game_engine_type != e_metadata_game_engine_type::survival as u8))]
    #[serde(skip_serializing,skip_deserializing)]
    pub pad3: StaticArray<u8, 16>,
}

impl c_content_item_metadata {
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.general.file_type = bitstream.read_integer::<i8>("type", 4)? - 1;
        self.general.size_in_bytes = bitstream.read_integer("file-size", 32)?;
        self.general.unique_id = bitstream.read_qword(64)?;
        self.general.parent_unique_id = bitstream.read_qword(64)?;
        self.general.root_unique_id = bitstream.read_qword(64)?;
        self.general.game_id = bitstream.read_qword(64)?;
        self.general.activity = OPTION_TO_RESULT!(
            num_traits::FromPrimitive::from_u8(
                bitstream.read_integer("activity", 2)?
            ),
            "invalid activity"
        )?;
        self.general.game_mode = OPTION_TO_RESULT!(
            num_traits::FromPrimitive::from_u8(
                bitstream.read_integer("game-mode", 3)?
            ),
            "invalid game-mode"
        )?;
        self.general.game_engine_type = bitstream.read_integer("game-engine-type", 3)?;
        self.general.map_id = bitstream.read_signed_integer("map-id", 32)?;
        self.display.megalo_category_index = bitstream.read_signed_integer("megalo-category-index", 8)?;
        self.creation_history.timestamp = bitstream.read_qword(64)?;
        self.creation_history.xuid = bitstream.read_qword(64)?;
        self.creation_history.name = StaticString::from_string(bitstream.read_string_extended_ascii(16)?)?;
        self.creation_history.is_online = bitstream.read_bool("author-flags")?;
        self.modification_history.timestamp = bitstream.read_qword(64)?;
        self.modification_history.xuid = bitstream.read_qword(64)?;
        self.modification_history.name = StaticString::from_string(bitstream.read_string_extended_ascii(16)?)?;
        self.modification_history.is_online = bitstream.read_bool("author-flags")?;
        self.name = StaticWcharString::from_string(bitstream.read_string_wchar(128)?)?;
        self.description = StaticWcharString::from_string(bitstream.read_string_wchar(128)?)?;

        match self.general.file_type {
            3 | 4 => {
                self.film_data = Some(s_content_item_film_metadata {
                    seconds: bitstream.read_unnamed_signed_integer(32)?
                })
            }
            6 => {
                self.game_variant_data = Some(s_content_item_game_variant_metadata {
                    icon_index: bitstream.read_unnamed_signed_integer(8)?,
                })
            }
            _ => {}
        }

        match self.general.activity {
            e_network_game_type::matchmaking => {
                self.matchmaking_data = Some(s_content_item_matchmaking_metadata {
                    hopper_identifier: bitstream.read_unnamed_integer(16)?,
                })
            }
            _ => {}
        }

        match self.general.game_engine_type {
            x if x == e_metadata_game_engine_type::campaign as u8 => {
                self.campaign_data = Some(s_content_item_campaign_metadata {
                    campaign_id: bitstream.read_integer("campaign-id", 8)?,
                    campaign_difficulty: bitstream.read_integer("difficulty-level", 2)?,
                    campaign_metagame_scoring: bitstream.read_integer("metagame-scoring", 2)?,
                    campaign_insertion_point: bitstream.read_integer("insertion-point", 8)?,
                    skull_flags: bitstream.read_integer("skull-flags", 32)?,
                })
            }
            x if x == e_metadata_game_engine_type::survival as u8 => {
                self.firefight_data = Some(s_content_item_firefight_metadata {
                    firefight_difficulty: bitstream.read_integer("difficulty-level", 2)?,
                    skull_flags: bitstream.read_integer("skull-flags", 32)?,
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
        bitstream.write_integer(self.general.activity as u8 as u32, 2)?;
        bitstream.write_integer(self.general.game_mode as u8 as u32, 3)?;
        bitstream.write_integer(self.general.game_engine_type, 3)?;
        bitstream.write_signed_integer(self.general.map_id, 32)?;
        bitstream.write_signed_integer(self.display.megalo_category_index, 8)?;
        bitstream.write_qword(self.creation_history.timestamp, 64)?;
        bitstream.write_qword(self.creation_history.xuid, 64)?;
        bitstream.write_string_extended_ascii(&self.creation_history.name.get_string()?, 16)?;
        bitstream.write_bool(self.creation_history.is_online)?;
        bitstream.write_qword(self.modification_history.timestamp, 64)?;
        bitstream.write_qword(self.modification_history.xuid, 64)?;
        bitstream.write_string_extended_ascii(&self.modification_history.name.get_string()?, 16)?;
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
            6 => {
                bitstream.write_signed_integer(
                    OPTION_TO_RESULT!(
                        &self.game_variant_data,
                        "Tried to serialize gametype with no game data."
                    )?.icon_index,
                    8
                )?;
            }
            _ => {}
        }

        match self.general.activity {
            e_network_game_type::matchmaking => {
                bitstream.write_integer(
                    OPTION_TO_RESULT!(
                        &self.matchmaking_data,
                        "Tried to serialize a file from matchmaking with no matchmaking data."
                    )?.hopper_identifier as u32,
                    16
                )?;
            }
            _ => {}
        }

        match self.general.game_engine_type {
            x if x == e_metadata_game_engine_type::campaign as u8 => {
                let campaign_data = OPTION_TO_RESULT!(
                    &self.campaign_data,
                    "Tried to serialize campaign file with no campaign data."
                )?;

                bitstream.write_integer(campaign_data.campaign_id as u32, 8)?;
                bitstream.write_integer(campaign_data.campaign_difficulty as u32, 2)?;
                bitstream.write_integer(campaign_data.campaign_metagame_scoring as u32, 2)?;
                bitstream.write_integer(campaign_data.campaign_insertion_point as u32, 8)?;
                bitstream.write_integer(campaign_data.skull_flags, 32)?;
            }
            x if x == e_metadata_game_engine_type::survival as u8 => {
                let firefight_data = OPTION_TO_RESULT!(
                    &self.firefight_data,
                    "Tried to serialize firefight file with no firefight data."
                )?;

                bitstream.write_integer(firefight_data.firefight_difficulty as u32, 2)?;
                bitstream.write_integer(firefight_data.skull_flags, 32)?;
            }
            _ => {}
        }

        Ok(())

    }

    pub fn content_item_metadata_set_defaults(&mut self) {
        *self = Self::default();
        self.general.file_type = -1;
        self.general.activity = e_network_game_type::none;
        self.general.game_mode = e_game_mode::none;
        self.general.game_engine_type = 0;
        self.general.map_id = -1;
        self.display.megalo_category_index = -1;
    }
}