use std::io::{Read, Seek, Write};
use binrw::{binrw, BinRead, BinResult, BinWrite, Endian};
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

#[derive(PartialEq,Debug,Clone,Serialize,Deserialize,Default)]
#[cfg_attr(feature = "napi", napi(object, namespace = "haloreach_12065_11_08_24_1738_tu1actual"))]
pub struct s_challenge_state {
    pub category: e_challenge_category,
    pub challenge: u8,
    // these are overrides, but some challenges dont have defaults so these become necessary.
    // -1 = no override.
    pub cookie_reward: Option<i16>,
    pub required_progress: Option<i32>, // var00000004
    pub minimum_score: Option<i32>, // &var00000009
    pub maximum_level_completion_time: Option<i32>, // &var0000000a
    pub skull_flags: Option<e_challenge_skull_flags>,
    pub maximum_death_count: Option<i32>,
    pub toast_progress_count: Option<i32>,
}

impl BinRead for s_challenge_state {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, args: Self::Args<'_>) -> BinResult<Self> {
        let mut result = Self::default();
        result.category = BinRead::read_options(reader, endian, args)?;
        result.challenge = BinRead::read_options(reader, endian, args)?;

        let cookie_reward = i16::read_options(reader, endian, args)?;
        if cookie_reward != -1 {
            result.cookie_reward = Some(cookie_reward);
        }

        let required_progress = i32::read_options(reader, endian, args)?;
        if required_progress != -1 {
            result.required_progress = Some(required_progress);
        }

        let minimum_score = i32::read_options(reader, endian, args)?;
        if minimum_score != -1 {
            result.minimum_score = Some(minimum_score);
        }

        let maximum_level_completion_time = i32::read_options(reader, endian, args)?;
        if maximum_level_completion_time != -1 {
            result.maximum_level_completion_time = Some(maximum_level_completion_time);
        }

        let skull_flags = i32::read_options(reader, endian, args)?;
        if skull_flags != -1 {
            result.skull_flags = Some(e_challenge_skull_flags::from_raw(skull_flags as u32));
        }

        let maximum_death_count = i32::read_options(reader, endian, args)?;
        if maximum_death_count != -1 {
            result.maximum_death_count = Some(maximum_death_count);
        }

        let toast_progress_count = i32::read_options(reader, endian, args)?;
        if toast_progress_count != -1 {
            result.toast_progress_count = Some(toast_progress_count);
        }

        Ok(result)
    }
}

impl BinWrite for s_challenge_state {
    type Args = ();
    fn write_options<W: Write + Seek>(&self, writer: &mut W, endian: Endian, args: Self::Args<'_>) -> BinResult<()> {
        self.category.write_options(writer, endian, args)?;
        self.challenge.write_options(writer, endian, args)?;
        self.cookie_reward.unwrap_or(-1).write_options(writer, endian, args)?;
        self.required_progress.unwrap_or(-1).write_options(writer, endian, args)?;
        self.minimum_score.unwrap_or(-1).write_options(writer, endian, args)?;
        self.maximum_level_completion_time.unwrap_or(-1).write_options(writer, endian, args)?;
        
        match self.skull_flags {
            Some(skull_flags) => {
                skull_flags.write_options(writer, endian, args)?;
            }
            None => {
                -1i32.write_options(writer, endian, args)?;
            }
        }

        self.maximum_death_count.unwrap_or(-1).write_options(writer, endian, args)?;
        self.toast_progress_count.unwrap_or(-1).write_options(writer, endian, args)?;
        
        Ok(())
    }
}

impl BlfChunkHooks for s_blf_chunk_challenge_state {}

