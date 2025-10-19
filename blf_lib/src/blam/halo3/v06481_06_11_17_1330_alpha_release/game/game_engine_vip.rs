use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v06481_06_11_17_1330_alpha_release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib_derivable::result::BLFLibResult;
use crate::types::bool::Bool;

#[derive(Default, PartialEq, Debug, Clone, BinRead, BinWrite, Serialize, Deserialize)]
pub struct c_game_engine_vip_variant {
    pub m_score_to_win_round: u16,
    pub m_single_vip: Bool,
    pub m_kill_points: i8,
    pub m_takedown_points: i8,
    pub m_kill_as_vip_points: i8,
    pub m_vip_death_points: i8,
    pub m_destination_arrival_points: i8,
    pub m_suicide_points: i8,
    pub m_betrayal_points: i8,
    pub m_vip_suicide_points: i8, // ok
    pub m_vip_selection: u8,
    pub m_zone_movement: u8,
    #[brw(pad_after = 1)]
    pub m_zone_order: u8,
    pub m_vip_team_traits: c_player_traits,
    pub m_vip_influence_traits: c_player_traits,
    #[brw(pad_after = 2)]
    pub m_vip_traits: c_player_traits,
}

impl c_game_engine_vip_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_single_vip)?;
        bitstream.write_integer(self.m_score_to_win_round as u32, 10)?;
        bitstream.write_signed_integer(self.m_kill_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_takedown_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_kill_as_vip_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_vip_death_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_destination_arrival_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_suicide_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_vip_suicide_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_betrayal_points as i32, 5)?;
        bitstream.write_integer(self.m_vip_selection as u32, 2)?;
        bitstream.write_integer(self.m_zone_movement as u32, 4)?;
        bitstream.write_integer(self.m_zone_order as u32, 1)?;
        self.m_vip_traits.encode(bitstream)?;
        self.m_vip_influence_traits.encode(bitstream)?;
        self.m_vip_team_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_single_vip = bitstream.read_unnamed_bool()?;
        self.m_score_to_win_round = bitstream.read_unnamed_integer(10)?;
        self.m_kill_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_takedown_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_kill_as_vip_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_vip_death_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_destination_arrival_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_suicide_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_vip_suicide_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_betrayal_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_vip_selection = bitstream.read_unnamed_integer(2)?;
        self.m_zone_movement = bitstream.read_unnamed_integer(4)?;
        self.m_zone_order = bitstream.read_unnamed_integer(1)?;
        self.m_vip_traits.decode(bitstream)?;
        self.m_vip_influence_traits.decode(bitstream)?;
        self.m_vip_team_traits.decode(bitstream)?;

        Ok(())
    }
}