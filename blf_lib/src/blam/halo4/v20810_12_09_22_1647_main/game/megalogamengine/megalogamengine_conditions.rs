use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_variant_variable::s_variant_variable;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::OPTION_TO_RESULT;
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::halo4::v20810_12_09_22_1647_main::game::megalogamengine::megalogamengine_object_reference::c_object_reference;
use blf_lib::bitfield;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_numeric_comparison {
    #[default]
    less_than = 0, // <
    greater_than = 1, // >
    equal_to = 2, // ==
    less_than_or_equal_to = 3, // <=
    greater_than_or_equal_to = 4, // >=
    not_equal_to = 5, // !=
}

/// Bit indices for `c_flags<e_player_death_killer_type, …, 5>`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ToPrimitive, FromPrimitive, Serialize, Deserialize)]
pub enum e_player_death_killer_type {
    #[default]
    environment = 0,
    suicide = 1,
    enemy = 2,
    betrayal = 3,
    quit_game = 4,
}

/// Wire type for `m_killer_type` (`c_flags<e_player_death_killer_type>`, 5 bits).
bitfield! {
    #[derive(Serialize, Deserialize)]
    pub struct e_player_death_killer_type_flags: u8 {
        environment,
        suicide,
        enemy,
        betrayal,
        quit_game,
    }
}

/// Matches `e_disposition` (`c_enum`, 2 bits, range 0..3).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ToPrimitive, FromPrimitive, Serialize, Deserialize)]
pub enum e_disposition {
    #[default]
    neutral = 0,
    friendly = 1,
    enemy = 2,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_if_parameters {
    pub m_left: s_variant_variable,
    pub m_right: s_variant_variable,
    pub m_comparison: e_numeric_comparison, // 3 bits
}

impl s_condition_if_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_left.encode(bitstream)?;
        self.m_right.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_comparison, 3)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_left.decode(bitstream)?;
        self.m_right.decode(bitstream)?;
        self.m_comparison = bitstream.read_enum_raw("comparison", 3)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_object_in_area_parameters {
    pub m_object_reference_1: c_object_reference,
    pub m_object_reference_2: c_object_reference,
}

impl s_condition_object_in_area_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_reference_1.encode(bitstream)?;
        self.m_object_reference_2.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_reference_1.decode(bitstream)?;
        self.m_object_reference_2.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_player_died_parameters {
    pub m_player: c_player_reference,
    pub m_killer_type: e_player_death_killer_type_flags,
}

