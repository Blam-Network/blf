use binrw::{binrw, BinRead, BinWrite};
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::{bitfield, BlfChunk};
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
#[cfg(feature = "napi")]
use napi_derive::napi;
use crate::types::time::time64_t;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::player_rewards::e_purchase_state;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::player_rewards::player_commendations::s_persistent_per_commendation_state;
use blf_lib::types::c_string::StaticString;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("rpul", 3.1)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
#[Size(0x778)]
pub struct s_blf_chunk_reward_persistence_upload_to_lsp {
    pub alltime_cookie_count: u32,
    pub alltime_cookie_award_count: u32,
    pub alltime_commendation_progress: StaticArray<s_persistent_per_commendation_state, 128>,
    pub alltime_purchased_items: StaticArray<u8, 256>,
    pub cookies_earned_today_online: u32,
    pub cookie_award_count_today_online: StaticArray<u8, 4>,
    pub commendation_progress_today_online: StaticArray<s_persistent_per_commendation_state, 128>,
    pub purchased_items_today_online: StaticArray<u8, 256>,
    pub cookies_earned_today_offline: u32,
    pub cookie_award_count_today_offline: StaticArray<u8, 4>,
    pub commendation_progress_today_offline: StaticArray<s_persistent_per_commendation_state, 128>,
    pub unknown_51b: StaticArray<u8, 516>, //Purchase order of item_absolute_index. Datatype is c_static_stack, with first 4 bytes being u32 of num of items. Populated in reward_persistence_cookie_purchasable_item_buy
    pub day_index: u32,
    pub user_timezone: u32,
    pub purchase_definition_checksum: u32,
    pub unknown_728: u32,
    pub last_modified_at: i64,
    pub unknown_flag: u8,
    pub player_name: StaticString<16>,
    pub last_hopper_id: i16,
    pub last_game_time: i64,
    pub unknown_last_game_results: u32,
    pub quit_last_game: u8,
    pub unknown754: u32,
    pub profile_unknown758: u32,
    pub profile_unknown75c: i64,
    pub profile_unknown764: i16,
    pub profile_unknown766: i16,
    pub profile_unknown768: u32,
    pub player_xuid: i64,
    pub profile_unknown774: u32,
}

impl BlfChunkHooks for s_blf_chunk_reward_persistence_upload_to_lsp {}

