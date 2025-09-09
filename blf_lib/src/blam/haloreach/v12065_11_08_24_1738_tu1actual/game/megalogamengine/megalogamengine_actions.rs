use binrw::{BinRead, BinWrite};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_text::c_dynamic_string;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_variant_variable::s_variant_variable;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::scenario_map_variant::e_boundary_shape;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::OPTION_TO_RESULT;
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_reference::c_object_reference;




#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_team_or_player_target {
    pub m_target: u8, // 2 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_team_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_player_reference>
}

impl s_team_or_player_target {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_target, 2)?;
        match (self.m_target, &self.m_team, &self.m_player) {
            (0, Some(team), None) => {
                team.encode(bitstream)?;
            }
            (1, None, Some(player)) => {
                player.encode(bitstream)?;
            }
            _ => {

            }
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_target = bitstream.read_integer("target", 2)?;
        match self.m_target {
            0 => {
                let mut team = c_team_reference::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
            }
            1 => {
                let mut player = c_player_reference::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            _ => {}
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_score_parameters {
    pub m_target: s_team_or_player_target,
    pub m_operation: u8, // 4 bits
    pub m_variable: c_custom_variable_reference
}

impl s_action_set_score_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_target.encode(bitstream)?;
        bitstream.write_integer(self.m_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_target.decode(bitstream)?;
        self.m_operation = bitstream.read_integer("operation", 4)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_create_object_parameters {
    pub m_object_type: c_object_type_reference,
    pub m_object_reference_1: c_object_reference,
    pub m_object_reference_2: c_object_reference,
    pub m_filter_index: i8, // 4 bits
    pub m_flags: u8, // 3 bits
    pub m_offset: u32, // 24 bits
    pub m_variant_name_index: u8, // 8 bits
}

impl s_action_create_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_type.encode(bitstream)?;
        self.m_object_reference_1.encode(bitstream)?;
        self.m_object_reference_2.encode(bitstream)?;
        bitstream.write_index::<16>(self.m_filter_index, 4)?;
        bitstream.write_integer(self.m_flags, 3)?;
        bitstream.write_integer(self.m_offset, 24)?;
        bitstream.write_integer(self.m_variant_name_index, 8)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_type.decode(bitstream)?;
        self.m_object_reference_1.decode(bitstream)?;
        self.m_object_reference_2.decode(bitstream)?;
        self.m_filter_index = bitstream.read_index::<16>("filter_index", 4)? as i8;
        self.m_flags = bitstream.read_integer("flags", 3)?;
        self.m_offset = bitstream.read_integer("offset", 24)?;
        self.m_variant_name_index = bitstream.read_integer("variant-name-index", 8)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_player_filter_modifier {
    pub m_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_player_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable: Option<c_custom_variable_reference>,
}

impl c_player_filter_modifier {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_type, 3)?;
        match (self.m_type, &self.m_player, &self.m_variable) {
            (4, Some(player), Some(variable)) => {
                player.encode(bitstream)?;
                variable.encode(bitstream)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_integer("type", 3)?;
        if self.m_type == 4 {
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

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_navpoint_set_icon_parameters {
    pub m_object: c_object_reference,
    pub m_navpoint_icon: u8, // 5 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable: Option<c_custom_variable_reference>,
}

impl s_action_navpoint_set_icon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_integer(self.m_navpoint_icon, 5)?;

        match (self.m_navpoint_icon, &self.m_variable) {
            (11, Some(variable)) => {
                variable.encode(bitstream)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_navpoint_icon = bitstream.read_integer("navpoint-icon", 5)?;

        if self.m_navpoint_icon == 11 {
            let mut variable = c_custom_variable_reference::default();
            variable.decode(bitstream)?;
            self.m_variable = Some(variable);
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_navpoint_set_priority_parameters {
    pub m_object: c_object_reference,
    pub m_priority: u8, // 2 bits
}

impl s_action_navpoint_set_priority_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_integer(self.m_priority, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_priority = bitstream.read_integer("priority", 2)?;

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

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_parameters {
    pub m_variable_1: s_variant_variable,
    pub m_variable_2: s_variant_variable,
    pub m_operation: u8, // 4 bits
}

impl s_action_set_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_variable_1.encode(bitstream)?;
        self.m_variable_2.encode(bitstream)?;
        bitstream.write_integer(self.m_operation, 4)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_variable_1.decode(bitstream)?;
        self.m_variable_2.decode(bitstream)?;
        self.m_operation = bitstream.read_integer("operation", 4)?;

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
        bitstream.write_enum(self.m_shape, 2)?;
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
        self.m_shape = bitstream.read_enum( 2)?;

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

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_fireteam_respawn_filter_parameters {
    pub m_object: c_object_reference,
    pub m_fireteam_filter: u8, // 8 bits
}

impl s_action_set_fireteam_respawn_filter_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_integer(self.m_fireteam_filter, 8)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_fireteam_filter = bitstream.read_integer("fireteam-filter", 8)?;

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
    pub m_sound_index: u8, // 7 bits
    pub m_string: c_dynamic_string,
}

impl s_action_hud_post_message_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_target.encode(bitstream)?;
        bitstream.write_integer(self.m_sound_index, 7)?;
        self.m_string.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_target.decode(bitstream)?;
        self.m_sound_index = bitstream.read_integer("sound-index", 2)?;
        self.m_string.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_timer_set_rate_parameters {
    pub m_timer: c_custom_timer_reference,
    pub m_rate: u8, // 5 bits
}

impl s_action_timer_set_rate_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_timer.encode(bitstream)?;
        bitstream.write_integer(self.m_rate, 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_timer.decode(bitstream)?;
        self.m_rate = bitstream.read_integer("timer-rate", 5)?;

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
    pub m_offset: u32,
    pub m_absolute_orientation: bool,
}

impl s_action_object_attach_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        bitstream.write_integer(self.m_offset, 24)?;
        bitstream.write_bool(self.m_absolute_orientation)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        self.m_offset = bitstream.read_integer("offset", 24)?;
        self.m_absolute_orientation = bitstream.read_bool("absolute_orientation")?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_adjust_money_parameters {
    pub m_player: c_player_reference,
    pub m_math_operation: u8,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_player_adjust_money_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_integer(self.m_math_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_math_operation = bitstream.read_integer("math-operation", 4)?;
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
    pub m_weapon_pickup_priority: u8, // 5 bits
}

impl s_action_weapon_set_pickup_priority_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_integer(self.m_weapon_pickup_priority, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_weapon_pickup_priority = bitstream.read_integer("weapon-pickup-priority", 2)?;

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
    pub m_type: u8, // 2 bits
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
            (1, Some(variable1), Some(variable2), None) => {
                bitstream.write_integer(1u8, 2)?;
                variable1.encode(bitstream)?;
                variable2.encode(bitstream)?;
            }
            (2, None, None, Some(timer)) => {
                bitstream.write_integer(2u8, 2)?;
                timer.encode(bitstream)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_integer("type", 2)?;
        match self.m_type {
            1 => {
                let mut variable1 = c_custom_variable_reference::default();
                let mut variable2 = c_custom_variable_reference::default();
                variable1.decode(bitstream)?;
                variable2.decode(bitstream)?;
                self.m_variable_1 = Some(variable1);
                self.m_variable_2 = Some(variable2);
            }
            2 => {
                let mut timer= c_custom_timer_reference::default();
                timer.decode(bitstream)?;
                self.m_timer = Some(timer);
            }
            _ => {}
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
        self.m_widget_index = bitstream.read_index::<64>("icon-index", 6)? as i8;

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
    pub m_sound_index: u8, // 7 bits
    pub m_immediate: bool,
    pub m_target: s_team_or_player_target,
}

impl s_action_play_sound_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_sound_index, 7)?;
        bitstream.write_bool(self.m_immediate)?;
        self.m_target.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_sound_index = bitstream.read_integer("sound-index", 7)?;
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
    pub m_operation: u8, // 4 bits
    pub m_variable: c_custom_variable_reference,
}

impl s_action_vitality_adjustment_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_integer(self.m_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_operation = bitstream.read_integer("operation", 4)?;
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
    pub m_grenade_type: u8, // 1 bit
    pub m_math_operation: u8, // 4 bits
    pub m_variable: c_custom_variable_reference,
}

impl s_action_adjust_grenades_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_integer(self.m_grenade_type, 1)?;
        bitstream.write_integer(self.m_math_operation, 4)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_grenade_type = bitstream.read_integer("grenade-type", 1)?;
        self.m_math_operation = bitstream.read_integer("math-operation", 4)?;
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
    pub m_offset: u32,
}

impl s_action_object_face_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_1.encode(bitstream)?;
        self.m_object_2.encode(bitstream)?;
        bitstream.write_integer(self.m_offset, 24)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_1.decode(bitstream)?;
        self.m_object_2.decode(bitstream)?;
        self.m_offset = bitstream.read_integer("offset", 24)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_biped_give_weapon_parameters {
    pub m_object: c_object_reference,
    pub m_object_type: c_object_type_reference,
    pub m_mode: u8, // 2 bits
}

impl s_action_biped_give_weapon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_object_type.encode(bitstream)?;
        bitstream.write_integer(self.m_mode, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_object_type.decode(bitstream)?;
        self.m_mode = bitstream.read_integer("mode", 2)?;

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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
pub enum e_action_type {
    #[default]
    none = 0,
    set_score = 1,
    place_at_me = 2,
    delete_object = 3,
    set_waypoint_visibility = 4,
    set_waypoint_icon = 5,
    set_waypoint_priority = 6,
    set_waypoint_timer = 7,
    set_waypoint_distance = 8,
    modify_variable = 9,
    set_object_shape = 10,
    apply_player_traits = 11,
    set_weapon_pickup_permissions = 12,
    set_spawn_location_permissions = 13,
    set_spawn_location_fireteams = 14,
    set_object_progress_bar = 15,
    kill_feed_message = 16,
    set_timer_rate = 17,
    debug_print = 18,
    get_carrier = 19,
    run_nested_trigger = 20,
    end_round = 21,
    set_object_shape_visibility = 22,
    kill_object_instantly = 23,
    set_object_invincibility = 24,
    random_number = 25,
    break_into_debugger = 26,
    get_object_orientation = 27,
    get_speed = 28,
    get_killer = 29,
    get_death_damage_type = 30,
    get_death_damage_modifier = 31,
    debugging_enable_tracing = 32,
    attach_objects = 33,
    detach_objects = 34,
    get_player_scoreboard_position = 35,
    get_team_scoreboard_position = 36,
    get_player_killstreak = 37,
    modify_player_requisition_money = 38,
    set_player_requisition_purchase_modes = 39,
    get_player_vehicle = 40,
    force_player_into_vehicle = 41,
    set_player_biped = 42,
    reset_timer = 43,
    set_weapon_pickup_priority = 44,
    push_object_up = 45,
    set_text = 46,
    set_value_text = 47,
    set_meter_parameters = 48,
    set_icon = 49,
    set_visibility = 50,
    play_sound = 51,
    modify_object_scale = 52,
    set_waypoint_text = 53,
    get_object_shields = 54,
    get_object_health = 55,
    set_objective_text = 56,
    set_objective_allegiance_name = 57,
    set_objective_allegiance_icon = 58,
    set_co_op_spawning = 59,
    set_primary_respawn_object_for_team = 60,
    set_primary_respawn_object_for_player = 61,
    get_player_fireteam = 62,
    set_player_fireteam = 63,
    modify_object_shields = 64,
    modify_object_health = 65,
    get_distance = 66,
    modify_object_max_shields = 67,
    modify_object_max_health = 68,
    set_player_requisition_palette = 69,
    set_device_power = 70,
    get_device_power = 71,
    set_device_position = 72,
    get_device_position = 73,
    modify_player_grenades = 74,
    send_incident = 75,
    send_incident_with_value = 76,
    set_player_loadout_palette = 77,
    set_device_position_track = 78,
    animate_device_position = 79,
    set_device_actual_position = 80,
    insert_theater_film_marker = 81,
    enable_disable_spawn_zone = 82,
    get_player_weapon = 83,
    get_player_armor_ability = 84,
    enable_disable_object_garbage_collection = 85,
    get_player_target_object = 86,
    create_object_equidistant = 87,
    debug_force_splitscreen_count = 88,
    add_weapon_to_player = 89,
    set_co_op_spawning_for_player = 90,
    copy_object_rotation = 91,
    point_object_toward_object = 92,
    add_weapon_to_biped = 93,
    remove_weapon_from = 94,
    set_scenario_interpolator_state = 95,
    get_random_object = 96,
    record_griefer_penalty = 97,
    set_shape_owner = 98,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_action {
    pub m_type: e_action_type, // 7 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_score_parameters: Option<s_action_set_score_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_create_object_parameters: Option<s_action_create_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object: Option<c_object_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_filter_modifier: Option<c_player_filter_modifier>,
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
    pub m_set_fireteam_respawn_filter_parameters: Option<s_action_set_fireteam_respawn_filter_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_progress_bar_parameters: Option<s_action_set_progress_bar_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hud_post_message_parameters: Option<s_action_hud_post_message_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_timer_set_rate_parameters: Option<s_action_timer_set_rate_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_string: Option<c_dynamic_string>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_1: Option<c_player_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_2: Option<c_player_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_for_each_parameters: Option<s_action_for_each_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_destroy_parameters: Option<s_action_object_destroy_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_1: Option<c_custom_variable_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_2: Option<c_custom_variable_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_tracing_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_attach_parameters: Option<s_action_object_attach_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_team_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_adjust_money_parameters: Option<s_action_player_adjust_money_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_enable_purchases_parameters: Option<s_action_player_enable_purchases_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_timer: Option<c_custom_timer_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_weapon_set_pickup_priority_parameters: Option<s_action_weapon_set_pickup_priority_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hud_widget_text_base: Option<s_action_hud_widget_text_base>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hud_widget_set_meter_parameters: Option<s_action_hud_widget_set_meter_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hud_widget_set_icon_parameters: Option<s_action_hud_widget_set_icon_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_hud_widget_set_visibility_parameters: Option<s_action_hud_widget_set_visibility_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_play_sound_parameters: Option<s_action_play_sound_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_objective_allegiance_icon_parameters: Option<s_action_player_set_objective_allegiance_icon_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team_set_coop_spawning_parameters: Option<s_action_team_set_coop_spawning_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_vitality_adjustment_parameters: Option<s_action_vitality_adjustment_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_get_distance_parameters: Option<s_action_object_get_distance_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player_set_requisition_palette_parameters: Option<s_action_player_set_requisition_palette_parameters>,
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
    pub m_player_get_weapon_parameters: Option<s_action_player_get_weapon_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_create_tunnel_parameters: Option<s_action_create_tunnel_parameters>,
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
    pub m_get_random_object_parameters: Option<s_action_get_random_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_boundary_set_player_color_parameters: Option<s_action_boundary_set_player_color_parameters>,
}

impl c_action {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_type.clone(), 7)?;

        match self.m_type.clone() as u32 {
            1 => self.m_set_score_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_score_parameters does not exist."))?
                .encode(bitstream)?,
            2 => self.m_create_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_create_object_parameters does not exist."))?
                .encode(bitstream)?,
            3 | 34 | 45 => self.m_object.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                .encode(bitstream)?,
            4 | 12 | 13 | 22 => {
                self.m_object.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
                self.m_player_filter_modifier.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_filter_modifier does not exist."))?
                    .encode(bitstream)?;
            }
            5 => self.m_navpoint_set_icon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_icon_parameters does not exist."))?
                .encode(bitstream)?,
            6 => self.m_navpoint_set_priority_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_priority_parameters does not exist."))?
                .encode(bitstream)?,
            7 => self.m_navpoint_set_timer_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_timer_parameters does not exist."))?
                .encode(bitstream)?,
            8 => self.m_navpoint_set_visible_range_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_visible_range_parameters does not exist."))?
                .encode(bitstream)?,
            9 => self.m_set_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_parameters does not exist."))?
                .encode(bitstream)?,
            10 => self.m_set_boundary_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_boundary_parameters does not exist."))?
                .encode(bitstream)?,
            11 => self.m_apply_player_traits_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_apply_player_traits_parameters does not exist."))?
                .encode(bitstream)?,
            14 => self.m_set_fireteam_respawn_filter_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_fireteam_respawn_filter_parameters does not exist."))?
                .encode(bitstream)?,
            15 => self.m_set_progress_bar_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_progress_bar_parameters does not exist."))?
                .encode(bitstream)?,
            16 => self.m_hud_post_message_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_post_message_parameters does not exist."))?
                .encode(bitstream)?,
            17 => self.m_timer_set_rate_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_timer_set_rate_parameters does not exist."))?
                .encode(bitstream)?,
            18 => self.m_string.as_ref()
                .ok_or_else(|| BLFLibError::from("m_string does not exist."))?
                .encode(bitstream)?,
            19 => {
                self.m_object.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
                self.m_player_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_1 does not exist."))?
                    .encode(bitstream)?;
            }
            20 => self.m_for_each_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_for_each_parameters does not exist."))?
                .encode(bitstream)?,
            23 => self.m_object_destroy_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_destroy_parameters does not exist."))?
                .encode(bitstream)?,
            24 | 27 | 28 | 52 | 54 | 55 | 70 | 71 | 72 | 73 | 80 | 82 | 85 => {
                self.m_object.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
                self.m_variable_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_1 does not exist."))?
                    .encode(bitstream)?;
            }
            25 | 95 => {
                self.m_variable_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_variable_2.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_2 does not exist."))?
                    .encode(bitstream)?;
            }
            29 => {
                self.m_player_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_player_2.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_2 does not exist."))?
                    .encode(bitstream)?;
            }
            30 | 31 | 35 | 37 | 62 | 63 | 97 => {
                self.m_player_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_variable_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_1 does not exist."))?
                    .encode(bitstream)?;
            }
            32 => {
                let flag = self.m_tracing_enabled
                    .ok_or_else(|| BLFLibError::from("m_tracing_enabled does not exist."))?;
                bitstream.write_bool(flag)?;
            }
            33 => self.m_object_attach_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_attach_parameters does not exist."))?
                .encode(bitstream)?,
            36 => {
                self.m_team.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_team does not exist."))?
                    .encode(bitstream)?;
                self.m_variable_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_1 does not exist."))?
                    .encode(bitstream)?;
            }
            38 => self.m_player_adjust_money_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_adjust_money_parameters does not exist."))?
                .encode(bitstream)?,
            39 => self.m_player_enable_purchases_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_enable_purchases_parameters does not exist."))?
                .encode(bitstream)?,
            40 | 41 | 42 | 61 | 84 | 86 | 89 => {
                self.m_player_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_object.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
            }
            43 => self.m_timer.as_ref()
                .ok_or_else(|| BLFLibError::from("m_timer does not exist."))?
                .encode(bitstream)?,
            44 => self.m_weapon_set_pickup_priority_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_weapon_set_pickup_priority_parameters does not exist."))?
                .encode(bitstream)?,
            46 | 47 => self.m_hud_widget_text_base.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_widget_text_base does not exist."))?
                .encode(bitstream)?,
            48 => self.m_hud_widget_set_meter_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_widget_set_meter_parameters does not exist."))?
                .encode(bitstream)?,
            49 => self.m_hud_widget_set_icon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_widget_set_icon_parameters does not exist."))?
                .encode(bitstream)?,
            50 => self.m_hud_widget_set_visibility_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_widget_set_visibility_parameters does not exist."))?
                .encode(bitstream)?,
            51 => self.m_play_sound_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_play_sound_parameters does not exist."))?
                .encode(bitstream)?,
            53 => {
                self.m_object.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
                self.m_string.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_string does not exist."))?
                    .encode(bitstream)?;
            }
            56 | 57 => {
                self.m_player_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_string.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_string does not exist."))?
                    .encode(bitstream)?;
            }
            58 => self.m_player_set_objective_allegiance_icon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_objective_allegiance_icon_parameters does not exist."))?
                .encode(bitstream)?,
            59 => self.m_team_set_coop_spawning_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_team_set_coop_spawning_parameters does not exist."))?
                .encode(bitstream)?,
            60 => {
                self.m_team.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_team does not exist."))?
                    .encode(bitstream)?;
                self.m_object.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
            }
            64 | 65 | 67 | 68 => self.m_vitality_adjustment_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_vitality_adjustment_parameters does not exist."))?
                .encode(bitstream)?,
            66 => self.m_object_get_distance_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_get_distance_parameters does not exist."))?
                .encode(bitstream)?,
            69 => self.m_player_set_requisition_palette_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_requisition_palette_parameters does not exist."))?
                .encode(bitstream)?,
            74 => self.m_adjust_grenades_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_adjust_grenades_parameters does not exist."))?
                .encode(bitstream)?,
            75 => self.m_submit_incident_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_submit_incident_parameters does not exist."))?
                .encode(bitstream)?,
            76 => self.m_submit_incident_with_custom_value_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_submit_incident_with_custom_value_parameters does not exist."))?
                .encode(bitstream)?,
            77 => self.m_set_loadout_palette_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_loadout_palette_parameters does not exist."))?
                .encode(bitstream)?,
            78 => self.m_device_set_position_track_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_set_position_track_parameters does not exist."))?
                .encode(bitstream)?,
            79 => self.m_device_animate_position_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_animate_position_parameters does not exist."))?
                .encode(bitstream)?,
            81 => {
                self.m_variable_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_string.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_string does not exist."))?
                    .encode(bitstream)?;
            }
            83 => self.m_player_get_weapon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_get_weapon_parameters does not exist."))?
                .encode(bitstream)?,
            87 => self.m_create_tunnel_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_create_tunnel_parameters does not exist."))?
                .encode(bitstream)?,
            88 => self.m_variable_1.as_ref()
                .ok_or_else(|| BLFLibError::from("m_variable_1 does not exist."))?
                .encode(bitstream)?,
            90 => self.m_player_set_coop_spawning_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_coop_spawning_parameters does not exist."))?
                .encode(bitstream)?,
            91 => self.m_object_set_orientation_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_set_orientation_parameters does not exist."))?
                .encode(bitstream)?,
            92 => self.m_object_face_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_face_object_parameters does not exist."))?
                .encode(bitstream)?,
            93 => self.m_biped_give_weapon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_biped_give_weapon_parameters does not exist."))?
                .encode(bitstream)?,
            94 => self.m_biped_drop_weapon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_biped_drop_weapon_parameters does not exist."))?
                .encode(bitstream)?,
            96 => self.m_get_random_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_get_random_object_parameters does not exist."))?
                .encode(bitstream)?,
            98 => self.m_boundary_set_player_color_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_boundary_set_player_color_parameters does not exist."))?
                .encode(bitstream)?,
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_enum(7)?;

        match self.m_type.clone() as u32 {
            1 => {
                let mut set_score_parameters = s_action_set_score_parameters::default();
                set_score_parameters.decode(bitstream)?;
                self.m_set_score_parameters = Some(set_score_parameters);
            }
            2 => {
                let mut create_object_parameters = s_action_create_object_parameters::default();
                create_object_parameters.decode(bitstream)?;
                self.m_create_object_parameters = Some(create_object_parameters);
            }
            3 | 34 | 45 => {
                let mut object = c_object_reference::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
            }
            4 | 12 | 13 | 22 => {
                let mut object = c_object_reference::default();
                let mut player_filter_modifier = c_player_filter_modifier::default();
                object.decode(bitstream)?;
                player_filter_modifier.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_player_filter_modifier = Some(player_filter_modifier);
            }
            5 => {
                let mut navpoint_set_icon_parameters = s_action_navpoint_set_icon_parameters::default();
                navpoint_set_icon_parameters.decode(bitstream)?;
                self.m_navpoint_set_icon_parameters = Some(navpoint_set_icon_parameters);
            }
            6 => {
                let mut navpoint_set_priority_parameters = s_action_navpoint_set_priority_parameters::default();
                navpoint_set_priority_parameters.decode(bitstream)?;
                self.m_navpoint_set_priority_parameters = Some(navpoint_set_priority_parameters);
            }
            7 => {
                let mut navpoint_set_timer_parameters = s_action_navpoint_set_timer_parameters::default();
                navpoint_set_timer_parameters.decode(bitstream)?;
                self.m_navpoint_set_timer_parameters = Some(navpoint_set_timer_parameters);
            }
            8 => {
                let mut navpoint_set_visible_range_parameters = s_action_navpoint_set_visible_range_parameters::default();
                navpoint_set_visible_range_parameters.decode(bitstream)?;
                self.m_navpoint_set_visible_range_parameters = Some(navpoint_set_visible_range_parameters);
            }
            9 => {
                let mut set_parameters = s_action_set_parameters::default();
                set_parameters.decode(bitstream)?;
                self.m_set_parameters = Some(set_parameters);
            }
            10 => {
                let mut set_boundary_parameters = s_action_set_boundary_parameters::default();
                set_boundary_parameters.decode(bitstream)?;
                self.m_set_boundary_parameters = Some(set_boundary_parameters);
            }
            11 => {
                let mut apply_player_traits_parameters = s_action_apply_player_traits_parameters::default();
                apply_player_traits_parameters.decode(bitstream)?;
                self.m_apply_player_traits_parameters = Some(apply_player_traits_parameters);
            }
            14 => {
                let mut set_fireteam_respawn_filter_parameters = s_action_set_fireteam_respawn_filter_parameters::default();
                set_fireteam_respawn_filter_parameters.decode(bitstream)?;
                self.m_set_fireteam_respawn_filter_parameters = Some(set_fireteam_respawn_filter_parameters);
            }
            15 => {
                let mut set_progress_bar_parameters = s_action_set_progress_bar_parameters::default();
                set_progress_bar_parameters.decode(bitstream)?;
                self.m_set_progress_bar_parameters = Some(set_progress_bar_parameters);
            }
            16 => {
                let mut hud_post_message_parameters = s_action_hud_post_message_parameters::default();
                hud_post_message_parameters.decode(bitstream)?;
                self.m_hud_post_message_parameters = Some(hud_post_message_parameters);
            }
            17 => {
                let mut timer_set_rate_parameters = s_action_timer_set_rate_parameters::default();
                timer_set_rate_parameters.decode(bitstream)?;
                self.m_timer_set_rate_parameters = Some(timer_set_rate_parameters);
            }
            18 => {
                let mut string = c_dynamic_string::default();
                string.decode(bitstream)?;
                self.m_string = Some(string);
            }
            19 => {
                let mut object = c_object_reference::default();
                let mut player = c_player_reference::default();
                object.decode(bitstream)?;
                player.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_player_1 = Some(player);
            }
            20 => {
                let mut for_each_parameters = s_action_for_each_parameters::default();
                for_each_parameters.decode(bitstream)?;
                self.m_for_each_parameters = Some(for_each_parameters);
            }
            23 => {
                let mut object_destroy_parameters = s_action_object_destroy_parameters::default();
                object_destroy_parameters.decode(bitstream)?;
                self.m_object_destroy_parameters = Some(object_destroy_parameters);
            }
            24 | 27 | 28 | 52 | 54 | 55 | 70 | 71 | 72 | 73 | 80 | 82 | 85 => {
                let mut object = c_object_reference::default();
                let mut variable = c_custom_variable_reference::default();
                object.decode(bitstream)?;
                variable.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_1 = Some(variable);
            }
            25 | 95 => {
                let mut variable1 = c_custom_variable_reference::default();
                let mut variable2 = c_custom_variable_reference::default();
                variable1.decode(bitstream)?;
                variable2.decode(bitstream)?;
                self.m_variable_1 = Some(variable1);
                self.m_variable_2 = Some(variable2);
            }
            29 => {
                let mut player1 = c_player_reference::default();
                let mut player2 = c_player_reference::default();
                player1.decode(bitstream)?;
                player2.decode(bitstream)?;
                self.m_player_1 = Some(player1);
                self.m_player_2 = Some(player2);
            }
            30 | 31 | 35 | 37 | 62 | 63 | 97 => {
                let mut player1 = c_player_reference::default();
                let mut variable1 = c_custom_variable_reference::default();
                player1.decode(bitstream)?;
                variable1.decode(bitstream)?;
                self.m_player_1 = Some(player1);
                self.m_variable_1 = Some(variable1);
            }
            32 => {
                self.m_tracing_enabled = Some(bitstream.read_bool("tracing-enabled")?);
            }
            33 => {
                let mut object_attach_parameters = s_action_object_attach_parameters::default();
                object_attach_parameters.decode(bitstream)?;
                self.m_object_attach_parameters = Some(object_attach_parameters);
            }
            36 => {
                let mut team = c_team_reference::default();
                let mut variable = c_custom_variable_reference::default();
                team.decode(bitstream)?;
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_variable_1 = Some(variable);
            }
            38 => {
                let mut player_adjust_money_parameters = s_action_player_adjust_money_parameters::default();
                player_adjust_money_parameters.decode(bitstream)?;
                self.m_player_adjust_money_parameters = Some(player_adjust_money_parameters);
            }
            39 => {
                let mut player_enable_purchases_parameters = s_action_player_enable_purchases_parameters::default();
                player_enable_purchases_parameters.decode(bitstream)?;
                self.m_player_enable_purchases_parameters = Some(player_enable_purchases_parameters);
            }
            40 | 41 | 42 | 61 | 84 | 86 | 89 => {
                let mut player = c_player_reference::default();
                let mut object = c_object_reference::default();
                object.decode(bitstream)?;
                object.decode(bitstream)?;
                self.m_player_1 = Some(player);
                self.m_object = Some(object);
            }
            43 => {
                let mut timer = c_custom_timer_reference::default();
                timer.decode(bitstream)?;
                self.m_timer = Some(timer);
            }
            44 => {
                let mut weapon_set_pickup_priority_parameters = s_action_weapon_set_pickup_priority_parameters::default();
                weapon_set_pickup_priority_parameters.decode(bitstream)?;
                self.m_weapon_set_pickup_priority_parameters = Some(weapon_set_pickup_priority_parameters);
            }
            46 | 47 => {
                let mut hud_widget_text_base = s_action_hud_widget_text_base::default();
                hud_widget_text_base.decode(bitstream)?;
                self.m_hud_widget_text_base = Some(hud_widget_text_base);
            }
            48 => {
                let mut hud_widget_set_meter_parameters = s_action_hud_widget_set_meter_parameters::default();
                hud_widget_set_meter_parameters.decode(bitstream)?;
                self.m_hud_widget_set_meter_parameters = Some(hud_widget_set_meter_parameters);
            }
            49 => {
                let mut hud_widget_set_icon_parameters = s_action_hud_widget_set_icon_parameters::default();
                hud_widget_set_icon_parameters.decode(bitstream)?;
                self.m_hud_widget_set_icon_parameters = Some(hud_widget_set_icon_parameters);
            }
            50 => {
                let mut hud_widget_set_visibility_parameters = s_action_hud_widget_set_visibility_parameters::default();
                hud_widget_set_visibility_parameters.decode(bitstream)?;
                self.m_hud_widget_set_visibility_parameters = Some(hud_widget_set_visibility_parameters);
            }
            51 => {
                let mut play_sound_parameters = s_action_play_sound_parameters::default();
                play_sound_parameters.decode(bitstream)?;
                self.m_play_sound_parameters = Some(play_sound_parameters);
            }
            53 => {
                let mut object = c_object_reference::default();
                let mut string = c_dynamic_string::default();
                string.decode(bitstream)?;
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_string = Some(string);
            }
            56 | 57 => {
                let mut player = c_player_reference::default();
                let mut string = c_dynamic_string::default();
                player.decode(bitstream)?;
                string.decode(bitstream)?;
                self.m_player_1 = Some(player);
                self.m_string = Some(string);
            }
            58 => {
                let mut player_set_objective_allegiance_icon_parameters = s_action_player_set_objective_allegiance_icon_parameters::default();
                player_set_objective_allegiance_icon_parameters.decode(bitstream)?;
                self.m_player_set_objective_allegiance_icon_parameters = Some(player_set_objective_allegiance_icon_parameters);
            }
            59 => {
                let mut team_set_coop_spawning_parameters = s_action_team_set_coop_spawning_parameters::default();
                team_set_coop_spawning_parameters.decode(bitstream)?;
                self.m_team_set_coop_spawning_parameters = Some(team_set_coop_spawning_parameters);
            }
            60 => {
                let mut team = c_team_reference::default();
                let mut object = c_object_reference::default();
                team.decode(bitstream)?;
                object.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_object = Some(object);
            }
            64 | 65 | 67 | 68 => {
                let mut vitality_adjustment_parameters = s_action_vitality_adjustment_parameters::default();
                vitality_adjustment_parameters.decode(bitstream)?;
                self.m_vitality_adjustment_parameters = Some(vitality_adjustment_parameters);
            }
            66 => {
                let mut object_get_distance_parameters = s_action_object_get_distance_parameters::default();
                object_get_distance_parameters.decode(bitstream)?;
                self.m_object_get_distance_parameters = Some(object_get_distance_parameters);
            }
            69 => {
                let mut player_set_requisition_palette_parameters = s_action_player_set_requisition_palette_parameters::default();
                player_set_requisition_palette_parameters.decode(bitstream)?;
                self.m_player_set_requisition_palette_parameters = Some(player_set_requisition_palette_parameters);
            }
            74 => {
                let mut adjust_grenades_parameters = s_action_adjust_grenades_parameters::default();
                adjust_grenades_parameters.decode(bitstream)?;
                self.m_adjust_grenades_parameters = Some(adjust_grenades_parameters);
            }
            75 => {
                let mut submit_incident_parameters = s_action_submit_incident_parameters::default();
                submit_incident_parameters.decode(bitstream)?;
                self.m_submit_incident_parameters = Some(submit_incident_parameters);
            }
            76 => {
                let mut submit_incident_with_custom_value_parameters = s_action_submit_incident_with_custom_value_parameters::default();
                submit_incident_with_custom_value_parameters.decode(bitstream)?;
                self.m_submit_incident_with_custom_value_parameters = Some(submit_incident_with_custom_value_parameters);
            }
            77 => {
                let mut set_loadout_palette_parameters = s_action_set_loadout_palette_parameters::default();
                set_loadout_palette_parameters.decode(bitstream)?;
                self.m_set_loadout_palette_parameters = Some(set_loadout_palette_parameters);
            }
            78 => {
                let mut device_set_position_track_parameters = s_action_device_set_position_track_parameters::default();
                device_set_position_track_parameters.decode(bitstream)?;
                self.m_device_set_position_track_parameters = Some(device_set_position_track_parameters);
            }
            79 => {
                let mut device_animate_position_parameters = s_action_device_animate_position_parameters::default();
                device_animate_position_parameters.decode(bitstream)?;
                self.m_device_animate_position_parameters = Some(device_animate_position_parameters);
            }
            81 => {
                let mut variable = c_custom_variable_reference::default();
                let mut string = c_dynamic_string::default();
                variable.decode(bitstream)?;
                string.decode(bitstream)?;
                self.m_variable_1 = Some(variable);
                self.m_string = Some(string);
            }
            83 => {
                let mut player_get_weapon_parameters = s_action_player_get_weapon_parameters::default();
                player_get_weapon_parameters.decode(bitstream)?;
                self.m_player_get_weapon_parameters = Some(player_get_weapon_parameters);
            }
            87 => {
                let mut create_tunnel_parameters = s_action_create_tunnel_parameters::default();
                create_tunnel_parameters.decode(bitstream)?;
                self.m_create_tunnel_parameters = Some(create_tunnel_parameters);
            }
            88 => {
                let mut variable = c_custom_variable_reference::default();
                variable.decode(bitstream)?;
                self.m_variable_1 = Some(variable);
            }
            90 => {
                let mut player_set_coop_spawning_parameters = s_action_player_set_coop_spawning_parameters::default();
                player_set_coop_spawning_parameters.decode(bitstream)?;
                self.m_player_set_coop_spawning_parameters = Some(player_set_coop_spawning_parameters);
            }
            91 => {
                let mut object_set_orientation_parameters = s_action_object_set_orientation_parameters::default();
                object_set_orientation_parameters.decode(bitstream)?;
                self.m_object_set_orientation_parameters = Some(object_set_orientation_parameters);
            }
            92 => {
                let mut object_face_object_parameters = s_action_object_face_object_parameters::default();
                object_face_object_parameters.decode(bitstream)?;
                self.m_object_face_object_parameters = Some(object_face_object_parameters);
            }
            93 => {
                let mut biped_give_weapon_parameters = s_action_biped_give_weapon_parameters::default();
                biped_give_weapon_parameters.decode(bitstream)?;
                self.m_biped_give_weapon_parameters = Some(biped_give_weapon_parameters);
            }
            94 => {
                let mut biped_drop_weapon_parameters = s_action_biped_drop_weapon_parameters::default();
                biped_drop_weapon_parameters.decode(bitstream)?;
                self.m_biped_drop_weapon_parameters = Some(biped_drop_weapon_parameters);
            }
            96 => {
                let mut get_random_object_parameters = s_action_get_random_object_parameters::default();
                get_random_object_parameters.decode(bitstream)?;
                self.m_get_random_object_parameters = Some(get_random_object_parameters);
            }
            98 => {
                let mut boundary_set_player_color_parameters = s_action_boundary_set_player_color_parameters::default();
                boundary_set_player_color_parameters.decode(bitstream)?;
                self.m_boundary_set_player_color_parameters = Some(boundary_set_player_color_parameters);
            }
            _ => {}
        }

        Ok(())
    }
}