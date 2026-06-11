use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_explicit_player_type {
    #[default]
    no_player = 0,
    player_0 = 1,
    player_1 = 2,
    player_2 = 3,
    player_3 = 4,
    player_4 = 5,
    player_5 = 6,
    player_6 = 7,
    player_7 = 8,
    player_8 = 9,
    player_9 = 10,
    player_10 = 11,
    player_11 = 12,
    player_12 = 13,
    player_13 = 14,
    player_14 = 15,
    player_15 = 16,
    global_0 = 17,
    global_1 = 18,
    global_2 = 19,
    global_3 = 20,
    global_4 = 21,
    global_5 = 22,
    global_6 = 23,
    global_7 = 24,
    current = 25,
    hud = 26, // local_player
    hud_target = 27, // target_player
    killer = 28,
    temporary_0 = 29,
    temporary_1 = 30,
    temporary_2 = 31,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_explicit_player {
    pub m_explicit_player_type: e_explicit_player_type, // 5 bits
}

impl c_explicit_player {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_explicit_player_type, 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_explicit_player_type = bitstream.read_enum_raw("explicit-player-type", 5)?;

        Ok(())
    }
}
