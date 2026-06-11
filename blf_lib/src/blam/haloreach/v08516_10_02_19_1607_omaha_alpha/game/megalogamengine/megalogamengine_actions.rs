use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::memory::bitstream_reader::c_bitstream_reader_extensions;
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::memory::bitstream_writer::c_bitstream_writer_extensions;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_custom_timer_reference::c_custom_timer_reference;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_object_type_reference::c_object_type_reference;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_player_reference::c_player_reference;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_team_reference::c_team_reference;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_text::c_dynamic_string;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_variant_variable::s_variant_variable;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::scenario_map_variant::e_boundary_shape;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_custom_variable_reference::c_custom_variable_reference;
use blf_lib::blam::haloreach::v08516_10_02_19_1607_omaha_alpha::game::megalogamengine::megalogamengine_object_reference::c_object_reference;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_megalo::e_weapon_pickup_priority;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_timer::e_game_engine_timer_rate;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_hud_widgets::e_megalogamengine_hud_meter_input_type;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_variant_icon::e_game_variant_icon;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_actions::{
    e_action_team_or_player_target, e_biped_give_weapon_mode, e_chud_navpoint_icon_type,
    e_grenade_type, e_navpoint_priority, e_player_filter_type,
};

/// Alpha encodes math operations in 3 bits (retail uses 4 bits with three additional operators).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive, Default, Serialize, Deserialize)]
pub enum e_math_operation {
    #[default]
    add = 0, // +=
    subtract = 1, // -=
    multiply_by = 2, // *=
    divide_by = 3, // /=
    set_to = 4, // =
    modulo_by = 5, // %=
    bitwise_and_with = 6, // &=
    bitwise_or_with = 7, // |=
    // -- not in alpha --
    // bitwise_xor_with = 8,
    // bitwise_not_with = 9,
    // set_to_absolute = 10,
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
        bitstream.write_enum(self.m_target, 2)?;
        match (self.m_target, &self.m_team, &self.m_player) {
            (e_action_team_or_player_target::team, Some(team), None) => {
                team.encode(bitstream)?;
            }
            (e_action_team_or_player_target::player, None, Some(player)) => {
                player.encode(bitstream)?;
            }
            (e_action_team_or_player_target::all_players, None, None) => {}
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_target = bitstream.read_enum("target", 2)?;
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
            e_action_team_or_player_target::all_players => {}
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_set_score_parameters {
    pub m_target: s_team_or_player_target,
    pub m_operation: e_math_operation, // 3 bits
    pub m_variable: c_custom_variable_reference
}

impl s_action_set_score_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_target.encode(bitstream)?;
        bitstream.write_enum(self.m_operation, 3)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_target.decode(bitstream)?;
        self.m_operation = bitstream.read_enum("operation", 3)?;
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
    pub m_flags: u8, // 1 bit
}

impl s_action_create_object_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object_type.encode(bitstream)?;
        self.m_object_reference_1.encode(bitstream)?;
        self.m_object_reference_2.encode(bitstream)?;
        bitstream.write_index::<16>(self.m_filter_index, 4)?;
        bitstream.write_integer(self.m_flags, 1)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object_type.decode(bitstream)?;
        self.m_object_reference_1.decode(bitstream)?;
        self.m_object_reference_2.decode(bitstream)?;
        self.m_filter_index = bitstream.read_index::<16>("filter_index", 4)? as i8;
        self.m_flags = bitstream.read_integer("flags", 1)?;

