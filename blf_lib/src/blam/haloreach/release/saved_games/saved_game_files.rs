use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use crate::types::c_string::StaticString;
use crate::types::c_string::StaticWcharString;
use blf_lib::types::time::time64_t;
use crate::types::bool::Bool;
use crate::types::u64::Unsigned64;
#[cfg(feature = "napi")]
use napi_derive::napi;
use blf_lib::types::array::StaticArray;

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
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_content_item_metadata_firefight_data {
    pub campaign_difficulty: i16,
    pub campaign_primary_skulls: i16,
    #[brw(pad_after = 10)]
    pub campaign_secondary_skulls: i16,
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
    pub game_variant_data: Option<s_content_item_metadata_film_data>,
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

