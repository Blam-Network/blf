use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_text::c_dynamic_string;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_variant_variable::s_variant_variable;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_hud_widgets::e_megalogamengine_hud_meter_input_type;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::game_engine_timer::e_game_engine_timer_rate;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::saved_games::scenario_map_variant::e_boundary_shape;
use blf_lib::bitfield;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::OPTION_TO_RESULT;
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use crate::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_object_reference::c_object_reference;
use crate::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_sounds::e_megalo_sound;

#[repr(u8)]
#[derive(Default, Clone, Copy, PartialEq, Debug, Serialize, Deserialize, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(2)]
pub enum e_multiplayer_powerup_flavor {
    #[default]
    red = 0,
    blue = 1,
    yellow = 2,
    custom = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_weapon_pickup_priority {
    #[default]
    normal = 0,
    high = 1,
    automatic = 2,
}

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
    ordnance = 20,
    interface = 21,
    recon = 22,
    ammunition = 23,
    recover = 24,
    defend = 25,
    neutralize = 26,
    coop_spawning = 27,
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
    not = 9, // ~= (sets result to ~rhs)
    /// Halo 4 `compute_math_operation` case 10 (`slw`); RVT `<<=`.
    left_shift = 10,
    /// Halo 4 `compute_math_operation` case 11 (`sraw`); RVT `>>=`.
    right_shift = 11,
    /// Halo 4 `compute_math_operation` case 12 (`int_abs`); RVT `__abs_assign`.
    abs = 12,
}

/// Halo 4 `_UnitGrenadeType` (`c_enum<...,0,8>` → 3 bits, values 0..7).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_grenade_type {
    #[default]
    frag_grenade = 0,
    plasma_grenade = 1,
    pulse_grenade = 2,
    needle_grenade = 3,
    claymore_grenade = 4,
    grenade_type_5 = 5,
    grenade_type_6 = 6,
    grenade_type_7 = 7,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_biped_give_weapon_mode {
    #[default]
    primary = 0,
    secondary = 1,
    force = 2,
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
    pub m_sound_index: e_megalo_sound, // biased index, max 240, 8 bits
    pub m_string: c_dynamic_string,
}

impl s_action_hud_post_message_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_target.encode(bitstream)?;
        bitstream.write_index::<240>(self.m_sound_index as i32, 8)?;
        self.m_string.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_target.decode(bitstream)?;
        self.m_sound_index = e_megalo_sound::from_i32(bitstream.read_index::<240>("sound-index", 8)?)
            .ok_or_else(|| BLFLibError::from("Unexpected enum value for sound-index"))?;
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
    pub m_trigger_index: u16, // 7 bits
}

impl s_action_for_each_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_trigger_index, 7)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_trigger_index = bitstream.read_integer("trigger-index", 7)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_megalo_action_scope {
    pub m_first_condition: i16,
    pub m_condition_count: u16,
    pub m_first_action: i16,
    pub m_action_count: u16,
}

impl s_megalo_action_scope {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<576>(self.m_first_condition as i32, 10)?;
        bitstream.write_integer(self.m_condition_count, 10)?;
        bitstream.write_index::<1088>(self.m_first_action as i32, 11)?;
        bitstream.write_integer(self.m_action_count, 11)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_first_condition =
            bitstream.read_index::<576>("first-condition-index", 10)? as i16;
        self.m_condition_count = bitstream.read_integer("condition-count", 10)?;
        self.m_first_action =
            bitstream.read_index::<1088>("first-action-index", 11)? as i16;
        self.m_action_count = bitstream.read_integer("action-count", 11)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_begin_parameters {
    pub m_scope: s_megalo_action_scope,
}

impl s_action_begin_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_scope.encode(bitstream)
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_scope.decode(bitstream)
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_variable_only_parameters {
    pub m_variable: c_custom_variable_reference,
}

impl s_action_variable_only_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_variable.encode(bitstream)
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_variable.decode(bitstream)
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_disallow_match_join_in_progress_parameters {
    pub m_flag: bool,
}

impl s_action_disallow_match_join_in_progress_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_flag)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_flag = bitstream.read_bool("flag")?;
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
        self.m_enabled = bitstream.read_bool("flag")?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_winning_player_for_final_kill_cam_parameters {
    pub m_player: c_player_reference,
    pub m_duration: f32,
}

impl s_action_set_winning_player_for_final_kill_cam_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        use crate::blam::halo4::v20810_12_09_22_1647_main::memory::bitstream_writer::c_bitstream_writer_extensions;
        self.m_player.encode(bitstream)?;
        bitstream.write_quantized_real(self.m_duration, 0.0, 1.0, 8, false, false)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        use crate::blam::halo4::v20810_12_09_22_1647_main::memory::bitstream_reader::c_bitstream_reader_extensions;
        self.m_player.decode(bitstream)?;
        self.m_duration = bitstream.read_quantized_real(0.0, 1.0, 8, false, false)?.0;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_medal_scoring_parameters {
    pub m_allow_medal_scoring: bool,
}

impl s_action_set_medal_scoring_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_allow_medal_scoring)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_allow_medal_scoring = bitstream.read_bool("allow-medal-scoring")?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_incident_get_player_parameters {
    pub m_player: c_player_reference,
}

impl s_action_incident_get_player_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_medal_override_parameters {
    pub m_medal_index: i32,
    pub m_variable_1: c_custom_variable_reference,
    pub m_variable_2: c_custom_variable_reference,
}

impl s_action_set_medal_override_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<256>(self.m_medal_index, 8)?;
        self.m_variable_1.encode(bitstream)?;
        self.m_variable_2.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_medal_index = bitstream.read_index::<256>("medal-index", 8)?;
        self.m_variable_1.decode(bitstream)?;
        self.m_variable_2.decode(bitstream)?;
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


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_enable_purchases_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
    pub m_mode: u8, // 5 bits
}

impl s_action_player_enable_purchases_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;
        bitstream.write_integer(self.m_mode, 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;
        self.m_mode = bitstream.read_integer("mode", 5)?;

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
    pub m_sound_index: e_megalo_sound, // biased index, max 240, 8 bits
    pub m_immediate: bool,
    pub m_target: s_team_or_player_target,
}

impl s_action_play_sound_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<240>(self.m_sound_index as i32, 8)?;
        bitstream.write_bool(self.m_immediate)?;
        self.m_target.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_sound_index = e_megalo_sound::from_i32(bitstream.read_index::<240>("sound-index", 8)?)
            .ok_or_else(|| BLFLibError::from("Unexpected enum value for sound-index"))?;
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
pub struct s_action_object_get_distance_parameters {
    pub m_object_1: c_object_reference,
    pub m_object_2: c_object_reference,
    pub m_variable: c_custom_variable_reference,
    pub m_allow_dead: bool,
}

impl s_action_object_get_distance_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;
        bitstream.write_bool(self.m_allow_dead)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;
        self.m_allow_dead = bitstream.read_bool("allow-dead")?;

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
    pub m_grenade_type: e_grenade_type, // 3 bits
    pub m_math_operation: e_math_operation, // 4 bits
    pub m_variable: c_custom_variable_reference,
}