        Ok(())
    }
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
        bitstream.write_enum(self.m_type, 3)?;
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
        self.m_type = bitstream.read_enum("type", 3)?;
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
        let icon = self.m_navpoint_icon.to_i8()
            .ok_or_else(|| BLFLibError::from("Failed to convert navpoint icon to integer."))?;
        bitstream.write_integer((icon as i32 + 1) as u32, 5)?;

        if self.m_navpoint_icon == e_chud_navpoint_icon_type::num {
            if let Some(navpoint_number) = &self.m_navpoint_number {
                navpoint_number.encode(bitstream)?;
            }
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        let wire_icon: u32 = bitstream.read_integer("navpoint-icon", 5)?;
        self.m_navpoint_icon = FromPrimitive::from_i8((wire_icon as i32 - 1) as i8)
            .ok_or_else(|| BLFLibError::from(format!("Unexpected navpoint icon wire value: {wire_icon}")))?;

        if self.m_navpoint_icon == e_chud_navpoint_icon_type::num {
            let mut navpoint_number = c_custom_variable_reference::default();
            navpoint_number.decode(bitstream)?;
            self.m_navpoint_number = Some(navpoint_number);
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_navpoint_set_priority_parameters {
    pub m_object: c_object_reference,
    pub m_priority: e_navpoint_priority, // 2 bits
}

impl s_action_navpoint_set_priority_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum(self.m_priority, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_priority = bitstream.read_enum("priority", 2)?;

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
    pub m_operation: e_math_operation, // 3 bits
}

impl s_action_set_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_variable_1.encode(bitstream)?;
        self.m_variable_2.encode(bitstream)?;
        bitstream.write_enum(self.m_operation, 3)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_variable_1.decode(bitstream)?;
        self.m_variable_2.decode(bitstream)?;
        self.m_operation = bitstream.read_enum("operation", 3)?;

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
        self.m_shape = bitstream.read_unnamed_enum( 2)?;

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
    pub m_team_filter: u8, // 2 bits
    pub m_timer_index: i8, // 2 bits
}

impl s_action_set_progress_bar_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_integer(self.m_team_filter, 2)?;
        if self.m_team_filter != 0 {
            bitstream.write_index::<4>(self.m_timer_index, 2)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_team_filter = bitstream.read_integer("team-filter", 2)?;
        if self.m_team_filter != 0 {
            self.m_timer_index = bitstream.read_index::<4>("timer-index", 2)? as i8;
        }

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
        self.m_sound_index = bitstream.read_integer("sound-index", 7)?;
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
        bitstream.write_enum(self.m_rate, 5)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_timer.decode(bitstream)?;
        self.m_rate = bitstream.read_enum("timer-rate", 5)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_for_each_parameters {
    pub m_trigger_index: u16, // 8 bits
}

impl s_action_for_each_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_trigger_index, 8)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_trigger_index = bitstream.read_integer("trigger-index", 8)?;

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
}

impl s_action_object_attach_parameters {
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
pub struct s_action_player_adjust_money_parameters {
    pub m_player: c_player_reference,
    pub m_math_operation: e_math_operation, // 3 bits
    pub m_variable: c_custom_variable_reference,
}

impl s_action_player_adjust_money_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_enum(self.m_math_operation, 3)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_math_operation = bitstream.read_enum("math-operation", 3)?;
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
        bitstream.write_enum(self.m_weapon_pickup_priority, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_weapon_pickup_priority = bitstream.read_enum("weapon-pickup-priority", 2)?;

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
                bitstream.write_enum(e_megalogamengine_hud_meter_input_type::number, 2)?;
                variable1.encode(bitstream)?;
                variable2.encode(bitstream)?;
            }
            (e_megalogamengine_hud_meter_input_type::timer, None, None, Some(timer)) => {
                bitstream.write_enum(e_megalogamengine_hud_meter_input_type::timer, 2)?;
                timer.encode(bitstream)?;
            }
            (e_megalogamengine_hud_meter_input_type::none, _, _, _) => {}
            _ => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_enum("type", 2)?;
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
    pub m_icon_index: e_game_variant_icon, // 6 bits
}

impl s_action_hud_widget_set_icon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<4>(self.m_widget_index, 2)?;
        bitstream.write_index::<64>(i8::from(self.m_icon_index), 6)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_widget_index = bitstream.read_index::<4>("widget-index", 2)? as i8;
        self.m_icon_index = e_game_variant_icon::from_i8(
            bitstream.read_index::<64>("icon-index", 6)? as i8,
        )?;

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
    pub m_target: s_team_or_player_target,
}

impl s_action_play_sound_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_sound_index, 7)?;
        self.m_target.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_sound_index = bitstream.read_integer("sound-index", 7)?;
        self.m_target.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_player_set_objective_allegiance_icon_parameters {
    pub m_player: c_player_reference,
    pub m_icon_index: e_game_variant_icon, // 7 bits
}

impl s_action_player_set_objective_allegiance_icon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_index::<128>(i8::from(self.m_icon_index), 7)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_icon_index = e_game_variant_icon::from_i8(
            bitstream.read_index::<128>("icon-index", 7)? as i8,
        )?;

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
    pub m_operation: e_math_operation, // 3 bits
    pub m_variable: c_custom_variable_reference,
}

