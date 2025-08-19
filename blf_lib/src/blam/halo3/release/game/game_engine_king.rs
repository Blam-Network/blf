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
pub struct c_game_engine_king_variant {
    pub m_variant_flags: u32,
    pub m_score_to_win: u16,
    pub m_moving_hill: u8,
    pub m_moving_hill_order: u8,
    pub m_uncontested_hill_bonus: i8,
    pub m_kill_points: i8,
    pub m_inside_hill_points: i8,
    pub m_outside_hill_points: i8,
    #[brw(pad_after = 6)] // seems sus
    pub m_inside_hill_traits: c_player_traits,
}

impl c_game_engine_king_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0))?;
        bitstream.write_integer(self.m_score_to_win as u32, 10)?;
        bitstream.write_integer(self.m_moving_hill as u32, 4)?;
        bitstream.write_integer(self.m_moving_hill_order as u32, 2)?;
        bitstream.write_signed_integer(self.m_inside_hill_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_outside_hill_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_uncontested_hill_bonus as i32, 5)?;
        bitstream.write_signed_integer(self.m_kill_points as i32, 5)?;
        self.m_inside_hill_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_bool()?);
        self.m_score_to_win = bitstream.read_u16(10)?;
        self.m_moving_hill = bitstream.read_u8(4)?;
        self.m_moving_hill_order = bitstream.read_u8(2)?;
        self.m_inside_hill_points = bitstream.read_signed_integer(5)? as i8;
        self.m_outside_hill_points = bitstream.read_signed_integer(5)? as i8;
        self.m_uncontested_hill_bonus = bitstream.read_signed_integer(5)? as i8;
        self.m_kill_points = bitstream.read_signed_integer(5)? as i8;
        self.m_inside_hill_traits.decode(bitstream)?;

        Ok(())
    }
}