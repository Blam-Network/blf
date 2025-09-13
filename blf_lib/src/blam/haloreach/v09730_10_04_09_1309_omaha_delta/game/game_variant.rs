use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::game_engine_campaign::c_game_engine_campaign_variant;
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::game_engine_default::c_game_engine_base_variant;
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::game_engine_survival::c_game_engine_survival_variant;
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::game_engine_traits::s_player_trait_option;
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::megalogamengine::megalogamengine_actions::c_action;
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::megalogamengine::megalogamengine_conditions::c_condition;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_map_objects::c_object_filter;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_statistics::c_megalo_game_statistic;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_trigger::c_trigger;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_user_defined_options::s_user_defined_option;
use blf_lib::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::megalogamengine::megalogamengine_variable_metadata::s_variable_metadata;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::string_table::c_string_table;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::memory::bitstream_reader::c_bitstream_reader_extensions;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::game_engine_loadout_traits::{s_loadout_palette_unknown_struct, s_loadout_unknown_struct};
use crate::blam::haloreach::v09730_10_04_09_1309_omaha_delta::game::megalogamengine::megalogamengine_requisitions::s_requisition_palette;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::memory::bitstream_writer::c_bitstream_writer_extensions;
use crate::types::array::StaticArray;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_custom_variant {
    pub m_encoding_version: i32,
    pub m_build_number: i32,
    pub m_base_variant: c_game_engine_base_variant,
    pub m_player_traits: Vec<s_player_trait_option>,
    pub m_user_defined_options: Vec<s_user_defined_option>,
    pub m_script_strings: c_string_table<112, 0x4C00, 14, 14, 7>,
    pub m_base_name_string_index: u8,
    pub m_localized_name: c_string_table<1, 0x180, 9, 9, 1>,
    pub m_localized_description: c_string_table<1, 0xC00, 12, 12, 1>,
    pub m_engine_icon: i8,
    pub m_score_to_win_round: u16,
    pub m_symmetric_gametype: bool,
    pub m_game_engine: s_custom_game_engine_definition,
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
        self.m_script_strings.encode(bitstream)?;
        bitstream.write_integer(self.m_base_name_string_index, 7)?;
        self.m_localized_name.encode(bitstream)?;
        self.m_localized_description.encode(bitstream)?;
        bitstream.write_index::<64>(self.m_engine_icon, 6)?;
        bitstream.write_integer(self.m_score_to_win_round, 16)?;
        bitstream.write_bool(self.m_symmetric_gametype)?;
        self.m_game_engine.encode(bitstream)?;

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
        self.m_engine_icon = bitstream.read_index::<64>("engine-icon-index", 6)? as i8;
        self.m_score_to_win_round = bitstream.read_integer("score-to-win-round", 16)?;
        self.m_symmetric_gametype = bitstream.read_bool("symmetric-gametype")?;
        self.m_game_engine.decode(bitstream)?;

        Ok(())
    }
}


#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_variant {
    pub m_game_engine: u8,

    // campaign only uses a base variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_campaign_variant: Option<c_game_engine_campaign_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_custom_variant: Option<c_game_engine_custom_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_survival_variant: Option<c_game_engine_survival_variant>,

}