impl s_action_vitality_adjustment_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_enum(self.m_operation, 3)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_operation = bitstream.read_enum("operation", 3)?;
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
    pub m_math_operation: e_math_operation, // 3 bits
    pub m_variable: c_custom_variable_reference,
}

impl s_action_adjust_grenades_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_player.encode(bitstream)?;
        bitstream.write_enum(self.m_grenade_type, 1)?;
        bitstream.write_enum(self.m_math_operation, 3)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_player.decode(bitstream)?;
        self.m_grenade_type = bitstream.read_enum("grenade-type", 1)?;
        self.m_math_operation = bitstream.read_enum("math-operation", 3)?;
        self.m_variable.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_submit_incident_parameters {
    pub m_incident_id: i8, // 8 bits
    pub m_target_1: s_team_or_player_target,
    pub m_target_2: s_team_or_player_target,
}

impl s_action_submit_incident_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<255>(self.m_incident_id, 8)?;
        self.m_target_1.encode(bitstream)?;
        self.m_target_2.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_incident_id = bitstream.read_index::<255>("incident-id", 8)? as i8;
        self.m_target_1.decode(bitstream)?;
        self.m_target_2.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_action_submit_incident_with_custom_value_parameters {
    pub m_incident_id: i8, // 8 bits
    pub m_target_1: s_team_or_player_target,
    pub m_target_2: s_team_or_player_target,
    pub m_variable: c_custom_variable_reference,
}

impl s_action_submit_incident_with_custom_value_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_index::<255>(self.m_incident_id, 8)?;
        self.m_target_1.encode(bitstream)?;
        self.m_target_2.encode(bitstream)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_incident_id = bitstream.read_index::<255>("incident-id", 8)? as i8;
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
    pub m_animation_name_index: i16, // 8 bits
    pub m_variable: c_custom_variable_reference,
}

impl s_action_device_set_position_track_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        bitstream.write_index::<255>(self.m_animation_name_index, 8)?;
        self.m_variable.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_animation_name_index = bitstream.read_index::<255>("animation-name-index", 8)? as i16;
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
    pub m_mode: e_biped_give_weapon_mode, // 2 bits
}

