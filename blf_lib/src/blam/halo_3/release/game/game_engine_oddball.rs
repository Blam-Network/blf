use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo_3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::{SET_BIT, TEST_BIT};

#[derive(Default, PartialEq, Debug, Clone, BinRead, BinWrite, Serialize, Deserialize)]
pub struct c_game_engine_oddball_variant {
    m_variant_flags: u32,
    m_score_to_win: i16,
    m_carrying_points: i16,
    m_kill_points: i8,
    m_ball_kill_points: i8,
    m_carrier_kill_points: i8,
    m_ball_count: u8,
    m_ball_spawn_delay: u16,
    m_ball_inactive_respawn_delay: u16,
    #[brw(pad_after = 2)]
    m_carrier_traits: c_player_traits,
    // #[serde(skip_serializing,skip_deserializing)]
    // m_pad1: StaticArray<u8, 2>,
}

impl c_game_engine_oddball_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) {
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 0));
        bitstream.write_bool(TEST_BIT!(self.m_variant_flags, 1));
        bitstream.write_signed_integer(self.m_score_to_win as i32, 11);
        bitstream.write_signed_integer(self.m_carrying_points as i32, 5);
        bitstream.write_signed_integer(self.m_kill_points as i32, 5);
        bitstream.write_signed_integer(self.m_ball_kill_points as i32, 5);
        bitstream.write_signed_integer(self.m_carrier_kill_points as i32, 5);
        bitstream.write_integer(self.m_ball_count as u32, 2);
        bitstream.write_integer(self.m_ball_spawn_delay as u32, 7);
        bitstream.write_integer(self.m_ball_inactive_respawn_delay as u32, 7);
        self.m_carrier_traits.encode(bitstream);
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) {
        SET_BIT!(self.m_variant_flags, 0, bitstream.read_bool());
        SET_BIT!(self.m_variant_flags, 1, bitstream.read_bool());
        self.m_score_to_win = bitstream.read_signed_integer(11) as i16;
        self.m_carrying_points = bitstream.read_signed_integer(5) as i16;
        self.m_kill_points = bitstream.read_signed_integer(5) as i8;
        self.m_ball_kill_points = bitstream.read_signed_integer(5) as i8;
        self.m_carrier_kill_points = bitstream.read_signed_integer(5) as i8;
        self.m_ball_count = bitstream.read_u8(2);
        self.m_ball_spawn_delay = bitstream.read_u16(7);
        self.m_ball_inactive_respawn_delay = bitstream.read_u16(7);
        self.m_carrier_traits.decode(bitstream);
    }
}