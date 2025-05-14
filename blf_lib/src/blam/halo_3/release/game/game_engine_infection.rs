use binrw::{BinRead, BinWrite};
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::SET_BIT;
use crate::TEST_BIT;

#[derive(Default, PartialEq, Debug, Clone, BinRead, BinWrite, Serialize, Deserialize)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_game_engine_infection_variant {
    pub m_variant_flags: u8,
    pub m_safe_havens: u8,
    pub m_next_zombie: u8,
    pub m_initial_zombie_count: u8,
    pub m_safe_haven_movement_time: u16,
    pub m_zombie_kill_points: i8,
    pub m_infection_points: i8,
    pub m_safe_haven_arrival_points: i8,
    pub m_suicide_points: i8,
    pub m_betrayal_points: i8,
    pub m_last_man_bonus_points: i8,
    pub m_zombie_traits: c_player_traits,
    pub m_first_zombie_traits: c_player_traits,
    pub m_safe_haven_defender_traits: c_player_traits,
    #[brw(pad_after = 4)]
    pub m_last_human_traits: c_player_traits,
}

impl c_game_engine_infection_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0));
        bitstream.write_integer(self.m_safe_havens as u32, 2);
        bitstream.write_integer(self.m_next_zombie as u32, 2);
        bitstream.write_integer(self.m_initial_zombie_count as u32, 5);
        bitstream.write_integer(self.m_safe_haven_movement_time as u32, 7);
        bitstream.write_signed_integer(self.m_zombie_kill_points as i32, 5);
        bitstream.write_signed_integer(self.m_infection_points as i32, 5);
        bitstream.write_signed_integer(self.m_safe_haven_arrival_points as i32, 5);
        bitstream.write_signed_integer(self.m_suicide_points as i32, 5);
        bitstream.write_signed_integer(self.m_betrayal_points as i32, 5);
        bitstream.write_signed_integer(self.m_last_man_bonus_points as i32, 5);
        self.m_zombie_traits.encode(bitstream);
        self.m_first_zombie_traits.encode(bitstream);
        self.m_last_human_traits.encode(bitstream);
        self.m_safe_haven_defender_traits.encode(bitstream);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_bool());
        self.m_safe_havens = bitstream.read_u8(2);
        self.m_next_zombie = bitstream.read_u8(2);
        self.m_initial_zombie_count = bitstream.read_u8(5);
        self.m_safe_haven_movement_time = bitstream.read_u16(7);
        self.m_zombie_kill_points = bitstream.read_signed_integer(5) as i8;
        self.m_infection_points = bitstream.read_signed_integer(5) as i8;
        self.m_safe_haven_arrival_points = bitstream.read_signed_integer(5) as i8;
        self.m_suicide_points = bitstream.read_signed_integer(5) as i8;
        self.m_betrayal_points = bitstream.read_signed_integer(5) as i8;
        self.m_last_man_bonus_points = bitstream.read_signed_integer(5) as i8;
        self.m_zombie_traits.decode(bitstream);
        self.m_first_zombie_traits.decode(bitstream);
        self.m_last_human_traits.decode(bitstream);
        self.m_safe_haven_defender_traits.decode(bitstream);
    }
}