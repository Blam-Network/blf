use binrw::{BinRead, BinWrite};
#[cfg(feature = "napi")]
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::blam::halo3::release::game::game_engine_player_traits::c_player_traits;
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize, BinRead, BinWrite)]
#[cfg_attr(feature = "napi", napi(object, namespace = "halo3_12070_08_09_05_2031_halo3_ship"))]
pub struct c_game_engine_slayer_variant {
    pub m_score_to_win: i16,
    pub m_kill_points: i16,
    pub m_assist_points: i8,
    pub m_death_points: i8,
    pub m_suicide_points: i8,
    pub m_betrayal_points: i8,
    pub m_leader_killed_points: i8,
    pub m_elimination_points: i8,
    pub m_assassination_points: i8,
    pub m_headshot_points: i8,
    pub m_melee_points: i8,
    pub m_sticky_points: i8,
    pub m_splatter_points: i8,
    pub m_killing_spree_points: i8,
    pub m_leader_traits: c_player_traits,
}

impl c_game_engine_slayer_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_signed_integer(self.m_score_to_win as i32, 10)?;
        bitstream.write_signed_integer(self.m_kill_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_assist_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_death_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_suicide_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_betrayal_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_leader_killed_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_elimination_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_assassination_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_headshot_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_melee_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_sticky_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_splatter_points as i32, 5)?;
        bitstream.write_signed_integer(self.m_killing_spree_points as i32, 5)?;
        self.m_leader_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_score_to_win = bitstream.read_unnamed_signed_integer(10)?;
        self.m_kill_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_assist_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_death_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_suicide_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_betrayal_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_leader_killed_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_elimination_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_assassination_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_headshot_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_melee_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_sticky_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_splatter_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_killing_spree_points = bitstream.read_unnamed_signed_integer(5)?;
        self.m_leader_traits.decode(bitstream)?;

        Ok(())
    }
}