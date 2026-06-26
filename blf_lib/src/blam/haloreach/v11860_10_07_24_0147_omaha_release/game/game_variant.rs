use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_campaign::c_game_engine_campaign_variant;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_default::c_game_engine_base_variant;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_player_rating_parameters::s_game_engine_player_rating_parameters;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_survival::c_game_engine_survival_variant;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_engine_sandbox::c_game_engine_sandbox_variant;
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
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_map_permissions::c_megalogamengine_map_permissions;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::saved_games::saved_game_files::c_content_item_metadata;
use crate::types::array::StaticArray;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_variant::s_game_variant_parameter_flags;

/// Release (pre-TU1) custom variant layout — same as TU1 v107 fields without AU1 settings.
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
    pub m_base_variant_parameters_locked: s_game_variant_parameter_flags,
    pub m_base_variant_parameters_hidden: s_game_variant_parameter_flags,
    pub m_user_defined_options_locked: StaticArray<bool, 32>,
    pub m_user_defined_options_hidden: StaticArray<bool, 32>,
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
        self.m_localized_category.encode(bitstream)?;
        bitstream.write_integer(self.m_engine_icon, 5)?;
        bitstream.write_integer(self.m_engine_category, 5)?;
        self.m_map_permissions.encode(bitstream)?;
        self.m_player_ratings.encode(bitstream)?;
        bitstream.write_signed_integer(self.m_score_to_win_round, 16)?;
        bitstream.write_bool(self.m_fire_teams_enabled)?;
        bitstream.write_bool(self.m_symmetric_gametype)?;
        self.m_base_variant_parameters_locked.encode(bitstream, "base-variant-parameters-locked")?;
        self.m_base_variant_parameters_hidden.encode(bitstream, "base-variant-parameters-hidden")?;
        for parameter in &self.m_user_defined_options_locked {
            bitstream.write_bool(*parameter)?;
        }
        for parameter in &self.m_user_defined_options_hidden {
            bitstream.write_bool(*parameter)?;
        }
        self.m_game_engine.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_encoding_version = bitstream.read_signed_integer("encoding-version", 32)?;
        self.m_build_number = bitstream.read_signed_integer("version", 32)?;
        self.m_base_variant.decode(bitstream)?;
        let player_trait_count = bitstream.read_integer("player-trait-count", 5)?;
        for _ in 0..player_trait_count {
            let mut traits = s_player_trait_option::default();
            traits.decode(bitstream)?;
            self.m_player_traits.push(traits);
        }
        let user_defined_option_count = bitstream.read_integer("user-defined-option-count", 5)?;
        for _ in 0..user_defined_option_count {
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
        self.m_base_variant_parameters_locked.decode(bitstream, "base-variant-parameters-locked")?;
        self.m_base_variant_parameters_hidden.decode(bitstream, "base-variant-parameters-hidden")?;
        for i in 0..32 {
            self.m_user_defined_options_locked[i] =
                bitstream.read_bool("user-defined-options-locked")?;
        }
        for i in 0..32 {
            self.m_user_defined_options_hidden[i] =
                bitstream.read_bool("user-defined-options-hidden")?;
        }
        self.m_game_engine.decode(bitstream)?;

        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
pub enum e_game_mode {
    none = 0,
    sandbox = 1,
    #[default]
    megalogamengine = 2,
    campaign = 3,
    survival = 4,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_variant {
    pub m_game_engine: e_game_mode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_campaign_variant: Option<c_game_engine_campaign_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_custom_variant: Option<c_game_engine_custom_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_survival_variant: Option<c_game_engine_survival_variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_sandbox_variant: Option<c_game_engine_sandbox_variant>,
}

impl c_game_variant {
    pub fn get_metadata(&self) -> BLFLibResult<&c_content_item_metadata> {
        match self.m_game_engine {
            e_game_mode::sandbox => Ok(
                &self
                    .m_sandbox_variant
                    .as_ref()
                    .ok_or_else(|| BLFLibError::from("m_sandbox_variant does not exist."))?
                    .m_custom_variant
                    .m_base_variant
                    .m_metadata,
            ),
            e_game_mode::megalogamengine => Ok(
                &self
                    .m_custom_variant
                    .as_ref()
                    .ok_or_else(|| BLFLibError::from("m_custom_variant does not exist."))?
                    .m_base_variant
                    .m_metadata,
            ),
            e_game_mode::campaign => Ok(
                &self
                    .m_campaign_variant
                    .as_ref()
                    .ok_or_else(|| BLFLibError::from("m_campaign_variant does not exist."))?
                    .m_metadata,
            ),
            e_game_mode::survival => Ok(
                &self
                    .m_survival_variant
                    .as_ref()
                    .ok_or_else(|| BLFLibError::from("m_survival_variant does not exist."))?
                    .m_base_variant
                    .m_metadata,
            ),
            e_game_mode::none => Err(BLFLibError::from("m_game_engine is none.")),
        }
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum_raw(self.m_game_engine.clone(), 4)?;

        match (
            &self.m_game_engine,
            &self.m_custom_variant,
            &self.m_campaign_variant,
            &self.m_survival_variant,
            &self.m_sandbox_variant,
        ) {
            (e_game_mode::sandbox, None, None, None, Some(sandbox_variant)) => {
                sandbox_variant.encode(bitstream)?;
            }
            (e_game_mode::megalogamengine, Some(custom_variant), None, None, None) => {
                custom_variant.encode(bitstream)?;
            }
            (e_game_mode::campaign, None, Some(campaign_variant), None, None) => {
                campaign_variant.encode(bitstream)?;
            }
            (e_game_mode::survival, None, None, Some(survival_variant), None) => {
                survival_variant.encode(bitstream)?;
            }
            _ => {
                Err(format!("Unrecognized game engine {:?}", self.m_game_engine))?;
            }
        }

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_game_engine = bitstream.read_unnamed_enum_raw(4)?;

        match self.m_game_engine {
            e_game_mode::sandbox => {
                let mut sandbox_variant = c_game_engine_sandbox_variant::default();
                sandbox_variant.decode(bitstream)?;
                self.m_sandbox_variant = Some(sandbox_variant);
            }
            e_game_mode::megalogamengine => {
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

pub use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::game_variant::s_custom_game_engine_definition;
