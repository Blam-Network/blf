use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_conditions::{s_condition_object_matches_filter_parameters, s_condition_player_died_parameters, s_condition_team_disposition_parameters};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::megalogamengine::megalogamengine_variant_variable::s_variant_variable;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::OPTION_TO_RESULT;
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_reference::c_object_reference;


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_if_parameters {
    pub m_left: s_variant_variable,
    pub m_right: s_variant_variable,
    pub m_comparison: u8, // 3 bit
}

impl s_condition_if_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_left.encode(bitstream)?;
        self.m_right.encode(bitstream)?;
        bitstream.write_integer(self.m_comparison, 3)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_left.decode(bitstream)?;
        self.m_right.decode(bitstream)?;
        self.m_comparison = bitstream.read_integer("comparison", 3)?;

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
    // -- not in beta --
    // is_spartan = 14,
    // is_elite = 15,
    // is_monitor = 16,
    // is_in_forge = 17,
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
    pub m_object_reference_1: Option<c_object_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_reference_2: Option<c_object_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_died_parameters: Option<s_condition_player_died_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_timer: Option<c_custom_timer_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team_disposition_parameters: Option<s_condition_team_disposition_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_type_reference: Option<c_object_type_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team_reference: Option<c_team_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_reference_1: Option<c_player_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_reference_2: Option<c_player_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_matches_filter_parameters: Option<s_condition_object_matches_filter_parameters>

}

