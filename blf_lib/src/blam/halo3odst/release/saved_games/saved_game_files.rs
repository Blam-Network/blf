use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use crate::types::c_string::StaticString;
use crate::types::c_string::StaticWcharString;
use serde_hex::{SerHex,StrictCap};
use blf_lib::types::time::time64_t;
use blf_lib_derive::TestSize;
use crate::types::bool::Bool;
use crate::types::u64::Unsigned64;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite, TestSize)]
#[Size(0xF8)]
pub struct s_content_item_metadata {
    pub unique_id: Unsigned64,
    pub name: StaticWcharString<0x10>,
    pub description: StaticString<128>,
    pub author: StaticString<16>,
    pub file_type: u32,
    #[brw(align_after = 4)]
    pub author_is_xuid_online: Bool,
    #[serde(with = "SerHex::<StrictCap>")]
    pub author_id: Unsigned64,
    pub size_in_bytes: Unsigned64,
    pub date: time64_t,
    pub length_seconds: u32,
    pub campaign_id: i32,
    pub map_id: i32,
    pub game_engine_type: u32,
    pub campaign_difficulty: i32,
    pub campaign_insertion_point: i8,
    pub campaign_survival_enabled: Bool,
    #[brw(pad_before = 2)]
    pub game_id: Unsigned64,
}