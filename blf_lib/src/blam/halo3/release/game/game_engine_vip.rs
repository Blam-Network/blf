use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, BinRead, BinWrite, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_game_engine_vip_variant {
    pub m_score_to_win_round: u16,
    pub m_variant_flags: u16,
    pub m_kill_points: i8,
    pub m_takedown_points: i8,
    pub m_kill_as_vip_points: i8,
    pub m_vip_death_points: i8,
    pub m_destination_arrival_points: i8,
    pub m_suicide_points: i8,
    pub m_betrayal_points: i8,
    pub m_vip_suicide_points: i8,
    pub m_vip_selection: u8,
    pub m_zone_movement: u8,
    #[brw(pad_after = 1)]
    pub m_zone_order: u8,
    pub m_influence_radius: u16,
    pub m_vip_team_traits: c_player_traits,
    pub m_vip_influence_traits: c_player_traits,
    #[brw(pad_after = 2)]
    pub m_vip_traits: c_player_traits,
}

impl c_game_engine_vip_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0))?;
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 1))?;
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 2))?;
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
        bitstream.write_integer(self.m_influence_radius as u32, 6)?;
        self.m_vip_traits.encode(bitstream)?;
        self.m_vip_influence_traits.encode(bitstream)?;
        self.m_vip_team_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_bool()?);
        SET_BIT!(self.m_variant_flags, 1, bitstream.read_bool()?);
        SET_BIT!(self.m_variant_flags, 2, bitstream.read_bool()?);
        self.m_score_to_win_round = bitstream.read_u16(10)?;
        self.m_kill_points = bitstream.read_signed_integer(5)? as i8;
        self.m_takedown_points = bitstream.read_signed_integer(5)? as i8;
        self.m_kill_as_vip_points = bitstream.read_signed_integer(5)? as i8;
        self.m_vip_death_points = bitstream.read_signed_integer(5)? as i8;
        self.m_destination_arrival_points = bitstream.read_signed_integer(5)? as i8;
        self.m_suicide_points = bitstream.read_signed_integer(5)? as i8;
        self.m_vip_suicide_points = bitstream.read_signed_integer(5)? as i8;
        self.m_betrayal_points = bitstream.read_signed_integer(5)? as i8;
        self.m_vip_selection = bitstream.read_u8(2)?;
        self.m_zone_movement = bitstream.read_u8(4)?;
        self.m_zone_order = bitstream.read_u8(1)?;
        self.m_influence_radius = bitstream.read_u16(6)?;
        self.m_vip_traits.decode(bitstream)?;
        self.m_vip_influence_traits.decode(bitstream)?;
        self.m_vip_team_traits.decode(bitstream)?;

        Ok(())
    }
}