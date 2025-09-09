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

pub struct c_game_engine_sandbox_variant {
    pub m_variant_flags: u8,
    pub m_edit_mode: u8,
    pub m_respawn_time: u16,
    pub m_player_traits: c_player_traits,
}

impl c_game_engine_sandbox_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0))?;
        bitstream.write_integer(self.m_edit_mode as u32, 2)?;
        bitstream.write_integer(self.m_respawn_time as u32, 6)?;
        self.m_player_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_unnamed_bool()?);
        self.m_edit_mode = bitstream.read_unnamed_integer(2)?;
        self.m_respawn_time = bitstream.read_unnamed_integer(6)?;
        self.m_player_traits.decode(bitstream)?;

        Ok(())
    }
}