impl s_action_adjust_grenades_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_grenade_type, 3)?;
        bitstream.write_enum_raw(self.m_math_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_grenade_type = bitstream.read_enum_raw("grenade-type", 3)?;
        self.m_math_operation = bitstream.read_enum_raw("math-operation", 4)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_submit_incident_parameters {
    pub m_incident_id: i16,
    pub m_target_1: s_team_or_player_target,
    pub m_target_2: s_team_or_player_target,
}

impl s_action_submit_incident_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer((self.m_incident_id + 1) as u16, 10)?;
        self.m_target_1.encode(bitstream)?;
        self.m_target_2.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_incident_id = bitstream.read_integer::<i32>("incident-id", 10)? as i16 - 1;
        self.m_target_1.decode(bitstream)?;
        self.m_target_2.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_submit_incident_with_custom_value_parameters {
    pub m_incident_id: i16,
    pub m_target_1: s_team_or_player_target,
    pub m_target_2: s_team_or_player_target,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_submit_incident_with_custom_value_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer((self.m_incident_id + 1) as u16, 10)?;
        self.m_target_1.encode(bitstream)?;
        self.m_target_2.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_incident_id = bitstream.read_integer::<i32>("incident-id", 10)? as i16 - 1;
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


/// Halo 4 biped drop-weapon mode (`c_enum_no_init<...,0,3>` → 0..2, 2 bits).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_biped_drop_weapon_mode {
    #[default]
    primary = 0,
    secondary = 1,
    both = 2,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_biped_drop_weapon_parameters {
    pub m_object: c_object_reference,
    pub m_drop_mode: e_biped_drop_weapon_mode,
    pub m_delete_on_drop: bool,
}

impl s_action_biped_drop_weapon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_drop_mode, 2)?;
        bitstream.write_bool(self.m_delete_on_drop)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_drop_mode = bitstream.read_enum_raw("drop-mode", 2)?;
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
    unknown_14 = 14,
    unknown_15 = 15,
    unknown_16 = 16,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_nav_point_set_type_parameters {
    pub m_object: c_object_reference,
    pub m_nav_point_name_type: i32,
}

impl s_action_nav_point_set_type_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_index::<255>(self.m_nav_point_name_type, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_nav_point_name_type = bitstream.read_index::<255>("nav-point-name-type", 8)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_award_medal_parameters {
    pub m_target: s_team_or_player_target,
    pub m_medal_index: i32,
    pub m_variable: c_custom_variable_reference,
    pub m_use_point_value_override: bool,
}

impl s_action_award_medal_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_target.encode(bitstream)?;
        bitstream.write_index::<256>(self.m_medal_index, 8)?;
        self.m_variable.encode(bitstream)?;
        bitstream.write_bool(self.m_use_point_value_override)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_target.decode(bitstream)?;
        self.m_medal_index = bitstream.read_index::<256>("medal-index", 8)?;
        self.m_variable.decode(bitstream)?;
        self.m_use_point_value_override = bitstream.read_bool("use-point-value-override")?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_momentum_parameters {
    pub m_target: s_team_or_player_target,
    pub m_operation: e_math_operation,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_set_momentum_parameters {
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

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_get_button_time_parameters {
    pub m_player: c_player_reference,
    pub m_buttons: e_scriptable_game_buttons,
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
pub struct s_action_data_mine_add_object_position_parameters {
    pub m_name_index: i32,
    pub m_object: c_object_reference,
}

impl s_action_data_mine_add_object_position_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_name_index, 8)?;
        self.m_object.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_name_index = bitstream.read_index::<148>("datamine-object-pos-name", 8)?;
        self.m_object.decode(bitstream)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_bool_enabled_parameters {
    pub m_enabled: bool,
}

impl s_action_bool_enabled_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_enabled)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_enabled = bitstream.read_bool("enabled")?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_clear_medal_override_parameters {
    pub m_medal_index: i32,
}

impl s_action_clear_medal_override_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<256>(self.m_medal_index, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_medal_index = bitstream.read_index::<256>("medal-index", 8)?;
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
pub struct s_action_nav_point_set_is_territory_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_nav_point_set_is_territory_parameters {
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
pub struct s_action_nav_point_set_is_spawning_territory_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_nav_point_set_is_spawning_territory_parameters {
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
pub struct s_action_nav_point_set_territory_level_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_nav_point_set_territory_level_parameters {
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
pub struct s_action_nav_point_set_max_territory_level_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_nav_point_set_max_territory_level_parameters {
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
pub struct s_action_nav_point_set_territory_sort_order_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_nav_point_set_territory_sort_order_parameters {
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
pub struct s_action_nav_point_set_territory_timer_parameters {
    pub m_object: c_object_reference,
    pub m_index: i32,
}

impl s_action_nav_point_set_territory_timer_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_index::<4>(self.m_index, 2)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_index = bitstream.read_index::<4>("index", 2)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_nav_point_set_action_team_parameters {
    pub m_object: c_object_reference,
    pub m_team: c_team_reference,
}

impl s_action_nav_point_set_action_team_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_team.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_team.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_load_game_hud_parameters {
    pub m_index: i32,
}

impl s_action_load_game_hud_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<255>(self.m_index, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<255>("index", 8)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_progress_bar_user_data_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_set_progress_bar_user_data_parameters {
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
pub struct s_action_player_get_team_place_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_player_get_team_place_parameters {
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
pub struct s_action_team_get_index_parameters {
    pub m_team: c_team_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_team_get_index_parameters {
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
pub struct s_action_player_get_ultimate_parent_not_self_parameters {
    pub m_player: c_player_reference,
    pub m_object: c_object_reference,
}

impl s_action_player_get_ultimate_parent_not_self_parameters {
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
pub struct s_action_player_report_health_as_shields_parameters {
    pub m_player: c_player_reference,
    pub m_flag: bool,
}

impl s_action_player_report_health_as_shields_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_bool(self.m_flag)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_flag = bitstream.read_bool("flag")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_get_immediate_parent_player_parameters {
    pub m_object_1: c_object_reference,
    pub m_object_2: c_object_reference,
}

impl s_action_object_get_immediate_parent_player_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_play_sound_on_object_parameters {
    pub m_object: c_object_reference,
    pub m_index: i32,
}

impl s_action_play_sound_on_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_index::<240>(self.m_index, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_index = bitstream.read_index::<240>("index", 8)?;
        Ok(())
    }
}
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_navpoint_set_secondary_text_parameters {
    pub m_object: c_object_reference,
    pub m_string: c_dynamic_string,
}

impl Default for s_action_navpoint_set_secondary_text_parameters {
    fn default() -> Self {
        Self {
            m_object: Default::default(),
            m_string: c_dynamic_string::with_max_tokens(1),
        }
    }
}

impl s_action_navpoint_set_secondary_text_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_string.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_string = c_dynamic_string::with_max_tokens(1);
        self.m_string.decode(bitstream)?;
        Ok(())
    }
}
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_mode_objective_text_line_parameters {
    pub m_team: c_team_reference,
    pub m_string: c_dynamic_string,
}

impl Default for s_action_set_mode_objective_text_line_parameters {
    fn default() -> Self {
        Self {
            m_team: Default::default(),
            m_string: c_dynamic_string::with_max_tokens(2),
        }
    }
}

impl s_action_set_mode_objective_text_line_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_team.encode(bitstream)?;
        self.m_string.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team.decode(bitstream)?;
        self.m_string = c_dynamic_string::with_max_tokens(2);
        self.m_string.decode(bitstream)?;
        Ok(())
    }
}
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_current_round_string_parameters {
    pub m_string: c_dynamic_string,
}

impl Default for s_action_set_current_round_string_parameters {
    fn default() -> Self {
        Self {
            m_string: c_dynamic_string::with_max_tokens(3),
        }
    }
}

impl s_action_set_current_round_string_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_string.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_string = c_dynamic_string::with_max_tokens(3);
        self.m_string.decode(bitstream)?;
        Ok(())
    }
}
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_previous_round_string_parameters {
    pub m_string: c_dynamic_string,
}

impl Default for s_action_set_previous_round_string_parameters {
    fn default() -> Self {
        Self {
            m_string: c_dynamic_string::with_max_tokens(3),
        }
    }
}

impl s_action_set_previous_round_string_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_string.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_string = c_dynamic_string::with_max_tokens(3);
        self.m_string.decode(bitstream)?;
        Ok(())
    }
}
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_team_win_loss_string_parameters {
    pub m_team: c_team_reference,
    pub m_string: c_dynamic_string,
}

impl Default for s_action_set_team_win_loss_string_parameters {
    fn default() -> Self {
        Self {
            m_team: Default::default(),
            m_string: c_dynamic_string::with_max_tokens(2),
        }
    }
}

impl s_action_set_team_win_loss_string_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_team.encode(bitstream)?;
        self.m_string.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team.decode(bitstream)?;
        self.m_string = c_dynamic_string::with_max_tokens(2);
        self.m_string.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_get_health_absolute_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_object_get_health_absolute_parameters {
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
pub struct s_action_hs_function_call_parameters {
    pub m_index: i32,
}

impl s_action_hs_function_call_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<255>(self.m_index, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<255>("index", 8)?;
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
    pub m_flag: bool,
}

impl s_action_hide_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_bool(self.m_flag)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_flag = bitstream.read_bool("flag")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_auto_turret_range_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_set_auto_turret_range_parameters {
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

/// IDA layout: object@0, var1@3, var2@6, var3@9; encode order object, var2, var3, var1.
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_auto_turret_parameters {
    pub m_object: c_object_reference,
    pub m_variable_1: c_custom_variable_reference,
    pub m_variable_2: c_custom_variable_reference,
    pub m_variable_3: c_custom_variable_reference,
}

impl s_action_set_auto_turret_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable_2.encode(bitstream)?;
        self.m_variable_3.encode(bitstream)?;
        self.m_variable_1.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable_2.decode(bitstream)?;
        self.m_variable_3.decode(bitstream)?;
        self.m_variable_1.decode(bitstream)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_get_teleporter_channel_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
    pub m_is_spawner: bool,
}

impl s_action_get_teleporter_channel_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;
        bitstream.write_bool(self.m_is_spawner)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;
        self.m_is_spawner = bitstream.read_bool("isSpawner")?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_get_digit_parameters {
    pub m_variable_1: c_custom_variable_reference,
    pub m_place: u8,
    pub m_variable_2: c_custom_variable_reference,
}

impl s_action_get_digit_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_variable_1.encode(bitstream)?;
        bitstream.write_integer(self.m_place as u32, 5)?;
        self.m_variable_2.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_variable_1.decode(bitstream)?;
        self.m_place = bitstream.read_integer("GetDigit-place", 5)?;
        self.m_variable_2.decode(bitstream)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_player_min_death_seconds_parameters {
    pub m_player: c_player_reference,
    pub m_seconds_dead: u32,
}

impl s_action_set_player_min_death_seconds_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_integer(self.m_seconds_dead, 32)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_seconds_dead = bitstream.read_integer("seconds-dead", 32)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_get_vehicle_entering_player_parameters {
    pub m_object: c_object_reference,
    pub m_player: c_player_reference,
}

impl s_action_get_vehicle_entering_player_parameters {
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
pub struct s_action_set_candy_spawner_active_parameters {
    pub m_object: c_object_reference,
    pub m_active: bool,
    pub m_spawnOnWake: bool,
    pub m_player: c_player_reference,
}

impl s_action_set_candy_spawner_active_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_bool(self.m_active)?;
        bitstream.write_bool(self.m_spawnOnWake)?;
        self.m_player.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_active = bitstream.read_bool("active")?;
        self.m_spawnOnWake = bitstream.read_bool("spawnOnWake")?;
        self.m_player.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_force_respawn_parameters {
    pub m_player: c_player_reference,
}

impl s_action_player_force_respawn_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_enable_spawning_parameters {
    pub m_player: c_player_reference,
    pub m_flag: bool,
}

impl s_action_player_enable_spawning_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_bool(self.m_flag)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_flag = bitstream.read_bool("flag")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_enable_territory_spawning_parameters {
    pub m_player: c_player_reference,
    pub m_flag: bool,
}

impl s_action_enable_territory_spawning_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_bool(self.m_flag)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_flag = bitstream.read_bool("flag")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_enable_territory_spawn_selection_parameters {
    pub m_player: c_player_reference,
    pub m_flag: bool,
}

impl s_action_enable_territory_spawn_selection_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_bool(self.m_flag)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_flag = bitstream.read_bool("flag")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_territory_friendly_and_selected_parameters {
    pub m_variable_1: c_custom_variable_reference,
    pub m_variable_2: c_custom_variable_reference,
}

impl s_action_set_territory_friendly_and_selected_parameters {
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
pub struct s_action_set_territory_last_stand_imminent_te_parameters {
    pub m_team: c_team_reference,
}

impl s_action_set_territory_last_stand_imminent_te_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_team.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_is_detectable_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_player_is_detectable_parameters {
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
pub struct s_action_device_get_player_user_parameters {
    pub m_object: c_object_reference,
    pub m_player: c_player_reference,
}

impl s_action_device_get_player_user_parameters {
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
pub struct s_action_device_get_interacting_player_user_parameters {
    pub m_object: c_object_reference,
    pub m_player: c_player_reference,
}

impl s_action_device_get_interacting_player_user_parameters {
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
pub struct s_action_device_get_hold_time_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_device_get_hold_time_parameters {
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
pub struct s_action_device_set_hold_time_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_device_set_hold_time_parameters {
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
pub struct s_action_set_teleporter_channel_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_set_teleporter_channel_parameters {
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
pub struct s_action_get_total_spawn_time_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_get_total_spawn_time_parameters {
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
pub struct s_action_data_mine_begin_parameters {
    pub m_index: i32,
}

impl s_action_data_mine_begin_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_index, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<148>("index", 8)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_data_mine_add_category_parameters {
    pub m_index: i32,
}

impl s_action_data_mine_add_category_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_index, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<148>("index", 8)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_data_mine_add_real_parameters {
    pub m_index: i32,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_data_mine_add_real_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_index, 8)?;
        self.m_variable.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<148>("index", 8)?;
        self.m_variable.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_data_mine_add_int_parameters {
    pub m_index: i32,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_data_mine_add_int_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_index, 8)?;
        self.m_variable.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<148>("index", 8)?;
        self.m_variable.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_data_mine_add_timer_ticks_remaining_parameters {
    pub m_index: i32,
    pub m_timer: c_custom_timer_reference,
}

impl s_action_data_mine_add_timer_ticks_remaining_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_index, 8)?;
        self.m_timer.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<148>("index", 8)?;
        self.m_timer.decode(bitstream)?;
        Ok(())
    }
}
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_data_mine_add_string_parameters {
    pub m_index: i32,
    pub m_string: c_dynamic_string,
}

impl Default for s_action_data_mine_add_string_parameters {
    fn default() -> Self {
        Self {
            m_index: Default::default(),
            m_string: c_dynamic_string::with_max_tokens(2),
        }
    }
}

impl s_action_data_mine_add_string_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_index, 8)?;
        self.m_string.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<148>("index", 8)?;
        self.m_string = c_dynamic_string::with_max_tokens(2);
        self.m_string.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_data_mine_add_player_parameters {
    pub m_index: i32,
    pub m_player: c_player_reference,
}

impl s_action_data_mine_add_player_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_index, 8)?;
        self.m_player.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<148>("index", 8)?;
        self.m_player.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_data_mine_add_team_parameters {
    pub m_index: i32,
    pub m_team: c_team_reference,
}

impl s_action_data_mine_add_team_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_index, 8)?;
        self.m_team.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<148>("index", 8)?;
        self.m_team.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_data_mine_add_player_position_parameters {
    pub m_index: i32,
    pub m_player: c_player_reference,
}

impl s_action_data_mine_add_player_position_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_index, 8)?;
        self.m_player.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<148>("index", 8)?;
        self.m_player.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_set_position_parameters {
    pub m_object_1: c_object_reference,
    pub m_object_2: c_object_reference,
}

impl s_action_object_set_position_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_find_drop_position_parameters {
    pub m_object_1: c_object_reference,
    pub m_object_2: c_object_reference,
    pub m_drop_range: u8,
    pub m_drop_height_delta: u8,
    pub m_drop_arc_angle: u8,
    pub m_drop_clear_dist: u8,
    pub m_object_3: c_object_reference,
}

impl s_action_find_drop_position_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        bitstream.write_integer(self.m_drop_range as u32, 8)?;
        bitstream.write_integer(self.m_drop_height_delta as u32, 8)?;
        bitstream.write_integer(self.m_drop_arc_angle as u32, 8)?;
        bitstream.write_integer(self.m_drop_clear_dist as u32, 8)?;
        self.m_object_3.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        self.m_drop_range = bitstream.read_integer("dropRange", 8)?;
        self.m_drop_height_delta = bitstream.read_integer("dropHeightDelta", 8)?;
        self.m_drop_arc_angle = bitstream.read_integer("dropArcAngle", 8)?;
        self.m_drop_clear_dist = bitstream.read_integer("dropClearDist", 8)?;
        self.m_object_3.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_copy_boundary_parameters {
    pub m_object_1: c_object_reference,
    pub m_object_2: c_object_reference,
}

impl s_action_object_copy_boundary_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_object_query_kill_boundaries_parameters {
    pub m_object: c_object_reference,
    pub m_variable_1: c_custom_variable_reference,
    pub m_variable_2: c_custom_variable_reference,
}

impl s_action_object_query_kill_boundaries_parameters {
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
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_sentry_active_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_set_sentry_active_parameters {
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
pub struct s_action_set_sentry_barrel_active_parameters {
    pub m_object: c_object_reference,
    pub m_use_primary: bool,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_set_sentry_barrel_active_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_bool(self.m_use_primary)?;
        self.m_variable.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_use_primary = bitstream.read_bool("use-primary")?;
        self.m_variable.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_is_spawner_blocked_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_is_spawner_blocked_parameters {
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
pub struct s_action_is_spawner_ready_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_is_spawner_ready_parameters {
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
pub struct s_action_set_momentum_tick_rate_parameters {
    pub m_variable: c_custom_variable_reference,
}

impl s_action_set_momentum_tick_rate_parameters {
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
pub struct s_action_device_control_set_exclusive_user_parameters {
    pub m_object: c_object_reference,
    pub m_player: c_player_reference,
}

impl s_action_device_control_set_exclusive_user_parameters {
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
pub struct s_action_device_control_set_action_mode_parameters {
    pub m_object: c_object_reference,
    pub m_flag: bool,
}

impl s_action_device_control_set_action_mode_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_bool(self.m_flag)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_flag = bitstream.read_bool("flag")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_device_dispenser_set_enabled_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
    pub m_enable_immediately: bool,
}

impl s_action_device_dispenser_set_enabled_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;
        bitstream.write_bool(self.m_enable_immediately)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;
        self.m_enable_immediately = bitstream.read_bool("enable-immediately")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_device_touch_parameters {
    pub m_object: c_object_reference,
    pub m_player: c_player_reference,
}

impl s_action_device_touch_parameters {
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
pub struct s_action_device_set_require_line_of_sight_parameters {
    pub m_object: c_object_reference,
    pub m_flag: bool,
}

impl s_action_device_set_require_line_of_sight_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_bool(self.m_flag)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_flag = bitstream.read_bool("flag")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_megalo_object_function_parameters {
    pub m_object: c_object_reference,
    pub m_variable: c_custom_variable_reference,
    pub m_function_value: bool,
}

impl s_action_set_megalo_object_function_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;
        bitstream.write_bool(self.m_function_value)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_variable.decode(bitstream)?;
        self.m_function_value = bitstream.read_bool("function-value")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_megalo_timer_object_function_parameters {
    pub m_object: c_object_reference,
    pub m_index: i32,
}

impl s_action_set_megalo_timer_object_function_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_index::<4>(self.m_index, 2)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_index = bitstream.read_index::<4>("index", 2)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_random_ordnance_get_enabled_parameters {
    pub m_variable: c_custom_variable_reference,
}

impl s_action_random_ordnance_get_enabled_parameters {
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
pub struct s_action_random_ordnance_set_count_parameters {
    pub m_variable: c_custom_variable_reference,
}

impl s_action_random_ordnance_set_count_parameters {
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
pub struct s_action_random_ordnance_set_delay_parameters {
    pub m_variable_1: c_custom_variable_reference,
    pub m_variable_2: c_custom_variable_reference,
}

impl s_action_random_ordnance_set_delay_parameters {
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
pub struct s_action_random_ordnance_do_drop_parameters {
    pub m_index: i32,
    pub m_randomize: bool,
    pub m_value: f32,
}

impl s_action_random_ordnance_do_drop_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_index, 8)?;
        bitstream.write_bool(self.m_randomize)?;
        use crate::blam::halo4::v20810_12_09_22_1647_main::memory::bitstream_writer::c_bitstream_writer_extensions;
        bitstream.write_quantized_real(self.m_value, 0.0, 10.0, 7, false, true)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<148>("index", 8)?;
        self.m_randomize = bitstream.read_bool("randomize")?;
        use crate::blam::halo4::v20810_12_09_22_1647_main::memory::bitstream_reader::c_bitstream_reader_extensions;
        self.m_value = bitstream.read_quantized_real(0.0, 10.0, 7, false, true)?.0;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_ordnance_set_clear_parameters {
    pub m_index: i32,
}

impl s_action_ordnance_set_clear_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<148>(self.m_index, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_index = bitstream.read_index::<148>("index", 8)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_blink_navpoint_parameters {
    pub m_object: c_object_reference,
    pub m_flag: bool,
}

impl s_action_blink_navpoint_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_bool(self.m_flag)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_flag = bitstream.read_bool("flag")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_pulse_navpoint_parameters {
    pub m_object: c_object_reference,
    pub m_flag: bool,
}

impl s_action_pulse_navpoint_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_bool(self.m_flag)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_flag = bitstream.read_bool("flag")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_track_carried_object_state_parameters {
    pub m_player: c_player_reference,
    pub m_objectName: u32,
}

impl s_action_track_carried_object_state_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_integer(self.m_objectName, 32)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_objectName = bitstream.read_integer("objectName", 32)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_nav_point_set_ignore_line_of_sight_parameters {
    pub m_object: c_object_reference,
    pub m_flag: bool,
}

impl s_action_nav_point_set_ignore_line_of_sight_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_bool(self.m_flag)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_flag = bitstream.read_bool("flag")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_incident_get_cause_team_parameters {
    pub m_team: c_team_reference,
}

impl s_action_incident_get_cause_team_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_team.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_incident_get_effect_team_parameters {
    pub m_team: c_team_reference,
}

impl s_action_incident_get_effect_team_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_team.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team.decode(bitstream)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_incident_get_cause_object_parameters {
    pub m_object: c_object_reference,
}

impl s_action_incident_get_cause_object_parameters {
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
pub struct s_action_incident_get_effect_object_parameters {
    pub m_object: c_object_reference,
}

impl s_action_incident_get_effect_object_parameters {
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
pub struct s_action_incident_get_special_death_type_parameters {
    pub m_variable: c_custom_variable_reference,
}

impl s_action_incident_get_special_death_type_parameters {
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
pub struct s_action_incident_get_custom_data_parameters {
    pub m_variable: c_custom_variable_reference,
}

impl s_action_incident_get_custom_data_parameters {
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
pub struct s_action_object_attach_to_marker_parameters {
    pub m_object_1: c_object_reference,
    pub m_index_1: i32,
    pub m_object_2: c_object_reference,
    pub m_index_2: i32,
}

impl s_action_object_attach_to_marker_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        bitstream.write_index::<255>(self.m_index_1, 8)?;
        self.m_object_2.encode(bitstream)?;
        bitstream.write_index::<255>(self.m_index_2, 8)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_index_1 = bitstream.read_index::<255>("index", 8)?;
        self.m_object_2.decode(bitstream)?;
        self.m_index_2 = bitstream.read_index::<255>("index", 8)?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_is_player_being_fancy_assassinated_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_is_player_being_fancy_assassinated_parameters {
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
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_give_powerup_parameters {
    pub m_player: c_player_reference,
    pub m_flavor: e_multiplayer_powerup_flavor,
}

impl Default for s_action_give_powerup_parameters {
    fn default() -> Self {
        Self {
            m_player: Default::default(),
            m_flavor: e_multiplayer_powerup_flavor::red,
        }
    }
}

impl s_action_give_powerup_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_enum(self.m_flavor)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_flavor = bitstream.read_enum("flavor")?;
        Ok(())
    }
}
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_give_ordnance_points_parameters {
    pub m_player: c_player_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_give_ordnance_points_parameters {
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
pub struct s_action_have_line_of_sight_parameters {
    pub m_object_1: c_object_reference,
    pub m_object_2: c_object_reference,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_have_line_of_sight_parameters {
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

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
pub enum e_action_type {
    #[default]
    none = 0,
    set_score = 1,
    create_object = 2,
    delete_object = 3,
    navpoint_set_visible = 4,
    navpoint_set_icon = 5,
    nav_point_set_secondary_icon = 6,
    navpoint_set_priority = 7,
    navpoint_set_timer = 8,
    navpoint_set_visible_range = 9,
    nav_point_set_is_territory = 10,
    nav_point_set_is_spawning_territory = 11,
    nav_point_set_territory_level = 12,
    nav_point_set_max_territory_level = 13,
    nav_point_set_territory_sort_order = 14,
    nav_point_set_territory_timer = 15,
    nav_point_set_type = 16,
    nav_point_set_action_team = 17,
    load_game_hud = 18,
    set = 19,
    set_boundary = 20,
    apply_player_traits = 21,
    set_pickup_filter = 22,
    set_respawn_filter = 23,
    set_fireteam_respawn_filter = 24,
    set_progress_bar = 25,
    set_progress_bar_user_data = 26,
    hud_post_message = 27,
    timer_set_rate = 28,
    print_variable = 29,
    get_player_holding_object = 30,
    for_each = 31,
    begin = 32,
    end_round = 33,
    end_game = 34,
    boundary_set_visible = 35,
    object_destroy = 36,
    object_set_invincibility = 37,
    random = 38,
    break_into_debugger = 39,
    object_get_orientation = 40,
    object_get_velocity = 41,
    player_death_get_killing_player = 42,
    player_death_get_damage_type = 43,
    player_death_get_special_type = 44,
    debugging_enable_tracing = 45,
    disallow_match_join_in_progress = 46,
    object_attach = 47,
    object_detach = 48,
    player_get_place = 49,
    multiple_teams_tied_for_first = 50,
    multiple_players_tied_for_first = 51,
    player_get_team_place = 52,
    team_get_place = 53,
    team_get_index = 54,
    player_get_killing_spree_count = 55,
    player_adjust_money = 56,
    player_enable_purchases = 57,
    player_get_vehicle = 58,
    player_set_vehicle = 59,
    player_get_ultimate_parent_not_self = 60,
    player_set_unit = 61,
    player_report_health_as_shields = 62,
    object_get_immediate_parent_player = 63,
    timer_reset = 64,
    weapon_set_pickup_priority = 65,
    object_bounce = 66,
    hud_widget_set_text = 67,
    hud_widget_set_value = 68,
    hud_widget_set_meter = 69,
    hud_widget_set_icon = 70,
    hud_widget_set_visibility = 71,
    play_sound = 72,
    play_sound_on_object = 73,
    object_set_scale = 74,
    navpoint_set_text = 75,
    navpoint_set_secondary_text = 76,
    set_mode_objective_text_line = 77,
    set_current_round_string = 78,
    set_previous_round_string = 79,
    set_team_win_loss_string = 80,
    object_get_shield = 81,
    object_get_health = 82,
    object_get_health_absolute = 83,
    player_set_objective = 84,
    player_set_objective_allegiance = 85,
    player_set_objective_allegiance_icon = 86,
    team_set_coop_spawning = 87,
    team_set_primary_respawn_object = 88,
    player_set_primary_respawn_object = 89,
    player_get_fireteam_index = 90,
    player_set_fireteam_index = 91,
    object_adjust_shield = 92,
    object_adjust_health = 93,
    object_get_distance = 94,
    object_adjust_maximum_shield = 95,
    object_adjust_maximum_health = 96,
    player_set_requisition_palette = 97,
    device_set_power = 98,
    device_get_power = 99,
    device_set_position = 100,
    device_get_position = 101,
    adjust_grenades = 102,
    submit_incident = 103,
    submit_incident_with_custom_value = 104,
    set_loadout_palette = 105,
    device_set_position_track = 106,
    device_animate_position = 107,
    device_set_position_immediate = 108,
    saved_film_insert_marker = 109,
    respawn_zone_enable = 110,
    player_get_weapon = 111,
    player_get_equipment = 112,
    object_set_never_garbage = 113,
    player_get_target_object = 114,
    create_tunnel = 115,
    debug_force_player_view_count = 116,
    player_pick_up_weapon = 117,
    player_set_coop_spawning = 118,
    object_set_orientation = 119,
    object_face_object = 120,
    biped_give_weapon = 121,
    biped_drop_weapon = 122,
    set_scenario_interpolator_state = 123,
    get_random_object = 124,
    game_grief_record_custom_penalty = 125,
    boundary_set_player_color = 126,
    hs_function_call = 127,
    get_button_time = 128,
    team_set_vehicle_spawning = 129,
    player_set_vehicle_spawning = 130,
    set_player_respawn_vehicle = 131,
    set_team_respawn_vehicle = 132,
    hide_object = 133,
    set_auto_turret = 134,
    set_auto_turret_range = 135,
    get_vehicle_entering_player = 136,
    set_candy_spawner_active = 137,
    player_force_respawn = 138,
    player_enable_spawning = 139,
    enable_territory_spawning = 140,
    enable_territory_spawn_selection = 141,
    set_territory_friendly_and_selected = 142,
    set_territory_last_stand_imminent_te = 143,
    player_is_detectable = 144,
    device_get_player_user = 145,
    device_get_interacting_player_user = 146,
    device_get_hold_time = 147,
    device_set_hold_time = 148,
    get_teleporter_channel = 149,
    set_teleporter_channel = 150,
    get_total_spawn_time = 151,
    get_digit = 152,
    data_mine_begin = 153,
    data_mine_add_category = 154,
    data_mine_add_real = 155,
    data_mine_add_int = 156,
    data_mine_add_timer_ticks_remaining = 157,
    data_mine_add_string = 158,
    data_mine_add_player = 159,
    data_mine_add_team = 160,
    data_mine_add_player_position = 161,
    data_mine_add_object_position = 162,
    data_mine_commit = 163,
    data_mine_clear = 164,
    object_set_position = 165,
    find_drop_position = 166,
    object_copy_boundary = 167,
    object_query_kill_boundaries = 168,
    set_sentry_active = 169,
    set_sentry_barrel_active = 170,
    is_spawner_blocked = 171,
    is_spawner_ready = 172,
    set_winning_player_for_final_kill_cam = 173,
    award_medal = 174,
    set_momentum = 175,
    set_momentum_tick_rate = 176,
    device_control_set_exclusive_user = 177,
    device_control_set_action_mode = 178,
    device_dispenser_set_enabled = 179,
    device_touch = 180,
    device_set_require_line_of_sight = 181,
    set_megalo_object_function = 182,
    set_megalo_timer_object_function = 183,
    random_ordnance_set_enabled = 184,
    random_ordnance_get_enabled = 185,
    random_ordnance_set_count = 186,
    random_ordnance_set_delay = 187,
    random_ordnance_do_drop = 188,
    ordnance_set_clear = 189,
    blink_navpoint = 190,
    pulse_navpoint = 191,
    track_carried_object_state = 192,
    set_medal_scoring = 193,
    nav_point_set_ignore_line_of_sight = 194,
    incident_get_cause_player = 195,
    incident_get_effect_player = 196,
    incident_get_cause_team = 197,
    incident_get_effect_team = 198,
    incident_get_cause_object = 199,
    incident_get_effect_object = 200,
    incident_get_special_death_type = 201,
    incident_get_custom_data = 202,
    object_attach_to_marker = 203,
    set_player_min_death_seconds = 204,
    is_player_being_fancy_assassinated = 205,
    give_powerup = 206,
    give_ordnance_points = 207,
    have_line_of_sight = 208,
    set_medal_override = 209,
    clear_medal_override = 210,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_action {
    pub m_type: e_action_type, // 8 bits
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
    pub m_disallow_match_join_in_progress_parameters: Option<s_action_disallow_match_join_in_progress_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_multiple_teams_tied_for_first_parameters: Option<s_action_variable_only_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_multiple_players_tied_for_first_parameters: Option<s_action_variable_only_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team_set_vehicle_spawning_parameters: Option<s_action_team_set_vehicle_spawning_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_winning_player_for_final_kill_cam_parameters: Option<s_action_set_winning_player_for_final_kill_cam_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_medal_scoring_parameters: Option<s_action_set_medal_scoring_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_incident_get_cause_player_parameters: Option<s_action_incident_get_player_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_incident_get_effect_player_parameters: Option<s_action_incident_get_player_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_medal_override_parameters: Option<s_action_set_medal_override_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_nav_point_set_type_parameters: Option<s_action_nav_point_set_type_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_award_medal_parameters: Option<s_action_award_medal_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_momentum_parameters: Option<s_action_set_momentum_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_get_button_time_parameters: Option<s_action_get_button_time_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_vehicle_spawning_parameters: Option<s_action_player_set_vehicle_spawning_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_data_mine_add_object_position_parameters: Option<s_action_data_mine_add_object_position_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_random_ordnance_set_enabled_parameters: Option<s_action_bool_enabled_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_clear_medal_override_parameters: Option<s_action_clear_medal_override_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_nav_point_set_is_territory_parameters: Option<s_action_nav_point_set_is_territory_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_nav_point_set_is_spawning_territory_parameters: Option<s_action_nav_point_set_is_spawning_territory_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_nav_point_set_territory_level_parameters: Option<s_action_nav_point_set_territory_level_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_nav_point_set_max_territory_level_parameters: Option<s_action_nav_point_set_max_territory_level_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_nav_point_set_territory_sort_order_parameters: Option<s_action_nav_point_set_territory_sort_order_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_nav_point_set_territory_timer_parameters: Option<s_action_nav_point_set_territory_timer_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_nav_point_set_action_team_parameters: Option<s_action_nav_point_set_action_team_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_load_game_hud_parameters: Option<s_action_load_game_hud_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_progress_bar_user_data_parameters: Option<s_action_set_progress_bar_user_data_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_get_team_place_parameters: Option<s_action_player_get_team_place_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team_get_index_parameters: Option<s_action_team_get_index_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_get_ultimate_parent_not_self_parameters: Option<s_action_player_get_ultimate_parent_not_self_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_report_health_as_shields_parameters: Option<s_action_player_report_health_as_shields_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_get_immediate_parent_player_parameters: Option<s_action_object_get_immediate_parent_player_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_play_sound_on_object_parameters: Option<s_action_play_sound_on_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_navpoint_set_secondary_text_parameters: Option<s_action_navpoint_set_secondary_text_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_mode_objective_text_line_parameters: Option<s_action_set_mode_objective_text_line_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_current_round_string_parameters: Option<s_action_set_current_round_string_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_previous_round_string_parameters: Option<s_action_set_previous_round_string_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_team_win_loss_string_parameters: Option<s_action_set_team_win_loss_string_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_get_health_absolute_parameters: Option<s_action_object_get_health_absolute_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hs_function_call_parameters: Option<s_action_hs_function_call_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_player_respawn_vehicle_parameters: Option<s_action_set_player_respawn_vehicle_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_team_respawn_vehicle_parameters: Option<s_action_set_team_respawn_vehicle_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hide_object_parameters: Option<s_action_hide_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_auto_turret_parameters: Option<s_action_set_auto_turret_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_auto_turret_range_parameters: Option<s_action_set_auto_turret_range_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_get_vehicle_entering_player_parameters: Option<s_action_get_vehicle_entering_player_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_candy_spawner_active_parameters: Option<s_action_set_candy_spawner_active_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_force_respawn_parameters: Option<s_action_player_force_respawn_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_enable_spawning_parameters: Option<s_action_player_enable_spawning_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_enable_territory_spawning_parameters: Option<s_action_enable_territory_spawning_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_enable_territory_spawn_selection_parameters: Option<s_action_enable_territory_spawn_selection_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_territory_friendly_and_selected_parameters: Option<s_action_set_territory_friendly_and_selected_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_territory_last_stand_imminent_te_parameters: Option<s_action_set_territory_last_stand_imminent_te_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_is_detectable_parameters: Option<s_action_player_is_detectable_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_get_player_user_parameters: Option<s_action_device_get_player_user_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_get_interacting_player_user_parameters: Option<s_action_device_get_interacting_player_user_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_get_hold_time_parameters: Option<s_action_device_get_hold_time_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_set_hold_time_parameters: Option<s_action_device_set_hold_time_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_teleporter_channel_parameters: Option<s_action_set_teleporter_channel_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_get_teleporter_channel_parameters: Option<s_action_get_teleporter_channel_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_get_digit_parameters: Option<s_action_get_digit_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_get_total_spawn_time_parameters: Option<s_action_get_total_spawn_time_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_data_mine_begin_parameters: Option<s_action_data_mine_begin_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_data_mine_add_category_parameters: Option<s_action_data_mine_add_category_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_data_mine_add_real_parameters: Option<s_action_data_mine_add_real_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_data_mine_add_int_parameters: Option<s_action_data_mine_add_int_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_data_mine_add_timer_ticks_remaining_parameters: Option<s_action_data_mine_add_timer_ticks_remaining_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_data_mine_add_string_parameters: Option<s_action_data_mine_add_string_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_data_mine_add_player_parameters: Option<s_action_data_mine_add_player_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_data_mine_add_team_parameters: Option<s_action_data_mine_add_team_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_data_mine_add_player_position_parameters: Option<s_action_data_mine_add_player_position_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_set_position_parameters: Option<s_action_object_set_position_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_find_drop_position_parameters: Option<s_action_find_drop_position_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_copy_boundary_parameters: Option<s_action_object_copy_boundary_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_query_kill_boundaries_parameters: Option<s_action_object_query_kill_boundaries_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_sentry_active_parameters: Option<s_action_set_sentry_active_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_sentry_barrel_active_parameters: Option<s_action_set_sentry_barrel_active_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_is_spawner_blocked_parameters: Option<s_action_is_spawner_blocked_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_is_spawner_ready_parameters: Option<s_action_is_spawner_ready_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_momentum_tick_rate_parameters: Option<s_action_set_momentum_tick_rate_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_control_set_exclusive_user_parameters: Option<s_action_device_control_set_exclusive_user_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_control_set_action_mode_parameters: Option<s_action_device_control_set_action_mode_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_dispenser_set_enabled_parameters: Option<s_action_device_dispenser_set_enabled_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_touch_parameters: Option<s_action_device_touch_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_device_set_require_line_of_sight_parameters: Option<s_action_device_set_require_line_of_sight_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_megalo_object_function_parameters: Option<s_action_set_megalo_object_function_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_megalo_timer_object_function_parameters: Option<s_action_set_megalo_timer_object_function_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_random_ordnance_get_enabled_parameters: Option<s_action_random_ordnance_get_enabled_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_random_ordnance_set_count_parameters: Option<s_action_random_ordnance_set_count_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_random_ordnance_set_delay_parameters: Option<s_action_random_ordnance_set_delay_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_random_ordnance_do_drop_parameters: Option<s_action_random_ordnance_do_drop_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_ordnance_set_clear_parameters: Option<s_action_ordnance_set_clear_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_blink_navpoint_parameters: Option<s_action_blink_navpoint_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_pulse_navpoint_parameters: Option<s_action_pulse_navpoint_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_track_carried_object_state_parameters: Option<s_action_track_carried_object_state_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_nav_point_set_ignore_line_of_sight_parameters: Option<s_action_nav_point_set_ignore_line_of_sight_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_incident_get_cause_team_parameters: Option<s_action_incident_get_cause_team_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_incident_get_effect_team_parameters: Option<s_action_incident_get_effect_team_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_incident_get_cause_object_parameters: Option<s_action_incident_get_cause_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_incident_get_effect_object_parameters: Option<s_action_incident_get_effect_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_incident_get_special_death_type_parameters: Option<s_action_incident_get_special_death_type_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_incident_get_custom_data_parameters: Option<s_action_incident_get_custom_data_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_attach_to_marker_parameters: Option<s_action_object_attach_to_marker_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_is_player_being_fancy_assassinated_parameters: Option<s_action_is_player_being_fancy_assassinated_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_give_powerup_parameters: Option<s_action_give_powerup_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_give_ordnance_points_parameters: Option<s_action_give_ordnance_points_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_have_line_of_sight_parameters: Option<s_action_have_line_of_sight_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_player_min_death_seconds_parameters: Option<s_action_set_player_min_death_seconds_parameters>,
}


impl c_action {
    pub fn executable_pregame(&self) -> bool {
        matches!(
            self.m_type,
            e_action_type::none | e_action_type::set | e_action_type::for_each
        )
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_type.clone(), 8)?;

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
            e_action_type::disallow_match_join_in_progress => self.m_disallow_match_join_in_progress_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_disallow_match_join_in_progress_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::multiple_teams_tied_for_first => self.m_multiple_teams_tied_for_first_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_multiple_teams_tied_for_first_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::multiple_players_tied_for_first => self.m_multiple_players_tied_for_first_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_multiple_players_tied_for_first_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::team_set_vehicle_spawning => self.m_team_set_vehicle_spawning_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_team_set_vehicle_spawning_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_winning_player_for_final_kill_cam => self.m_set_winning_player_for_final_kill_cam_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_winning_player_for_final_kill_cam_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_medal_scoring => self.m_set_medal_scoring_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_medal_scoring_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::incident_get_cause_player => self.m_incident_get_cause_player_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_incident_get_cause_player_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::incident_get_effect_player => self.m_incident_get_effect_player_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_incident_get_effect_player_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::nav_point_set_secondary_icon => self.m_navpoint_set_icon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_icon_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::nav_point_set_type => self.m_nav_point_set_type_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_nav_point_set_type_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::award_medal => self.m_award_medal_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_award_medal_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_momentum => self.m_set_momentum_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_momentum_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::get_button_time => self.m_get_button_time_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_get_button_time_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_vehicle_spawning => self.m_player_set_vehicle_spawning_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_vehicle_spawning_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::data_mine_add_object_position => self.m_data_mine_add_object_position_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_data_mine_add_object_position_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::random_ordnance_set_enabled => self.m_random_ordnance_set_enabled_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_random_ordnance_set_enabled_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::clear_medal_override => self.m_clear_medal_override_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_clear_medal_override_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_medal_override => self.m_set_medal_override_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_medal_override_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::end_game
            | e_action_type::none
            | e_action_type::data_mine_commit
            | e_action_type::data_mine_clear => {}
            e_action_type::nav_point_set_is_territory => self.m_nav_point_set_is_territory_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_nav_point_set_is_territory_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::nav_point_set_is_spawning_territory => self.m_nav_point_set_is_spawning_territory_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_nav_point_set_is_spawning_territory_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::nav_point_set_territory_level => self.m_nav_point_set_territory_level_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_nav_point_set_territory_level_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::nav_point_set_max_territory_level => self.m_nav_point_set_max_territory_level_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_nav_point_set_max_territory_level_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::nav_point_set_territory_sort_order => self.m_nav_point_set_territory_sort_order_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_nav_point_set_territory_sort_order_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::nav_point_set_territory_timer => self.m_nav_point_set_territory_timer_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_nav_point_set_territory_timer_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::nav_point_set_action_team => self.m_nav_point_set_action_team_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_nav_point_set_action_team_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::load_game_hud => self.m_load_game_hud_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_load_game_hud_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_progress_bar_user_data => self.m_set_progress_bar_user_data_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_progress_bar_user_data_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_get_team_place => self.m_player_get_team_place_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_get_team_place_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::team_get_index => self.m_team_get_index_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_team_get_index_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_get_ultimate_parent_not_self => self.m_player_get_ultimate_parent_not_self_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_get_ultimate_parent_not_self_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_report_health_as_shields => self.m_player_report_health_as_shields_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_report_health_as_shields_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_get_immediate_parent_player => self.m_object_get_immediate_parent_player_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_get_immediate_parent_player_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::play_sound_on_object => self.m_play_sound_on_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_play_sound_on_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::navpoint_set_secondary_text => self.m_navpoint_set_secondary_text_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_secondary_text_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_mode_objective_text_line => self.m_set_mode_objective_text_line_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_mode_objective_text_line_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_current_round_string => self.m_set_current_round_string_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_current_round_string_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_previous_round_string => self.m_set_previous_round_string_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_previous_round_string_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_team_win_loss_string => self.m_set_team_win_loss_string_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_team_win_loss_string_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_get_health_absolute => self.m_object_get_health_absolute_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_get_health_absolute_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::hs_function_call => self.m_hs_function_call_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hs_function_call_parameters does not exist."))?
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
            e_action_type::set_auto_turret => self.m_set_auto_turret_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_auto_turret_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_auto_turret_range => self.m_set_auto_turret_range_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_auto_turret_range_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::get_vehicle_entering_player => self.m_get_vehicle_entering_player_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_get_vehicle_entering_player_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_candy_spawner_active => self.m_set_candy_spawner_active_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_candy_spawner_active_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_force_respawn => self.m_player_force_respawn_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_force_respawn_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_enable_spawning => self.m_player_enable_spawning_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_enable_spawning_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::enable_territory_spawning => self.m_enable_territory_spawning_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_enable_territory_spawning_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::enable_territory_spawn_selection => self.m_enable_territory_spawn_selection_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_enable_territory_spawn_selection_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_territory_friendly_and_selected => self.m_set_territory_friendly_and_selected_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_territory_friendly_and_selected_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_territory_last_stand_imminent_te => self.m_set_territory_last_stand_imminent_te_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_territory_last_stand_imminent_te_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_is_detectable => self.m_player_is_detectable_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_is_detectable_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_get_player_user => self.m_device_get_player_user_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_get_player_user_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_get_interacting_player_user => self.m_device_get_interacting_player_user_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_get_interacting_player_user_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_get_hold_time => self.m_device_get_hold_time_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_get_hold_time_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_set_hold_time => self.m_device_set_hold_time_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_set_hold_time_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_teleporter_channel => self.m_set_teleporter_channel_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_teleporter_channel_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::get_teleporter_channel => self.m_get_teleporter_channel_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_get_teleporter_channel_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::get_digit => self.m_get_digit_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_get_digit_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::get_total_spawn_time => self.m_get_total_spawn_time_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_get_total_spawn_time_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::data_mine_begin => self.m_data_mine_begin_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_data_mine_begin_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::data_mine_add_category => self.m_data_mine_add_category_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_data_mine_add_category_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::data_mine_add_real => self.m_data_mine_add_real_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_data_mine_add_real_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::data_mine_add_int => self.m_data_mine_add_int_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_data_mine_add_int_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::data_mine_add_timer_ticks_remaining => self.m_data_mine_add_timer_ticks_remaining_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_data_mine_add_timer_ticks_remaining_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::data_mine_add_string => self.m_data_mine_add_string_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_data_mine_add_string_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::data_mine_add_player => self.m_data_mine_add_player_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_data_mine_add_player_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::data_mine_add_team => self.m_data_mine_add_team_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_data_mine_add_team_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::data_mine_add_player_position => self.m_data_mine_add_player_position_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_data_mine_add_player_position_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_set_position => self.m_object_set_position_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_set_position_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::find_drop_position => self.m_find_drop_position_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_find_drop_position_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_copy_boundary => self.m_object_copy_boundary_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_copy_boundary_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_query_kill_boundaries => self.m_object_query_kill_boundaries_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_query_kill_boundaries_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_sentry_active => self.m_set_sentry_active_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_sentry_active_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_sentry_barrel_active => self.m_set_sentry_barrel_active_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_sentry_barrel_active_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::is_spawner_blocked => self.m_is_spawner_blocked_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_is_spawner_blocked_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::is_spawner_ready => self.m_is_spawner_ready_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_is_spawner_ready_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_momentum_tick_rate => self.m_set_momentum_tick_rate_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_momentum_tick_rate_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_control_set_exclusive_user => self.m_device_control_set_exclusive_user_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_control_set_exclusive_user_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_control_set_action_mode => self.m_device_control_set_action_mode_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_control_set_action_mode_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_dispenser_set_enabled => self.m_device_dispenser_set_enabled_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_dispenser_set_enabled_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_touch => self.m_device_touch_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_touch_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_set_require_line_of_sight => self.m_device_set_require_line_of_sight_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_set_require_line_of_sight_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_megalo_object_function => self.m_set_megalo_object_function_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_megalo_object_function_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_megalo_timer_object_function => self.m_set_megalo_timer_object_function_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_megalo_timer_object_function_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::random_ordnance_get_enabled => self.m_random_ordnance_get_enabled_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_random_ordnance_get_enabled_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::random_ordnance_set_count => self.m_random_ordnance_set_count_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_random_ordnance_set_count_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::random_ordnance_set_delay => self.m_random_ordnance_set_delay_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_random_ordnance_set_delay_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::random_ordnance_do_drop => self.m_random_ordnance_do_drop_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_random_ordnance_do_drop_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::ordnance_set_clear => self.m_ordnance_set_clear_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_ordnance_set_clear_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::blink_navpoint => self.m_blink_navpoint_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_blink_navpoint_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::pulse_navpoint => self.m_pulse_navpoint_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_pulse_navpoint_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::track_carried_object_state => self.m_track_carried_object_state_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_track_carried_object_state_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::nav_point_set_ignore_line_of_sight => self.m_nav_point_set_ignore_line_of_sight_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_nav_point_set_ignore_line_of_sight_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::incident_get_cause_team => self.m_incident_get_cause_team_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_incident_get_cause_team_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::incident_get_effect_team => self.m_incident_get_effect_team_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_incident_get_effect_team_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::incident_get_cause_object => self.m_incident_get_cause_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_incident_get_cause_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::incident_get_effect_object => self.m_incident_get_effect_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_incident_get_effect_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::incident_get_special_death_type => self.m_incident_get_special_death_type_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_incident_get_special_death_type_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::incident_get_custom_data => self.m_incident_get_custom_data_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_incident_get_custom_data_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_attach_to_marker => self.m_object_attach_to_marker_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_attach_to_marker_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::is_player_being_fancy_assassinated => self.m_is_player_being_fancy_assassinated_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_is_player_being_fancy_assassinated_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::give_powerup => self.m_give_powerup_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_give_powerup_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::give_ordnance_points => self.m_give_ordnance_points_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_give_ordnance_points_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::have_line_of_sight => self.m_have_line_of_sight_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_have_line_of_sight_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_player_min_death_seconds => self.m_set_player_min_death_seconds_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_player_min_death_seconds_parameters does not exist."))?
                .encode(bitstream)?,
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let action_type = bitstream.read_integer("action-type", 8)?;
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
            e_action_type::disallow_match_join_in_progress => {
                let mut params = s_action_disallow_match_join_in_progress_parameters::default();
                params.decode(bitstream)?;
                self.m_disallow_match_join_in_progress_parameters = Some(params);
            }
            e_action_type::multiple_teams_tied_for_first => {
                let mut params = s_action_variable_only_parameters::default();
                params.decode(bitstream)?;
                self.m_multiple_teams_tied_for_first_parameters = Some(params);
            }
            e_action_type::multiple_players_tied_for_first => {
                let mut params = s_action_variable_only_parameters::default();
                params.decode(bitstream)?;
                self.m_multiple_players_tied_for_first_parameters = Some(params);
            }
            e_action_type::team_set_vehicle_spawning => {
                let mut params = s_action_team_set_vehicle_spawning_parameters::default();
                params.decode(bitstream)?;
                self.m_team_set_vehicle_spawning_parameters = Some(params);
            }
            e_action_type::set_winning_player_for_final_kill_cam => {
                let mut params = s_action_set_winning_player_for_final_kill_cam_parameters::default();
                params.decode(bitstream)?;
                self.m_set_winning_player_for_final_kill_cam_parameters = Some(params);
            }
            e_action_type::set_medal_scoring => {
                let mut params = s_action_set_medal_scoring_parameters::default();
                params.decode(bitstream)?;
                self.m_set_medal_scoring_parameters = Some(params);
            }
            e_action_type::incident_get_cause_player => {
                let mut params = s_action_incident_get_player_parameters::default();
                params.decode(bitstream)?;
                self.m_incident_get_cause_player_parameters = Some(params);
            }
            e_action_type::incident_get_effect_player => {
                let mut params = s_action_incident_get_player_parameters::default();
                params.decode(bitstream)?;
                self.m_incident_get_effect_player_parameters = Some(params);
            }
            e_action_type::nav_point_set_secondary_icon => {
                let mut navpoint_set_icon_parameters = s_action_navpoint_set_icon_parameters::default();
                navpoint_set_icon_parameters.decode(bitstream)?;
                self.m_navpoint_set_icon_parameters = Some(navpoint_set_icon_parameters);
            }
            e_action_type::nav_point_set_type => {
                let mut params = s_action_nav_point_set_type_parameters::default();
                params.decode(bitstream)?;
                self.m_nav_point_set_type_parameters = Some(params);
            }
            e_action_type::award_medal => {
                let mut params = s_action_award_medal_parameters::default();
                params.decode(bitstream)?;
                self.m_award_medal_parameters = Some(params);
            }
            e_action_type::set_momentum => {
                let mut params = s_action_set_momentum_parameters::default();
                params.decode(bitstream)?;
                self.m_set_momentum_parameters = Some(params);
            }
            e_action_type::get_button_time => {
                let mut params = s_action_get_button_time_parameters::default();
                params.decode(bitstream)?;
                self.m_get_button_time_parameters = Some(params);
            }
            e_action_type::player_set_vehicle_spawning => {
                let mut params = s_action_player_set_vehicle_spawning_parameters::default();
                params.decode(bitstream)?;
                self.m_player_set_vehicle_spawning_parameters = Some(params);
            }
            e_action_type::data_mine_add_object_position => {
                let mut params = s_action_data_mine_add_object_position_parameters::default();
                params.decode(bitstream)?;
                self.m_data_mine_add_object_position_parameters = Some(params);
            }
            e_action_type::random_ordnance_set_enabled => {
                let mut params = s_action_bool_enabled_parameters::default();
                params.decode(bitstream)?;
                self.m_random_ordnance_set_enabled_parameters = Some(params);
            }
            e_action_type::clear_medal_override => {
                let mut params = s_action_clear_medal_override_parameters::default();
                params.decode(bitstream)?;
                self.m_clear_medal_override_parameters = Some(params);
            }
            e_action_type::set_medal_override => {
                let mut params = s_action_set_medal_override_parameters::default();
                params.decode(bitstream)?;
                self.m_set_medal_override_parameters = Some(params);
            }
            e_action_type::nav_point_set_is_territory => {
                let mut params = s_action_nav_point_set_is_territory_parameters::default();
                params.decode(bitstream)?;
                self.m_nav_point_set_is_territory_parameters = Some(params);
            }
            e_action_type::nav_point_set_is_spawning_territory => {
                let mut params = s_action_nav_point_set_is_spawning_territory_parameters::default();
                params.decode(bitstream)?;
                self.m_nav_point_set_is_spawning_territory_parameters = Some(params);
            }
            e_action_type::nav_point_set_territory_level => {
                let mut params = s_action_nav_point_set_territory_level_parameters::default();
                params.decode(bitstream)?;
                self.m_nav_point_set_territory_level_parameters = Some(params);
            }
            e_action_type::nav_point_set_max_territory_level => {
                let mut params = s_action_nav_point_set_max_territory_level_parameters::default();
                params.decode(bitstream)?;
                self.m_nav_point_set_max_territory_level_parameters = Some(params);
            }
            e_action_type::nav_point_set_territory_sort_order => {
                let mut params = s_action_nav_point_set_territory_sort_order_parameters::default();
                params.decode(bitstream)?;
                self.m_nav_point_set_territory_sort_order_parameters = Some(params);
            }
            e_action_type::nav_point_set_territory_timer => {
                let mut params = s_action_nav_point_set_territory_timer_parameters::default();
                params.decode(bitstream)?;
                self.m_nav_point_set_territory_timer_parameters = Some(params);
            }
            e_action_type::nav_point_set_action_team => {
                let mut params = s_action_nav_point_set_action_team_parameters::default();
                params.decode(bitstream)?;
                self.m_nav_point_set_action_team_parameters = Some(params);
            }
            e_action_type::load_game_hud => {
                let mut params = s_action_load_game_hud_parameters::default();
                params.decode(bitstream)?;
                self.m_load_game_hud_parameters = Some(params);
            }
            e_action_type::set_progress_bar_user_data => {
                let mut params = s_action_set_progress_bar_user_data_parameters::default();
                params.decode(bitstream)?;
                self.m_set_progress_bar_user_data_parameters = Some(params);
            }
            e_action_type::player_get_team_place => {
                let mut params = s_action_player_get_team_place_parameters::default();
                params.decode(bitstream)?;
                self.m_player_get_team_place_parameters = Some(params);
            }
            e_action_type::team_get_index => {
                let mut params = s_action_team_get_index_parameters::default();
                params.decode(bitstream)?;
                self.m_team_get_index_parameters = Some(params);
            }
            e_action_type::player_get_ultimate_parent_not_self => {
                let mut params = s_action_player_get_ultimate_parent_not_self_parameters::default();
                params.decode(bitstream)?;
                self.m_player_get_ultimate_parent_not_self_parameters = Some(params);
            }
            e_action_type::player_report_health_as_shields => {
                let mut params = s_action_player_report_health_as_shields_parameters::default();
                params.decode(bitstream)?;
                self.m_player_report_health_as_shields_parameters = Some(params);
            }
            e_action_type::object_get_immediate_parent_player => {
                let mut params = s_action_object_get_immediate_parent_player_parameters::default();
                params.decode(bitstream)?;
                self.m_object_get_immediate_parent_player_parameters = Some(params);
            }
            e_action_type::play_sound_on_object => {
                let mut params = s_action_play_sound_on_object_parameters::default();
                params.decode(bitstream)?;
                self.m_play_sound_on_object_parameters = Some(params);
            }
            e_action_type::navpoint_set_secondary_text => {
                let mut params = s_action_navpoint_set_secondary_text_parameters::default();
                params.decode(bitstream)?;
                self.m_navpoint_set_secondary_text_parameters = Some(params);
            }
            e_action_type::set_mode_objective_text_line => {
                let mut params = s_action_set_mode_objective_text_line_parameters::default();
                params.decode(bitstream)?;
                self.m_set_mode_objective_text_line_parameters = Some(params);
            }
            e_action_type::set_current_round_string => {
                let mut params = s_action_set_current_round_string_parameters::default();
                params.decode(bitstream)?;
                self.m_set_current_round_string_parameters = Some(params);
            }
            e_action_type::set_previous_round_string => {
                let mut params = s_action_set_previous_round_string_parameters::default();
                params.decode(bitstream)?;
                self.m_set_previous_round_string_parameters = Some(params);
            }
            e_action_type::set_team_win_loss_string => {
                let mut params = s_action_set_team_win_loss_string_parameters::default();
                params.decode(bitstream)?;
                self.m_set_team_win_loss_string_parameters = Some(params);
            }
            e_action_type::object_get_health_absolute => {
                let mut params = s_action_object_get_health_absolute_parameters::default();
                params.decode(bitstream)?;
                self.m_object_get_health_absolute_parameters = Some(params);
            }
            e_action_type::hs_function_call => {
                let mut params = s_action_hs_function_call_parameters::default();
                params.decode(bitstream)?;
                self.m_hs_function_call_parameters = Some(params);
            }
            e_action_type::set_player_respawn_vehicle => {
                let mut params = s_action_set_player_respawn_vehicle_parameters::default();
                params.decode(bitstream)?;
                self.m_set_player_respawn_vehicle_parameters = Some(params);
            }
            e_action_type::set_team_respawn_vehicle => {
                let mut params = s_action_set_team_respawn_vehicle_parameters::default();
                params.decode(bitstream)?;
                self.m_set_team_respawn_vehicle_parameters = Some(params);
            }
            e_action_type::hide_object => {
                let mut params = s_action_hide_object_parameters::default();
                params.decode(bitstream)?;
                self.m_hide_object_parameters = Some(params);
            }
            e_action_type::set_auto_turret => {
                let mut params = s_action_set_auto_turret_parameters::default();
                params.decode(bitstream)?;
                self.m_set_auto_turret_parameters = Some(params);
            }
            e_action_type::set_auto_turret_range => {
                let mut params = s_action_set_auto_turret_range_parameters::default();
                params.decode(bitstream)?;
                self.m_set_auto_turret_range_parameters = Some(params);
            }
            e_action_type::get_vehicle_entering_player => {
                let mut params = s_action_get_vehicle_entering_player_parameters::default();
                params.decode(bitstream)?;
                self.m_get_vehicle_entering_player_parameters = Some(params);
            }
            e_action_type::set_candy_spawner_active => {
                let mut params = s_action_set_candy_spawner_active_parameters::default();
                params.decode(bitstream)?;
                self.m_set_candy_spawner_active_parameters = Some(params);
            }
            e_action_type::player_force_respawn => {
                let mut params = s_action_player_force_respawn_parameters::default();
                params.decode(bitstream)?;
                self.m_player_force_respawn_parameters = Some(params);
            }
            e_action_type::player_enable_spawning => {
                let mut params = s_action_player_enable_spawning_parameters::default();
                params.decode(bitstream)?;
                self.m_player_enable_spawning_parameters = Some(params);
            }
            e_action_type::enable_territory_spawning => {
                let mut params = s_action_enable_territory_spawning_parameters::default();
                params.decode(bitstream)?;
                self.m_enable_territory_spawning_parameters = Some(params);
            }
            e_action_type::enable_territory_spawn_selection => {
                let mut params = s_action_enable_territory_spawn_selection_parameters::default();
                params.decode(bitstream)?;
                self.m_enable_territory_spawn_selection_parameters = Some(params);
            }
            e_action_type::set_territory_friendly_and_selected => {
                let mut params = s_action_set_territory_friendly_and_selected_parameters::default();
                params.decode(bitstream)?;
                self.m_set_territory_friendly_and_selected_parameters = Some(params);
            }
            e_action_type::set_territory_last_stand_imminent_te => {
                let mut params = s_action_set_territory_last_stand_imminent_te_parameters::default();
                params.decode(bitstream)?;
                self.m_set_territory_last_stand_imminent_te_parameters = Some(params);
            }
            e_action_type::player_is_detectable => {
                let mut params = s_action_player_is_detectable_parameters::default();
                params.decode(bitstream)?;
                self.m_player_is_detectable_parameters = Some(params);
            }
            e_action_type::device_get_player_user => {
                let mut params = s_action_device_get_player_user_parameters::default();
                params.decode(bitstream)?;
                self.m_device_get_player_user_parameters = Some(params);
            }
            e_action_type::device_get_interacting_player_user => {
                let mut params = s_action_device_get_interacting_player_user_parameters::default();
                params.decode(bitstream)?;
                self.m_device_get_interacting_player_user_parameters = Some(params);
            }
            e_action_type::device_get_hold_time => {
                let mut params = s_action_device_get_hold_time_parameters::default();
                params.decode(bitstream)?;
                self.m_device_get_hold_time_parameters = Some(params);
            }
            e_action_type::device_set_hold_time => {
                let mut params = s_action_device_set_hold_time_parameters::default();
                params.decode(bitstream)?;
                self.m_device_set_hold_time_parameters = Some(params);
            }
            e_action_type::set_teleporter_channel => {
                let mut params = s_action_set_teleporter_channel_parameters::default();
                params.decode(bitstream)?;
                self.m_set_teleporter_channel_parameters = Some(params);
            }
            e_action_type::get_teleporter_channel => {
                let mut params = s_action_get_teleporter_channel_parameters::default();
                params.decode(bitstream)?;
                self.m_get_teleporter_channel_parameters = Some(params);
            }
            e_action_type::get_digit => {
                let mut params = s_action_get_digit_parameters::default();
                params.decode(bitstream)?;
                self.m_get_digit_parameters = Some(params);
            }
            e_action_type::get_total_spawn_time => {
                let mut params = s_action_get_total_spawn_time_parameters::default();
                params.decode(bitstream)?;
                self.m_get_total_spawn_time_parameters = Some(params);
            }
            e_action_type::data_mine_begin => {
                let mut params = s_action_data_mine_begin_parameters::default();
                params.decode(bitstream)?;
                self.m_data_mine_begin_parameters = Some(params);
            }
            e_action_type::data_mine_commit | e_action_type::data_mine_clear => {}
            e_action_type::data_mine_add_category => {
                let mut params = s_action_data_mine_add_category_parameters::default();
                params.decode(bitstream)?;
                self.m_data_mine_add_category_parameters = Some(params);
            }
            e_action_type::data_mine_add_real => {
                let mut params = s_action_data_mine_add_real_parameters::default();
                params.decode(bitstream)?;
                self.m_data_mine_add_real_parameters = Some(params);
            }
            e_action_type::data_mine_add_int => {
                let mut params = s_action_data_mine_add_int_parameters::default();
                params.decode(bitstream)?;
                self.m_data_mine_add_int_parameters = Some(params);
            }
            e_action_type::data_mine_add_timer_ticks_remaining => {
                let mut params = s_action_data_mine_add_timer_ticks_remaining_parameters::default();
                params.decode(bitstream)?;
                self.m_data_mine_add_timer_ticks_remaining_parameters = Some(params);
            }
            e_action_type::data_mine_add_string => {
                let mut params = s_action_data_mine_add_string_parameters::default();
                params.decode(bitstream)?;
                self.m_data_mine_add_string_parameters = Some(params);
            }
            e_action_type::data_mine_add_player => {
                let mut params = s_action_data_mine_add_player_parameters::default();
                params.decode(bitstream)?;
                self.m_data_mine_add_player_parameters = Some(params);
            }
            e_action_type::data_mine_add_team => {
                let mut params = s_action_data_mine_add_team_parameters::default();
                params.decode(bitstream)?;
                self.m_data_mine_add_team_parameters = Some(params);
            }
            e_action_type::data_mine_add_player_position => {
                let mut params = s_action_data_mine_add_player_position_parameters::default();
                params.decode(bitstream)?;
                self.m_data_mine_add_player_position_parameters = Some(params);
            }
            e_action_type::object_set_position => {
                let mut params = s_action_object_set_position_parameters::default();
                params.decode(bitstream)?;
                self.m_object_set_position_parameters = Some(params);
            }
            e_action_type::find_drop_position => {
                let mut params = s_action_find_drop_position_parameters::default();
                params.decode(bitstream)?;
                self.m_find_drop_position_parameters = Some(params);
            }
            e_action_type::object_copy_boundary => {
                let mut params = s_action_object_copy_boundary_parameters::default();
                params.decode(bitstream)?;
                self.m_object_copy_boundary_parameters = Some(params);
            }
            e_action_type::object_query_kill_boundaries => {
                let mut params = s_action_object_query_kill_boundaries_parameters::default();
                params.decode(bitstream)?;
                self.m_object_query_kill_boundaries_parameters = Some(params);
            }
            e_action_type::set_sentry_active => {
                let mut params = s_action_set_sentry_active_parameters::default();
                params.decode(bitstream)?;
                self.m_set_sentry_active_parameters = Some(params);
            }
            e_action_type::set_sentry_barrel_active => {
                let mut params = s_action_set_sentry_barrel_active_parameters::default();
                params.decode(bitstream)?;
                self.m_set_sentry_barrel_active_parameters = Some(params);
            }
            e_action_type::is_spawner_blocked => {
                let mut params = s_action_is_spawner_blocked_parameters::default();
                params.decode(bitstream)?;
                self.m_is_spawner_blocked_parameters = Some(params);
            }
            e_action_type::is_spawner_ready => {
                let mut params = s_action_is_spawner_ready_parameters::default();
                params.decode(bitstream)?;
                self.m_is_spawner_ready_parameters = Some(params);
            }
            e_action_type::set_momentum_tick_rate => {
                let mut params = s_action_set_momentum_tick_rate_parameters::default();
                params.decode(bitstream)?;
                self.m_set_momentum_tick_rate_parameters = Some(params);
            }
            e_action_type::device_control_set_exclusive_user => {
                let mut params = s_action_device_control_set_exclusive_user_parameters::default();
                params.decode(bitstream)?;
                self.m_device_control_set_exclusive_user_parameters = Some(params);
            }
            e_action_type::device_control_set_action_mode => {
                let mut params = s_action_device_control_set_action_mode_parameters::default();
                params.decode(bitstream)?;
                self.m_device_control_set_action_mode_parameters = Some(params);
            }
            e_action_type::device_dispenser_set_enabled => {
                let mut params = s_action_device_dispenser_set_enabled_parameters::default();
                params.decode(bitstream)?;
                self.m_device_dispenser_set_enabled_parameters = Some(params);
            }
            e_action_type::device_touch => {
                let mut params = s_action_device_touch_parameters::default();
                params.decode(bitstream)?;
                self.m_device_touch_parameters = Some(params);
            }
            e_action_type::device_set_require_line_of_sight => {
                let mut params = s_action_device_set_require_line_of_sight_parameters::default();
                params.decode(bitstream)?;
                self.m_device_set_require_line_of_sight_parameters = Some(params);
            }
            e_action_type::set_megalo_object_function => {
                let mut params = s_action_set_megalo_object_function_parameters::default();
                params.decode(bitstream)?;
                self.m_set_megalo_object_function_parameters = Some(params);
            }
            e_action_type::set_megalo_timer_object_function => {
                let mut params = s_action_set_megalo_timer_object_function_parameters::default();
                params.decode(bitstream)?;
                self.m_set_megalo_timer_object_function_parameters = Some(params);
            }
            e_action_type::random_ordnance_get_enabled => {
                let mut params = s_action_random_ordnance_get_enabled_parameters::default();
                params.decode(bitstream)?;
                self.m_random_ordnance_get_enabled_parameters = Some(params);
            }
            e_action_type::random_ordnance_set_count => {
                let mut params = s_action_random_ordnance_set_count_parameters::default();
                params.decode(bitstream)?;
                self.m_random_ordnance_set_count_parameters = Some(params);
            }
            e_action_type::random_ordnance_set_delay => {
                let mut params = s_action_random_ordnance_set_delay_parameters::default();
                params.decode(bitstream)?;
                self.m_random_ordnance_set_delay_parameters = Some(params);
            }
            e_action_type::random_ordnance_do_drop => {
                let mut params = s_action_random_ordnance_do_drop_parameters::default();
                params.decode(bitstream)?;
                self.m_random_ordnance_do_drop_parameters = Some(params);
            }
            e_action_type::ordnance_set_clear => {
                let mut params = s_action_ordnance_set_clear_parameters::default();
                params.decode(bitstream)?;
                self.m_ordnance_set_clear_parameters = Some(params);
            }
            e_action_type::blink_navpoint => {
                let mut params = s_action_blink_navpoint_parameters::default();
                params.decode(bitstream)?;
                self.m_blink_navpoint_parameters = Some(params);
            }
            e_action_type::pulse_navpoint => {
                let mut params = s_action_pulse_navpoint_parameters::default();
                params.decode(bitstream)?;
                self.m_pulse_navpoint_parameters = Some(params);
            }
            e_action_type::track_carried_object_state => {
                let mut params = s_action_track_carried_object_state_parameters::default();
                params.decode(bitstream)?;
                self.m_track_carried_object_state_parameters = Some(params);
            }
            e_action_type::nav_point_set_ignore_line_of_sight => {
                let mut params = s_action_nav_point_set_ignore_line_of_sight_parameters::default();
                params.decode(bitstream)?;
                self.m_nav_point_set_ignore_line_of_sight_parameters = Some(params);
            }
            e_action_type::incident_get_cause_team => {
                let mut params = s_action_incident_get_cause_team_parameters::default();
                params.decode(bitstream)?;
                self.m_incident_get_cause_team_parameters = Some(params);
            }
            e_action_type::incident_get_effect_team => {
                let mut params = s_action_incident_get_effect_team_parameters::default();
                params.decode(bitstream)?;
                self.m_incident_get_effect_team_parameters = Some(params);
            }
            e_action_type::incident_get_cause_object => {
                let mut params = s_action_incident_get_cause_object_parameters::default();
                params.decode(bitstream)?;
                self.m_incident_get_cause_object_parameters = Some(params);
            }
            e_action_type::incident_get_effect_object => {
                let mut params = s_action_incident_get_effect_object_parameters::default();
                params.decode(bitstream)?;
                self.m_incident_get_effect_object_parameters = Some(params);
            }
            e_action_type::incident_get_special_death_type => {
                let mut params = s_action_incident_get_special_death_type_parameters::default();
                params.decode(bitstream)?;
                self.m_incident_get_special_death_type_parameters = Some(params);
            }
            e_action_type::incident_get_custom_data => {
                let mut params = s_action_incident_get_custom_data_parameters::default();
                params.decode(bitstream)?;
                self.m_incident_get_custom_data_parameters = Some(params);
            }
            e_action_type::object_attach_to_marker => {
                let mut params = s_action_object_attach_to_marker_parameters::default();
                params.decode(bitstream)?;
                self.m_object_attach_to_marker_parameters = Some(params);
            }
            e_action_type::is_player_being_fancy_assassinated => {
                let mut params = s_action_is_player_being_fancy_assassinated_parameters::default();
                params.decode(bitstream)?;
                self.m_is_player_being_fancy_assassinated_parameters = Some(params);
            }
            e_action_type::give_powerup => {
                let mut params = s_action_give_powerup_parameters::default();
                params.decode(bitstream)?;
                self.m_give_powerup_parameters = Some(params);
            }
            e_action_type::give_ordnance_points => {
                let mut params = s_action_give_ordnance_points_parameters::default();
                params.decode(bitstream)?;
                self.m_give_ordnance_points_parameters = Some(params);
            }
            e_action_type::have_line_of_sight => {
                let mut params = s_action_have_line_of_sight_parameters::default();
                params.decode(bitstream)?;
                self.m_have_line_of_sight_parameters = Some(params);
            }
            e_action_type::set_player_min_death_seconds => {
                let mut params = s_action_set_player_min_death_seconds_parameters::default();
                params.decode(bitstream)?;
                self.m_set_player_min_death_seconds_parameters = Some(params);
            }
            _ => {}
        }

        Ok(())
    }
}