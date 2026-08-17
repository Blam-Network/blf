use serde::{Deserialize, Serialize};
use num_derive::{FromPrimitive, ToPrimitive};
use blf_lib::bitfield;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::string_table::c_string_table;
use blf_lib_derivable::result::BLFLibResult;
use crate::types::array::StaticArray;
use serde_hex::{SerHex, StrictCap};

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

/// `e_game_engine_team_options_model_override_type` (ManagedMegalo 0..4), 3 bits.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(3)]
pub enum e_game_engine_team_options_model_override_type {
    #[default]
    player_preference = 0,
    all_spartans = 1,
    all_elites = 2,
    use_team_species = 3,
    by_designator = 4,
    unknown_5 = 5,
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(1)]
pub enum e_player_model_choice {
    #[default]
    spartan = 0,
    elite = 1,
}

/// `e_player_color_index` (none=-1 .. 31), 6 bits on wire via enum index.
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(6)]
pub enum e_player_color_index {
    #[default]
    none = -1,
    color_0 = 0,
    color_1 = 1,
    color_2 = 2,
    color_3 = 3,
    color_4 = 4,
    color_5 = 5,
    color_6 = 6,
    color_7 = 7,
    color_8 = 8,
    color_9 = 9,
    color_10 = 10,
    color_11 = 11,
    color_12 = 12,
    color_13 = 13,
    color_14 = 14,
    color_15 = 15,
    color_16 = 16,
    color_17 = 17,
    color_18 = 18,
    color_19 = 19,
    color_20 = 20,
    color_21 = 21,
    color_22 = 22,
    color_23 = 23,
    color_24 = 24,
    color_25 = 25,
    color_26 = 26,
    color_27 = 27,
    color_28 = 28,
    color_29 = 29,
    color_30 = 30,
    color_31 = 31,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_emblem_info {
    pub m_foreground_emblem_index: u8,
    pub m_background_emblem_index: u8,
    pub m_flags: u8,
    pub m_primary_color: e_player_color_index,
    pub m_secondary_color: e_player_color_index,
    pub m_background_color: e_player_color_index,
}

impl s_emblem_info {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_foreground_emblem_index, 8)?;
        bitstream.write_integer(self.m_background_emblem_index, 8)?;
        bitstream.write_integer(self.m_flags, 3)?;
        bitstream.write_enum(self.m_primary_color)?;
        bitstream.write_enum(self.m_secondary_color)?;
        bitstream.write_enum(self.m_background_color)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_foreground_emblem_index =
            bitstream.read_integer("foreground-emblem-index", 8)?;
        self.m_background_emblem_index =
            bitstream.read_integer("background-emblem-index", 8)?;
        self.m_flags = bitstream.read_integer("emblem-info-flags", 3)?;
        self.m_primary_color = bitstream.read_enum("primary-color")?;
        self.m_secondary_color = bitstream.read_enum("secondary-color")?;
        self.m_background_color = bitstream.read_enum("background-color")?;
        Ok(())
    }
}

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct e_game_engine_team_options_team_flags: u8 {
        enabled,
        override_primary_color,
        override_secondary_color,
        override_ui_text_color,
        override_ui_bitmap_color,
        unknown_5,
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_team_options_team {
    pub m_flags: e_game_engine_team_options_team_flags,
    pub m_name: c_string_table<1, 544, 10, 10, 1>,
    pub m_team_initial_designator: e_multiplayer_team_designator,
    pub m_model_override: e_player_model_choice,
    #[serde(with = "SerHex::<StrictCap>")]
    pub m_primary_color_override: u32,
    #[serde(with = "SerHex::<StrictCap>")]
    pub m_secondary_color_override: u32,
    #[serde(with = "SerHex::<StrictCap>")]
    pub m_team_ui_text_tint_color_override: u32,
    #[serde(with = "SerHex::<StrictCap>")]
    pub m_team_ui_bitmap_tint_color_override: u32,
    pub m_fireteam_count: u8,
    pub m_emblem: s_emblem_info,
}

impl c_game_engine_team_options_team {
    pub fn initialize(&mut self, team_index: usize) {
        *self = Self::default();
        self.m_flags.enabled = true;
        self.m_primary_color_override = 0xFFFF_FFFF;
        self.m_secondary_color_override = 0xFFFF_FFFF;
        self.m_team_ui_text_tint_color_override = 0xFFFF_FFFF;
        self.m_team_ui_bitmap_tint_color_override = 0xFFFF_FFFF;
        self.m_team_initial_designator = match team_index {
            0 => e_multiplayer_team_designator::defenders,
            1 => e_multiplayer_team_designator::attackers,
            2 => e_multiplayer_team_designator::third_party,
            3 => e_multiplayer_team_designator::fourth_party,
            4 => e_multiplayer_team_designator::fifth_party,
            5 => e_multiplayer_team_designator::sixth_party,
            6 => e_multiplayer_team_designator::seventh_party,
            7 => e_multiplayer_team_designator::eighth_party,
            _ => e_multiplayer_team_designator::none,
        };
        self.m_model_override = e_player_model_choice::spartan;
        self.m_fireteam_count = 1;
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_flags.to_raw(), 6)?;
        self.m_name.encode(bitstream)?;
        bitstream.write_enum(self.m_team_initial_designator)?;
        bitstream.write_enum(self.m_model_override)?;
        bitstream.write_integer(self.m_primary_color_override, 32)?;
        bitstream.write_integer(self.m_secondary_color_override, 32)?;
        bitstream.write_integer(self.m_team_ui_text_tint_color_override, 32)?;
        bitstream.write_integer(self.m_team_ui_bitmap_tint_color_override, 32)?;
        bitstream.write_integer(self.m_fireteam_count, 5)?;
        self.m_emblem.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_flags =
            e_game_engine_team_options_team_flags::from_raw(bitstream.read_integer("team-flags", 6)?);
        self.m_name.decode(bitstream)?;
        self.m_team_initial_designator = bitstream.read_enum("team-initial-designator")?;
        self.m_model_override = bitstream.read_enum("team-model-override")?;
        self.m_primary_color_override = bitstream.read_integer("primary-color-override", 32)?;
        self.m_secondary_color_override = bitstream.read_integer("secondary-color-override", 32)?;
        self.m_team_ui_text_tint_color_override =
            bitstream.read_integer("team-ui-text-tint-color-override", 32)?;
        self.m_team_ui_bitmap_tint_color_override =
            bitstream.read_integer("team-ui-bitmap-tint-color-override", 32)?;
        self.m_fireteam_count = bitstream.read_integer("fireteam-count", 5)?;
        self.m_emblem.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_team_options {
    pub m_model_override: e_game_engine_team_options_model_override_type,
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
        bitstream.write_enum(self.m_model_override)?;
        bitstream.write_enum(self.m_designator_switch_type)?;
        for i in 0..k_game_variant_team_count {
            self.m_teams[i].encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_model_override = bitstream.read_enum("model-override")?;
        self.m_designator_switch_type = bitstream.read_enum("designator-switch-type")?;
        for team in self.m_teams.get_mut().iter_mut() {
            team.decode(bitstream)?;
        }

        Ok(())
    }
}
