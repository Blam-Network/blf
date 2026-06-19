use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_conditions::{
    s_condition_equipment_is_active_parameters, s_condition_game_is_forge_parameters,
    s_condition_object_in_area_parameters, s_condition_object_is_type_parameters,
    s_condition_object_matches_filter_parameters, s_condition_object_out_of_bounds_parameters,
    s_condition_player_assisted_with_kill_parameters, s_condition_player_died_parameters,
    s_condition_player_is_active_parameters, s_condition_player_is_editor_parameters,
    s_condition_player_is_elite_parameters, s_condition_player_is_fire_team_leader_parameters,
    s_condition_player_is_spartan_parameters, s_condition_team_disposition_parameters,
    s_condition_team_is_active_parameters, s_condition_timer_expired_parameters, e_numeric_comparison,
};
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::megalogamengine::megalogamengine_variant_variable::s_variant_variable;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::OPTION_TO_RESULT;
use blf_lib_derivable::result::BLFLibResult;

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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
pub enum e_condition_type {
    #[default]
    none = 0,
    r#if = 1,
    object_in_area = 2,
    player_died = 3,
    team_disposition = 4,
    timer_expired = 5,
    object_is_type = 6,
    team_is_active = 7,
    object_out_of_bounds = 8,
    player_is_fire_team_leader = 9,
    player_assisted_with_kill = 10,
    object_matches_filter = 11,
    player_is_active = 12,
    equipment_is_active = 13,
    player_is_spartan = 14,
    player_is_elite = 15,
    player_is_editor = 16,
    game_is_forge = 17,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_condition {
    pub m_type: e_condition_type, // 4 bits
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
        bitstream.write_enum_raw(self.m_type.clone(), 4)?;
        if self.m_type == e_condition_type::none {
            return Ok(());
        }

        bitstream.write_bool(self.m_negated)?;
        bitstream.write_integer(self.m_union_group, 9)?;
        bitstream.write_integer(self.m_execute_before_action, 10)?;

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
            e_condition_type::none => unreachable!(),
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let condition_type = bitstream.read_integer("condition-type", 4)?;
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
            e_condition_type::none => {}
        }

        Ok(())
    }
}
