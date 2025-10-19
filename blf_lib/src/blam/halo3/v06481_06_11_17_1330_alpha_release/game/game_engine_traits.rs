use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v06481_06_11_17_1330_alpha_release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
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
        SET_BIT!(self.m_flags, 0, bitstream.read_bool("miscellaneous-options-teams")?);
        SET_BIT!(self.m_flags, 1, bitstream.read_bool("miscellaneous-options-round-reset-players")?);
        SET_BIT!(self.m_flags, 2, bitstream.read_bool("miscellaneous-options-round-reset-map")?);
        self.m_round_time_limit_minutes = bitstream.read_integer("miscellaneous-options-round-time-limit-minutes", 8)?;
        self.m_round_limit = bitstream.read_integer("miscellaneous-options-round-limit", 4)?;
        self.m_early_victory_win_count = bitstream.read_integer("miscellaneous-options-early-victory-win-count", 4)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
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
        SET_BIT!(self.m_flags, 0, bitstream.read_bool("respawn-options-inherit-respawn-time")?);
        SET_BIT!(self.m_flags, 1, bitstream.read_bool("respawn-options-respawn-with-teammate")?);
        SET_BIT!(self.m_flags, 2, bitstream.read_bool("respawn-options-respawn-at-location")?);
        SET_BIT!(self.m_flags, 3, bitstream.read_bool("respawn-options-respawn-on-kills")?);
        self.m_lives_per_round = bitstream.read_integer("respawn-options-lives-per-round", 6)?;
        self.m_team_lives_per_round = bitstream.read_integer("respawn-options-team-lives-per-round", 7)?;
        self.m_respawn_time_seconds = bitstream.read_integer("respawn-options-respawn-time", 8)?;
        self.m_suicide_penalty_seconds = bitstream.read_integer("respawn-options-suicide-time", 8)?;
        self.m_betrayal_penalty_seconds = bitstream.read_integer("respawn-options-betrayal-time", 8)?;
        self.m_respawn_growth_seconds = bitstream.read_integer("respawn-options-respawn-growth-time", 4)?;
        self.m_respawn_player_traits_duration_seconds = bitstream.read_integer("respawn-options-player-traits-duration", 6)?;
        self.m_respawn_player_traits.decode(bitstream)?;

        Ok(())
    }
}