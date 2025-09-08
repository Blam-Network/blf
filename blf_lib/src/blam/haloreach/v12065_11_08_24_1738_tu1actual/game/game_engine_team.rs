use serde::{Deserialize, Serialize};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::string_table::c_single_language_string_table;
use blf_lib_derivable::result::BLFLibResult;
use crate::types::array::StaticArray;
use serde_hex::SerHex;

pub const k_game_variant_team_count: usize = 8;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_team_options_team {
    pub m_team_enabled: bool,
    pub m_override_color_armour: bool,
    pub m_override_color_ui_text: bool,
    pub m_override_color_ui_bitmap: bool,
    pub m_name: c_single_language_string_table<1, 32, 5, 6, 1>,
    pub m_team_initial_designator: u8,
    pub m_model_override: u8,
    pub m_team_color_override: i32,
    pub m_team_ui_text_tint_color_override: i32,
    pub m_team_ui_bitmap_tint_color_override: i32,
    pub m_fireteam_count: u8,
}

impl c_game_engine_team_options_team {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_team_enabled)?;
        bitstream.write_bool(self.m_override_color_armour)?;
        bitstream.write_bool(self.m_override_color_ui_text)?;
        bitstream.write_bool(self.m_override_color_ui_bitmap)?;
        self.m_name.encode(bitstream)?;
        bitstream.write_integer(self.m_team_initial_designator, 4)?;
        bitstream.write_integer(self.m_model_override, 1)?;
        bitstream.write_signed_integer(self.m_team_color_override, 32)?;
        bitstream.write_signed_integer(self.m_team_ui_text_tint_color_override, 32)?;
        bitstream.write_signed_integer(self.m_team_ui_bitmap_tint_color_override, 32)?;
        bitstream.write_integer(self.m_fireteam_count, 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team_enabled = bitstream.read_bool("team-enabled")?;
        self.m_override_color_armour = bitstream.read_bool("team-override-armour-color")?;
        self.m_override_color_ui_text = bitstream.read_bool("team-override-ui-text-color")?;
        self.m_override_color_ui_bitmap = bitstream.read_bool("team-override-ui-bitmap-color")?;
        self.m_name.decode(bitstream)?;
        self.m_team_initial_designator = bitstream.read_integer("team-initial-designator", 4)?;
        self.m_model_override = bitstream.read_integer("team-model-override", 1)?;
        self.m_team_color_override = bitstream.read_signed_integer("team-color-override", 32)?;
        self.m_team_ui_text_tint_color_override = bitstream.read_signed_integer("team-ui-text-tint-color-override", 32)?;
        self.m_team_ui_bitmap_tint_color_override = bitstream.read_signed_integer("team-ui-bitmap-tint-color-override", 32)?;
        self.m_fireteam_count = bitstream.read_integer("fireteam-count", 5)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_team_options {
    pub m_model_override: u16,
    pub m_designator_switch_type: u16,
    pub m_teams: StaticArray<c_game_engine_team_options_team, k_game_variant_team_count>,
}

impl c_game_engine_team_options {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_model_override, 3)?;
        bitstream.write_integer(self.m_designator_switch_type, 2)?;
        for i in 0..k_game_variant_team_count {
            self.m_teams[i].encode(bitstream)?
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_designator_switch_type = bitstream.read_integer("designator-switch-type", 2)?;
        self.m_model_override = bitstream.read_integer("model-override", 3)?;
        for team in self.m_teams.get_mut().iter_mut() {
            team.decode(bitstream)?
        }

        Ok(())
    }
}