impl s_condition_player_died_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_integer(self.m_killer_type.to_raw(), 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_killer_type = e_player_death_killer_type_flags::from_raw(
            bitstream.read_integer("killer-type", 5)?,
        );

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_team_disposition_parameters {
    pub m_team_1: c_team_reference,
    pub m_team_2: c_team_reference,
    pub m_disposition: e_disposition,
}

impl s_condition_team_disposition_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_team_1.encode(bitstream)?;
        self.m_team_2.encode(bitstream)?;
        bitstream.write_enum_raw(self.m_disposition, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team_1.decode(bitstream)?;
        self.m_team_2.decode(bitstream)?;
        self.m_disposition = bitstream.read_enum_raw("disposition", 2)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_timer_expired_parameters {
    pub m_timer: c_custom_timer_reference,
}

impl s_condition_timer_expired_parameters {
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
pub struct s_condition_object_is_type_parameters {
    pub m_object: c_object_reference,
    pub m_object_type: c_object_type_reference,
}

impl s_condition_object_is_type_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_object_type.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_object_type.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_team_is_active_parameters {
    pub m_team: c_team_reference,
}

impl s_condition_team_is_active_parameters {
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
pub struct s_condition_object_out_of_bounds_parameters {
    pub m_object: c_object_reference,
}

impl s_condition_object_out_of_bounds_parameters {
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
pub struct s_condition_player_is_fire_team_leader_parameters {
    pub m_player: c_player_reference,
}

impl s_condition_player_is_fire_team_leader_parameters {
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
pub struct s_condition_player_assisted_with_kill_parameters {
    pub m_player_1: c_player_reference,
    pub m_player_2: c_player_reference,
}

impl s_condition_player_assisted_with_kill_parameters {
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
pub struct s_condition_object_matches_filter_parameters {
    pub m_object: c_object_reference,
    pub m_filter_index: i8,
}

impl s_condition_object_matches_filter_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_index::<16>(self.m_filter_index, 4)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_filter_index = bitstream.read_index::<16>("filter-index", 4)? as i8;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_player_is_active_parameters {
    pub m_player: c_player_reference,
}

impl s_condition_player_is_active_parameters {
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
pub struct s_condition_equipment_is_active_parameters {
    pub m_object: c_object_reference,
}

impl s_condition_equipment_is_active_parameters {
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
pub struct s_condition_player_is_spartan_parameters {
    pub m_player: c_player_reference,
}

impl s_condition_player_is_spartan_parameters {
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
pub struct s_condition_player_is_elite_parameters {
    pub m_player: c_player_reference,
}

impl s_condition_player_is_elite_parameters {
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
pub struct s_condition_player_is_editor_parameters {
    pub m_player: c_player_reference,
}

impl s_condition_player_is_editor_parameters {
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
pub struct s_condition_game_is_forge_parameters {}

impl s_condition_game_is_forge_parameters {
    pub fn encode(&self, _bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        Ok(())
    }

    pub fn decode(&mut self, _bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_was_recently_damaged_by_parameters {
    pub m_player: c_player_reference,
    pub m_attacker: c_player_reference,
}

impl s_condition_was_recently_damaged_by_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        self.m_attacker.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_attacker.decode(bitstream)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_vehicle_entered_parameters {
    pub m_object: c_object_reference,
}

impl s_condition_vehicle_entered_parameters {
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
pub struct s_condition_vehicle_exited_parameters {
    pub m_object: c_object_reference,
}

impl s_condition_vehicle_exited_parameters {
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
pub struct s_condition_vehicle_jacked_parameters {
    pub m_object: c_object_reference,
}

impl s_condition_vehicle_jacked_parameters {
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
pub struct s_condition_is_update_frame_parameters {
    pub m_left: s_variant_variable,
    pub m_right: s_variant_variable,
}

impl s_condition_is_update_frame_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_left.encode(bitstream)?;
        self.m_right.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_left.decode(bitstream)?;
        self.m_right.decode(bitstream)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_current_incident_parameters {
    pub m_incident_index: u32,
}

impl s_condition_current_incident_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_incident_index, 32)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_incident_index = bitstream.read_integer("incident-index", 32)?;
        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_object_physics_collided_parameters {
    pub m_object: c_object_reference,
}

impl s_condition_object_physics_collided_parameters {
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
pub struct s_condition_weapon_is_held_by_hologram_parameters {
    pub m_object: c_object_reference,
}

impl s_condition_weapon_is_held_by_hologram_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
pub enum e_condition_type {
    #[default]
    none = 0,
    r#if = 1,
    object_in_area = 2,
    player_died = 3,
    was_recently_damaged_by = 4,
    team_disposition = 5,
    timer_expired = 6,
    object_is_type = 7,
    team_is_active = 8,
    object_out_of_bounds = 9,
    player_is_fire_team_leader = 10,
    player_assisted_with_kill = 11,
    object_matches_filter = 12,
    player_is_active = 13,
    equipment_is_active = 14,
    player_is_spartan = 15,
    player_is_elite = 16,
    player_is_editor = 17,
    game_is_forge = 18,
    vehicle_entered = 19,
    vehicle_exited = 20,
    vehicle_jacked = 21,
    is_update_frame = 22,
    current_incident = 23,
    object_physics_collided = 24,
    weapon_is_held_by_hologram = 25,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_condition {
    pub m_type: e_condition_type, // 5 bits
    pub m_negated: bool,
    pub m_union_group: u16, // 10 bits
    pub m_execute_before_action: u16, // 11 bits

    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_if_parameters: Option<s_condition_if_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_in_area_parameters: Option<s_condition_object_in_area_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_died_parameters: Option<s_condition_player_died_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_was_recently_damaged_by_parameters: Option<s_condition_was_recently_damaged_by_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team_disposition_parameters: Option<s_condition_team_disposition_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_timer_expired_parameters: Option<s_condition_timer_expired_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_is_type_parameters: Option<s_condition_object_is_type_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team_is_active_parameters: Option<s_condition_team_is_active_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_out_of_bounds_parameters: Option<s_condition_object_out_of_bounds_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_is_fire_team_leader_parameters: Option<s_condition_player_is_fire_team_leader_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_assisted_with_kill_parameters: Option<s_condition_player_assisted_with_kill_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_matches_filter_parameters: Option<s_condition_object_matches_filter_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_is_active_parameters: Option<s_condition_player_is_active_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_equipment_is_active_parameters: Option<s_condition_equipment_is_active_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_is_spartan_parameters: Option<s_condition_player_is_spartan_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_is_elite_parameters: Option<s_condition_player_is_elite_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_is_editor_parameters: Option<s_condition_player_is_editor_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_game_is_forge_parameters: Option<s_condition_game_is_forge_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_vehicle_entered_parameters: Option<s_condition_vehicle_entered_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_vehicle_exited_parameters: Option<s_condition_vehicle_exited_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_vehicle_jacked_parameters: Option<s_condition_vehicle_jacked_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_is_update_frame_parameters: Option<s_condition_is_update_frame_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_current_incident_parameters: Option<s_condition_current_incident_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_physics_collided_parameters: Option<s_condition_object_physics_collided_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_weapon_is_held_by_hologram_parameters: Option<s_condition_weapon_is_held_by_hologram_parameters>,
}

impl c_condition {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_type.clone(), 5)?;
        if self.m_type == e_condition_type::none {
            return Ok(());
        }

        bitstream.write_bool(self.m_negated)?;
        bitstream.write_integer(self.m_union_group, 10)?;
        bitstream.write_integer(self.m_execute_before_action, 11)?;

        match self.m_type {
            e_condition_type::r#if => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_if_parameters,
                    format!("Can't encode condition type {:?} without m_if_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::object_in_area => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_object_in_area_parameters,
                    format!("Can't encode condition type {:?} without m_object_in_area_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::player_died => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_died_parameters,
                    format!("Can't encode condition type {:?} without m_player_died_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::was_recently_damaged_by => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_was_recently_damaged_by_parameters,
                    format!("Can't encode condition type {:?} without m_was_recently_damaged_by_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::team_disposition => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_team_disposition_parameters,
                    format!("Can't encode condition type {:?} without m_team_disposition_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::timer_expired => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_timer_expired_parameters,
                    format!("Can't encode condition type {:?} without m_timer_expired_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::object_is_type => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_object_is_type_parameters,
                    format!("Can't encode condition type {:?} without m_object_is_type_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::team_is_active => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_team_is_active_parameters,
                    format!("Can't encode condition type {:?} without m_team_is_active_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::object_out_of_bounds => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_object_out_of_bounds_parameters,
                    format!("Can't encode condition type {:?} without m_object_out_of_bounds_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::player_is_fire_team_leader => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_is_fire_team_leader_parameters,
                    format!("Can't encode condition type {:?} without m_player_is_fire_team_leader_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::player_assisted_with_kill => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_assisted_with_kill_parameters,
                    format!("Can't encode condition type {:?} without m_player_assisted_with_kill_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::object_matches_filter => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_object_matches_filter_parameters,
                    format!("Can't encode condition type {:?} without m_object_matches_filter_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::player_is_active => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_is_active_parameters,
                    format!("Can't encode condition type {:?} without m_player_is_active_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::equipment_is_active => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_equipment_is_active_parameters,
                    format!("Can't encode condition type {:?} without m_equipment_is_active_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::player_is_spartan => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_is_spartan_parameters,
                    format!("Can't encode condition type {:?} without m_player_is_spartan_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::player_is_elite => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_is_elite_parameters,
                    format!("Can't encode condition type {:?} without m_player_is_elite_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::player_is_editor => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_is_editor_parameters,
                    format!("Can't encode condition type {:?} without m_player_is_editor_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::game_is_forge => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_game_is_forge_parameters,
                    format!("Can't encode condition type {:?} without m_game_is_forge_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::vehicle_entered => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_vehicle_entered_parameters,
                    format!("Can't encode condition type {:?} without m_vehicle_entered_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::vehicle_exited => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_vehicle_exited_parameters,
                    format!("Can't encode condition type {:?} without m_vehicle_exited_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::vehicle_jacked => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_vehicle_jacked_parameters,
                    format!("Can't encode condition type {:?} without m_vehicle_jacked_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::is_update_frame => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_is_update_frame_parameters,
                    format!("Can't encode condition type {:?} without m_is_update_frame_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::current_incident => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_current_incident_parameters,
                    format!("Can't encode condition type {:?} without m_current_incident_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::object_physics_collided => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_object_physics_collided_parameters,
                    format!("Can't encode condition type {:?} without m_object_physics_collided_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::weapon_is_held_by_hologram => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_weapon_is_held_by_hologram_parameters,
                    format!("Can't encode condition type {:?} without m_weapon_is_held_by_hologram_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::none => unreachable!(),
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_enum_raw("condition-type", 5)?;

        if self.m_type == e_condition_type::none {
            return Ok(());
        }

        self.m_negated = bitstream.read_bool("negated")?;
        self.m_union_group = bitstream.read_integer("union-group", 10)?;
        self.m_execute_before_action = bitstream.read_integer("execute-before-action", 11)?;

        match self.m_type {
            e_condition_type::r#if => {
                let mut parameters = s_condition_if_parameters::default();
                parameters.decode(bitstream)?;
                self.m_if_parameters = Some(parameters);
            }
            e_condition_type::object_in_area => {
                let mut parameters = s_condition_object_in_area_parameters::default();
                parameters.decode(bitstream)?;
                self.m_object_in_area_parameters = Some(parameters);
            }
            e_condition_type::player_died => {
                let mut parameters = s_condition_player_died_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_died_parameters = Some(parameters);
            }
            e_condition_type::was_recently_damaged_by => {
                let mut parameters = s_condition_was_recently_damaged_by_parameters::default();
                parameters.decode(bitstream)?;
                self.m_was_recently_damaged_by_parameters = Some(parameters);
            }
            e_condition_type::team_disposition => {
                let mut parameters = s_condition_team_disposition_parameters::default();
                parameters.decode(bitstream)?;
                self.m_team_disposition_parameters = Some(parameters);
            }
            e_condition_type::timer_expired => {
                let mut parameters = s_condition_timer_expired_parameters::default();
                parameters.decode(bitstream)?;
                self.m_timer_expired_parameters = Some(parameters);
            }
            e_condition_type::object_is_type => {
                let mut parameters = s_condition_object_is_type_parameters::default();
                parameters.decode(bitstream)?;
                self.m_object_is_type_parameters = Some(parameters);
            }
            e_condition_type::team_is_active => {
                let mut parameters = s_condition_team_is_active_parameters::default();
                parameters.decode(bitstream)?;
                self.m_team_is_active_parameters = Some(parameters);
            }
            e_condition_type::object_out_of_bounds => {
                let mut parameters = s_condition_object_out_of_bounds_parameters::default();
                parameters.decode(bitstream)?;
                self.m_object_out_of_bounds_parameters = Some(parameters);
            }
            e_condition_type::player_is_fire_team_leader => {
                let mut parameters = s_condition_player_is_fire_team_leader_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_is_fire_team_leader_parameters = Some(parameters);
            }
            e_condition_type::player_assisted_with_kill => {
                let mut parameters = s_condition_player_assisted_with_kill_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_assisted_with_kill_parameters = Some(parameters);
            }
            e_condition_type::object_matches_filter => {
                let mut parameters = s_condition_object_matches_filter_parameters::default();
                parameters.decode(bitstream)?;
                self.m_object_matches_filter_parameters = Some(parameters);
            }
            e_condition_type::player_is_active => {
                let mut parameters = s_condition_player_is_active_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_is_active_parameters = Some(parameters);
            }
            e_condition_type::equipment_is_active => {
                let mut parameters = s_condition_equipment_is_active_parameters::default();
                parameters.decode(bitstream)?;
                self.m_equipment_is_active_parameters = Some(parameters);
            }
            e_condition_type::player_is_spartan => {
                let mut parameters = s_condition_player_is_spartan_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_is_spartan_parameters = Some(parameters);
            }
            e_condition_type::player_is_elite => {
                let mut parameters = s_condition_player_is_elite_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_is_elite_parameters = Some(parameters);
            }
            e_condition_type::player_is_editor => {
                let mut parameters = s_condition_player_is_editor_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_is_editor_parameters = Some(parameters);
            }
            e_condition_type::game_is_forge => {
                let mut parameters = s_condition_game_is_forge_parameters::default();
                parameters.decode(bitstream)?;
                self.m_game_is_forge_parameters = Some(parameters);
            }
            e_condition_type::vehicle_entered => {
                let mut parameters = s_condition_vehicle_entered_parameters::default();
                parameters.decode(bitstream)?;
                self.m_vehicle_entered_parameters = Some(parameters);
            }
            e_condition_type::vehicle_exited => {
                let mut parameters = s_condition_vehicle_exited_parameters::default();
                parameters.decode(bitstream)?;
                self.m_vehicle_exited_parameters = Some(parameters);
            }
            e_condition_type::vehicle_jacked => {
                let mut parameters = s_condition_vehicle_jacked_parameters::default();
                parameters.decode(bitstream)?;
                self.m_vehicle_jacked_parameters = Some(parameters);
            }
            e_condition_type::is_update_frame => {
                let mut parameters = s_condition_is_update_frame_parameters::default();
                parameters.decode(bitstream)?;
                self.m_is_update_frame_parameters = Some(parameters);
            }
            e_condition_type::current_incident => {
                let mut parameters = s_condition_current_incident_parameters::default();
                parameters.decode(bitstream)?;
                self.m_current_incident_parameters = Some(parameters);
            }
            e_condition_type::object_physics_collided => {
                let mut parameters = s_condition_object_physics_collided_parameters::default();
                parameters.decode(bitstream)?;
                self.m_object_physics_collided_parameters = Some(parameters);
            }
            e_condition_type::weapon_is_held_by_hologram => {
                let mut parameters = s_condition_weapon_is_held_by_hologram_parameters::default();
                parameters.decode(bitstream)?;
                self.m_weapon_is_held_by_hologram_parameters = Some(parameters);
            }
            e_condition_type::none => {}
        }

        Ok(())
    }
}
