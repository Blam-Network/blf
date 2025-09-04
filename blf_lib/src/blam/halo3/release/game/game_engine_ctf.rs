use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_game_engine_ctf_variant {
    pub m_variant_flags: u8,
    pub m_home_flag_waypoint: u8,
    pub m_game_type: u8,
    pub m_respawn: u8,
    pub m_touch_return_timeout: i16,
    pub m_sudden_death_time: i16,
    pub m_score_to_win: u16,
    pub m_flag_reset_time: u16,
    pub m_carrier_traits: c_player_traits,
}

impl c_game_engine_ctf_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0))?;
        bitstream.write_integer(self.m_home_flag_waypoint as u32, 2)?;
        bitstream.write_integer(self.m_game_type as u32, 2)?;
        bitstream.write_integer(self.m_respawn as u32, 2)?;
        bitstream.write_integer(self.m_score_to_win as u32, 6)?;
        bitstream.write_signed_integer(self.m_sudden_death_time as i32, 9)?;
        bitstream.write_integer(self.m_flag_reset_time as u32, 9)?;
        bitstream.write_signed_integer(self.m_touch_return_timeout as i32, 9)?;
        self.m_carrier_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_bool()?);
        self.m_home_flag_waypoint = bitstream.read_integer(2)?;
        self.m_game_type = bitstream.read_integer(2)?;
        self.m_respawn = bitstream.read_integer(2)?;
        self.m_score_to_win = bitstream.read_integer(6)?;
        self.m_sudden_death_time = bitstream.read_signed_integer(9)?;
        self.m_flag_reset_time = bitstream.read_integer(9)?;
        self.m_touch_return_timeout = bitstream.read_signed_integer(9)?;
        self.m_carrier_traits.decode(bitstream)?;

        Ok(())
    }
}