use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_variant_variable::s_variant_variable;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::OPTION_TO_RESULT;
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_reference::c_object_reference;

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
    pub m_killer_type: u8, // 5 bits
}

impl s_condition_player_died_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_integer(self.m_killer_type, 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_killer_type = bitstream.read_integer("killer-type", 5)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_team_disposition_parameters {
    pub m_team_1: c_team_reference,
    pub m_team_2: c_team_reference,
    pub m_disposition: u8, // 2 bits
}

impl s_condition_team_disposition_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_team_1.encode(bitstream)?;
        self.m_team_2.encode(bitstream)?;
        bitstream.write_integer(self.m_disposition, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_team_1.decode(bitstream)?;
        self.m_team_2.decode(bitstream)?;
        self.m_disposition = bitstream.read_integer("disposition", 2)?;

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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
pub enum e_condition_type {
    #[default]
    none = 0,
    compare = 1,
    shape_contains = 2,
    killer_type_is = 3,
    has_alliance_status = 4,
    is_zero = 5,
    is_of_type = 6,
    has_any_players = 7,
    is_out_of_bounds = 8,
    is_fireteam_leader = 9,
    assisted_kill_of = 10,
    has_forge_label = 11,
    is_not_respawning = 12,
    is_in_use = 13,
    is_spartan = 14,
    is_elite = 15,
    is_monitor = 16,
    is_in_forge = 17,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_condition {
    pub m_type: e_condition_type, // 5 bits
    pub m_negated: bool,
    pub m_union_group: u16, // 9 bits
    pub m_execute_before_action: u16, // 10 bits

    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_if_parameters: Option<s_condition_if_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_in_area_parameters: Option<s_condition_object_in_area_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_died_parameters: Option<s_condition_player_died_parameters>,
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
}

impl c_condition {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_type.clone(), 5)?;
        if self.m_type == e_condition_type::none {
            return Ok(());
        }

        bitstream.write_bool(self.m_negated)?;
        bitstream.write_integer(self.m_union_group, 9)?;
        bitstream.write_integer(self.m_execute_before_action, 10)?;

        match self.m_type {
            e_condition_type::compare => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_if_parameters,
                    format!("Can't encode condition type {:?} without m_if_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::shape_contains => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_object_in_area_parameters,
                    format!("Can't encode condition type {:?} without m_object_in_area_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::killer_type_is => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_died_parameters,
                    format!("Can't encode condition type {:?} without m_player_died_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::has_alliance_status => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_team_disposition_parameters,
                    format!("Can't encode condition type {:?} without m_team_disposition_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::is_zero => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_timer_expired_parameters,
                    format!("Can't encode condition type {:?} without m_timer_expired_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::is_of_type => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_object_is_type_parameters,
                    format!("Can't encode condition type {:?} without m_object_is_type_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::has_any_players => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_team_is_active_parameters,
                    format!("Can't encode condition type {:?} without m_team_is_active_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::is_out_of_bounds => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_object_out_of_bounds_parameters,
                    format!("Can't encode condition type {:?} without m_object_out_of_bounds_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::is_fireteam_leader => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_is_fire_team_leader_parameters,
                    format!("Can't encode condition type {:?} without m_player_is_fire_team_leader_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::assisted_kill_of => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_assisted_with_kill_parameters,
                    format!("Can't encode condition type {:?} without m_player_assisted_with_kill_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::has_forge_label => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_object_matches_filter_parameters,
                    format!("Can't encode condition type {:?} without m_object_matches_filter_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::is_not_respawning => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_is_active_parameters,
                    format!("Can't encode condition type {:?} without m_player_is_active_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::is_in_use => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_equipment_is_active_parameters,
                    format!("Can't encode condition type {:?} without m_equipment_is_active_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::is_spartan => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_is_spartan_parameters,
                    format!("Can't encode condition type {:?} without m_player_is_spartan_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::is_elite => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_is_elite_parameters,
                    format!("Can't encode condition type {:?} without m_player_is_elite_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::is_monitor => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_player_is_editor_parameters,
                    format!("Can't encode condition type {:?} without m_player_is_editor_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::is_in_forge => {
                let parameters = OPTION_TO_RESULT!(
                    &self.m_game_is_forge_parameters,
                    format!("Can't encode condition type {:?} without m_game_is_forge_parameters", self.m_type)
                )?;
                parameters.encode(bitstream)?;
            }
            e_condition_type::none => unreachable!(),
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let condition_type = bitstream.read_integer("condition-type", 5)?;
        if let Some(condition_type) = FromPrimitive::from_u32(condition_type) {
            self.m_type = condition_type;
        } else {
            return Ok(())
        }

        if self.m_type == e_condition_type::none {
            return Ok(());
        }

        self.m_negated = bitstream.read_bool("negated")?;
        self.m_union_group = bitstream.read_integer("union-group", 9)?;
        self.m_execute_before_action = bitstream.read_integer("execute-before-action", 10)?;

        match self.m_type {
            e_condition_type::compare => {
                let mut parameters = s_condition_if_parameters::default();
                parameters.decode(bitstream)?;
                self.m_if_parameters = Some(parameters);
            }
            e_condition_type::shape_contains => {
                let mut parameters = s_condition_object_in_area_parameters::default();
                parameters.decode(bitstream)?;
                self.m_object_in_area_parameters = Some(parameters);
            }
            e_condition_type::killer_type_is => {
                let mut parameters = s_condition_player_died_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_died_parameters = Some(parameters);
            }
            e_condition_type::has_alliance_status => {
                let mut parameters = s_condition_team_disposition_parameters::default();
                parameters.decode(bitstream)?;
                self.m_team_disposition_parameters = Some(parameters);
            }
            e_condition_type::is_zero => {
                let mut parameters = s_condition_timer_expired_parameters::default();
                parameters.decode(bitstream)?;
                self.m_timer_expired_parameters = Some(parameters);
            }
            e_condition_type::is_of_type => {
                let mut parameters = s_condition_object_is_type_parameters::default();
                parameters.decode(bitstream)?;
                self.m_object_is_type_parameters = Some(parameters);
            }
            e_condition_type::has_any_players => {
                let mut parameters = s_condition_team_is_active_parameters::default();
                parameters.decode(bitstream)?;
                self.m_team_is_active_parameters = Some(parameters);
            }
            e_condition_type::is_out_of_bounds => {
                let mut parameters = s_condition_object_out_of_bounds_parameters::default();
                parameters.decode(bitstream)?;
                self.m_object_out_of_bounds_parameters = Some(parameters);
            }
            e_condition_type::is_fireteam_leader => {
                let mut parameters = s_condition_player_is_fire_team_leader_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_is_fire_team_leader_parameters = Some(parameters);
            }
            e_condition_type::assisted_kill_of => {
                let mut parameters = s_condition_player_assisted_with_kill_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_assisted_with_kill_parameters = Some(parameters);
            }
            e_condition_type::has_forge_label => {
                let mut parameters = s_condition_object_matches_filter_parameters::default();
                parameters.decode(bitstream)?;
                self.m_object_matches_filter_parameters = Some(parameters);
            }
            e_condition_type::is_not_respawning => {
                let mut parameters = s_condition_player_is_active_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_is_active_parameters = Some(parameters);
            }
            e_condition_type::is_in_use => {
                let mut parameters = s_condition_equipment_is_active_parameters::default();
                parameters.decode(bitstream)?;
                self.m_equipment_is_active_parameters = Some(parameters);
            }
            e_condition_type::is_spartan => {
                let mut parameters = s_condition_player_is_spartan_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_is_spartan_parameters = Some(parameters);
            }
            e_condition_type::is_elite => {
                let mut parameters = s_condition_player_is_elite_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_is_elite_parameters = Some(parameters);
            }
            e_condition_type::is_monitor => {
                let mut parameters = s_condition_player_is_editor_parameters::default();
                parameters.decode(bitstream)?;
                self.m_player_is_editor_parameters = Some(parameters);
            }
            e_condition_type::is_in_forge => {
                let mut parameters = s_condition_game_is_forge_parameters::default();
                parameters.decode(bitstream)?;
                self.m_game_is_forge_parameters = Some(parameters);
            }
            e_condition_type::none => {}
        }

        Ok(())
    }
}