impl c_game_variant {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_game_engine, 4)?;

        match (self.m_game_engine, &self.m_custom_variant, &self.m_campaign_variant, &self.m_survival_variant) {
            (1, None, None, None) => {
                return Err("Encoding forge variants is currently unsupported. If you have an example file, please send it to us!".into())
            }
            (2, Some(custom_variant), None, None) => {
                custom_variant.encode(bitstream)?;
            }
            (3, None, Some(campaign_variant), None) => {
                campaign_variant.encode(bitstream)?;
            }
            (4, None, None, Some(survival_variant)) => {
                survival_variant.encode(bitstream)?;
            }
            _ => {
                Err(format!("Unrecognized game engine {}", self.m_game_engine))?;
            }
        }


        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_game_engine = bitstream.read_unnamed_enum(4)?;

        match self.m_game_engine {
            1 => {
                return Err("Decoding forge variants is currently unsupported. If you have an example file, please send it to us!".into())
            }
            2 => {
                // customs
                let mut custom_variant = c_game_engine_custom_variant::default();
                custom_variant.decode(bitstream)?;
                self.m_custom_variant = Some(custom_variant);
            }
            3 => {
                let mut campaign_variant = c_game_engine_campaign_variant::default();
                campaign_variant.decode(bitstream)?;
                self.m_campaign_variant = Some(campaign_variant);
            }
            4 => {
                let mut survival_variant = c_game_engine_survival_variant::default();
                survival_variant.decode(bitstream)?;
                self.m_survival_variant = Some(survival_variant);
            }
            _ => {
                Err(format!("Unrecognized game engine {}", self.m_game_engine))?;
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
    pub m_requisitions: Vec<s_requisition_palette>,
    pub m_loadouts: Vec<s_loadout_unknown_struct>,
    pub m_loadout_palette: Vec<s_loadout_palette_unknown_struct>,
    pub m_statistics: Vec<c_megalo_game_statistic>,
    pub m_global_variable_metadata: s_variable_metadata<4, 4, 4, 4, 5>,
    pub m_player_variable_metadata: s_variable_metadata<4, 3, 3, 3, 3>,
    pub m_object_variable_metadata: s_variable_metadata<4, 3, 2, 3, 3>,
    pub m_team_variable_metadata: s_variable_metadata<4, 3, 3, 3, 3>,
    pub m_hud_widgets: Vec<u8>,
    pub m_initialization_trigger_index: i16,
    pub m_local_initialization_trigger_index: i16,
    pub m_host_migration_trigger_index: i16,
    pub m_object_death_event_trigger_index: i16,
    pub m_local_trigger_index: i16,
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

        bitstream.write_integer(self.m_requisitions.len() as u32, 4)?;
        for requisition in &self.m_requisitions {
            requisition.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_loadouts.len() as u32, 6)?;
        for loadout in &self.m_loadouts {
            loadout.encode(bitstream)?;
        }

        bitstream.write_integer(self.m_loadout_palette.len() as u32, 5)?;
        for loadout in &self.m_loadout_palette {
            loadout.encode(bitstream)?;
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

        bitstream.write_index::<320>(self.m_initialization_trigger_index, 8)?;
        bitstream.write_index::<320>(self.m_local_initialization_trigger_index, 8)?;
        bitstream.write_index::<320>(self.m_host_migration_trigger_index, 8)?;
        bitstream.write_index::<320>(self.m_object_death_event_trigger_index, 8)?;
        bitstream.write_index::<320>(self.m_local_trigger_index, 8)?;

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

        let requisition_palette_count = bitstream.read_integer("requisition-palette-count", 4)?;
        for i in 0..requisition_palette_count {
            let mut requisition = s_requisition_palette::default();
            requisition.decode(bitstream)?;
            self.m_requisitions.push(requisition);
        }

        let loadout_count: u8 = bitstream.read_integer("loadout-count", 6)?;
        for i in 0..loadout_count {
            let mut loadout = s_loadout_unknown_struct::default();
            loadout.decode(bitstream)?;
            self.m_loadouts.push(loadout);
        }

        let loadout_palette_count: u8 = bitstream.read_integer("loadout-palette-count", 5)?;
        for i in 0..loadout_palette_count {
            let mut loadout_palette = s_loadout_palette_unknown_struct::default();
            loadout_palette.decode(bitstream)?;
            self.m_loadout_palette.push(loadout_palette);
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

        self.m_initialization_trigger_index = bitstream.read_index::<320>("initial-trigger-index", 8)? as i16;
        self.m_local_initialization_trigger_index = bitstream.read_index::<320>("local-initialization-trigger-index", 8)? as i16;
        self.m_host_migration_trigger_index = bitstream.read_index::<320>("host-migration-trigger-index", 8)? as i16;
        self.m_object_death_event_trigger_index = bitstream.read_index::<320>("death-event-trigger-index", 8)? as i16;
        self.m_local_trigger_index = bitstream.read_index::<320>("local-trigger-index", 8)? as i16;

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
