use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v06481_06_11_17_1330_alpha_release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, BinRead, BinWrite, Serialize, Deserialize)]
pub struct c_game_engine_territories_variant {
    pub m_variant_flags: u16,
    pub m_respawn_on_capture: u16,
    pub m_capture_time: i16,
    pub m_sudden_death_time: i16,
    pub m_defender_traits: c_player_traits,
    pub m_attacker_traits: c_player_traits,
}

impl c_game_engine_territories_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0))?;
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 1))?;
        bitstream.write_integer(self.m_respawn_on_capture as u32, 2)?;
        bitstream.write_integer(self.m_capture_time as u32, 7)?;
        bitstream.write_signed_integer(self.m_sudden_death_time as i32, 10)?;
        self.m_defender_traits.encode(bitstream)?;
        self.m_attacker_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_unnamed_bool()?);
        SET_BIT!(self.m_variant_flags, 1, bitstream.read_unnamed_bool()?);
        self.m_respawn_on_capture = bitstream.read_unnamed_integer(2)?;
        self.m_capture_time = bitstream.read_unnamed_integer(7)?;
        self.m_sudden_death_time = bitstream.read_unnamed_signed_integer(10)?;
        self.m_defender_traits.decode(bitstream)?;
        self.m_attacker_traits.decode(bitstream)?;

        Ok(())
    }
}