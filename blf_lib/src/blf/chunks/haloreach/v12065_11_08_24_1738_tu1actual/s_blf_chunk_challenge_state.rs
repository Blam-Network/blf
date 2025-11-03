use binrw::{binrw, BinRead, BinWrite};
use blf_lib::blf::chunks::BlfChunkHooks;
use blf_lib::{bitfield, BlfChunk};
use serde::{Deserialize, Serialize};
use blf_lib::types::array::StaticArray;
#[cfg(feature = "napi")]
use napi_derive::napi;
use num_derive::{FromPrimitive, ToPrimitive};
use crate::types::time::time64_t;

#[binrw]
#[derive(BlfChunk,PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[Header("dcha", 3.1)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_blf_chunk_challenge_state {
    pub active_challenge_set_1: u32,
    pub active_challenge_set_2: u32,
    pub chalenge_set_1_timestamp: time64_t,
    pub chalenge_set_2_timestamp: time64_t,
    pub chalenge_set_1_count: u8,
    pub chalenge_set_2_count: u8,
    pub chalenge_set_1: StaticArray<s_challenge_state, 10>,
    pub chalenge_set_2: StaticArray<s_challenge_state, 10>,
}

// This is probably used in many other places, may want to move it in future.
bitfield! {
    #[derive(Serialize,Deserialize)]
    #[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
    pub struct e_challenge_skull_flags: u32 {
        iron,
        black_eye,
        tough_luck,
        catch,
        fog,
        famine,
        thunderstorm,
        tilt,
        mythic,
        assasin,
        blind,
        superman,
        grunt_birthday_party,
        iwhbyd
    }
}

bitfield! {
    #[derive(Serialize,Deserialize)]
    #[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
    pub struct e_challenge_difficulty_flags: u8 {
        easy,
        normal,
        heroic,
        legendary,
    }
}

bitfield! {
    #[derive(Serialize,Deserialize)]
    #[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
    pub struct e_challenge_activity_type_flags: u8 {
        custom,
        matchmaking
    }
}

bitfield! {
    #[derive(Serialize,Deserialize)]
    #[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
    pub struct e_challenge_player_count_flags: u8 {
        player_count_any,
        player_count_1,
        player_count_4,
        player_count_greater_than_1,
    }
}

bitfield! {
    #[derive(Serialize,Deserialize)]
    #[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
    pub struct e_challenge_game_type_flags: u8 {
        campaign,
        firefight,
        multiplayer
    }
}

#[derive(BinRead, BinWrite, Serialize, Deserialize, Default, PartialEq, Debug, Copy, Clone, FromPrimitive, ToPrimitive)]
#[brw(repr = u8)]
#[cfg_attr(feature = "napi", napi(namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
#[repr(u8)]
pub enum e_challenge_category {
    #[default]
    bounty = 0,
    weekly = 1,
    campaign = 2,
    firefight = 3,
    matchmaking = 4,
}

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize,Default,BinRead,BinWrite)]
#[brw(big)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_challenge_state {
    pub category: e_challenge_category,
    pub challenge: u8,
    // these are overrides, but some challenges dont have defaults so these become necessary.
    // 0 = no override.
    pub cookie_reward: u16,
    pub required_progress: u32, // var00000004
    pub minimum_score: u32, // &var00000009
    pub maximum_level_completion_time: u32, // &var0000000a
    pub skull_flags: e_challenge_skull_flags,
    pub maximum_death_count: u32,
    pub toast_progress_count: u32,
}

impl BlfChunkHooks for s_blf_chunk_challenge_state {}