impl s_action_biped_give_weapon_parameters {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        self.m_object.encode(bitstream)?;
        self.m_object_type.encode(bitstream)?;
        bitstream.write_enum(self.m_mode, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_object.decode(bitstream)?;
        self.m_object_type.decode(bitstream)?;
        self.m_mode = bitstream.read_enum("mode", 2)?;

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
    timer_reset = 42,
    weapon_set_pickup_priority = 43,
    object_bounce = 44,
    hud_widget_set_text = 45,
    hud_widget_set_value = 46,
    hud_widget_set_meter = 47,
    hud_widget_set_icon = 48,
    hud_widget_set_visibility = 49,
    play_sound = 50,
    object_set_scale = 51,
    navpoint_set_text = 52,
    object_get_shield = 53,
    player_set_objective = 54,
    team_set_coop_spawning = 55,
    object_set_team_filter = 56,
    object_set_role_filter = 57,
    object_set_spawn_priority = 58,
    team_set_primary_respawn_object = 59,
    player_set_primary_respawn_object = 60,
    player_get_fireteam_index = 61,
    player_set_fireteam_index = 62,
    object_adjust_shield = 63,
    object_adjust_health = 64,
    object_get_distance = 65,
    object_adjust_maximum_shield = 66,
    object_adjust_maximum_health = 67,
    player_set_requisition_palette = 68,
    player_set_fireteam_tier = 69,
    device_set_power = 70,
    device_get_power = 71,
    device_set_position = 72,
    device_get_position = 73,
    player_give_weapon = 74,
    adjust_grenades = 75,
    submit_incident = 76,
    submit_incident_with_custom_value = 77,
    set_loadout = 78,
    set_loadout_palette = 79,
    device_set_position_track = 80,
    device_animate_position = 81,
    device_set_position_immediate = 82,
    saved_film_insert_marker = 83,
    respawn_zone_enable = 84,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_action {
    pub m_type: e_action_type, // 7 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_set_score_parameters: Option<s_action_set_score_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_create_object_parameters: Option<s_action_create_object_parameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_1: Option<c_object_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_2: Option<c_object_reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object_type: Option<c_object_type_reference>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_target: Option<s_team_or_player_target>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_unknown_data: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_unknown_float: Option<f32>,
}

impl c_action {
    pub fn encode(&self, mut bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_type.clone(), 7)?;

        match self.m_type {
            e_action_type::set_score => self.m_set_score_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_set_score_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::create_object => self.m_create_object_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_create_object_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::delete_object
                | e_action_type::object_destroy
                | e_action_type::object_detach
                | e_action_type::object_bounce
            => self.m_object_1.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                .encode(bitstream)?,
            e_action_type::navpoint_set_visible
                | e_action_type::navpoint_set_priority
                | e_action_type::set_pickup_filter
                | e_action_type::set_respawn_filter
                | e_action_type::weapon_set_pickup_priority
                | e_action_type::object_set_team_filter
                | e_action_type::object_set_role_filter
            => {
                self.m_object_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
                bitstream.write_integer(
                    *self.m_unknown_data.as_ref()
                        .ok_or_else(|| BLFLibError::from("m_unknown_data"))?,
                    2
                )?;
            }
            e_action_type::navpoint_set_icon => self.m_navpoint_set_icon_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_navpoint_set_icon_parameters does not exist."))?
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
            e_action_type::print_variable => self.m_string.as_ref()
                .ok_or_else(|| BLFLibError::from("m_string does not exist."))?
                .encode(bitstream)?,
            e_action_type::get_player_holding_object => {
                self.m_object_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
                self.m_player_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_1 does not exist."))?
                    .encode(bitstream)?;
            }
            e_action_type::for_each => self.m_for_each_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_for_each_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::boundary_set_visible
                | e_action_type::object_set_invincibility
                | e_action_type::object_get_orientation
                | e_action_type::object_get_velocity
                | e_action_type::device_set_power
                | e_action_type::device_get_power
                | e_action_type::device_set_position
                | e_action_type::device_get_position
                | e_action_type::device_set_position_immediate
                | e_action_type::respawn_zone_enable
                | e_action_type::object_get_shield
            => {
                self.m_object_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
                self.m_variable_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_1 does not exist."))?
                    .encode(bitstream)?;
            }
            e_action_type::random => {
                self.m_variable_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_variable_2.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_2 does not exist."))?
                    .encode(bitstream)?;
            }
            e_action_type::player_death_get_killing_player => {
                self.m_player_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_player_2.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_2 does not exist."))?
                    .encode(bitstream)?;
            }
            e_action_type::player_death_get_damage_type
                | e_action_type::player_death_get_special_type
                | e_action_type::player_get_place
                | e_action_type::player_get_killing_spree_count
                | e_action_type::player_get_fireteam_index
                | e_action_type::player_set_fireteam_index
                | e_action_type::player_set_fireteam_tier => {
                self.m_player_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_variable_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_1 does not exist."))?
                    .encode(bitstream)?;
            }
            e_action_type::debugging_enable_tracing => {
                let flag = self.m_tracing_enabled
                    .ok_or_else(|| BLFLibError::from("m_tracing_enabled does not exist."))?;
                bitstream.write_bool(flag)?;
            }
            e_action_type::object_attach => self.m_object_attach_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_attach_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::team_get_place => {
                self.m_team.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_team does not exist."))?
                    .encode(bitstream)?;
                self.m_variable_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_1 does not exist."))?
                    .encode(bitstream)?;
            }
            e_action_type::player_adjust_money => self.m_player_adjust_money_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_adjust_money_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_enable_purchases => self.m_player_enable_purchases_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_enable_purchases_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_get_vehicle
                | e_action_type::player_set_vehicle
                | e_action_type::player_set_primary_respawn_object
            => {
                self.m_player_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_object_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
            }
            e_action_type::timer_reset => self.m_timer.as_ref()
                .ok_or_else(|| BLFLibError::from("m_timer does not exist."))?
                .encode(bitstream)?,
            e_action_type::hud_widget_set_text
                | e_action_type::hud_widget_set_value
            => self.m_hud_widget_text_base.as_ref()
                .ok_or_else(|| BLFLibError::from("m_hud_widget_text_base does not exist."))?
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
            e_action_type::object_set_scale => {
                self.m_object_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object_1 does not exist."))?
                    .encode(bitstream)?;
                bitstream.write_quantized_real(
                    *self.m_unknown_float.as_ref()
                        .ok_or_else(|| BLFLibError::from("m_unknown_float does not exist."))?,
                    0.0f32,
                    5.0f32,
                    7,
                    true
                )?
            }
            e_action_type::navpoint_set_text => {
                self.m_object_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
                self.m_string.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_string does not exist."))?
                    .encode(bitstream)?;
            }
            e_action_type::player_set_objective => {
                self.m_player_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_player_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_string.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_string does not exist."))?
                    .encode(bitstream)?;
            }
            e_action_type::team_set_coop_spawning => self.m_team_set_coop_spawning_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_team_set_coop_spawning_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_set_spawn_priority => {
                self.m_object_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
                bitstream.write_integer(
                    *self.m_unknown_data.as_ref()
                        .ok_or_else(|| BLFLibError::from("m_unknown_data does not exist."))?,
                    4
                )?;
            }
            e_action_type::team_set_primary_respawn_object => {
                self.m_team.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_team does not exist."))?
                    .encode(bitstream)?;
                self.m_object_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object does not exist."))?
                    .encode(bitstream)?;
            }
            e_action_type::object_adjust_shield
                | e_action_type::object_adjust_health
                | e_action_type::object_adjust_maximum_shield
                | e_action_type::object_adjust_maximum_health
            => self.m_vitality_adjustment_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_vitality_adjustment_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::object_get_distance => self.m_object_get_distance_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_object_get_distance_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_set_requisition_palette => self.m_player_set_requisition_palette_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_player_set_requisition_palette_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::player_give_weapon => {
                self.m_object_type.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object_type does not exist."))?
                    .encode(bitstream)?;
                self.m_player_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_object_type does not exist."))?
                    .encode(bitstream)?;
                bitstream.write_integer(
                    self.m_unknown_data
                        .ok_or_else(|| BLFLibError::from("m_unknown_data does not exist."))?,
                    1
                )?;
            }
            e_action_type::adjust_grenades => self.m_adjust_grenades_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_adjust_grenades_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::submit_incident => self.m_submit_incident_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_submit_incident_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::submit_incident_with_custom_value => self.m_submit_incident_with_custom_value_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_submit_incident_with_custom_value_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::set_loadout
                | e_action_type::set_loadout_palette
            => {
                self.m_target.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_target does not exist."))?
                    .encode(bitstream)?;
                bitstream.write_integer(
                    self.m_unknown_data
                        .ok_or_else(|| BLFLibError::from("m_unknown_data does not exist."))?,
                    8
                )?;
            },
            e_action_type::device_set_position_track => self.m_device_set_position_track_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_animate_position_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::device_animate_position => self.m_device_animate_position_parameters.as_ref()
                .ok_or_else(|| BLFLibError::from("m_device_animate_position_parameters does not exist."))?
                .encode(bitstream)?,
            e_action_type::saved_film_insert_marker => {
                self.m_variable_1.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_variable_1 does not exist."))?
                    .encode(bitstream)?;
                self.m_string.as_ref()
                    .ok_or_else(|| BLFLibError::from("m_string does not exist."))?
                    .encode(bitstream)?;
            }
            e_action_type::none => {}
            e_action_type::end_round => {}
            e_action_type::break_into_debugger => {}
        }

        Ok(())
    }

    pub fn decode(&mut self, mut bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let action_type = bitstream.read_integer("action-type", 7)?;

        if let Some(action_type) = FromPrimitive::from_u32(action_type) {
            self.m_type = action_type;
        }
        else {
            return Err(format!("unsupported action type: {}", action_type).into())
        }

        self.decode_payload(bitstream)
    }

    pub fn decode_payload(&mut self, mut bitstream: &mut c_bitstream_reader) -> BLFLibResult {
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
            e_action_type::delete_object
                | e_action_type::object_destroy
                | e_action_type::object_detach
                | e_action_type::object_bounce
            => {
                let mut object = c_object_reference::default();
                object.decode(bitstream)?;
                self.m_object_1 = Some(object);
            }
            e_action_type::navpoint_set_visible
                | e_action_type::navpoint_set_priority
                | e_action_type::set_pickup_filter
                | e_action_type::set_respawn_filter
                | e_action_type::weapon_set_pickup_priority
                | e_action_type::object_set_team_filter
                | e_action_type::object_set_role_filter
            => {
                let mut object = c_object_reference::default();
                object.decode(bitstream)?;
                self.m_object_1 = Some(object);
                self.m_unknown_data = Some(bitstream.read_integer("unknown", 2)?);
            }
            e_action_type::navpoint_set_icon => {
                let mut navpoint_set_icon_parameters = s_action_navpoint_set_icon_parameters::default();
                navpoint_set_icon_parameters.decode(bitstream)?;
                self.m_navpoint_set_icon_parameters = Some(navpoint_set_icon_parameters);
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
                let mut string = c_dynamic_string::default();
                string.decode(bitstream)?;
                self.m_string = Some(string);
            }
            e_action_type::get_player_holding_object => {
                let mut object = c_object_reference::default();
                let mut player = c_player_reference::default();
                object.decode(bitstream)?;
                player.decode(bitstream)?;
                self.m_object_1 = Some(object);
                self.m_player_1 = Some(player);
            }
            e_action_type::for_each => {
                let mut for_each_parameters = s_action_for_each_parameters::default();
                for_each_parameters.decode(bitstream)?;
                self.m_for_each_parameters = Some(for_each_parameters);
            }
            e_action_type::boundary_set_visible
                | e_action_type::object_set_invincibility
                | e_action_type::object_get_orientation
                | e_action_type::object_get_velocity
                | e_action_type::object_get_shield
                | e_action_type::device_set_power
                | e_action_type::device_get_power
                | e_action_type::device_set_position
                | e_action_type::device_get_position
                | e_action_type::device_set_position_immediate
                | e_action_type::respawn_zone_enable
            => {
                let mut object = c_object_reference::default();
                let mut variable = c_custom_variable_reference::default();
                object.decode(bitstream)?;
                variable.decode(bitstream)?;
                self.m_object_1 = Some(object);
                self.m_variable_1 = Some(variable);
            }
            e_action_type::random => {
                let mut variable1 = c_custom_variable_reference::default();
                let mut variable2 = c_custom_variable_reference::default();
                variable1.decode(bitstream)?;
                variable2.decode(bitstream)?;
                self.m_variable_1 = Some(variable1);
                self.m_variable_2 = Some(variable2);
            }
            e_action_type::player_death_get_killing_player => {
                let mut player1 = c_player_reference::default();
                let mut player2 = c_player_reference::default();
                player1.decode(bitstream)?;
                player2.decode(bitstream)?;
                self.m_player_1 = Some(player1);
                self.m_player_2 = Some(player2);
            }
            e_action_type::player_death_get_damage_type
                | e_action_type::player_death_get_special_type
                | e_action_type::player_get_place
                | e_action_type::player_get_killing_spree_count
                | e_action_type::player_get_fireteam_index
                | e_action_type::player_set_fireteam_index
                | e_action_type::player_set_fireteam_tier
            => {
                let mut player1 = c_player_reference::default();
                let mut variable1 = c_custom_variable_reference::default();
                player1.decode(bitstream)?;
                variable1.decode(bitstream)?;
                self.m_player_1 = Some(player1);
                self.m_variable_1 = Some(variable1);
            }
            e_action_type::debugging_enable_tracing => {
                self.m_tracing_enabled = Some(bitstream.read_bool("tracing-enabled")?);
            }
            e_action_type::object_attach => {
                let mut object_attach_parameters = s_action_object_attach_parameters::default();
                object_attach_parameters.decode(bitstream)?;
                self.m_object_attach_parameters = Some(object_attach_parameters);
            }
            e_action_type::team_get_place => {
                let mut team = c_team_reference::default();
                let mut variable = c_custom_variable_reference::default();
                team.decode(bitstream)?;
                variable.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_variable_1 = Some(variable);
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
            e_action_type::player_get_vehicle
                | e_action_type::player_set_vehicle
                | e_action_type::player_set_primary_respawn_object
            => {
                let mut player = c_player_reference::default();
                let mut object = c_object_reference::default();
                player.decode(bitstream)?;
                object.decode(bitstream)?;
                self.m_player_1 = Some(player);
                self.m_object_1 = Some(object);
            }
            e_action_type::timer_reset => {
                let mut timer = c_custom_timer_reference::default();
                timer.decode(bitstream)?;
                self.m_timer = Some(timer);
            }
            e_action_type::hud_widget_set_text
                | e_action_type::hud_widget_set_value
            => {
                let mut hud_widget_text_base = s_action_hud_widget_text_base::default();
                hud_widget_text_base.decode(bitstream)?;
                self.m_hud_widget_text_base = Some(hud_widget_text_base);
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
                let mut object = c_object_reference::default();
                object.decode(bitstream)?;
                self.m_object_1 = Some(object);
                self.m_unknown_float = Some(
                    *bitstream.read_quantized_real(0.0f32, 5.0f32, 7, true)?
                )
            }
            e_action_type::navpoint_set_text => {
                let mut object = c_object_reference::default();
                let mut string = c_dynamic_string::default();
                object.decode(bitstream)?;
                string.decode(bitstream)?;
                self.m_object_1 = Some(object);
                self.m_string = Some(string);
            }
            e_action_type::player_set_objective => {
                let mut player = c_player_reference::default();
                let mut string = c_dynamic_string::default();
                player.decode(bitstream)?;
                string.decode(bitstream)?;
                self.m_player_1 = Some(player);
                self.m_string = Some(string);
            }
            e_action_type::team_set_coop_spawning => {
                let mut team_set_coop_spawning_parameters = s_action_team_set_coop_spawning_parameters::default();
                team_set_coop_spawning_parameters.decode(bitstream)?;
                self.m_team_set_coop_spawning_parameters = Some(team_set_coop_spawning_parameters);
            }
            e_action_type::object_set_spawn_priority => {
                let mut object = c_object_reference::default();
                object.decode(bitstream)?;
                self.m_object_1 = Some(object);
                self.m_unknown_data = Some(
                    bitstream.read_integer(
                        "icon",
                        4
                    )?
                );
            }
            e_action_type::team_set_primary_respawn_object => {
                let mut team = c_team_reference::default();
                let mut object = c_object_reference::default();
                team.decode(bitstream)?;
                object.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_object_1 = Some(object);
            }
            e_action_type::object_adjust_shield
                | e_action_type::object_adjust_health
                | e_action_type::object_adjust_maximum_health
                | e_action_type::object_adjust_maximum_shield
            => {
                let mut vitality_adjustment_parameters = s_action_vitality_adjustment_parameters::default();
                vitality_adjustment_parameters.decode(bitstream)?;
                self.m_vitality_adjustment_parameters = Some(vitality_adjustment_parameters);
            }
            e_action_type::object_get_distance => {
                let mut object_get_distance_parameters = s_action_object_get_distance_parameters::default();
                object_get_distance_parameters.decode(bitstream)?;
                self.m_object_get_distance_parameters = Some(object_get_distance_parameters);
            }
            e_action_type::player_set_requisition_palette => {
                let mut player_set_requisition_palette_parameters = s_action_player_set_requisition_palette_parameters::default();
                player_set_requisition_palette_parameters.decode(bitstream)?;
                self.m_player_set_requisition_palette_parameters = Some(player_set_requisition_palette_parameters);
            }
            e_action_type::player_give_weapon => {
                let mut object_type = c_object_type_reference::default();
                let mut player = c_player_reference::default();
                object_type.decode(bitstream)?;
                player.decode(bitstream)?;
                self.m_object_type = Some(object_type);
                self.m_player_1 = Some(player);
                self.m_unknown_data = Some(bitstream.read_integer("unknown-data", 1)?)
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
            e_action_type::set_loadout => {
                let mut target = s_team_or_player_target::default();
                target.decode(bitstream)?;
                self.m_target = Some(target);
                self.m_unknown_data = Some(bitstream.read_integer("loadout-reference-index", 8)?);
            }
            e_action_type::set_loadout_palette => {
                let mut target = s_team_or_player_target::default();
                target.decode(bitstream)?;
                self.m_target = Some(target);
                self.m_unknown_data = Some(bitstream.read_integer("loadout-palette-reference-index", 8)?);
            }
            e_action_type::device_set_position_track => {
                let mut params = s_action_device_set_position_track_parameters::default();
                params.decode(bitstream)?;
                self.m_device_set_position_track_parameters = Some(params);
            }
            e_action_type::device_animate_position => {
                let mut device_animate_position_parameters = s_action_device_animate_position_parameters::default();
                device_animate_position_parameters.decode(bitstream)?;
                self.m_device_animate_position_parameters = Some(device_animate_position_parameters);
            }
            e_action_type::saved_film_insert_marker => {
                let mut variable = c_custom_variable_reference::default();
                let mut string = c_dynamic_string::default();
                variable.decode(bitstream)?;
                string.decode(bitstream)?;
                self.m_variable_1 = Some(variable);
                self.m_string = Some(string);
            }
            e_action_type::none => {}
            e_action_type::end_round => {}
            e_action_type::break_into_debugger => {}
        }

        Ok(())
    }
}