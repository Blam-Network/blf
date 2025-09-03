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

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_metadata_film_data {
    #[brw(pad_after = 12)]
    pub seconds: i32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_metadata_game_variant_data {
    #[brw(pad_after = 15)]
    pub icon_index: i8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_metadata_matchmaking_data {
    #[brw(pad_after = 14)]
    pub hopper_identifier: u16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_metadata_campaign_data {
    pub campaign_id: i32,
    pub campaign_difficulty: i16,
    pub campaign_metagame_scoring: i16,
    #[brw(pad_after = 4)]
    pub campaign_insertion_point: i32,
    pub campaign_primary_skulls: i16,
    pub campaign_secondary_skulls: i16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_metadata_firefight_data {
    pub firefight_difficulty: i16,
    pub firefight_primary_skulls: i16,
    #[brw(pad_after = 10)]
    pub firefight_secondary_skulls: i16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_metadata {
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
    #[brw(pad_after = 7)]
    pub megalo_category_index: i8,
    pub creation_time: time64_t,
    pub creator_xuid: Unsigned64,
    pub creator_name: StaticString<16>,
    #[brw(pad_after = 3)]
    pub creator_xuid_is_online: Bool,
    pub modification_time: time64_t,
    pub modifier_xuid: Unsigned64,
    pub modifier_name: StaticString<16>,
    #[brw(pad_after = 3)]
    pub modifier_xuid_is_online: Bool,
    pub name: StaticWcharString<0x80>,
    pub description: StaticWcharString<0x80>,

    #[br(if(file_type == 3))]
    #[bw(if(*file_type == 3))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub film_data: Option<s_content_item_metadata_film_data>,
    #[br(if(file_type == 6))]
    #[bw(if(*file_type == 6))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_variant_data: Option<s_content_item_metadata_game_variant_data>,
    #[br(if(file_type != 6 && file_type != 3))]
    #[bw(if(*file_type != 6 && *file_type != 3))]
    #[serde(skip_serializing,skip_deserializing)]
    pub pad1: StaticArray<u8, 16>,

    #[br(if(activity == 3))]
    #[bw(if(*activity == 3))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchmaking_data: Option<s_content_item_metadata_matchmaking_data>,
    #[br(if(activity != 3))]
    #[bw(if(*activity != 3))]
    #[serde(skip_serializing,skip_deserializing)]
    pub pad2: StaticArray<u8, 16>,

    #[br(if(game_mode == 1))]
    #[bw(if(*game_mode == 1))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_data: Option<s_content_item_metadata_campaign_data>,
    #[br(if(game_mode == 2))]
    #[bw(if(*game_mode == 2))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firefight_data: Option<s_content_item_metadata_firefight_data>,
    #[br(if(game_mode != 1 && game_mode != 2))]
    #[bw(if(*game_mode != 1 && *game_mode != 2))]
    #[serde(skip_serializing,skip_deserializing)]
    pub pad3: StaticArray<u8, 16>,
}

impl s_content_item_metadata {
    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.file_type = (bitstream.read_integer(4)? as i8) - 1;
        self.size_in_bytes = bitstream.read_integer(32)?;
        self.unique_id = bitstream.read_qword(64)?;
        self.parent_unique_id = bitstream.read_qword(64)?;
        self.root_unique_id = bitstream.read_qword(64)?;
        self.game_id = bitstream.read_qword(64)?;
        self.activity = (bitstream.read_integer(3)? as i8) - 1;
        self.game_mode = bitstream.read_integer(3)? as u8;
        self.game_engine_type = bitstream.read_integer(3)? as u8;
        self.map_id = bitstream.read_integer(32)? as i32;
        self.megalo_category_index = bitstream.read_integer(8)? as i8;
        self.creation_time = time64_t::from(bitstream.read_qword(64)?);
        self.creator_xuid = bitstream.read_qword(64)?;
        self.creator_name = StaticString::from_string(bitstream.read_string_utf8(16)?)?;
        self.creator_xuid_is_online = Bool::from(bitstream.read_bool()?);
        self.modification_time = time64_t::from(bitstream.read_qword(64)?);
        self.modifier_xuid = bitstream.read_qword(64)?;
        self.modifier_name = StaticString::from_string(bitstream.read_string_utf8(16)?)?;
        self.modifier_xuid_is_online = Bool::from(bitstream.read_bool()?);
        self.name = StaticWcharString::from_string(bitstream.read_string_whar(128)?)?;
        self.description = StaticWcharString::from_string(bitstream.read_string_whar(128)?)?;

        match self.file_type {
            3 | 4 => {
                self.film_data = Some(s_content_item_metadata_film_data {
                    seconds: bitstream.read_signed_integer(32)?
                })
            }
            6 => {
                self.game_variant_data = Some(s_content_item_metadata_game_variant_data {
                    icon_index: bitstream.read_signed_integer(8)? as i8,
                })
            }
            _ => {}
        }

        match self.activity {
            2 => {
                self.matchmaking_data = Some(s_content_item_metadata_matchmaking_data {
                    hopper_identifier: bitstream.read_integer(16)? as u16,
                })
            }
            _ => {}
        }

        match self.game_mode {
            1 => {
                self.campaign_data = Some(s_content_item_metadata_campaign_data {
                    campaign_id: bitstream.read_integer(8)? as i32,
                    campaign_difficulty: bitstream.read_integer(2)? as i16,
                    campaign_metagame_scoring: bitstream.read_integer(2)? as i16,
                    campaign_insertion_point: bitstream.read_integer(2)? as i32,
                    campaign_primary_skulls: bitstream.read_integer(16)? as i16,
                    campaign_secondary_skulls: bitstream.read_integer(16)? as i16,
                })
            }
            2 => {
                self.firefight_data = Some(s_content_item_metadata_firefight_data {
                    firefight_difficulty: bitstream.read_integer(2)? as i16,
                    firefight_primary_skulls: bitstream.read_integer(16)? as i16,
                    firefight_secondary_skulls: bitstream.read_integer(16)? as i16,
                })
            }
            _ => {}
        }

        Ok(())
    }
}