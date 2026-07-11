use serde::{Deserialize, Serialize};
use num_derive::{FromPrimitive, ToPrimitive};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::string_table::{c_single_language_string_table, c_string_table};
use blf_lib_derivable::result::BLFLibResult;
use crate::types::array::StaticArray;
use serde_hex::{SerHex,StrictCap};

pub const k_game_variant_team_count: usize = 8;

/// Team designator switch mode (`m_designator_switch_type`, 2 bits).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_game_engine_team_options_designator_switch_type {
    #[default]
    none = 0,
    random = 1,
    rotate = 2,
}

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(4)]
pub enum e_multiplayer_team_designator {
    #[default]
    none = -1,
    defenders = 0,
    attackers = 1,
    third_party = 2,
    fourth_party = 3,
    fifth_party = 4,
    sixth_party = 5,
    seventh_party = 6,
    eighth_party = 7,
    neutral = 8,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_team_options_team {
    pub m_team_enabled: bool,
    pub m_override_color_armour: bool,
    pub m_override_color_ui_text: bool,
    pub m_override_color_ui_bitmap: bool,
    pub m_name: c_string_table<1, 32, 5, 6, 1>,
    pub m_team_initial_designator: u8,
    pub m_model_override: u8,
    #[serde(with = "SerHex::<StrictCap>")]
    pub m_team_color_override: u32,
    #[serde(with = "SerHex::<StrictCap>")]
    pub m_team_ui_text_tint_color_override: u32,
    #[serde(with = "SerHex::<StrictCap>")]
    pub m_team_ui_bitmap_tint_color_override: u32,
    pub m_fireteam_count: u8,
}

impl c_game_engine_team_options_team {
    pub fn initialize(&mut self, team_index: usize) {
        *self = Self::default();
        self.m_team_enabled = true;
        self.m_team_color_override = 0xFFFF_FFFF;
        self.m_team_ui_text_tint_color_override = 0xFFFF_FFFF;
        self.m_team_ui_bitmap_tint_color_override = 0xFFFF_FFFF;
        self.m_team_initial_designator = (team_index + 1).min(8) as u8;
        self.m_fireteam_count = 1;
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_override_color_ui_bitmap)?;
        bitstream.write_bool(self.m_override_color_ui_text)?;
        bitstream.write_bool(self.m_override_color_armour)?;
        bitstream.write_bool(self.m_team_enabled)?;
        self.m_name.encode(bitstream)?;
        bitstream.write_integer(self.m_team_initial_designator, 4)?;
        bitstream.write_integer(self.m_model_override, 1)?;
        bitstream.write_integer(self.m_team_color_override, 32)?;
        bitstream.write_integer(self.m_team_ui_text_tint_color_override, 32)?;
        bitstream.write_integer(self.m_team_ui_bitmap_tint_color_override, 32)?;
        bitstream.write_integer(self.m_fireteam_count, 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_override_color_ui_bitmap = bitstream.read_bool("team-override-ui-bitmap-color")?;
        self.m_override_color_ui_text = bitstream.read_bool("team-override-ui-text-color")?;
        self.m_override_color_armour = bitstream.read_bool("team-override-armour-color")?;
        self.m_team_enabled = bitstream.read_bool("team-enabled")?;
        self.m_name.decode(bitstream)?;
        self.m_team_initial_designator = bitstream.read_integer("team-initial-designator", 4)?;
        self.m_model_override = bitstream.read_integer("team-model-override", 1)?;
        self.m_team_color_override = bitstream.read_integer("team-color-override", 32)?;
        self.m_team_ui_text_tint_color_override = bitstream.read_integer("team-ui-text-tint-color-override", 32)?;
        self.m_team_ui_bitmap_tint_color_override = bitstream.read_integer("team-ui-bitmap-tint-color-override", 32)?;
        self.m_fireteam_count = bitstream.read_integer("fireteam-count", 5)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_team_options {
    pub m_model_override: u16,
    pub m_designator_switch_type: e_game_engine_team_options_designator_switch_type,
    pub m_teams: StaticArray<c_game_engine_team_options_team, k_game_variant_team_count>,
}

impl c_game_engine_team_options {
    pub fn initialize(&mut self) {
        *self = Self::default();
        self.m_designator_switch_type = e_game_engine_team_options_designator_switch_type::rotate;
        for i in 0..k_game_variant_team_count {
            self.m_teams[i].initialize(i);
        }
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_model_override, 3)?;
        bitstream.write_enum(self.m_designator_switch_type)?;
        for i in 0..k_game_variant_team_count {
            self.m_teams[i].encode(bitstream)?
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_model_override = bitstream.read_integer("model-override", 3)?;
        self.m_designator_switch_type = bitstream.read_enum("designator-switch-type")?;
        for team in self.m_teams.get_mut().iter_mut() {
            team.decode(bitstream)?
        }

        Ok(())
    }
}
