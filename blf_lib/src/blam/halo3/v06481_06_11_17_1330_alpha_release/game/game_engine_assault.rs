use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v06481_06_11_17_1330_alpha_release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, BinRead, BinWrite, Serialize, Deserialize)]
pub struct c_game_engine_assault_variant {
    pub m_variant_flags: u16,
    pub m_respawn: u16,
    pub m_game_type: u16,
    pub m_enemy_bomb_waypoint: u16,
    pub m_score_to_win: u16,
    pub m_sudden_death_time: i16,
    pub m_bomb_reset_time: u16,
    pub m_bomb_arming_time: u16,
    pub m_bomb_disarming_time: u16,
    pub m_bomb_fuse_time: u16,
    pub m_carrier_traits: c_player_traits,
    #[brw(pad_after = 4)]
    pub m_arming_traits: c_player_traits,
}

impl c_game_engine_assault_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0))?;
        bitstream.write_integer(self.m_game_type as u32, 2)?;
        bitstream.write_integer(self.m_respawn as u32, 3)?;
        bitstream.write_integer(self.m_enemy_bomb_waypoint as u32, 3)?;
        bitstream.write_integer(self.m_score_to_win as u32, 4)?;
        bitstream.write_signed_integer(self.m_sudden_death_time as i32, 9)?;
        bitstream.write_integer(self.m_bomb_arming_time as u32, 7)?;
        bitstream.write_integer(self.m_bomb_disarming_time as u32, 7)?;
        bitstream.write_integer(self.m_bomb_fuse_time as u32, 7)?;
        bitstream.write_integer(self.m_bomb_reset_time as u32, 9)?;
        self.m_carrier_traits.encode(bitstream)?;
        self.m_arming_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_unnamed_bool()?);
        self.m_game_type = bitstream.read_unnamed_integer(2)?;
        self.m_respawn = bitstream.read_unnamed_integer(3)?;
        self.m_enemy_bomb_waypoint = bitstream.read_unnamed_integer(3)?;
        self.m_score_to_win = bitstream.read_unnamed_integer(4)?;
        self.m_sudden_death_time = bitstream.read_unnamed_signed_integer(9)?;
        self.m_bomb_arming_time = bitstream.read_unnamed_integer(7)?;
        self.m_bomb_disarming_time = bitstream.read_unnamed_integer(7)?;
        self.m_bomb_fuse_time = bitstream.read_unnamed_integer(7)?;
        self.m_bomb_reset_time = bitstream.read_unnamed_integer(9)?;
        self.m_carrier_traits.decode(bitstream)?;
        self.m_arming_traits.decode(bitstream)?;

        Ok(())
    }
}