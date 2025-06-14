use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_game_engine_miscellaneous_options {
    pub m_flags: u8,
    pub m_round_time_limit_minutes: u8,
    pub m_round_limit: u8,
    pub m_early_victory_win_count: u8,
}

impl c_game_engine_miscellaneous_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_flags, 0))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 1))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 2))?;
        bitstream.write_integer(self.m_round_time_limit_minutes as u32, 8)?;
        bitstream.write_integer(self.m_round_limit as u32, 4)?;
        bitstream.write_integer(self.m_early_victory_win_count as u32, 4)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        SET_BIT!(self.m_flags, 0, bitstream.read_bool()?);
        SET_BIT!(self.m_flags, 1, bitstream.read_bool()?);
        SET_BIT!(self.m_flags, 2, bitstream.read_bool()?);
        self.m_round_time_limit_minutes = bitstream.read_u8(8)?;
        self.m_round_limit = bitstream.read_u8(4)?;
        self.m_early_victory_win_count = bitstream.read_u8(4)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_game_engine_respawn_options {
    pub m_flags: u8,
    pub m_lives_per_round: u8,
    pub m_team_lives_per_round: u8,
    pub m_respawn_time_seconds: u8,
    pub m_suicide_penalty_seconds: u8,
    pub m_betrayal_penalty_seconds: u8,
    // m_unknown_penalty_seconds: u8,
    pub m_respawn_growth_seconds: u8,
    pub m_respawn_player_traits_duration_seconds: u8,
    pub m_respawn_player_traits: c_player_traits,
}

impl c_game_engine_respawn_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_flags, 0))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 1))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 2))?;
        bitstream.write_bool(TEST_BIT!(self.m_flags, 3))?;
        bitstream.write_integer(self.m_lives_per_round as u32, 6)?;
        bitstream.write_integer(self.m_team_lives_per_round as u32, 7)?;
        bitstream.write_integer(self.m_respawn_time_seconds as u32, 8)?;
        bitstream.write_integer(self.m_suicide_penalty_seconds as u32, 8)?;
        bitstream.write_integer(self.m_betrayal_penalty_seconds as u32, 8)?;
        bitstream.write_integer(self.m_respawn_growth_seconds as u32, 4)?;
        bitstream.write_integer(self.m_respawn_player_traits_duration_seconds as u32, 6)?;
        self.m_respawn_player_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        SET_BIT!(self.m_flags, 0, bitstream.read_bool()?);
        SET_BIT!(self.m_flags, 1, bitstream.read_bool()?);
        SET_BIT!(self.m_flags, 2, bitstream.read_bool()?);
        SET_BIT!(self.m_flags, 3, bitstream.read_bool()?);
        self.m_lives_per_round = bitstream.read_u8(6)?;
        self.m_team_lives_per_round = bitstream.read_u8(7)?;
        self.m_respawn_time_seconds = bitstream.read_u8(8)?;
        self.m_suicide_penalty_seconds = bitstream.read_u8(8)?;
        self.m_betrayal_penalty_seconds = bitstream.read_u8(8)?;
        self.m_respawn_growth_seconds = bitstream.read_u8(4)?;
        self.m_respawn_player_traits_duration_seconds = bitstream.read_u8(6)?;
        self.m_respawn_player_traits.decode(bitstream)?;

        Ok(())
    }
}