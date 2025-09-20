use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_campaign::c_game_engine_campaign_variant;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_default::c_game_engine_base_variant;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_player_rating_parameters::s_game_engine_player_rating_parameters;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_survival::c_game_engine_survival_variant;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_team::{c_game_engine_team_options_team, k_game_variant_team_count};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_traits::s_player_trait_option;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_actions::c_action;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_conditions::c_condition;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_map_objects::c_object_filter;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_statistics::c_megalo_game_statistic;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_trigger::c_trigger;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_user_defined_options::s_user_defined_option;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_variable_metadata::s_variable_metadata;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::string_table::c_string_table;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::memory::bitstream_reader::c_bitstream_reader_extensions;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::memory::bitstream_writer::c_bitstream_writer_extensions;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::OPTION_TO_RESULT;
use blf_lib::types::numbers::Float32;
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_map_permissions::c_megalogamengine_map_permissions;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::saved_game_files::s_content_item_metadata;
use crate::types::array::StaticArray;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_custom_variant {
    pub m_encoding_version: i32,
    pub m_build_number: i32,
    pub m_base_variant: c_game_engine_base_variant,
    pub m_player_traits: Vec<s_player_trait_option>,
    pub m_user_defined_options: Vec<s_user_defined_option>,
    pub m_script_strings: c_string_table<112, 0x4C00, 15, 15, 7>,
    pub m_base_name_string_index: u8,
    pub m_localized_name: c_string_table<1, 0x180, 9, 9, 1>,
    pub m_localized_description: c_string_table<1, 0xC00, 12, 12, 1>,
    pub m_localized_category: c_string_table<1, 0x180, 9, 9, 1>,
    pub m_engine_icon: u8,
    pub m_engine_category: u8,
    pub m_map_permissions: c_megalogamengine_map_permissions,
    pub m_player_ratings: s_game_engine_player_rating_parameters,
    pub m_score_to_win_round: u16,
    pub m_fire_teams_enabled: bool,
    pub m_symmetric_gametype: bool,
    // These should each be named at some point, but as I'm still researching I'm going to leave them like this.
    pub m_base_variant_parameters_locked: StaticArray<bool, 1280>,
    pub m_base_variant_parameters_hidden: StaticArray<bool, 1280>,
    pub m_user_defined_options_locked: StaticArray<bool, 32>,
    pub m_user_defined_options_hidden: StaticArray<bool, 32>,
    pub m_game_engine: s_custom_game_engine_definition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_au1_settings: Option<c_game_engine_custom_variant_au1_settings>,

}