impl c_condition {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_type.clone(), 5)?;
        if self.m_type == e_condition_type::none {
            return Ok(());
        }

        bitstream.write_bool(self.m_negated)?;
        bitstream.write_integer(self.m_union_group, 9)?;
        bitstream.write_integer(self.m_execute_before_action, 10)?;

        match self.m_type.clone() as u32 {
            1 => {
                let if_parameters = OPTION_TO_RESULT!(
                    &self.m_if_parameters,
                    format!("Can't encode condition type {:?} without if_parameters", &self.m_type)
                )?;
                if_parameters.encode(bitstream)?; // OK
            }
            2 => {
                let object_reference_1 = OPTION_TO_RESULT!(
                    &self.m_object_reference_1,
                    format!("Can't encode condition type {:?} without object_reference_1", &self.m_type)
                )?;
                let object_reference_2 = OPTION_TO_RESULT!(
                    &self.m_object_reference_2,
                    format!("Can't encode condition type {:?} without object_reference_2", &self.m_type)
                )?;
                object_reference_1.encode(bitstream)?;
                object_reference_2.encode(bitstream)?;
            }
            3 => {
                let player_died_parameters = OPTION_TO_RESULT!(
                    &self.m_player_died_parameters,
                    format!("Can't encode condition type {:?} without player_died_parameters", &self.m_type)
                )?;
                player_died_parameters.encode(bitstream)?;
            }
            4 => {
                let team_disposition_parameters = OPTION_TO_RESULT!(
                    &self.m_team_disposition_parameters,
                    format!("Can't encode condition type {:?} without team_disposition_parameters", &self.m_type)
                )?;
                team_disposition_parameters.encode(bitstream)?;
            }
            5 => {
                let timer = OPTION_TO_RESULT!(
                    &self.m_timer,
                    format!("Can't encode condition type {:?} without timer", &self.m_type)
                )?;
                timer.encode(bitstream)?;
            }
            6 => {
                let object_reference = OPTION_TO_RESULT!(
                    &self.m_object_reference_1,
                    format!("Can't encode condition type {:?} without object_reference", &self.m_type)
                )?;
                let object_type_reference = OPTION_TO_RESULT!(
                    &self.m_object_type_reference,
                    format!("Can't encode condition type {:?} without object_type_reference", &self.m_type)
                )?;
                object_reference.encode(bitstream)?;
                object_type_reference.encode(bitstream)?;
            }
            7 => {
                let team = OPTION_TO_RESULT!(
                    &self.m_team_reference,
                    format!("Can't encode condition type {:?} without team", &self.m_type)
                )?;
                team.encode(bitstream)?;
            }
            8 | 13 => {
                let object = OPTION_TO_RESULT!(
                    &self.m_object_reference_1,
                    format!("Can't encode condition type {:?} without object reference", &self.m_type)
                )?;
                object.encode(bitstream)?;
            }
            9 | 12 => {
                let player = OPTION_TO_RESULT!(
                    &self.m_player_reference_1,
                    format!("Can't encode condition type {:?} without player reference", &self.m_type)
                )?;
                player.encode(bitstream)?;
            }
            10 => {
                let player_1 = OPTION_TO_RESULT!(
                    &self.m_player_reference_1,
                    format!("Can't encode condition type {:?} without m_player_reference_1", &self.m_type)
                )?;
                let player_2 = OPTION_TO_RESULT!(
                    &self.m_player_reference_2,
                    format!("Can't encode condition type {:?} without m_player_reference_2", &self.m_type)
                )?;
                player_1.encode(bitstream)?;
                player_2.encode(bitstream)?;
            }
            11 => {
                let object_matches_filter_parameters = OPTION_TO_RESULT!(
                    &self.m_object_matches_filter_parameters,
                    format!("Can't encode condition type {:?} without object_matches_filter_parameters", &self.m_type)
                )?;
                object_matches_filter_parameters.encode(bitstream)?;
            }
            _ => {
                return Err(format!("Invalid c_condition: {self:?}").into())
            }
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let condition_type = bitstream.read_integer("condition-type", 4)?;
        if let Some(condition_type) = FromPrimitive::from_u32(condition_type) {
            self.m_type = condition_type;
        }
        else {
            return Ok(())
            // return Err(format!("unsupported action type: {}", action_type).into())
        }

        self.m_negated = bitstream.read_bool("negated")?;
        self.m_union_group = bitstream.read_integer("union-group", 9)?;
        self.m_execute_before_action = bitstream.read_integer("execute-before-action", 10)?;

        match self.m_type.clone() as u32 {
            1 => {
                let mut if_parameters = s_condition_if_parameters::default();
                if_parameters.decode(bitstream)?;
                self.m_if_parameters = Some(if_parameters);
            }
            2 => {
                let mut object_reference_1 = c_object_reference::default();
                let mut object_reference_2 = c_object_reference::default();
                object_reference_1.decode(bitstream)?;
                object_reference_2.decode(bitstream)?;
                self.m_object_reference_1 = Some(object_reference_1);
                self.m_object_reference_2 = Some(object_reference_2);
            }
            3 => {
                let mut player_died_parameters = s_condition_player_died_parameters::default();
                player_died_parameters.decode(bitstream)?;
                self.m_player_died_parameters = Some(player_died_parameters);
            }
            4 => {
                let mut team_disposition_parameters = s_condition_team_disposition_parameters::default();
                team_disposition_parameters.decode(bitstream)?;
                self.m_team_disposition_parameters = Some(team_disposition_parameters);
            }
            5 => {
                let mut timer = c_custom_timer_reference::default();
                timer.decode(bitstream)?;
                self.m_timer = Some(timer);
            }
            6 => {
                let mut object_reference = c_object_reference::default();
                let mut object_type_reference = c_object_type_reference::default();
                object_reference.decode(bitstream)?;
                object_type_reference.decode(bitstream)?;
                self.m_object_reference_1 = Some(object_reference);
                self.m_object_type_reference = Some(object_type_reference);
            }
            7 => {
                let mut team = c_team_reference::default();
                team.decode(bitstream)?;
                self.m_team_reference = Some(team);
            }
            8 | 13 => {
                let mut object_reference = c_object_reference::default();
                object_reference.decode(bitstream)?;
                self.m_object_reference_1 = Some(object_reference);
            }
            9 | 12 | 14 | 15 | 16 => {
                let mut player = c_player_reference::default();
                player.decode(bitstream)?;
                self.m_player_reference_1 = Some(player);
            }
            10 => {
                let mut player_1 = c_player_reference::default();
                let mut player_2 = c_player_reference::default();
                player_1.decode(bitstream)?;
                player_2.decode(bitstream)?;
                self.m_player_reference_1 = Some(player_1);
                self.m_player_reference_2 = Some(player_2);
            }
            11 => {
                let mut object_matches_filter_parameters = s_condition_object_matches_filter_parameters::default();
                object_matches_filter_parameters.decode(bitstream)?;
                self.m_object_matches_filter_parameters = Some(object_matches_filter_parameters);
            }
            _ => {}
        }

        Ok(())
    }
}