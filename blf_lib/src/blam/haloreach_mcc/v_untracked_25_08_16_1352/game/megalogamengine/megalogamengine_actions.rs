use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_text::c_dynamic_string;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_variant_variable::s_variant_variable;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_hud_widgets::e_megalogamengine_hud_meter_input_type;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::game_engine_megalo::e_weapon_pickup_priority;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::game_engine_timer::e_game_engine_timer_rate;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::scenario_map_variant::e_boundary_shape;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_sounds::e_megalo_sound;
use blf_lib::bitfield;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::OPTION_TO_RESULT;
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use crate::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_object_reference::c_object_reference;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_action_team_or_player_target {
    #[default]
    team = 0,
    player = 1,
    everyone = 2,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_team_or_player_target {
    pub m_target: e_action_team_or_player_target, // 2 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_team_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_player_reference>
}

impl s_team_or_player_target {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_target, 2)?;
        match (self.m_target, &self.m_team, &self.m_player) {
            (e_action_team_or_player_target::team, Some(team), None) => {
                team.encode(bitstream)?;
            }
            (e_action_team_or_player_target::player, None, Some(player)) => {
                player.encode(bitstream)?;
            }
            (e_action_team_or_player_target::everyone, None, None) => {}
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_target = bitstream.read_enum_raw("target", 2)?;
        match self.m_target {
            e_action_team_or_player_target::team => {
                let mut team = c_team_reference::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
            }
            e_action_team_or_player_target::player => {
                let mut player = c_player_reference::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            e_action_team_or_player_target::everyone => {}
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_score_parameters {
    pub m_target: s_team_or_player_target,
    pub m_operation: e_math_operation, // 4 bits
    pub m_variable: c_custom_variable_reference
}

impl s_action_set_score_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_target.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_target.decode(bitstream)?;
        self.m_operation = bitstream.read_enum_raw("operation", 4)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct e_create_object_flags: u8 {
        never_garbage_collect,
        suppress_effect,
        absolute_orientation,
    }
}

/// Three signed bytes written as 24 raw bits (`write_raw_data` in managedmegalo).
/// Engine scales each axis by 0.1 in `get_offset_relative_to_forward_and_up`.
/// Wire packing via `write_integer` keeps z in the low byte (matches existing MCC decode).
#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct s_object_offset {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl s_object_offset {
    pub fn to_raw(self) -> u32 {
        (self.z as u8 as u32)
            | ((self.y as u8 as u32) << 8)
            | ((self.x as u8 as u32) << 16)
    }

    pub fn from_raw(raw: u32) -> Self {
        Self {
            z: raw as u8 as i8,
            y: (raw >> 8) as u8 as i8,
            x: (raw >> 16) as u8 as i8,
        }
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.to_raw(), 24)
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        *self = Self::from_raw(bitstream.read_integer("offset", 24)?);
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_create_object_parameters {
    pub m_object_type: c_object_type_reference,
    pub m_object_reference_1: c_object_reference,
    pub m_object_reference_2: c_object_reference,
    pub m_filter_index: i8, // 4 bits
    pub m_flags: e_create_object_flags, // 3 bits
    pub m_offset: s_object_offset, // 24 bits
    pub m_variant_name_index: u8, // 8 bits
}

impl s_action_create_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_type.encode(bitstream)?;
        self.m_object_reference_1.encode(bitstream)?;
        self.m_object_reference_2.encode(bitstream)?;
        bitstream.write_index::<16>(self.m_filter_index, 4)?;
        bitstream.write_integer(self.m_flags.to_raw(), 3)?;
        self.m_offset.encode(bitstream)?;
        bitstream.write_integer(self.m_variant_name_index, 8)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_type.decode(bitstream)?;
        self.m_object_reference_1.decode(bitstream)?;
        self.m_object_reference_2.decode(bitstream)?;
        self.m_filter_index = bitstream.read_index::<16>("filter_index", 4)? as i8;
        self.m_flags = e_create_object_flags::from_raw(bitstream.read_integer("flags", 3)?);
        self.m_offset.decode(bitstream)?;
        self.m_variant_name_index = bitstream.read_integer("variant-name-index", 8)?;

        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_player_filter_type {
    #[default]
    no_one = 0,
    everyone = 1,
    allies = 2, // for teams
    enemies = 3, // for teams
    specific_player = 4,
    normal = 5,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_filter_modifier {
    pub m_type: e_player_filter_type, // 3 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_player_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable: Option<c_custom_variable_reference>,
}

impl c_player_filter_modifier {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_type, 3)?;
        match (self.m_type, &self.m_player, &self.m_variable) {
            (e_player_filter_type::specific_player, Some(player), Some(variable)) => {
                player.encode(bitstream)?;
                variable.encode(bitstream)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_enum_raw("type", 3)?;
        if self.m_type == e_player_filter_type::specific_player {
            let mut player = c_player_reference::default();
            let mut variable = c_custom_variable_reference::default();
            player.decode(bitstream)?;
            variable.decode(bitstream)?;
            self.m_player = Some(player);
            self.m_variable = Some(variable);
        }

        Ok(())
    }
}

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize, crate::derive::c_enum)]
#[bits(5)]
pub enum e_chud_navpoint_icon_type {
    none = -1,
    #[default]
    speaker = 0,
    dead_teammate = 1,
    unused = 2,
    target = 3,
    destination = 4,
    bomb = 5,
    flag = 6,
    skull = 7,
    king = 8,
    vip = 9,
    lock = 10,
    num = 11,
    num_1 = 12,
    num_2 = 13,
    num_3 = 14,
    num_4 = 15,
    num_5 = 16,
    num_6 = 17,
    num_7 = 18,
    num_8 = 19,
    num_9 = 20,
    ordnance = 21,
    interface = 22,
    recon = 23,
    ammunition = 24,
    recover = 25,
    defend = 26,
    neutralize = 27,
    coop_spawning = 28,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_navpoint_set_icon_parameters {
    pub m_object: c_object_reference,
    pub m_navpoint_icon: e_chud_navpoint_icon_type, // 5 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_navpoint_number: Option<c_custom_variable_reference>,
}

impl s_action_navpoint_set_icon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum(self.m_navpoint_icon)?;

        match (self.m_navpoint_icon, &self.m_navpoint_number) {
            (e_chud_navpoint_icon_type::num, Some(navpoint_number)) => {
                navpoint_number.encode(bitstream)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_navpoint_icon = bitstream.read_enum("navpoint-icon")?;

        if self.m_navpoint_icon == e_chud_navpoint_icon_type::num {
            let mut navpoint_number = c_custom_variable_reference::default();
            navpoint_number.decode(bitstream)?;
            self.m_navpoint_number = Some(navpoint_number);
        }

        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_navpoint_priority {
    #[default]
    low = 0,
    normal = 1,
    high = 2,
    blink = 3,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_navpoint_set_priority_parameters {
    pub m_object: c_object_reference,
    pub m_priority: e_navpoint_priority, // 2 bits
}

impl s_action_navpoint_set_priority_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_priority, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_priority = bitstream.read_enum_raw("priority", 2)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_navpoint_set_timer_parameters {
    pub m_object: c_object_reference,
    pub m_timer_index: i8, // 2 bits
}

impl s_action_navpoint_set_timer_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_index::<4>(self.m_timer_index, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_timer_index = bitstream.read_index::<4>("timer-index", 2)? as i8;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_navpoint_set_visible_range_parameters {
    pub m_object: c_object_reference,
    pub m_variable_1: c_custom_variable_reference,
    pub m_variable_2: c_custom_variable_reference,
}

impl s_action_navpoint_set_visible_range_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable_1.encode(bitstream)?;
        self.m_variable_2.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable_1.decode(bitstream)?;
        self.m_variable_2.decode(bitstream)?;

        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_math_operation {
    #[default]
    add = 0, // +=
    subtract = 1, // -=
    multiply = 2, // *=
    divide = 3, // /=
    set_to = 4, // =
    modulo = 5, // %=
    and = 6, // &=
    or = 7, // |=
    xor = 8, // ^=
    not = 9, // ~= (a not b) == (a &= ~b)
    lshift = 10, // <<=
    rshift = 11, // >>=
    abs = 12, // abs=
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_grenade_type {
    #[default]
    frag_grenade = 0,
    plasma_grenade = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_biped_give_weapon_mode {
    #[default]
    primary = 0,
    secondary = 1,
    force = 2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_scriptable_game_buttons {
    #[default]
    jump = 0,
    grenade = 1,
    switch_weapon = 2,
    context_primary = 3,
    melee_attack = 4,
    equipment = 5,
    throw_grenade = 6,
    fire_primary = 7,
    crouch = 8,
    scope_zoom = 9,
    night_vision = 10,
    fire_secondary = 11,
    fire_tertiary = 12,
    vehicle_trick = 13,
    // These are not supported by MegaloEdit.exe
    unknown = 14,
    unknown_1 = 15,
    unknown_2 = 16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_parameters {
    pub m_variable_1: s_variant_variable,
    pub m_variable_2: s_variant_variable,
    pub m_operation: e_math_operation, // 4 bits
}

impl s_action_set_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_variable_1.encode(bitstream)?;
        self.m_variable_2.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_operation, 4)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_variable_1.decode(bitstream)?;
        self.m_variable_2.decode(bitstream)?;
        self.m_operation = bitstream.read_enum_raw("operation", 4)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_boundary_parameters {
    pub m_object: c_object_reference,
    pub m_shape: e_boundary_shape, // 2 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_1: Option<c_custom_variable_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_2: Option<c_custom_variable_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_3: Option<c_custom_variable_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_4: Option<c_custom_variable_reference>,
}

impl s_action_set_boundary_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_shape, 2)?;
        match (self.m_shape, &self.m_variable_1, &self.m_variable_2, &self.m_variable_3, &self.m_variable_4) {
            (e_boundary_shape::sphere, Some(radius), None, None, None) => {
                radius.encode(bitstream)?;
            }
            (e_boundary_shape::cylinder, Some(variable1), Some(variable2), Some(variable3), None) => {
                variable1.encode(bitstream)?;
                variable2.encode(bitstream)?;
                variable3.encode(bitstream)?;
            }
            (e_boundary_shape::r#box, Some(variable1), Some(variable2), Some(variable3), Some(variable4)) => {
                variable1.encode(bitstream)?;
                variable2.encode(bitstream)?;
                variable3.encode(bitstream)?;
                variable4.encode(bitstream)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_shape = bitstream.read_unnamed_enum_raw( 2)?;

        match self.m_shape {
            e_boundary_shape::sphere => {
                let mut radius = c_custom_variable_reference::default();
                radius.decode(bitstream)?;
                self.m_variable_1 = Some(radius);
            }
            e_boundary_shape::cylinder => {
                let mut variable1 = c_custom_variable_reference::default();
                let mut variable2 = c_custom_variable_reference::default();
                let mut variable3 = c_custom_variable_reference::default();
                variable1.decode(bitstream)?;
                variable2.decode(bitstream)?;
                variable3.decode(bitstream)?;
                self.m_variable_1 = Some(variable1);
                self.m_variable_2 = Some(variable2);
                self.m_variable_3 = Some(variable3);
            }
            e_boundary_shape::r#box => {
                let mut variable1 = c_custom_variable_reference::default();
                let mut variable2 = c_custom_variable_reference::default();
                let mut variable3 = c_custom_variable_reference::default();
                let mut variable4 = c_custom_variable_reference::default();
                variable1.decode(bitstream)?;
                variable2.decode(bitstream)?;
                variable3.decode(bitstream)?;
                variable4.decode(bitstream)?;
                self.m_variable_1 = Some(variable1);
                self.m_variable_2 = Some(variable2);
                self.m_variable_3 = Some(variable3);
                self.m_variable_4 = Some(variable4);
            }
            _ => {}
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_apply_player_traits_parameters {
    pub m_player: c_player_reference,
    pub m_trait_index: u8, // 4 bits
}

impl s_action_apply_player_traits_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_integer(self.m_trait_index, 4)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_trait_index = bitstream.read_integer("player-trait-index", 4)?;

        Ok(())
    }
}

bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct e_fireteam_filter_flags: u8 {
        fireteam1,
        fireteam2,
        fireteam3,
        fireteam4,
        fireteam5,
        fireteam6,
        fireteam7,
        fireteam8,
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_fireteam_respawn_filter_parameters {
    pub m_object: c_object_reference,
    pub m_fireteam_filter: e_fireteam_filter_flags, // 8 bits
}

impl s_action_set_fireteam_respawn_filter_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_integer(self.m_fireteam_filter.to_raw(), 8)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_fireteam_filter = e_fireteam_filter_flags::from_raw(
            bitstream.read_integer("fireteam-filter", 8)?,
        );

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_progress_bar_parameters {
    pub m_object: c_object_reference,
    pub m_player_filter_modifier: c_player_filter_modifier,
    pub m_timer_index: i8, // 2 bits
}

impl s_action_set_progress_bar_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_player_filter_modifier.encode(bitstream)?;
        bitstream.write_index::<4>(self.m_timer_index, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_player_filter_modifier.decode(bitstream)?;
        self.m_timer_index = bitstream.read_index::<4>("timer-index", 2)? as i8;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_hud_post_message_parameters {
    pub m_target: s_team_or_player_target,
    pub m_sound_index: e_megalo_sound, // 7 bits
    pub m_string: c_dynamic_string,
}

impl s_action_hud_post_message_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_target.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_sound_index, 7)?;
        self.m_string.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_target.decode(bitstream)?;
        self.m_sound_index = bitstream.read_enum_raw("sound-index", 7)?;
        self.m_string.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_timer_set_rate_parameters {
    pub m_timer: c_custom_timer_reference,
    pub m_rate: e_game_engine_timer_rate, // 5 bits
}

impl s_action_timer_set_rate_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_timer.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_rate, 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_timer.decode(bitstream)?;
        self.m_rate = bitstream.read_enum_raw("timer-rate", 5)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_for_each_parameters {
    pub m_trigger_index: u16, // 9 bits
}

impl s_action_for_each_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_trigger_index, 9)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_trigger_index = bitstream.read_integer("trigger-index", 9)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_destroy_parameters {
    pub m_object: c_object_reference,
    pub m_no_statistics: bool,
}

impl s_action_object_destroy_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_bool(self.m_no_statistics)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_no_statistics = bitstream.read_bool("no-statistics")?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_attach_parameters {
    pub m_object_1: c_object_reference,
    pub m_object_2: c_object_reference,
    pub m_offset: s_object_offset, // 24 bits
    pub m_absolute_orientation: bool,
}

impl s_action_object_attach_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        self.m_offset.encode(bitstream)?;
        bitstream.write_bool(self.m_absolute_orientation)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        self.m_offset.decode(bitstream)?;
        self.m_absolute_orientation = bitstream.read_bool("absolute_orientation")?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_adjust_money_parameters {
    pub m_player: c_player_reference,
    pub m_math_operation: e_math_operation,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_player_adjust_money_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_math_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_math_operation = bitstream.read_enum_raw("math-operation", 4)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


bitfield! {
    /// `c_flags<e_player_purchase_mode, unsigned char, 5>`.
    /// Script form: `{alive|dead|both} {weapons|equipment|vehicles|all}`.
    /// No dead+vehicles bit exists (that combination clears the mask).
    #[derive(Serialize, Deserialize)]
    pub struct e_player_purchase_mode_flags: u8 {
        alive_weapons,
        alive_equipment,
        alive_vehicles,
        dead_weapons,
        dead_equipment,
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_enable_purchases_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
    pub m_mode: e_player_purchase_mode_flags, // 5 bits
}

impl s_action_player_enable_purchases_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;
        bitstream.write_integer(self.m_mode.to_raw(), 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;
        self.m_mode = e_player_purchase_mode_flags::from_raw(
            bitstream.read_integer("mode", 5)?,
        );

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_weapon_set_pickup_priority_parameters {
    pub m_object: c_object_reference,
    pub m_weapon_pickup_priority: e_weapon_pickup_priority, // 2 bits
}

impl s_action_weapon_set_pickup_priority_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_weapon_pickup_priority, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_weapon_pickup_priority = bitstream.read_enum_raw("weapon-pickup-priority", 2)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_hud_widget_text_base {
    pub m_widget_index: i8, // 2 bits
    pub m_string: c_dynamic_string,
}

impl s_action_hud_widget_text_base {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<4>(self.m_widget_index, 2)?;
        self.m_string.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_widget_index = bitstream.read_index::<4>("widget-index", 2)? as i8;
        self.m_string.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_megalogamengine_hud_meter_input {
    pub m_type: e_megalogamengine_hud_meter_input_type, // 2 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_1: Option<c_custom_variable_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_2: Option<c_custom_variable_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_timer: Option<c_custom_timer_reference>,
}

impl c_megalogamengine_hud_meter_input {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        match (self.m_type, &self.m_variable_1, &self.m_variable_2, &self.m_timer) {
            (e_megalogamengine_hud_meter_input_type::number, Some(variable1), Some(variable2), None) => {
                bitstream.write_enum_raw(e_megalogamengine_hud_meter_input_type::number, 2)?;
                variable1.encode(bitstream)?;
                variable2.encode(bitstream)?;
            }
            (e_megalogamengine_hud_meter_input_type::timer, None, None, Some(timer)) => {
                bitstream.write_enum_raw(e_megalogamengine_hud_meter_input_type::timer, 2)?;
                timer.encode(bitstream)?;
            }
            (e_megalogamengine_hud_meter_input_type::none, _, _, _) => {}
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_enum_raw("type", 2)?;
        match self.m_type {
            e_megalogamengine_hud_meter_input_type::number => {
                let mut variable1 = c_custom_variable_reference::default();
                let mut variable2 = c_custom_variable_reference::default();
                variable1.decode(bitstream)?;
                variable2.decode(bitstream)?;
                self.m_variable_1 = Some(variable1);
                self.m_variable_2 = Some(variable2);
            }
            e_megalogamengine_hud_meter_input_type::timer => {
                let mut timer = c_custom_timer_reference::default();
                timer.decode(bitstream)?;
                self.m_timer = Some(timer);
            }
            e_megalogamengine_hud_meter_input_type::none => {}
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_hud_widget_set_meter_parameters {
    pub m_widget_index: i8, // 2 bits
    pub m_meter_input: c_megalogamengine_hud_meter_input,
}

impl s_action_hud_widget_set_meter_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<4>(self.m_widget_index, 2)?;
        self.m_meter_input.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_widget_index = bitstream.read_index::<4>("widget-index", 2)? as i8;
        self.m_meter_input.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_hud_widget_set_icon_parameters {
    pub m_widget_index: i8, // 2 bits
    pub m_icon_index: i8, // 6 bits
}

impl s_action_hud_widget_set_icon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<4>(self.m_widget_index, 2)?;
        bitstream.write_index::<64>(self.m_icon_index, 6)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_widget_index = bitstream.read_index::<4>("widget-index", 2)? as i8;
        self.m_icon_index = bitstream.read_index::<64>("icon-index", 6)? as i8;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_hud_widget_set_visibility_parameters {
    pub m_widget_index: i8, // 2 bits
    pub m_player: c_player_reference,
    pub m_visible: bool, // 6 bits
}

impl s_action_hud_widget_set_visibility_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<4>(self.m_widget_index, 2)?;
        self.m_player.encode(bitstream)?;
        bitstream.write_bool(self.m_visible)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_widget_index = bitstream.read_index::<4>("widget-index", 2)? as i8;
        self.m_player.decode(bitstream)?;
        self.m_visible = bitstream.read_bool("visible")?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_play_sound_parameters {
    pub m_sound_index: e_megalo_sound, // 7 bits
    pub m_immediate: bool,
    pub m_target: s_team_or_player_target,
}

impl s_action_play_sound_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_sound_index, 7)?;
        bitstream.write_bool(self.m_immediate)?;
        self.m_target.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_sound_index = bitstream.read_enum_raw("sound-index", 7)?;
        self.m_immediate = bitstream.read_bool("immediate")?;
        self.m_target.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_set_objective_allegiance_icon_parameters {
    pub m_player: c_player_reference,
    pub m_icon_index: i8, // 7 bits
}

impl s_action_player_set_objective_allegiance_icon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_index::<128>(self.m_icon_index, 7)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_icon_index = bitstream.read_index::<128>("icon-index", 7)? as i8;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_team_set_coop_spawning_parameters {
    pub m_team: c_team_reference,
    pub m_enabled: bool,
}

impl s_action_team_set_coop_spawning_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_team.encode(bitstream)?;
        bitstream.write_bool(self.m_enabled)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team.decode(bitstream)?;
        self.m_enabled = bitstream.read_bool("enabled")?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_vitality_adjustment_parameters {
    pub m_object: c_object_reference,
    pub m_operation: e_math_operation, // 4 bits
    pub m_variable: c_custom_variable_reference,
}

impl s_action_vitality_adjustment_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_operation = bitstream.read_enum_raw("operation", 4)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_get_distance_parameters {
    pub m_object_1: c_object_reference,
    pub m_object_2: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_get_distance_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_set_requisition_palette_parameters {
    pub m_player: c_player_reference,
    pub m_new_palette: u8, // 4 bits
}

impl s_action_player_set_requisition_palette_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_integer(self.m_new_palette, 4)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_new_palette = bitstream.read_integer("new-palette", 4)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_adjust_grenades_parameters {
    pub m_player: c_player_reference,
    pub m_grenade_type: e_grenade_type, // 1 bit
    pub m_math_operation: e_math_operation, // 4 bits
    pub m_variable: c_custom_variable_reference,
}

impl s_action_adjust_grenades_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_grenade_type, 1)?;
        bitstream.write_enum_raw(self.m_math_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_grenade_type = bitstream.read_enum_raw("grenade-type", 1)?;
        self.m_math_operation = bitstream.read_enum_raw("math-operation", 4)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_submit_incident_parameters {
    pub m_incident_id: u16, // 10 bits
    pub m_target_1: s_team_or_player_target,
    pub m_target_2: s_team_or_player_target,
}

impl s_action_submit_incident_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_incident_id, 10)?;
        self.m_target_1.encode(bitstream)?;
        self.m_target_2.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_incident_id = bitstream.read_integer("incident-id", 10)?;
        self.m_target_1.decode(bitstream)?;
        self.m_target_2.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_submit_incident_with_custom_value_parameters {
    pub m_incident_id: u16, // 10 bits
    pub m_target_1: s_team_or_player_target,
    pub m_target_2: s_team_or_player_target,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_submit_incident_with_custom_value_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_incident_id, 10)?;
        self.m_target_1.encode(bitstream)?;
        self.m_target_2.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_incident_id = bitstream.read_integer("incident-id", 10)?;
        self.m_target_1.decode(bitstream)?;
        self.m_target_2.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_loadout_palette_parameters {
    pub m_target: s_team_or_player_target,
    pub m_loadout_palette_index: u8, // 3 bits
}

impl s_action_set_loadout_palette_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_target.encode(bitstream)?;
        bitstream.write_integer(self.m_loadout_palette_index, 3)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_target.decode(bitstream)?;
        self.m_loadout_palette_index = bitstream.read_integer("loadout-palette-index", 3)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_device_set_position_track_parameters {
    pub m_object: c_object_reference,
    pub m_animation_name_index: u8, // 8 bits
    pub m_variable: c_custom_variable_reference,
}

impl s_action_device_set_position_track_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_integer(self.m_animation_name_index, 8)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_animation_name_index = bitstream.read_integer("animation-name-index", 8)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_device_animate_position_parameters {
    pub m_object: c_object_reference,
    pub m_variable_1: c_custom_variable_reference,
    pub m_variable_2: c_custom_variable_reference,
    pub m_variable_3: c_custom_variable_reference,
    pub m_variable_4: c_custom_variable_reference,
}

impl s_action_device_animate_position_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable_1.encode(bitstream)?;
        self.m_variable_2.encode(bitstream)?;
        self.m_variable_3.encode(bitstream)?;
        self.m_variable_4.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable_1.decode(bitstream)?;
        self.m_variable_2.decode(bitstream)?;
        self.m_variable_3.decode(bitstream)?;
        self.m_variable_4.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_get_weapon_parameters {
    pub m_player: c_player_reference,
    pub m_primary: bool,
    pub m_object: c_object_reference,
}

impl s_action_player_get_weapon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_bool(self.m_primary)?;
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_primary = bitstream.read_bool("primary")?;
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_create_tunnel_parameters {
    pub m_object_1: c_player_reference,
    pub m_object_2: c_player_reference,
    pub m_object_type: c_object_reference,
    pub m_variable: c_custom_variable_reference,
    pub m_object_3: c_player_reference,
}

impl s_action_create_tunnel_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        self.m_object_type.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;
        self.m_object_3.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        self.m_object_type.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;
        self.m_object_3.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_set_coop_spawning_parameters {
    pub m_player: c_player_reference,
    pub m_enabled: bool,
}

impl s_action_player_set_coop_spawning_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_bool(self.m_enabled)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_enabled = bitstream.read_bool("enabled")?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_set_orientation_parameters {
    pub m_object_1: c_object_reference,
    pub m_object_2: c_object_reference,
    pub m_absolute_orientation: bool,
}

impl s_action_object_set_orientation_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        bitstream.write_bool(self.m_absolute_orientation)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        self.m_absolute_orientation = bitstream.read_bool("absolute-orientation")?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_face_object_parameters {
    pub m_object_1: c_object_reference,
    pub m_object_2: c_object_reference,
    pub m_offset: s_object_offset, // 24 bits
}

impl s_action_object_face_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        self.m_offset.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        self.m_offset.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_biped_give_weapon_parameters {
    pub m_object: c_object_reference,
    pub m_object_type: c_object_type_reference,
    pub m_mode: e_biped_give_weapon_mode, // 2 bits
}

impl s_action_biped_give_weapon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_object_type.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_mode, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_object_type.decode(bitstream)?;
        self.m_mode = bitstream.read_enum_raw("mode", 2)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_biped_drop_weapon_parameters {
    pub m_object: c_object_reference,
    pub m_primary: bool,
    pub m_delete_on_drop: bool,
}

impl s_action_biped_drop_weapon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_bool(self.m_primary)?;
        bitstream.write_bool(self.m_delete_on_drop)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_primary = bitstream.read_bool("primary")?;
        self.m_delete_on_drop = bitstream.read_bool("delete_on_drop")?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_get_random_object_parameters {
    pub m_object_1: c_object_reference,
    pub m_object_2: c_object_reference,
    pub m_filter_index: i8,
}

impl s_action_get_random_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        bitstream.write_index::<16>(self.m_filter_index, 4)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        self.m_filter_index = bitstream.read_index::<16>("filter-index", 4)? as i8;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_boundary_set_player_color_parameters {
    pub m_object: c_object_reference,
    pub m_player_index: i8,
}

impl s_action_boundary_set_player_color_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_index::<4>(self.m_player_index, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_player_index = bitstream.read_index::<4>("player-index", 2)? as i8;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_delete_object_parameters {
    pub m_object: c_object_reference,
}

impl s_action_delete_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_navpoint_set_visible_parameters {
    pub m_object: c_object_reference,
    pub m_player_filter_modifier: c_player_filter_modifier,
}

impl s_action_navpoint_set_visible_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_player_filter_modifier.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_player_filter_modifier.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_pickup_filter_parameters {
    pub m_object: c_object_reference,
    pub m_player_filter_modifier: c_player_filter_modifier,
}

impl s_action_set_pickup_filter_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_player_filter_modifier.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_player_filter_modifier.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_respawn_filter_parameters {
    pub m_object: c_object_reference,
    pub m_player_filter_modifier: c_player_filter_modifier,
}

impl s_action_set_respawn_filter_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_player_filter_modifier.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_player_filter_modifier.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_print_variable_parameters {
    pub m_string: c_dynamic_string,
}

impl s_action_print_variable_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_string.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_string.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_get_player_holding_object_parameters {
    pub m_object: c_object_reference,
    pub m_player: c_player_reference,
}

impl s_action_get_player_holding_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_player.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_player.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_end_round_parameters {}

impl s_action_end_round_parameters {
    pub fn encode(&self, _bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        Ok(())
    }

    pub fn decode(&mut self, _bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_boundary_set_visible_parameters {
    pub m_object: c_object_reference,
    pub m_player_filter_modifier: c_player_filter_modifier,
}

impl s_action_boundary_set_visible_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_player_filter_modifier.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_player_filter_modifier.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_set_invincibility_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_set_invincibility_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_random_parameters {
    pub m_variable_1: c_custom_variable_reference,
    pub m_variable_2: c_custom_variable_reference,
}

impl s_action_random_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_variable_1.encode(bitstream)?;
        self.m_variable_2.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_variable_1.decode(bitstream)?;
        self.m_variable_2.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_break_into_debugger_parameters {}

impl s_action_break_into_debugger_parameters {
    pub fn encode(&self, _bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        Ok(())
    }

    pub fn decode(&mut self, _bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_get_orientation_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_get_orientation_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_get_velocity_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_get_velocity_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_death_get_killing_player_parameters {
    pub m_player_1: c_player_reference,
    pub m_player_2: c_player_reference,
}

impl s_action_player_death_get_killing_player_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player_1.encode(bitstream)?;
        self.m_player_2.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player_1.decode(bitstream)?;
        self.m_player_2.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_death_get_damage_type_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_player_death_get_damage_type_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_death_get_special_type_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_player_death_get_special_type_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_debugging_enable_tracing_parameters {
    pub m_tracing_enabled: bool,
}

impl s_action_debugging_enable_tracing_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_tracing_enabled)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_tracing_enabled = bitstream.read_bool("tracing-enabled")?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_detach_parameters {
    pub m_object: c_object_reference,
}

impl s_action_object_detach_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_get_place_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_player_get_place_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_team_get_place_parameters {
    pub m_team: c_team_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_team_get_place_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_team.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_get_killing_spree_count_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_player_get_killing_spree_count_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_get_vehicle_parameters {
    pub m_player: c_player_reference,
    pub m_object: c_object_reference,
}

impl s_action_player_get_vehicle_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_set_vehicle_parameters {
    pub m_player: c_player_reference,
    pub m_object: c_object_reference,
}

impl s_action_player_set_vehicle_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_set_unit_parameters {
    pub m_player: c_player_reference,
    pub m_object: c_object_reference,
}

impl s_action_player_set_unit_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_timer_reset_parameters {
    pub m_timer: c_custom_timer_reference,
}

impl s_action_timer_reset_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_timer.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_timer.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_bounce_parameters {
    pub m_object: c_object_reference,
}

impl s_action_object_bounce_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_hud_widget_set_text_parameters {
    pub m_widget_index: i8,
    pub m_string: c_dynamic_string,
}

impl s_action_hud_widget_set_text_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<4>(self.m_widget_index, 2)?;
        self.m_string.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_widget_index = bitstream.read_index::<4>("widget-index", 2)? as i8;
        self.m_string.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_hud_widget_set_value_parameters {
    pub m_widget_index: i8,
    pub m_string: c_dynamic_string,
}

impl s_action_hud_widget_set_value_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<4>(self.m_widget_index, 2)?;
        self.m_string.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_widget_index = bitstream.read_index::<4>("widget-index", 2)? as i8;
        self.m_string.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_set_scale_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_set_scale_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_navpoint_set_text_parameters {
    pub m_object: c_object_reference,
    pub m_string: c_dynamic_string,
}

impl s_action_navpoint_set_text_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_string.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_string.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_get_shield_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_get_shield_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_get_health_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_get_health_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_set_objective_parameters {
    pub m_player: c_player_reference,
    pub m_string: c_dynamic_string,
}

impl s_action_player_set_objective_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_string.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_string.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_set_objective_allegiance_parameters {
    pub m_player: c_player_reference,
    pub m_string: c_dynamic_string,
}

impl s_action_player_set_objective_allegiance_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_string.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_string.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_team_set_primary_respawn_object_parameters {
    pub m_team: c_team_reference,
    pub m_object: c_object_reference,
}

impl s_action_team_set_primary_respawn_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_team.encode(bitstream)?;
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team.decode(bitstream)?;
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_set_primary_respawn_object_parameters {
    pub m_player: c_player_reference,
    pub m_object: c_object_reference,
}

impl s_action_player_set_primary_respawn_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_get_fireteam_index_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_player_get_fireteam_index_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_set_fireteam_index_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_player_set_fireteam_index_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_adjust_shield_parameters {
    pub m_object: c_object_reference,
    pub m_operation: e_math_operation,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_adjust_shield_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_operation = bitstream.read_enum_raw("operation", 4)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_adjust_health_parameters {
    pub m_object: c_object_reference,
    pub m_operation: e_math_operation,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_adjust_health_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_operation = bitstream.read_enum_raw("operation", 4)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_adjust_maximum_shield_parameters {
    pub m_object: c_object_reference,
    pub m_operation: e_math_operation,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_adjust_maximum_shield_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_operation = bitstream.read_enum_raw("operation", 4)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_adjust_maximum_health_parameters {
    pub m_object: c_object_reference,
    pub m_operation: e_math_operation,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_adjust_maximum_health_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_operation = bitstream.read_enum_raw("operation", 4)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_device_set_power_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_device_set_power_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_device_get_power_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_device_get_power_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_device_set_position_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_device_set_position_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_device_get_position_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_device_get_position_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_device_set_position_immediate_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_device_set_position_immediate_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_saved_film_insert_marker_parameters {
    pub m_variable: c_custom_variable_reference,
    pub m_string: c_dynamic_string,
}

impl s_action_saved_film_insert_marker_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_variable.encode(bitstream)?;
        self.m_string.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_variable.decode(bitstream)?;
        self.m_string.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_respawn_zone_enable_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_respawn_zone_enable_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_get_equipment_parameters {
    pub m_player: c_player_reference,
    pub m_object: c_object_reference,
}

impl s_action_player_get_equipment_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_set_never_garbage_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_set_never_garbage_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_get_target_object_parameters {
    pub m_player: c_player_reference,
    pub m_object: c_object_reference,
}

impl s_action_player_get_target_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_debug_force_player_view_count_parameters {
    pub m_variable: c_custom_variable_reference,
}

impl s_action_debug_force_player_view_count_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_pick_up_weapon_parameters {
    pub m_player: c_player_reference,
    pub m_object: c_object_reference,
}

impl s_action_player_pick_up_weapon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_object.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_object.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_scenario_interpolator_state_parameters {
    pub m_variable_1: c_custom_variable_reference,
    pub m_variable_2: c_custom_variable_reference,
}

impl s_action_set_scenario_interpolator_state_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_variable_1.encode(bitstream)?;
        self.m_variable_2.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_variable_1.decode(bitstream)?;
        self.m_variable_2.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_game_grief_record_custom_penalty_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_game_grief_record_custom_penalty_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_begin_parameters {
    pub m_first_condition_index: u16, // 9 bits
    pub m_condition_count: u16, // 10 bits
    pub m_first_action_index: u16, // 10 bits
    pub m_action_count: u16, // 11 bits
}

impl s_action_begin_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_first_condition_index, 9)?;
        bitstream.write_integer(self.m_condition_count, 10)?;
        bitstream.write_integer(self.m_first_action_index, 10)?;
        bitstream.write_integer(self.m_action_count, 11)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_first_condition_index = bitstream.read_integer("first-condition-index", 9)?;
        self.m_condition_count = bitstream.read_integer("condition-count", 10)?;
        self.m_first_action_index = bitstream.read_integer("first-action-index", 10)?;
        self.m_action_count = bitstream.read_integer("action-count", 11)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_hs_function_call_parameters {
    pub m_function_name_index: u8,
}

impl s_action_hs_function_call_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer((self.m_function_name_index as u32) + 1, 8)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let encoded: u32 = bitstream.read_integer("function-name-index", 8)?;
        self.m_function_name_index = (encoded - 1) as u8;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_get_button_time_parameters {
    pub m_player: c_player_reference,
    pub m_buttons: e_scriptable_game_buttons, // 5 bits
    pub m_variable: c_custom_variable_reference,
}

impl s_action_get_button_time_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_buttons, 5)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_buttons = bitstream.read_enum_raw("buttons", 5)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_team_set_vehicle_spawning_parameters {
    pub m_team: c_team_reference,
    pub m_enabled: bool,
}

impl s_action_team_set_vehicle_spawning_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_team.encode(bitstream)?;
        bitstream.write_bool(self.m_enabled)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team.decode(bitstream)?;
        self.m_enabled = bitstream.read_bool("enabled")?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_set_vehicle_spawning_parameters {
    pub m_player: c_player_reference,
    pub m_enabled: bool,
}

impl s_action_player_set_vehicle_spawning_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_bool(self.m_enabled)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_enabled = bitstream.read_bool("enabled")?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_player_respawn_vehicle_parameters {
    pub m_object_type: c_object_type_reference,
    pub m_player: c_player_reference,
}

impl s_action_set_player_respawn_vehicle_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_type.encode(bitstream)?;
        self.m_player.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_type.decode(bitstream)?;
        self.m_player.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_team_respawn_vehicle_parameters {
    pub m_object_type: c_object_type_reference,
    pub m_team: c_team_reference,
}

impl s_action_set_team_respawn_vehicle_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_type.encode(bitstream)?;
        self.m_team.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_type.decode(bitstream)?;
        self.m_team.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_hide_object_parameters {
    pub m_object: c_object_reference,
    pub m_should_hide: bool,
}

impl s_action_hide_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_bool(self.m_should_hide)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_should_hide = bitstream.read_bool("should hide")?;

        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
pub enum e_action_type {
    #[default]
    none = 0,
    set_score = 1,
    create_object = 2,
    delete_object = 3,
    navpoint_set_visible = 4,
    navpoint_set_icon = 5,
    navpoint_set_priority = 6,
    navpoint_set_timer = 7,
    navpoint_set_visible_range = 8,
    set = 9,
    set_boundary = 10,
    apply_player_traits = 11,
    set_pickup_filter = 12,
    set_respawn_filter = 13,
    set_fireteam_respawn_filter = 14,
    set_progress_bar = 15,
    hud_post_message = 16,
    timer_set_rate = 17,
    print_variable = 18,
    get_player_holding_object = 19,
    for_each = 20,
    end_round = 21,
    boundary_set_visible = 22,
    object_destroy = 23,
    object_set_invincibility = 24,
    random = 25,
    break_into_debugger = 26,
    object_get_orientation = 27,
    object_get_velocity = 28,
    player_death_get_killing_player = 29,
    player_death_get_damage_type = 30,
    player_death_get_special_type = 31,
    debugging_enable_tracing = 32,
    object_attach = 33,
    object_detach = 34,
    player_get_place = 35,
    team_get_place = 36,
    player_get_killing_spree_count = 37,
    player_adjust_money = 38,
    player_enable_purchases = 39,
    player_get_vehicle = 40,
    player_set_vehicle = 41,
    player_set_unit = 42,
    timer_reset = 43,
    weapon_set_pickup_priority = 44,
    object_bounce = 45,
    hud_widget_set_text = 46,
    hud_widget_set_value = 47,
    hud_widget_set_meter = 48,
    hud_widget_set_icon = 49,
    hud_widget_set_visibility = 50,
    play_sound = 51,
    object_set_scale = 52,
    navpoint_set_text = 53,
    object_get_shield = 54,
    object_get_health = 55,
    player_set_objective = 56,
    player_set_objective_allegiance = 57,
    player_set_objective_allegiance_icon = 58,
    team_set_coop_spawning = 59,
    team_set_primary_respawn_object = 60,
    player_set_primary_respawn_object = 61,
    player_get_fireteam_index = 62,
    player_set_fireteam_index = 63,
    object_adjust_shield = 64,
    object_adjust_health = 65,
    object_get_distance = 66,
    object_adjust_maximum_shield = 67,
    object_adjust_maximum_health = 68,
    player_set_requisition_palette = 69,
    device_set_power = 70,
    device_get_power = 71,
    device_set_position = 72,
    device_get_position = 73,
    adjust_grenades = 74,
    submit_incident = 75,
    submit_incident_with_custom_value = 76,
    set_loadout_palette = 77,
    device_set_position_track = 78,
    device_animate_position = 79,
    device_set_position_immediate = 80,
    saved_film_insert_marker = 81,
    respawn_zone_enable = 82,
    player_get_weapon = 83,
    player_get_equipment = 84,
    object_set_never_garbage = 85,
    player_get_target_object = 86,
    create_tunnel = 87,
    debug_force_player_view_count = 88,
    player_pick_up_weapon = 89,
    player_set_coop_spawning = 90,
    object_set_orientation = 91,
    object_face_object = 92,
    biped_give_weapon = 93,
    biped_drop_weapon = 94,
    set_scenario_interpolator_state = 95,
    get_random_object = 96,
    game_grief_record_custom_penalty = 97,
    boundary_set_player_color = 98,
    begin = 99,
    hs_function_call = 100,
    get_button_time = 101,
    team_set_vehicle_spawning = 102,
    player_set_vehicle_spawning = 103,
    set_player_respawn_vehicle = 104,
    set_team_respawn_vehicle = 105,
    hide_object = 106,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_action {
    pub m_type: e_action_type, // 7 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_score_parameters: Option<s_action_set_score_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_create_object_parameters: Option<s_action_create_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_delete_object_parameters: Option<s_action_delete_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_navpoint_set_visible_parameters: Option<s_action_navpoint_set_visible_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_navpoint_set_icon_parameters: Option<s_action_navpoint_set_icon_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_navpoint_set_priority_parameters: Option<s_action_navpoint_set_priority_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_navpoint_set_timer_parameters: Option<s_action_navpoint_set_timer_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_navpoint_set_visible_range_parameters: Option<s_action_navpoint_set_visible_range_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_parameters: Option<s_action_set_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_boundary_parameters: Option<s_action_set_boundary_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_apply_player_traits_parameters: Option<s_action_apply_player_traits_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_pickup_filter_parameters: Option<s_action_set_pickup_filter_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_respawn_filter_parameters: Option<s_action_set_respawn_filter_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_fireteam_respawn_filter_parameters: Option<s_action_set_fireteam_respawn_filter_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_progress_bar_parameters: Option<s_action_set_progress_bar_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hud_post_message_parameters: Option<s_action_hud_post_message_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_timer_set_rate_parameters: Option<s_action_timer_set_rate_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_print_variable_parameters: Option<s_action_print_variable_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_get_player_holding_object_parameters: Option<s_action_get_player_holding_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_for_each_parameters: Option<s_action_for_each_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_end_round_parameters: Option<s_action_end_round_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_boundary_set_visible_parameters: Option<s_action_boundary_set_visible_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_destroy_parameters: Option<s_action_object_destroy_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_set_invincibility_parameters: Option<s_action_object_set_invincibility_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_random_parameters: Option<s_action_random_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_break_into_debugger_parameters: Option<s_action_break_into_debugger_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_get_orientation_parameters: Option<s_action_object_get_orientation_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_get_velocity_parameters: Option<s_action_object_get_velocity_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_death_get_killing_player_parameters: Option<s_action_player_death_get_killing_player_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_death_get_damage_type_parameters: Option<s_action_player_death_get_damage_type_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_death_get_special_type_parameters: Option<s_action_player_death_get_special_type_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_debugging_enable_tracing_parameters: Option<s_action_debugging_enable_tracing_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_attach_parameters: Option<s_action_object_attach_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_detach_parameters: Option<s_action_object_detach_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_get_place_parameters: Option<s_action_player_get_place_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team_get_place_parameters: Option<s_action_team_get_place_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_get_killing_spree_count_parameters: Option<s_action_player_get_killing_spree_count_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_adjust_money_parameters: Option<s_action_player_adjust_money_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_enable_purchases_parameters: Option<s_action_player_enable_purchases_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_get_vehicle_parameters: Option<s_action_player_get_vehicle_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_vehicle_parameters: Option<s_action_player_set_vehicle_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_unit_parameters: Option<s_action_player_set_unit_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_timer_reset_parameters: Option<s_action_timer_reset_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_weapon_set_pickup_priority_parameters: Option<s_action_weapon_set_pickup_priority_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_bounce_parameters: Option<s_action_object_bounce_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hud_widget_set_text_parameters: Option<s_action_hud_widget_set_text_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hud_widget_set_value_parameters: Option<s_action_hud_widget_set_value_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hud_widget_set_meter_parameters: Option<s_action_hud_widget_set_meter_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hud_widget_set_icon_parameters: Option<s_action_hud_widget_set_icon_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hud_widget_set_visibility_parameters: Option<s_action_hud_widget_set_visibility_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_play_sound_parameters: Option<s_action_play_sound_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_set_scale_parameters: Option<s_action_object_set_scale_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_navpoint_set_text_parameters: Option<s_action_navpoint_set_text_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_get_shield_parameters: Option<s_action_object_get_shield_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_get_health_parameters: Option<s_action_object_get_health_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_objective_parameters: Option<s_action_player_set_objective_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_objective_allegiance_parameters: Option<s_action_player_set_objective_allegiance_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_objective_allegiance_icon_parameters: Option<s_action_player_set_objective_allegiance_icon_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team_set_coop_spawning_parameters: Option<s_action_team_set_coop_spawning_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team_set_primary_respawn_object_parameters: Option<s_action_team_set_primary_respawn_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_primary_respawn_object_parameters: Option<s_action_player_set_primary_respawn_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_get_fireteam_index_parameters: Option<s_action_player_get_fireteam_index_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_fireteam_index_parameters: Option<s_action_player_set_fireteam_index_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_adjust_shield_parameters: Option<s_action_object_adjust_shield_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_adjust_health_parameters: Option<s_action_object_adjust_health_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_get_distance_parameters: Option<s_action_object_get_distance_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_adjust_maximum_shield_parameters: Option<s_action_object_adjust_maximum_shield_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_adjust_maximum_health_parameters: Option<s_action_object_adjust_maximum_health_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_requisition_palette_parameters: Option<s_action_player_set_requisition_palette_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_set_power_parameters: Option<s_action_device_set_power_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_get_power_parameters: Option<s_action_device_get_power_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_set_position_parameters: Option<s_action_device_set_position_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_get_position_parameters: Option<s_action_device_get_position_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_adjust_grenades_parameters: Option<s_action_adjust_grenades_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_submit_incident_parameters: Option<s_action_submit_incident_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_submit_incident_with_custom_value_parameters: Option<s_action_submit_incident_with_custom_value_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_loadout_palette_parameters: Option<s_action_set_loadout_palette_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_set_position_track_parameters: Option<s_action_device_set_position_track_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_animate_position_parameters: Option<s_action_device_animate_position_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_set_position_immediate_parameters: Option<s_action_device_set_position_immediate_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_saved_film_insert_marker_parameters: Option<s_action_saved_film_insert_marker_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_respawn_zone_enable_parameters: Option<s_action_respawn_zone_enable_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_get_weapon_parameters: Option<s_action_player_get_weapon_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_get_equipment_parameters: Option<s_action_player_get_equipment_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_set_never_garbage_parameters: Option<s_action_object_set_never_garbage_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_get_target_object_parameters: Option<s_action_player_get_target_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_create_tunnel_parameters: Option<s_action_create_tunnel_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_debug_force_player_view_count_parameters: Option<s_action_debug_force_player_view_count_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_pick_up_weapon_parameters: Option<s_action_player_pick_up_weapon_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_coop_spawning_parameters: Option<s_action_player_set_coop_spawning_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_set_orientation_parameters: Option<s_action_object_set_orientation_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_face_object_parameters: Option<s_action_object_face_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_biped_give_weapon_parameters: Option<s_action_biped_give_weapon_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_biped_drop_weapon_parameters: Option<s_action_biped_drop_weapon_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_scenario_interpolator_state_parameters: Option<s_action_set_scenario_interpolator_state_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_get_random_object_parameters: Option<s_action_get_random_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_game_grief_record_custom_penalty_parameters: Option<s_action_game_grief_record_custom_penalty_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_boundary_set_player_color_parameters: Option<s_action_boundary_set_player_color_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_begin_parameters: Option<s_action_begin_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hs_function_call_parameters: Option<s_action_hs_function_call_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_get_button_time_parameters: Option<s_action_get_button_time_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team_set_vehicle_spawning_parameters: Option<s_action_team_set_vehicle_spawning_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_vehicle_spawning_parameters: Option<s_action_player_set_vehicle_spawning_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_player_respawn_vehicle_parameters: Option<s_action_set_player_respawn_vehicle_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_team_respawn_vehicle_parameters: Option<s_action_set_team_respawn_vehicle_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hide_object_parameters: Option<s_action_hide_object_parameters>,
}

impl c_action {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_type.clone(), 7)?;

        match self.m_type {
            e_action_type::set_score => self.m_set_score_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_score_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::create_object => self.m_create_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_create_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::delete_object => self.m_delete_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_delete_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::navpoint_set_visible => self.m_navpoint_set_visible_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_visible_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::navpoint_set_icon => self.m_navpoint_set_icon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_icon_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::navpoint_set_priority => self.m_navpoint_set_priority_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_priority_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::navpoint_set_timer => self.m_navpoint_set_timer_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_timer_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::navpoint_set_visible_range => self.m_navpoint_set_visible_range_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_visible_range_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set => self.m_set_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_boundary => self.m_set_boundary_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_boundary_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::apply_player_traits => self.m_apply_player_traits_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_apply_player_traits_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_pickup_filter => self.m_set_pickup_filter_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_pickup_filter_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_respawn_filter => self.m_set_respawn_filter_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_respawn_filter_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_fireteam_respawn_filter => self.m_set_fireteam_respawn_filter_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_fireteam_respawn_filter_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_progress_bar => self.m_set_progress_bar_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_progress_bar_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::hud_post_message => self.m_hud_post_message_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_post_message_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::timer_set_rate => self.m_timer_set_rate_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_timer_set_rate_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::print_variable => self.m_print_variable_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_print_variable_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::get_player_holding_object => self.m_get_player_holding_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_get_player_holding_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::for_each => self.m_for_each_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_for_each_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::end_round => {}
            e_action_type::boundary_set_visible => self.m_boundary_set_visible_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_boundary_set_visible_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_destroy => self.m_object_destroy_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_destroy_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_set_invincibility => self.m_object_set_invincibility_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_set_invincibility_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::random => self.m_random_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_random_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::break_into_debugger => {}
            e_action_type::object_get_orientation => self.m_object_get_orientation_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_get_orientation_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_get_velocity => self.m_object_get_velocity_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_get_velocity_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_death_get_killing_player => self.m_player_death_get_killing_player_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_death_get_killing_player_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_death_get_damage_type => self.m_player_death_get_damage_type_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_death_get_damage_type_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_death_get_special_type => self.m_player_death_get_special_type_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_death_get_special_type_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::debugging_enable_tracing => self.m_debugging_enable_tracing_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_debugging_enable_tracing_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_attach => self.m_object_attach_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_attach_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_detach => self.m_object_detach_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_detach_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_get_place => self.m_player_get_place_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_get_place_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::team_get_place => self.m_team_get_place_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_team_get_place_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_get_killing_spree_count => self.m_player_get_killing_spree_count_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_get_killing_spree_count_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_adjust_money => self.m_player_adjust_money_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_adjust_money_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_enable_purchases => self.m_player_enable_purchases_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_enable_purchases_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_get_vehicle => self.m_player_get_vehicle_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_get_vehicle_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_vehicle => self.m_player_set_vehicle_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_vehicle_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_unit => self.m_player_set_unit_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_unit_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::timer_reset => self.m_timer_reset_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_timer_reset_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::weapon_set_pickup_priority => self.m_weapon_set_pickup_priority_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_weapon_set_pickup_priority_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_bounce => self.m_object_bounce_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_bounce_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::hud_widget_set_text => self.m_hud_widget_set_text_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_widget_set_text_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::hud_widget_set_value => self.m_hud_widget_set_value_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_widget_set_value_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::hud_widget_set_meter => self.m_hud_widget_set_meter_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_widget_set_meter_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::hud_widget_set_icon => self.m_hud_widget_set_icon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_widget_set_icon_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::hud_widget_set_visibility => self.m_hud_widget_set_visibility_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_widget_set_visibility_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::play_sound => self.m_play_sound_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_play_sound_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_set_scale => self.m_object_set_scale_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_set_scale_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::navpoint_set_text => self.m_navpoint_set_text_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_text_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_get_shield => self.m_object_get_shield_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_get_shield_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_get_health => self.m_object_get_health_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_get_health_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_objective => self.m_player_set_objective_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_objective_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_objective_allegiance => self.m_player_set_objective_allegiance_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_objective_allegiance_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_objective_allegiance_icon => self.m_player_set_objective_allegiance_icon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_objective_allegiance_icon_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::team_set_coop_spawning => self.m_team_set_coop_spawning_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_team_set_coop_spawning_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::team_set_primary_respawn_object => self.m_team_set_primary_respawn_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_team_set_primary_respawn_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_primary_respawn_object => self.m_player_set_primary_respawn_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_primary_respawn_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_get_fireteam_index => self.m_player_get_fireteam_index_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_get_fireteam_index_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_fireteam_index => self.m_player_set_fireteam_index_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_fireteam_index_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_adjust_shield => self.m_object_adjust_shield_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_adjust_shield_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_adjust_health => self.m_object_adjust_health_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_adjust_health_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_get_distance => self.m_object_get_distance_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_get_distance_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_adjust_maximum_shield => self.m_object_adjust_maximum_shield_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_adjust_maximum_shield_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_adjust_maximum_health => self.m_object_adjust_maximum_health_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_adjust_maximum_health_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_requisition_palette => self.m_player_set_requisition_palette_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_requisition_palette_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_set_power => self.m_device_set_power_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_set_power_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_get_power => self.m_device_get_power_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_get_power_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_set_position => self.m_device_set_position_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_set_position_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_get_position => self.m_device_get_position_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_get_position_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::adjust_grenades => self.m_adjust_grenades_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_adjust_grenades_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::submit_incident => self.m_submit_incident_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_submit_incident_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::submit_incident_with_custom_value => self.m_submit_incident_with_custom_value_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_submit_incident_with_custom_value_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_loadout_palette => self.m_set_loadout_palette_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_loadout_palette_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_set_position_track => self.m_device_set_position_track_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_set_position_track_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_animate_position => self.m_device_animate_position_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_animate_position_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_set_position_immediate => self.m_device_set_position_immediate_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_set_position_immediate_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::saved_film_insert_marker => self.m_saved_film_insert_marker_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_saved_film_insert_marker_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::respawn_zone_enable => self.m_respawn_zone_enable_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_respawn_zone_enable_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_get_weapon => self.m_player_get_weapon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_get_weapon_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_get_equipment => self.m_player_get_equipment_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_get_equipment_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_set_never_garbage => self.m_object_set_never_garbage_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_set_never_garbage_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_get_target_object => self.m_player_get_target_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_get_target_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::create_tunnel => self.m_create_tunnel_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_create_tunnel_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::debug_force_player_view_count => self.m_debug_force_player_view_count_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_debug_force_player_view_count_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_pick_up_weapon => self.m_player_pick_up_weapon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_pick_up_weapon_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_coop_spawning => self.m_player_set_coop_spawning_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_coop_spawning_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_set_orientation => self.m_object_set_orientation_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_set_orientation_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_face_object => self.m_object_face_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_face_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::biped_give_weapon => self.m_biped_give_weapon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_biped_give_weapon_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::biped_drop_weapon => self.m_biped_drop_weapon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_biped_drop_weapon_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_scenario_interpolator_state => self.m_set_scenario_interpolator_state_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_scenario_interpolator_state_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::get_random_object => self.m_get_random_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_get_random_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::game_grief_record_custom_penalty => self.m_game_grief_record_custom_penalty_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_game_grief_record_custom_penalty_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::boundary_set_player_color => self.m_boundary_set_player_color_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_boundary_set_player_color_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::begin => self.m_begin_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_begin_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::hs_function_call => self.m_hs_function_call_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hs_function_call_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::get_button_time => self.m_get_button_time_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_get_button_time_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::team_set_vehicle_spawning => self.m_team_set_vehicle_spawning_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_team_set_vehicle_spawning_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_vehicle_spawning => self.m_player_set_vehicle_spawning_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_vehicle_spawning_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_player_respawn_vehicle => self.m_set_player_respawn_vehicle_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_player_respawn_vehicle_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_team_respawn_vehicle => self.m_set_team_respawn_vehicle_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_team_respawn_vehicle_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::hide_object => self.m_hide_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hide_object_parameters does not exist."))?
                .encode(bitstream)?,
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let action_type = bitstream.read_integer("action-type", 7)?;
        if let Some(action_type) = FromPrimitive::from_u32(action_type) {
            self.m_type = action_type;
        }
        else {
            return Err(format!("unsupported action type: {}", action_type).into())
        }


        match self.m_type {
            e_action_type::set_score => {
                let mut set_score_parameters = s_action_set_score_parameters::default();
                set_score_parameters.decode(bitstream)?;
                self.m_set_score_parameters = Some(set_score_parameters);
            }
            e_action_type::create_object => {
                let mut create_object_parameters = s_action_create_object_parameters::default();
                create_object_parameters.decode(bitstream)?;
                self.m_create_object_parameters = Some(create_object_parameters);
            }
            e_action_type::delete_object => {
                let mut delete_object_parameters = s_action_delete_object_parameters::default();
                delete_object_parameters.decode(bitstream)?;
                self.m_delete_object_parameters = Some(delete_object_parameters);
            }
            e_action_type::navpoint_set_visible => {
                let mut navpoint_set_visible_parameters = s_action_navpoint_set_visible_parameters::default();
                navpoint_set_visible_parameters.decode(bitstream)?;
                self.m_navpoint_set_visible_parameters = Some(navpoint_set_visible_parameters);
            }
            e_action_type::navpoint_set_icon => {
                let mut navpoint_set_icon_parameters = s_action_navpoint_set_icon_parameters::default();
                navpoint_set_icon_parameters.decode(bitstream)?;
                self.m_navpoint_set_icon_parameters = Some(navpoint_set_icon_parameters);
            }
            e_action_type::navpoint_set_priority => {
                let mut navpoint_set_priority_parameters = s_action_navpoint_set_priority_parameters::default();
                navpoint_set_priority_parameters.decode(bitstream)?;
                self.m_navpoint_set_priority_parameters = Some(navpoint_set_priority_parameters);
            }
            e_action_type::navpoint_set_timer => {
                let mut navpoint_set_timer_parameters = s_action_navpoint_set_timer_parameters::default();
                navpoint_set_timer_parameters.decode(bitstream)?;
                self.m_navpoint_set_timer_parameters = Some(navpoint_set_timer_parameters);
            }
            e_action_type::navpoint_set_visible_range => {
                let mut navpoint_set_visible_range_parameters = s_action_navpoint_set_visible_range_parameters::default();
                navpoint_set_visible_range_parameters.decode(bitstream)?;
                self.m_navpoint_set_visible_range_parameters = Some(navpoint_set_visible_range_parameters);
            }
            e_action_type::set => {
                let mut set_parameters = s_action_set_parameters::default();
                set_parameters.decode(bitstream)?;
                self.m_set_parameters = Some(set_parameters);
            }
            e_action_type::set_boundary => {
                let mut set_boundary_parameters = s_action_set_boundary_parameters::default();
                set_boundary_parameters.decode(bitstream)?;
                self.m_set_boundary_parameters = Some(set_boundary_parameters);
            }
            e_action_type::apply_player_traits => {
                let mut apply_player_traits_parameters = s_action_apply_player_traits_parameters::default();
                apply_player_traits_parameters.decode(bitstream)?;
                self.m_apply_player_traits_parameters = Some(apply_player_traits_parameters);
            }
            e_action_type::set_pickup_filter => {
                let mut set_pickup_filter_parameters = s_action_set_pickup_filter_parameters::default();
                set_pickup_filter_parameters.decode(bitstream)?;
                self.m_set_pickup_filter_parameters = Some(set_pickup_filter_parameters);
            }
            e_action_type::set_respawn_filter => {
                let mut set_respawn_filter_parameters = s_action_set_respawn_filter_parameters::default();
                set_respawn_filter_parameters.decode(bitstream)?;
                self.m_set_respawn_filter_parameters = Some(set_respawn_filter_parameters);
            }
            e_action_type::set_fireteam_respawn_filter => {
                let mut set_fireteam_respawn_filter_parameters = s_action_set_fireteam_respawn_filter_parameters::default();
                set_fireteam_respawn_filter_parameters.decode(bitstream)?;
                self.m_set_fireteam_respawn_filter_parameters = Some(set_fireteam_respawn_filter_parameters);
            }
            e_action_type::set_progress_bar => {
                let mut set_progress_bar_parameters = s_action_set_progress_bar_parameters::default();
                set_progress_bar_parameters.decode(bitstream)?;
                self.m_set_progress_bar_parameters = Some(set_progress_bar_parameters);
            }
            e_action_type::hud_post_message => {
                let mut hud_post_message_parameters = s_action_hud_post_message_parameters::default();
                hud_post_message_parameters.decode(bitstream)?;
                self.m_hud_post_message_parameters = Some(hud_post_message_parameters);
            }
            e_action_type::timer_set_rate => {
                let mut timer_set_rate_parameters = s_action_timer_set_rate_parameters::default();
                timer_set_rate_parameters.decode(bitstream)?;
                self.m_timer_set_rate_parameters = Some(timer_set_rate_parameters);
            }
            e_action_type::print_variable => {
                let mut print_variable_parameters = s_action_print_variable_parameters::default();
                print_variable_parameters.decode(bitstream)?;
                self.m_print_variable_parameters = Some(print_variable_parameters);
            }
            e_action_type::get_player_holding_object => {
                let mut get_player_holding_object_parameters = s_action_get_player_holding_object_parameters::default();
                get_player_holding_object_parameters.decode(bitstream)?;
                self.m_get_player_holding_object_parameters = Some(get_player_holding_object_parameters);
            }
            e_action_type::for_each => {
                let mut for_each_parameters = s_action_for_each_parameters::default();
                for_each_parameters.decode(bitstream)?;
                self.m_for_each_parameters = Some(for_each_parameters);
            }
            e_action_type::end_round => {}
            e_action_type::boundary_set_visible => {
                let mut boundary_set_visible_parameters = s_action_boundary_set_visible_parameters::default();
                boundary_set_visible_parameters.decode(bitstream)?;
                self.m_boundary_set_visible_parameters = Some(boundary_set_visible_parameters);
            }
            e_action_type::object_destroy => {
                let mut object_destroy_parameters = s_action_object_destroy_parameters::default();
                object_destroy_parameters.decode(bitstream)?;
                self.m_object_destroy_parameters = Some(object_destroy_parameters);
            }
            e_action_type::object_set_invincibility => {
                let mut object_set_invincibility_parameters = s_action_object_set_invincibility_parameters::default();
                object_set_invincibility_parameters.decode(bitstream)?;
                self.m_object_set_invincibility_parameters = Some(object_set_invincibility_parameters);
            }
            e_action_type::random => {
                let mut random_parameters = s_action_random_parameters::default();
                random_parameters.decode(bitstream)?;
                self.m_random_parameters = Some(random_parameters);
            }
            e_action_type::break_into_debugger => {}
            e_action_type::object_get_orientation => {
                let mut object_get_orientation_parameters = s_action_object_get_orientation_parameters::default();
                object_get_orientation_parameters.decode(bitstream)?;
                self.m_object_get_orientation_parameters = Some(object_get_orientation_parameters);
            }
            e_action_type::object_get_velocity => {
                let mut object_get_velocity_parameters = s_action_object_get_velocity_parameters::default();
                object_get_velocity_parameters.decode(bitstream)?;
                self.m_object_get_velocity_parameters = Some(object_get_velocity_parameters);
            }
            e_action_type::player_death_get_killing_player => {
                let mut player_death_get_killing_player_parameters = s_action_player_death_get_killing_player_parameters::default();
                player_death_get_killing_player_parameters.decode(bitstream)?;
                self.m_player_death_get_killing_player_parameters = Some(player_death_get_killing_player_parameters);
            }
            e_action_type::player_death_get_damage_type => {
                let mut player_death_get_damage_type_parameters = s_action_player_death_get_damage_type_parameters::default();
                player_death_get_damage_type_parameters.decode(bitstream)?;
                self.m_player_death_get_damage_type_parameters = Some(player_death_get_damage_type_parameters);
            }
            e_action_type::player_death_get_special_type => {
                let mut player_death_get_special_type_parameters = s_action_player_death_get_special_type_parameters::default();
                player_death_get_special_type_parameters.decode(bitstream)?;
                self.m_player_death_get_special_type_parameters = Some(player_death_get_special_type_parameters);
            }
            e_action_type::debugging_enable_tracing => {
                let mut debugging_enable_tracing_parameters = s_action_debugging_enable_tracing_parameters::default();
                debugging_enable_tracing_parameters.decode(bitstream)?;
                self.m_debugging_enable_tracing_parameters = Some(debugging_enable_tracing_parameters);
            }
            e_action_type::object_attach => {
                let mut object_attach_parameters = s_action_object_attach_parameters::default();
                object_attach_parameters.decode(bitstream)?;
                self.m_object_attach_parameters = Some(object_attach_parameters);
            }
            e_action_type::object_detach => {
                let mut object_detach_parameters = s_action_object_detach_parameters::default();
                object_detach_parameters.decode(bitstream)?;
                self.m_object_detach_parameters = Some(object_detach_parameters);
            }
            e_action_type::player_get_place => {
                let mut player_get_place_parameters = s_action_player_get_place_parameters::default();
                player_get_place_parameters.decode(bitstream)?;
                self.m_player_get_place_parameters = Some(player_get_place_parameters);
            }
            e_action_type::team_get_place => {
                let mut team_get_place_parameters = s_action_team_get_place_parameters::default();
                team_get_place_parameters.decode(bitstream)?;
                self.m_team_get_place_parameters = Some(team_get_place_parameters);
            }
            e_action_type::player_get_killing_spree_count => {
                let mut player_get_killing_spree_count_parameters = s_action_player_get_killing_spree_count_parameters::default();
                player_get_killing_spree_count_parameters.decode(bitstream)?;
                self.m_player_get_killing_spree_count_parameters = Some(player_get_killing_spree_count_parameters);
            }
            e_action_type::player_adjust_money => {
                let mut player_adjust_money_parameters = s_action_player_adjust_money_parameters::default();
                player_adjust_money_parameters.decode(bitstream)?;
                self.m_player_adjust_money_parameters = Some(player_adjust_money_parameters);
            }
            e_action_type::player_enable_purchases => {
                let mut player_enable_purchases_parameters = s_action_player_enable_purchases_parameters::default();
                player_enable_purchases_parameters.decode(bitstream)?;
                self.m_player_enable_purchases_parameters = Some(player_enable_purchases_parameters);
            }
            e_action_type::player_get_vehicle => {
                let mut player_get_vehicle_parameters = s_action_player_get_vehicle_parameters::default();
                player_get_vehicle_parameters.decode(bitstream)?;
                self.m_player_get_vehicle_parameters = Some(player_get_vehicle_parameters);
            }
            e_action_type::player_set_vehicle => {
                let mut player_set_vehicle_parameters = s_action_player_set_vehicle_parameters::default();
                player_set_vehicle_parameters.decode(bitstream)?;
                self.m_player_set_vehicle_parameters = Some(player_set_vehicle_parameters);
            }
            e_action_type::player_set_unit => {
                let mut player_set_unit_parameters = s_action_player_set_unit_parameters::default();
                player_set_unit_parameters.decode(bitstream)?;
                self.m_player_set_unit_parameters = Some(player_set_unit_parameters);
            }
            e_action_type::timer_reset => {
                let mut timer_reset_parameters = s_action_timer_reset_parameters::default();
                timer_reset_parameters.decode(bitstream)?;
                self.m_timer_reset_parameters = Some(timer_reset_parameters);
            }
            e_action_type::weapon_set_pickup_priority => {
                let mut weapon_set_pickup_priority_parameters = s_action_weapon_set_pickup_priority_parameters::default();
                weapon_set_pickup_priority_parameters.decode(bitstream)?;
                self.m_weapon_set_pickup_priority_parameters = Some(weapon_set_pickup_priority_parameters);
            }
            e_action_type::object_bounce => {
                let mut object_bounce_parameters = s_action_object_bounce_parameters::default();
                object_bounce_parameters.decode(bitstream)?;
                self.m_object_bounce_parameters = Some(object_bounce_parameters);
            }
            e_action_type::hud_widget_set_text => {
                let mut hud_widget_set_text_parameters = s_action_hud_widget_set_text_parameters::default();
                hud_widget_set_text_parameters.decode(bitstream)?;
                self.m_hud_widget_set_text_parameters = Some(hud_widget_set_text_parameters);
            }
            e_action_type::hud_widget_set_value => {
                let mut hud_widget_set_value_parameters = s_action_hud_widget_set_value_parameters::default();
                hud_widget_set_value_parameters.decode(bitstream)?;
                self.m_hud_widget_set_value_parameters = Some(hud_widget_set_value_parameters);
            }
            e_action_type::hud_widget_set_meter => {
                let mut hud_widget_set_meter_parameters = s_action_hud_widget_set_meter_parameters::default();
                hud_widget_set_meter_parameters.decode(bitstream)?;
                self.m_hud_widget_set_meter_parameters = Some(hud_widget_set_meter_parameters);
            }
            e_action_type::hud_widget_set_icon => {
                let mut hud_widget_set_icon_parameters = s_action_hud_widget_set_icon_parameters::default();
                hud_widget_set_icon_parameters.decode(bitstream)?;
                self.m_hud_widget_set_icon_parameters = Some(hud_widget_set_icon_parameters);
            }
            e_action_type::hud_widget_set_visibility => {
                let mut hud_widget_set_visibility_parameters = s_action_hud_widget_set_visibility_parameters::default();
                hud_widget_set_visibility_parameters.decode(bitstream)?;
                self.m_hud_widget_set_visibility_parameters = Some(hud_widget_set_visibility_parameters);
            }
            e_action_type::play_sound => {
                let mut play_sound_parameters = s_action_play_sound_parameters::default();
                play_sound_parameters.decode(bitstream)?;
                self.m_play_sound_parameters = Some(play_sound_parameters);
            }
            e_action_type::object_set_scale => {
                let mut object_set_scale_parameters = s_action_object_set_scale_parameters::default();
                object_set_scale_parameters.decode(bitstream)?;
                self.m_object_set_scale_parameters = Some(object_set_scale_parameters);
            }
            e_action_type::navpoint_set_text => {
                let mut navpoint_set_text_parameters = s_action_navpoint_set_text_parameters::default();
                navpoint_set_text_parameters.decode(bitstream)?;
                self.m_navpoint_set_text_parameters = Some(navpoint_set_text_parameters);
            }
            e_action_type::object_get_shield => {
                let mut object_get_shield_parameters = s_action_object_get_shield_parameters::default();
                object_get_shield_parameters.decode(bitstream)?;
                self.m_object_get_shield_parameters = Some(object_get_shield_parameters);
            }
            e_action_type::object_get_health => {
                let mut object_get_health_parameters = s_action_object_get_health_parameters::default();
                object_get_health_parameters.decode(bitstream)?;
                self.m_object_get_health_parameters = Some(object_get_health_parameters);
            }
            e_action_type::player_set_objective => {
                let mut player_set_objective_parameters = s_action_player_set_objective_parameters::default();
                player_set_objective_parameters.decode(bitstream)?;
                self.m_player_set_objective_parameters = Some(player_set_objective_parameters);
            }
            e_action_type::player_set_objective_allegiance => {
                let mut player_set_objective_allegiance_parameters = s_action_player_set_objective_allegiance_parameters::default();
                player_set_objective_allegiance_parameters.decode(bitstream)?;
                self.m_player_set_objective_allegiance_parameters = Some(player_set_objective_allegiance_parameters);
            }
            e_action_type::player_set_objective_allegiance_icon => {
                let mut player_set_objective_allegiance_icon_parameters = s_action_player_set_objective_allegiance_icon_parameters::default();
                player_set_objective_allegiance_icon_parameters.decode(bitstream)?;
                self.m_player_set_objective_allegiance_icon_parameters = Some(player_set_objective_allegiance_icon_parameters);
            }
            e_action_type::team_set_coop_spawning => {
                let mut team_set_coop_spawning_parameters = s_action_team_set_coop_spawning_parameters::default();
                team_set_coop_spawning_parameters.decode(bitstream)?;
                self.m_team_set_coop_spawning_parameters = Some(team_set_coop_spawning_parameters);
            }
            e_action_type::team_set_primary_respawn_object => {
                let mut team_set_primary_respawn_object_parameters = s_action_team_set_primary_respawn_object_parameters::default();
                team_set_primary_respawn_object_parameters.decode(bitstream)?;
                self.m_team_set_primary_respawn_object_parameters = Some(team_set_primary_respawn_object_parameters);
            }
            e_action_type::player_set_primary_respawn_object => {
                let mut player_set_primary_respawn_object_parameters = s_action_player_set_primary_respawn_object_parameters::default();
                player_set_primary_respawn_object_parameters.decode(bitstream)?;
                self.m_player_set_primary_respawn_object_parameters = Some(player_set_primary_respawn_object_parameters);
            }
            e_action_type::player_get_fireteam_index => {
                let mut player_get_fireteam_index_parameters = s_action_player_get_fireteam_index_parameters::default();
                player_get_fireteam_index_parameters.decode(bitstream)?;
                self.m_player_get_fireteam_index_parameters = Some(player_get_fireteam_index_parameters);
            }
            e_action_type::player_set_fireteam_index => {
                let mut player_set_fireteam_index_parameters = s_action_player_set_fireteam_index_parameters::default();
                player_set_fireteam_index_parameters.decode(bitstream)?;
                self.m_player_set_fireteam_index_parameters = Some(player_set_fireteam_index_parameters);
            }
            e_action_type::object_adjust_shield => {
                let mut object_adjust_shield_parameters = s_action_object_adjust_shield_parameters::default();
                object_adjust_shield_parameters.decode(bitstream)?;
                self.m_object_adjust_shield_parameters = Some(object_adjust_shield_parameters);
            }
            e_action_type::object_adjust_health => {
                let mut object_adjust_health_parameters = s_action_object_adjust_health_parameters::default();
                object_adjust_health_parameters.decode(bitstream)?;
                self.m_object_adjust_health_parameters = Some(object_adjust_health_parameters);
            }
            e_action_type::object_get_distance => {
                let mut object_get_distance_parameters = s_action_object_get_distance_parameters::default();
                object_get_distance_parameters.decode(bitstream)?;
                self.m_object_get_distance_parameters = Some(object_get_distance_parameters);
            }
            e_action_type::object_adjust_maximum_shield => {
                let mut object_adjust_maximum_shield_parameters = s_action_object_adjust_maximum_shield_parameters::default();
                object_adjust_maximum_shield_parameters.decode(bitstream)?;
                self.m_object_adjust_maximum_shield_parameters = Some(object_adjust_maximum_shield_parameters);
            }
            e_action_type::object_adjust_maximum_health => {
                let mut object_adjust_maximum_health_parameters = s_action_object_adjust_maximum_health_parameters::default();
                object_adjust_maximum_health_parameters.decode(bitstream)?;
                self.m_object_adjust_maximum_health_parameters = Some(object_adjust_maximum_health_parameters);
            }
            e_action_type::player_set_requisition_palette => {
                let mut player_set_requisition_palette_parameters = s_action_player_set_requisition_palette_parameters::default();
                player_set_requisition_palette_parameters.decode(bitstream)?;
                self.m_player_set_requisition_palette_parameters = Some(player_set_requisition_palette_parameters);
            }
            e_action_type::device_set_power => {
                let mut device_set_power_parameters = s_action_device_set_power_parameters::default();
                device_set_power_parameters.decode(bitstream)?;
                self.m_device_set_power_parameters = Some(device_set_power_parameters);
            }
            e_action_type::device_get_power => {
                let mut device_get_power_parameters = s_action_device_get_power_parameters::default();
                device_get_power_parameters.decode(bitstream)?;
                self.m_device_get_power_parameters = Some(device_get_power_parameters);
            }
            e_action_type::device_set_position => {
                let mut device_set_position_parameters = s_action_device_set_position_parameters::default();
                device_set_position_parameters.decode(bitstream)?;
                self.m_device_set_position_parameters = Some(device_set_position_parameters);
            }
            e_action_type::device_get_position => {
                let mut device_get_position_parameters = s_action_device_get_position_parameters::default();
                device_get_position_parameters.decode(bitstream)?;
                self.m_device_get_position_parameters = Some(device_get_position_parameters);
            }
            e_action_type::adjust_grenades => {
                let mut adjust_grenades_parameters = s_action_adjust_grenades_parameters::default();
                adjust_grenades_parameters.decode(bitstream)?;
                self.m_adjust_grenades_parameters = Some(adjust_grenades_parameters);
            }
            e_action_type::submit_incident => {
                let mut submit_incident_parameters = s_action_submit_incident_parameters::default();
                submit_incident_parameters.decode(bitstream)?;
                self.m_submit_incident_parameters = Some(submit_incident_parameters);
            }
            e_action_type::submit_incident_with_custom_value => {
                let mut submit_incident_with_custom_value_parameters = s_action_submit_incident_with_custom_value_parameters::default();
                submit_incident_with_custom_value_parameters.decode(bitstream)?;
                self.m_submit_incident_with_custom_value_parameters = Some(submit_incident_with_custom_value_parameters);
            }
            e_action_type::set_loadout_palette => {
                let mut set_loadout_palette_parameters = s_action_set_loadout_palette_parameters::default();
                set_loadout_palette_parameters.decode(bitstream)?;
                self.m_set_loadout_palette_parameters = Some(set_loadout_palette_parameters);
            }
            e_action_type::device_set_position_track => {
                let mut device_set_position_track_parameters = s_action_device_set_position_track_parameters::default();
                device_set_position_track_parameters.decode(bitstream)?;
                self.m_device_set_position_track_parameters = Some(device_set_position_track_parameters);
            }
            e_action_type::device_animate_position => {
                let mut device_animate_position_parameters = s_action_device_animate_position_parameters::default();
                device_animate_position_parameters.decode(bitstream)?;
                self.m_device_animate_position_parameters = Some(device_animate_position_parameters);
            }
            e_action_type::device_set_position_immediate => {
                let mut device_set_position_immediate_parameters = s_action_device_set_position_immediate_parameters::default();
                device_set_position_immediate_parameters.decode(bitstream)?;
                self.m_device_set_position_immediate_parameters = Some(device_set_position_immediate_parameters);
            }
            e_action_type::saved_film_insert_marker => {
                let mut saved_film_insert_marker_parameters = s_action_saved_film_insert_marker_parameters::default();
                saved_film_insert_marker_parameters.decode(bitstream)?;
                self.m_saved_film_insert_marker_parameters = Some(saved_film_insert_marker_parameters);
            }
            e_action_type::respawn_zone_enable => {
                let mut respawn_zone_enable_parameters = s_action_respawn_zone_enable_parameters::default();
                respawn_zone_enable_parameters.decode(bitstream)?;
                self.m_respawn_zone_enable_parameters = Some(respawn_zone_enable_parameters);
            }
            e_action_type::player_get_weapon => {
                let mut player_get_weapon_parameters = s_action_player_get_weapon_parameters::default();
                player_get_weapon_parameters.decode(bitstream)?;
                self.m_player_get_weapon_parameters = Some(player_get_weapon_parameters);
            }
            e_action_type::player_get_equipment => {
                let mut player_get_equipment_parameters = s_action_player_get_equipment_parameters::default();
                player_get_equipment_parameters.decode(bitstream)?;
                self.m_player_get_equipment_parameters = Some(player_get_equipment_parameters);
            }
            e_action_type::object_set_never_garbage => {
                let mut object_set_never_garbage_parameters = s_action_object_set_never_garbage_parameters::default();
                object_set_never_garbage_parameters.decode(bitstream)?;
                self.m_object_set_never_garbage_parameters = Some(object_set_never_garbage_parameters);
            }
            e_action_type::player_get_target_object => {
                let mut player_get_target_object_parameters = s_action_player_get_target_object_parameters::default();
                player_get_target_object_parameters.decode(bitstream)?;
                self.m_player_get_target_object_parameters = Some(player_get_target_object_parameters);
            }
            e_action_type::create_tunnel => {
                let mut create_tunnel_parameters = s_action_create_tunnel_parameters::default();
                create_tunnel_parameters.decode(bitstream)?;
                self.m_create_tunnel_parameters = Some(create_tunnel_parameters);
            }
            e_action_type::debug_force_player_view_count => {
                let mut debug_force_player_view_count_parameters = s_action_debug_force_player_view_count_parameters::default();
                debug_force_player_view_count_parameters.decode(bitstream)?;
                self.m_debug_force_player_view_count_parameters = Some(debug_force_player_view_count_parameters);
            }
            e_action_type::player_pick_up_weapon => {
                let mut player_pick_up_weapon_parameters = s_action_player_pick_up_weapon_parameters::default();
                player_pick_up_weapon_parameters.decode(bitstream)?;
                self.m_player_pick_up_weapon_parameters = Some(player_pick_up_weapon_parameters);
            }
            e_action_type::player_set_coop_spawning => {
                let mut player_set_coop_spawning_parameters = s_action_player_set_coop_spawning_parameters::default();
                player_set_coop_spawning_parameters.decode(bitstream)?;
                self.m_player_set_coop_spawning_parameters = Some(player_set_coop_spawning_parameters);
            }
            e_action_type::object_set_orientation => {
                let mut object_set_orientation_parameters = s_action_object_set_orientation_parameters::default();
                object_set_orientation_parameters.decode(bitstream)?;
                self.m_object_set_orientation_parameters = Some(object_set_orientation_parameters);
            }
            e_action_type::object_face_object => {
                let mut object_face_object_parameters = s_action_object_face_object_parameters::default();
                object_face_object_parameters.decode(bitstream)?;
                self.m_object_face_object_parameters = Some(object_face_object_parameters);
            }
            e_action_type::biped_give_weapon => {
                let mut biped_give_weapon_parameters = s_action_biped_give_weapon_parameters::default();
                biped_give_weapon_parameters.decode(bitstream)?;
                self.m_biped_give_weapon_parameters = Some(biped_give_weapon_parameters);
            }
            e_action_type::biped_drop_weapon => {
                let mut biped_drop_weapon_parameters = s_action_biped_drop_weapon_parameters::default();
                biped_drop_weapon_parameters.decode(bitstream)?;
                self.m_biped_drop_weapon_parameters = Some(biped_drop_weapon_parameters);
            }
            e_action_type::set_scenario_interpolator_state => {
                let mut set_scenario_interpolator_state_parameters = s_action_set_scenario_interpolator_state_parameters::default();
                set_scenario_interpolator_state_parameters.decode(bitstream)?;
                self.m_set_scenario_interpolator_state_parameters = Some(set_scenario_interpolator_state_parameters);
            }
            e_action_type::get_random_object => {
                let mut get_random_object_parameters = s_action_get_random_object_parameters::default();
                get_random_object_parameters.decode(bitstream)?;
                self.m_get_random_object_parameters = Some(get_random_object_parameters);
            }
            e_action_type::game_grief_record_custom_penalty => {
                let mut game_grief_record_custom_penalty_parameters = s_action_game_grief_record_custom_penalty_parameters::default();
                game_grief_record_custom_penalty_parameters.decode(bitstream)?;
                self.m_game_grief_record_custom_penalty_parameters = Some(game_grief_record_custom_penalty_parameters);
            }
            e_action_type::boundary_set_player_color => {
                let mut boundary_set_player_color_parameters = s_action_boundary_set_player_color_parameters::default();
                boundary_set_player_color_parameters.decode(bitstream)?;
                self.m_boundary_set_player_color_parameters = Some(boundary_set_player_color_parameters);
            }
            e_action_type::begin => {
                let mut begin_parameters = s_action_begin_parameters::default();
                begin_parameters.decode(bitstream)?;
                self.m_begin_parameters = Some(begin_parameters);
            }
            _ => {}
        }

        Ok(())
    }
}