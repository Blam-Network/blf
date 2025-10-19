use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v06481_06_11_17_1330_alpha_release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, BinRead, BinWrite, Serialize, Deserialize)]
pub struct c_game_engine_oddball_variant {
    pub m_variant_flags: u32,
    #[brw(pad_after = 1)]
    pub m_team_scoring: u8,
    pub m_score_to_win: i16,
    pub m_oddball_waypoint: u8,
    pub m_carrying_points: i8,
    pub m_kill_points: i8,
    pub m_ball_kill_points: i8,
    pub m_carrier_kill_points: i8,
    #[brw(pad_after = 1)]
    pub m_ball_count: u8,
    pub m_ball_spawn_delay: u16,
    pub m_ball_inactive_respawn_delay: u16,
    #[brw(pad_after = 2)]
    pub m_carrier_traits: c_player_traits,
}

impl c_game_engine_oddball_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0))?;
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 1))?;
        bitstream.write_integer(self.m_team_scoring as u32, 2)?;
        bitstream.write_integer(self.m_oddball_waypoint as u32, 2)?;
        bitstream.write_signed_integer(self.m_score_to_win as i32, 11)?;
        bitstream.write_signed_integer(self.m_carrying_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_kill_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_ball_kill_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_carrier_kill_points as i32, 5)?;
        bitstream.write_integer(self.m_ball_count as u32, 2)?;
        bitstream.write_integer(self.m_ball_spawn_delay as u32, 7)?;
        bitstream.write_integer(self.m_ball_inactive_respawn_delay as u32, 7)?;
        self.m_carrier_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_unnamed_bool()?);
        SET_BIT!(self.m_variant_flags, 1, bitstream.read_unnamed_bool()?);
        self.m_team_scoring = bitstream.read_unnamed_integer(2)?;
        self.m_oddball_waypoint = bitstream.read_unnamed_integer(2)?;
        self.m_score_to_win = bitstream.read_unnamed_signed_integer(11)?;
        self.m_carrying_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_kill_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_ball_kill_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_carrier_kill_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_ball_count = bitstream.read_unnamed_integer(2)?;
        self.m_ball_spawn_delay = bitstream.read_unnamed_integer(7)?;
        self.m_ball_inactive_respawn_delay = bitstream.read_unnamed_integer(7)?;
        self.m_carrier_traits.decode(bitstream)?;

        Ok(())
    }
}