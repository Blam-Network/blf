use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
use crate::types::string::{StaticString, StaticWcharString};
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::saved_game_files::{
    s_content_item_campaign_metadata,
    s_content_item_firefight_metadata,
    s_content_item_film_metadata,
    s_content_item_game_variant_metadata,
    s_content_item_history,
    s_content_item_matchmaking_metadata,
};

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_online_file_general_metadata {
    pub id: u64,
    pub file_type: u8,
    pub tag_count: u8,
    #[brw(pad_after = 1)]
    pub megalo_category_index: u8,
    pub size_in_bytes: u32,
    pub activity: u8,
    pub game_mode: u8,
    #[brw(pad_after = 1)]
    pub game_engine_type: u8,
    pub unknown3: StaticArray<u8, 8>,
    pub map_id: i32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_online_file_metadata {
    pub general: s_online_file_general_metadata,
    pub created: s_content_item_history,
    pub modified: s_content_item_history,
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
    #[serde(skip_serializing, skip_deserializing)]
    pub pad1: StaticArray<u8, 16>,

    #[br(if(general.activity == 3))]
    #[bw(if(general.activity == 3))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchmaking_data: Option<s_content_item_matchmaking_metadata>,
    #[br(if(general.activity != 3))]
    #[bw(if(general.activity != 3))]
    #[serde(skip_serializing, skip_deserializing)]
    pub pad2: StaticArray<u8, 16>,

    #[br(if(general.game_mode == 1))]
    #[bw(if(general.game_mode == 1))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_data: Option<s_content_item_campaign_metadata>,
    #[br(if(general.game_mode == 2))]
    #[bw(if(general.game_mode == 2))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firefight_data: Option<s_content_item_firefight_metadata>,
    #[br(if(general.game_mode != 1 && general.game_mode != 2))]
    #[bw(if(general.game_mode != 1 && general.game_mode != 2))]
    #[serde(skip_serializing, skip_deserializing)]
    pub pad3: StaticArray<u8, 16>,
    pub screenshot_length: u32,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
pub struct s_online_file_tag {
    pub tag: StaticString<23>,
    pub unknown: u32,
}
