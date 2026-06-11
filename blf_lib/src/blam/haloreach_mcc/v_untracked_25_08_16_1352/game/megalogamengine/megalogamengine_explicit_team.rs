use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_explicit_team_type {
    #[default]
    no_team = 0,
    team_0 = 1,
    team_1 = 2,
    team_2 = 3,
    team_3 = 4,
    team_4 = 5,
    team_5 = 6,
    team_6 = 7,
    team_7 = 8,
    neutral_team = 9,
    global_0 = 10,
    global_1 = 11,
    global_2 = 12,
    global_3 = 13,
    global_4 = 14,
    global_5 = 15,
    global_6 = 16,
    global_7 = 17,
    current = 18,
    hud_player_owner_team = 19, // local_team
    hud_target_player_owner_team = 20, // target_team: the team designator for `current_target_player` if it exists or `current_target_object` otherwise, per 343i
    temporary_0 = 21,
    temporary_1 = 22,
    temporary_2 = 23,
    temporary_3 = 24,
    temporary_4 = 25,
    temporary_5 = 26,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_explicit_team {
    pub m_explicit_team_type: e_explicit_team_type, // 5 bits
}

impl c_explicit_team {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_explicit_team_type, 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_explicit_team_type = bitstream.read_enum_raw("explicit-team-type", 5)?;

        Ok(())
    }
}
