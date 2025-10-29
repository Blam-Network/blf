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

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("rpul", 3.1)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
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
    pub unknown_520: u32,
    pub unknown_524: StaticArray<u8, 516>,
    pub last_modified_time: i64,
    pub unknown_730: u32,
    pub flags_734: StaticArray<u8, 4>,
    #[brw(pad_after = 2)]
    pub unknown_hopper_id: i16,
    pub unknown_73c: u32,
    pub time64_740: time64_t,
    pub unknown_last_game_results_748: u32,
    #[brw(pad_after = 3)]
    pub quit_last_game: u8,
    #[brw(pad_after = 4)]
    pub lsp_cookie_jackpot: u32,
    pub jackpot_recieved_at: time64_t,
    pub last_profile_sync_time: i64,
    pub last_lsp_sync_time: i64,
    pub time64_778: time64_t,
    pub unknown_780: u8,
    pub daily_offline_cookie_cap_reached: u8,
    pub daily_online_cookie_cap_reached: u8,
    pub unknown_783: u8,
    pub persisted_rewards_data_784: u32,
    pub persisted_rewards_data_788: u32,
    pub persisted_rewards_data_78C: u32,
}

impl BlfChunkHooks for s_blf_chunk_challenge_state {}