impl c_game_engine_custom_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_signed_integer(self.m_encoding_version, 32)?;
        bitstream.write_signed_integer(self.m_build_number, 32)?;
        self.m_base_variant.encode(bitstream)?;
        bitstream.write_integer(self.m_player_traits.len() as u16, 5)?;
        for player_trait in self.m_player_traits.iter() {
            player_trait.encode(bitstream)?;
        }
        bitstream.write_integer(self.m_user_defined_options.len() as u16, 5)?;
        for option in self.m_user_defined_options.iter() {
            option.encode(bitstream)?;
        }
        self.m_script_strings.encode(bitstream)?; // this is broken
        bitstream.write_integer(self.m_base_name_string_index, 7)?;
        self.m_localized_name.encode(bitstream)?;
        self.m_localized_description.encode(bitstream)?;
        self.m_localized_category.encode(bitstream)?;
        bitstream.write_integer(self.m_engine_icon, 5)?;
        bitstream.write_integer(self.m_engine_category, 5)?;
        self.m_map_permissions.encode(bitstream)?;
        self.m_player_ratings.encode(bitstream)?;
        bitstream.write_signed_integer(self.m_score_to_win_round, 16)?;
        bitstream.write_bool(self.m_fire_teams_enabled)?;
        bitstream.write_bool(self.m_symmetric_gametype)?;
        for parameter in &self.m_base_variant_parameters_locked {
            bitstream.write_bool(*parameter)?
        }
        for parameter in &self.m_base_variant_parameters_hidden {
            bitstream.write_bool(*parameter)?
        }
        for parameter in &self.m_user_defined_options_locked {
            bitstream.write_bool(*parameter)?
        }
        for parameter in &self.m_user_defined_options_hidden {
            bitstream.write_bool(*parameter)?
        }
        self.m_game_engine.encode(bitstream)?;
        if self.m_encoding_version > 106 {
            if let Some(au1_settings) = &self.m_au1_settings {
                au1_settings.encode(bitstream)?;
            } else {
                return Err("Writing v107 gametypes (and higher) requires AU1 Options to be set.".into());
            }
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_encoding_version = bitstream.read_signed_integer("encoding-version", 32)?;
        self.m_build_number = bitstream.read_signed_integer("version", 32)?;
        self.m_base_variant.decode(bitstream)?;
        let player_trait_count = bitstream.read_integer("player-trait-count", 5)?;
        for i in 0..player_trait_count {
            let mut traits = s_player_trait_option::default();
            traits.decode(bitstream)?;
            self.m_player_traits.push(traits);
        }
        let user_defined_option_count = bitstream.read_integer("user-defined-option-count", 5)?;
        for i in 0..user_defined_option_count {
            let mut option = s_user_defined_option::default();
            option.decode(bitstream)?;
            self.m_user_defined_options.push(option);
        }
        self.m_script_strings.decode(bitstream)?;
        self.m_base_name_string_index = bitstream.read_integer("base-name-string-index", 7)?;
        self.m_localized_name.decode(bitstream)?;
        self.m_localized_description.decode(bitstream)?;
        self.m_localized_category.decode(bitstream)?;
        self.m_engine_icon = bitstream.read_integer("engine-icon-index", 5)?;
        self.m_engine_category = bitstream.read_integer("engine-category", 5)?;
        self.m_map_permissions.decode(bitstream)?;
        self.m_player_ratings.decode(bitstream)?;
        self.m_score_to_win_round = bitstream.read_signed_integer("score-to-win-round", 16)?;
        self.m_fire_teams_enabled = bitstream.read_bool("fire-teams-enabled")?;
        self.m_symmetric_gametype = bitstream.read_bool("symmetric-gametype")?;
        for i in 0..1280 {
            self.m_base_variant_parameters_locked[i] = bitstream.read_bool("base-variant-parameters-locked")?;
        }
        for i in 0..1280 {
            self.m_base_variant_parameters_hidden[i] = bitstream.read_bool("base-variant-parameters-hidden")?;
        }
        for i in 0..32 {
            self.m_user_defined_options_locked[i] = bitstream.read_bool("user-defined-options-locked")?;
        }
        for i in 0..32 {
            self.m_user_defined_options_hidden[i] = bitstream.read_bool("user-defined-options-hidden")?;
        }
        self.m_game_engine.decode(bitstream)?;
        if self.m_encoding_version > 106 {
            let mut au1_settings = c_game_engine_custom_variant_au1_settings::default();
            au1_settings.decode(bitstream)?;
            self.m_au1_settings = Some(au1_settings);
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
pub enum e_game_mode {
    sandbox = 1,
    #[default]
    custom = 2,
    campaign = 3,
    survival = 4,
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_variant {
    pub m_game_engine: e_game_mode,

    // campaign only uses a base variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_campaign_variant: Option<c_game_engine_campaign_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_custom_variant: Option<c_game_engine_custom_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_survival_variant: Option<c_game_engine_survival_variant>,

}

impl c_game_variant {
    pub fn get_metadata(&self) -> BLFLibResult<&s_content_item_metadata> {
        match self.m_game_engine {
            e_game_mode::sandbox => {
                Err("Forge variants are currently unsupported.".into())
            }
            e_game_mode::custom => {
                Ok(
                    &self.m_custom_variant.as_ref()
                        .ok_or_else(|| BLFLibError::from("m_custom_variant does not exist."))?
                        .m_base_variant.m_metadata
                )
            }
            e_game_mode::campaign => {
                Ok(
                    &self.m_campaign_variant.as_ref()
                        .ok_or_else(|| BLFLibError::from("m_campaign_variant does not exist."))?
                        .m_metadata
                )
            }
            e_game_mode::survival => {
                Ok(
                    &self.m_survival_variant.as_ref()
                        .ok_or_else(|| BLFLibError::from("m_survival_variant does not exist."))?
                        .m_base_variant.m_metadata
                )
            }
            _ => {
                Err("Unsupported game variant".into())
            }
        }
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_game_engine.clone(), 4)?;

        match (&self.m_game_engine, &self.m_custom_variant, &self.m_campaign_variant, &self.m_survival_variant) {
            (e_game_mode::sandbox, None, None, None) => {
                return Err("Encoding forge variants is currently unsupported. If you have an example file, please send it to us!".into())
            }
            (e_game_mode::custom, Some(custom_variant), None, None) => {
                custom_variant.encode(bitstream)?;
            }
            (e_game_mode::campaign, None, Some(campaign_variant), None) => {
                campaign_variant.encode(bitstream)?;
            }
            (e_game_mode::survival, None, None, Some(survival_variant)) => {
                survival_variant.encode(bitstream)?;
            }
            _ => {
                Err(format!("Unrecognized game engine {:?}", self.m_game_engine))?;
            }
        }


        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_game_engine = bitstream.read_unnamed_enum(4)?;

        match self.m_game_engine {
            e_game_mode::sandbox => {
                return Err("Decoding forge variants is currently unsupported. If you have an example file, please send it to us!".into())
            }
            e_game_mode::custom => {
                // customs
                let mut custom_variant = c_game_engine_custom_variant::default();
                custom_variant.decode(bitstream)?;
                self.m_custom_variant = Some(custom_variant);
            }
            e_game_mode::campaign => {
                let mut campaign_variant = c_game_engine_campaign_variant::default();
                campaign_variant.decode(bitstream)?;
                self.m_campaign_variant = Some(campaign_variant);
            }
            e_game_mode::survival => {
                let mut survival_variant = c_game_engine_survival_variant::default();
                survival_variant.decode(bitstream)?;
                self.m_survival_variant = Some(survival_variant);
            }
            _ => {
                Err(format!("Unrecognized game engine {:?}", self.m_game_engine))?;
            }
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_custom_game_engine_definition {
    pub m_conditions: Vec<c_condition>,
    pub m_actions: Vec<c_action>,
    pub m_triggers: Vec<c_trigger>,
    pub m_statistics: Vec<c_megalo_game_statistic>,
    pub m_global_variable_metadata: s_variable_metadata<4, 4, 4, 4, 5>,
    pub m_player_variable_metadata: s_variable_metadata<4, 3, 3, 3, 3>,
    pub m_object_variable_metadata: s_variable_metadata<4, 3, 2, 3, 3>,
    pub m_team_variable_metadata: s_variable_metadata<4, 3, 3, 3, 3>,
    pub m_hud_widgets: Vec<u8>,
    pub m_initialization_trigger_index: u16,
    pub m_local_initialization_trigger_index: u16,
    pub m_host_migration_trigger_index: u16,
    pub m_double_migration_trigger_index: u16,
    pub m_object_death_event_trigger_index: u16,
    pub m_local_trigger_index: u16,
    pub m_pregame_trigger_index: u16,
    pub m_objects_used: StaticArray<bool, 2048>,
    pub m_object_filters: Vec<c_object_filter>,
}

impl s_custom_game_engine_definition {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_conditions.len() as u32, 10)?;
        for condition in &self.m_conditions {
            condition.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_actions.len() as u32, 11)?;
        for action in &self.m_actions {
            action.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_triggers.len() as u32, 9)?;
        for trigger in &self.m_triggers {
            trigger.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_statistics.len() as u32, 3)?;
        for statistic in &self.m_statistics {
            statistic.encode(bitstream)?;
        }

        self.m_global_variable_metadata.encode(bitstream)?;
        self.m_player_variable_metadata.encode(bitstream)?;
        self.m_object_variable_metadata.encode(bitstream)?;
        self.m_team_variable_metadata.encode(bitstream)?;

        bitstream.write_integer(self.m_hud_widgets.len() as u32, 3)?;
        for widget in &self.m_hud_widgets {
            bitstream.write_integer(*widget, 4)?;
        }

        bitstream.write_integer(self.m_initialization_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_local_initialization_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_host_migration_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_double_migration_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_object_death_event_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_local_trigger_index as u32, 9)?;
        bitstream.write_integer(self.m_pregame_trigger_index as u32, 9)?;

        for object_type in self.m_objects_used.get() {
            bitstream.write_bool(*object_type)?
        }

        bitstream.write_integer(self.m_object_filters.len() as u32, 5)?;
        for filter in &self.m_object_filters {
            filter.encode(bitstream)?;
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        let condition_count: u16 = bitstream.read_integer("condition-count", 10)?;
        for i in 0..condition_count {
            let mut condition = c_condition::default();
            condition.decode(bitstream)?;
            self.m_conditions.push(condition);
        }

        let action_count: u16 = bitstream.read_integer("action-count", 11)?;
        for i in 0..action_count {
            let mut action = c_action::default();
            action.decode(bitstream)?;
            self.m_actions.push(action);
        }

        let trigger_count: u16 = bitstream.read_integer("trigger-count", 9)?;
        for i in 0..trigger_count {
            let mut trigger = c_trigger::default();
            trigger.decode(bitstream)?;
            self.m_triggers.push(trigger);
        }

        let statistic_count: u8 = bitstream.read_integer("game-statistic-count", 3)?;
        for i in 0..statistic_count {
            let mut statistic = c_megalo_game_statistic::default();
            statistic.decode(bitstream)?;
            self.m_statistics.push(statistic);
        }

        self.m_global_variable_metadata.decode(bitstream)?;
        self.m_player_variable_metadata.decode(bitstream)?;
        self.m_object_variable_metadata.decode(bitstream)?;
        self.m_team_variable_metadata.decode(bitstream)?;

        let widget_count: u8 = bitstream.read_integer("hud-widget-count", 3)?;
        for i in 0..widget_count {
            self.m_hud_widgets.push(bitstream.read_integer("position", 4)?);
        }

        self.m_initialization_trigger_index = bitstream.read_integer("initial-trigger-index", 9)?;
        self.m_local_initialization_trigger_index = bitstream.read_integer("local-initialization-trigger-index", 9)?;
        self.m_host_migration_trigger_index = bitstream.read_integer("host-migration-trigger-index", 9)?;
        self.m_double_migration_trigger_index = bitstream.read_integer("double-migration-trigger-index", 9)?;
        self.m_object_death_event_trigger_index = bitstream.read_integer("death-event-trigger-index", 9)?;
        self.m_local_trigger_index = bitstream.read_integer("local-trigger-index", 9)?;
        self.m_pregame_trigger_index = bitstream.read_integer("pregame-trigger-index", 9)?;

        for i in 0..2048 {
            self.m_objects_used[i] = bitstream.read_bool("object-types-used")?;
        }

        let object_filter_count: u8 = bitstream.read_integer("object-filter-count", 5)?;
        for i in 0..object_filter_count {
            let mut object_filter = c_object_filter::default();
            object_filter.decode(bitstream)?;
            self.m_object_filters.push(object_filter);
        }

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
// no idea what this is called
pub struct c_game_engine_custom_variant_au1_settings {
    pub m_flags: u32,
    pub m_precision_bloom: Float32,
    pub m_active_camo_energy_curve_min: Float32,
    pub m_active_camo_energy_curve_max: Float32,
    pub m_armor_lock_damage_drain: Float32,
    pub m_armor_lock_damage_drain_limit: Float32,
    pub m_magnum_damage: Float32,
    pub m_magnum_fire_delay: Float32,
}

impl c_game_engine_custom_variant_au1_settings {
    pub fn encode(&self, mut bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_flags, 32)?;
        bitstream.write_quantized_real(self.m_precision_bloom, 0f32, 2f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_active_camo_energy_curve_min, 0f32, 2f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_active_camo_energy_curve_max, 0f32, 2f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_armor_lock_damage_drain, 0f32, 2f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_armor_lock_damage_drain_limit, 0f32, 2f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_magnum_damage, 0f32, 10f32, 8, false, true)?;
        bitstream.write_quantized_real(self.m_magnum_fire_delay, 0f32, 10f32, 8, false, true)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_flags = bitstream.read_integer("flags", 32)?;
        self.m_precision_bloom = bitstream.read_quantized_real(0f32, 2f32, 8, false, true)?;
        self.m_active_camo_energy_curve_min = bitstream.read_quantized_real(0f32, 2f32, 8, false, true)?;
        self.m_active_camo_energy_curve_max = bitstream.read_quantized_real(0f32, 2f32, 8, false, true)?;
        self.m_armor_lock_damage_drain = bitstream.read_quantized_real(0f32, 2f32, 8, false, true)?;
        self.m_armor_lock_damage_drain_limit = bitstream.read_quantized_real(0f32, 2f32, 8, false, true)?;
        self.m_magnum_damage = bitstream.read_quantized_real(0f32, 10f32, 8, false, true)?;
        self.m_magnum_fire_delay = bitstream.read_quantized_real(0f32, 10f32, 8, false, true)?;

        Ok(())
    }
}