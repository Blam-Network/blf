use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_variant_variable::s_variant_variable;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_object_reference::c_object_reference;
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::memory::bitstream_reader::c_bitstream_reader_extensions;
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::memory::bitstream_writer::c_bitstream_writer_extensions;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::OPTION_TO_RESULT;
use blf_lib_derivable::result::BLFLibResult;

/// Alpha uses 2 bits for comparison (retail uses 3 bits with two additional operators).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_numeric_comparison {
    #[default]
    less_than = 0, // <
    greater_than = 1, // >
    equal_to = 2, // ==
    less_than_or_equal_to = 3, // <=
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_condition_if_parameters {
    pub m_left: s_variant_variable,
    pub m_right: s_variant_variable,
    pub m_comparison: e_numeric_comparison, // 2 bits
}

impl s_condition_if_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_left.encode(bitstream)?;
        self.m_right.encode(bitstream)?;
        bitstream.write_enum(self.m_comparison, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_left.decode(bitstream)?;
        self.m_right.decode(bitstream)?;
        self.m_comparison = bitstream.read_enum("comparison", 2)?;

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

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
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
        bitstream.write_enum(self.m_type, 4)?;
        if self.m_type == e_condition_type::none {
            return Ok(());
        }

        bitstream.write_bool(self.m_negated)?;
        bitstream.write_integer(self.m_union_group, 9)?;
        bitstream.write_integer(self.m_execute_before_action, 10)?;

        match self.m_type {
            e_condition_type::compare => {
                let if_parameters = OPTION_TO_RESULT!(
                    &self.m_if_parameters,
                    format!("Can't encode condition type {:?} without if_parameters", self.m_type)
                )?;
                if_parameters.encode(bitstream)?;
            }
            e_condition_type::shape_contains => {
                let object_reference_1 = OPTION_TO_RESULT!(
                    &self.m_object_reference_1,
                    format!("Can't encode condition type {:?} without object_reference_1", self.m_type)
                )?;
                let object_reference_2 = OPTION_TO_RESULT!(
                    &self.m_object_reference_2,
                    format!("Can't encode condition type {:?} without object_reference_2", self.m_type)
                )?;
                object_reference_1.encode(bitstream)?;
                object_reference_2.encode(bitstream)?;
            }
            e_condition_type::killer_type_is => {
                let player_died_parameters = OPTION_TO_RESULT!(
                    &self.m_player_died_parameters,
                    format!("Can't encode condition type {:?} without player_died_parameters", self.m_type)
                )?;
                player_died_parameters.encode(bitstream)?;
            }
            e_condition_type::has_alliance_status => {
                let team_disposition_parameters = OPTION_TO_RESULT!(
                    &self.m_team_disposition_parameters,
                    format!("Can't encode condition type {:?} without team_disposition_parameters", self.m_type)
                )?;
                team_disposition_parameters.encode(bitstream)?;
            }
            e_condition_type::is_zero => {
                let timer = OPTION_TO_RESULT!(
                    &self.m_timer,
                    format!("Can't encode condition type {:?} without timer", self.m_type)
                )?;
                timer.encode(bitstream)?;
            }
            e_condition_type::is_of_type => {
                let object_reference = OPTION_TO_RESULT!(
                    &self.m_object_reference_1,
                    format!("Can't encode condition type {:?} without object_reference", self.m_type)
                )?;
                let object_type_reference = OPTION_TO_RESULT!(
                    &self.m_object_type_reference,
                    format!("Can't encode condition type {:?} without object_type_reference", self.m_type)
                )?;
                object_reference.encode(bitstream)?;
                object_type_reference.encode(bitstream)?;
            }
            e_condition_type::has_any_players => {
                let team = OPTION_TO_RESULT!(
                    &self.m_team_reference,
                    format!("Can't encode condition type {:?} without team", self.m_type)
                )?;
                team.encode(bitstream)?;
            }
            e_condition_type::is_out_of_bounds | e_condition_type::is_in_use => {
                let object = OPTION_TO_RESULT!(
                    &self.m_object_reference_1,
                    format!("Can't encode condition type {:?} without object reference", self.m_type)
                )?;
                object.encode(bitstream)?;
            }
            e_condition_type::is_fireteam_leader | e_condition_type::is_not_respawning => {
                let player = OPTION_TO_RESULT!(
                    &self.m_player_reference_1,
                    format!("Can't encode condition type {:?} without player reference", self.m_type)
                )?;
                player.encode(bitstream)?;
            }
            e_condition_type::assisted_kill_of => {
                let player_1 = OPTION_TO_RESULT!(
                    &self.m_player_reference_1,
                    format!("Can't encode condition type {:?} without m_player_reference_1", self.m_type)
                )?;
                let player_2 = OPTION_TO_RESULT!(
                    &self.m_player_reference_2,
                    format!("Can't encode condition type {:?} without m_player_reference_2", self.m_type)
                )?;
                player_1.encode(bitstream)?;
                player_2.encode(bitstream)?;
            }
            e_condition_type::has_forge_label => {
                let object_matches_filter_parameters = OPTION_TO_RESULT!(
                    &self.m_object_matches_filter_parameters,
                    format!("Can't encode condition type {:?} without object_matches_filter_parameters", self.m_type)
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
        self.m_type = FromPrimitive::from_u32(condition_type)
            .ok_or_else(|| format!("unsupported condition type: {condition_type}"))?;

        if self.m_type == e_condition_type::none {
            return Ok(());
        }

        self.m_negated = bitstream.read_bool("negated")?;
        self.m_union_group = bitstream.read_integer("union-group", 9)?;
        self.m_execute_before_action = bitstream.read_integer("execute-before-action", 10)?;

        match self.m_type {
            e_condition_type::compare => {
                let mut if_parameters = s_condition_if_parameters::default();
                if_parameters.decode(bitstream)?;
                self.m_if_parameters = Some(if_parameters);
            }
            e_condition_type::shape_contains => {
                let mut object_reference_1 = c_object_reference::default();
                let mut object_reference_2 = c_object_reference::default();
                object_reference_1.decode(bitstream)?;
                object_reference_2.decode(bitstream)?;
                self.m_object_reference_1 = Some(object_reference_1);
                self.m_object_reference_2 = Some(object_reference_2);
            }
            e_condition_type::killer_type_is => {
                let mut player_died_parameters = s_condition_player_died_parameters::default();
                player_died_parameters.decode(bitstream)?;
                self.m_player_died_parameters = Some(player_died_parameters);
            }
            e_condition_type::has_alliance_status => {
                let mut team_disposition_parameters = s_condition_team_disposition_parameters::default();
                team_disposition_parameters.decode(bitstream)?;
                self.m_team_disposition_parameters = Some(team_disposition_parameters);
            }
            e_condition_type::is_zero => {
                let mut timer = c_custom_timer_reference::default();
                timer.decode(bitstream)?;
                self.m_timer = Some(timer);
            }
            e_condition_type::is_of_type => {
                let mut object_reference = c_object_reference::default();
                let mut object_type_reference = c_object_type_reference::default();
                object_reference.decode(bitstream)?;
                object_type_reference.decode(bitstream)?;
                self.m_object_reference_1 = Some(object_reference);
                self.m_object_type_reference = Some(object_type_reference);
            }
            e_condition_type::has_any_players => {
                let mut team = c_team_reference::default();
                team.decode(bitstream)?;
                self.m_team_reference = Some(team);
            }
            e_condition_type::is_out_of_bounds | e_condition_type::is_in_use => {
                let mut object_reference = c_object_reference::default();
                object_reference.decode(bitstream)?;
                self.m_object_reference_1 = Some(object_reference);
            }
            e_condition_type::is_fireteam_leader | e_condition_type::is_not_respawning => {
                let mut player = c_player_reference::default();
                player.decode(bitstream)?;
                self.m_player_reference_1 = Some(player);
            }
            e_condition_type::assisted_kill_of => {
                let mut player_1 = c_player_reference::default();
                let mut player_2 = c_player_reference::default();
                player_1.decode(bitstream)?;
                player_2.decode(bitstream)?;
                self.m_player_reference_1 = Some(player_1);
                self.m_player_reference_2 = Some(player_2);
            }
            e_condition_type::has_forge_label => {
                let mut object_matches_filter_parameters = s_condition_object_matches_filter_parameters::default();
                object_matches_filter_parameters.decode(bitstream)?;
                self.m_object_matches_filter_parameters = Some(object_matches_filter_parameters);
            }
            e_condition_type::none => unreachable!(),
        }

        Ok(())
    }
}
