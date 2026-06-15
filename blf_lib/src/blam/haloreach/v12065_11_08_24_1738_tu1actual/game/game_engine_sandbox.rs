use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::bitfield;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_player_traits::c_player_traits;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_variant::c_game_engine_custom_variant;

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct e_sandbox_variant_flags: u8 {
        open_channel_voice,
        requires_all_objects,
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_sandbox_edit_mode_settings {
    #[default]
    all_players = 0,
    only_leader = 1,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_sandbox_variant {
    pub m_custom_variant: c_game_engine_custom_variant,
    pub m_variant_flags: e_sandbox_variant_flags,
    pub m_edit_mode: e_sandbox_edit_mode_settings,
    pub m_respawn_time: u8,
    pub m_editor_traits: c_player_traits,
}

impl c_game_engine_sandbox_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_custom_variant.encode(bitstream)?;
        bitstream.write_integer(self.m_variant_flags.to_raw(), 2)?;
        bitstream.write_enum(self.m_edit_mode)?;
        bitstream.write_integer(self.m_respawn_time, 6)?;
        self.m_editor_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_custom_variant.decode(bitstream)?;
        self.m_variant_flags =
            e_sandbox_variant_flags::from_raw(bitstream.read_integer("variant-flags", 2)?);
        self.m_edit_mode = bitstream.read_enum("edit-mode")?;
        self.m_respawn_time = bitstream.read_integer("respawn-time", 6)?;
        self.m_editor_traits.decode(bitstream)?;

        Ok(())
    }
}
