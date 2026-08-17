use serde::{Deserialize, Serialize};
use num_derive::{FromPrimitive, ToPrimitive};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib::blam::halo4::v20810_12_09_22_1647_main::game::game_engine_player_traits::c_player_traits;
use blf_lib_derivable::result::BLFLibResult;

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_miscellaneous_options {
    pub m_teams_enabled: bool,
    pub m_round_reset_players: bool,
    pub m_round_reset_map: bool,
    pub m_perfection_enabled: bool,
    pub m_round_time_limit_minutes: u8,
    pub m_round_limit: u8,
    pub m_early_victory_win_count: u8,
    pub m_killcam_enabled: bool,
    pub m_medal_scoring_enabled: bool,
    pub m_asymmetric_round_scoring: bool,
    pub m_overshield_deplete_time: u8,
    pub m_mosh_enabled: bool,
    pub m_drop_weapons_on_death: bool,
    pub m_mosh_difficulty: u8,
}

impl c_game_engine_miscellaneous_options {
    pub fn initialize(&mut self) {
        *self = Self::default();
        self.m_round_reset_map = true;
        self.m_round_time_limit_minutes = 8;
        self.m_round_limit = 1;
        self.m_early_victory_win_count = 2;
        self.m_killcam_enabled = true;
        self.m_medal_scoring_enabled = true;
        self.m_drop_weapons_on_death = true;
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_teams_enabled)?;
        bitstream.write_bool(self.m_round_reset_players)?;
        bitstream.write_bool(self.m_round_reset_map)?;
        bitstream.write_bool(self.m_perfection_enabled)?;
        bitstream.write_integer(self.m_round_time_limit_minutes as u32, 8)?;
        bitstream.write_integer(self.m_round_limit as u32, 5)?;
        bitstream.write_integer(self.m_early_victory_win_count as u32, 4)?;
        bitstream.write_bool(self.m_killcam_enabled)?;
        bitstream.write_bool(self.m_medal_scoring_enabled)?;
        bitstream.write_bool(self.m_asymmetric_round_scoring)?;
        bitstream.write_integer(self.m_overshield_deplete_time as u32, 8)?;
        bitstream.write_bool(self.m_mosh_enabled)?;
        bitstream.write_bool(self.m_drop_weapons_on_death)?;
        bitstream.write_integer(self.m_mosh_difficulty as u32, 2)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_teams_enabled = bitstream.read_bool("miscellaneous-options-teams")?;
        self.m_round_reset_players = bitstream.read_bool("miscellaneous-options-round-reset-players")?;
        self.m_round_reset_map = bitstream.read_bool("miscellaneous-options-round-reset-map")?;
        self.m_perfection_enabled = bitstream.read_bool("miscellaneous-options-perfection-enabled")?;
        self.m_round_time_limit_minutes = bitstream.read_integer("miscellaneous-options-round-time-limit-minutes", 8)?;
        self.m_round_limit = bitstream.read_integer("miscellaneous-options-round-limit", 5)?;
        self.m_early_victory_win_count = bitstream.read_integer("miscellaneous-options-early-victory-win-count", 4)?;
        self.m_killcam_enabled = bitstream.read_bool("miscellaneous-options-killcam-enabled")?;
        self.m_medal_scoring_enabled = bitstream.read_bool("miscellaneous-options-medal-scoring-enabled")?;
        self.m_asymmetric_round_scoring = bitstream.read_bool("miscellaneous-options-asymmetric-round-scoring")?;
        self.m_overshield_deplete_time = bitstream.read_integer("miscellaneous-options-overshield-deplete-time", 8)?;
        self.m_mosh_enabled = bitstream.read_bool("miscellaneous-options-mosh")?;
        self.m_drop_weapons_on_death = bitstream.read_bool("miscellaneous-options-drop-weapons-on-death")?;
        self.m_mosh_difficulty = bitstream.read_integer("miscellaneous-options-mosh-difficulty", 2)?;

        Ok(())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_game_engine_respawn_options {
    pub m_inherit_respawn_time: bool,
    pub m_respawn_with_teammate: bool,
    pub m_respawn_at_location: bool,
    pub m_respawn_on_kills: bool,
    pub m_early_respawn_allowed: bool,
    pub m_lives_per_round: u8,
    pub m_team_lives_per_round: u8,
    pub m_min_respawn_time_seconds: u8,
    pub m_respawn_time_seconds: u8,
    pub m_suicide_penalty_seconds: u8,
    pub m_betrayal_penalty_seconds: u8,
    pub m_respawn_growth_seconds: u8,
    pub m_loadout_cam_time: u8,
    pub m_respawn_player_traits_duration_seconds: u8,
    pub m_respawn_player_traits: c_player_traits,
}

impl c_game_engine_respawn_options {
    pub fn initialize(&mut self) {
        *self = Self::default();
        self.m_respawn_time_seconds = 5;
        self.m_suicide_penalty_seconds = 5;
        self.m_betrayal_penalty_seconds = 5;
        self.m_loadout_cam_time = 10;
        self.m_respawn_player_traits_duration_seconds = 5;
        self.m_respawn_player_traits.clear();
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_bool(self.m_inherit_respawn_time)?;
        bitstream.write_bool(self.m_respawn_with_teammate)?;
        bitstream.write_bool(self.m_respawn_at_location)?;
        bitstream.write_bool(self.m_respawn_on_kills)?;
        bitstream.write_bool(self.m_early_respawn_allowed)?;
        bitstream.write_integer(self.m_lives_per_round as u32, 6)?;
        bitstream.write_integer(self.m_team_lives_per_round as u32, 7)?;
        bitstream.write_integer(self.m_min_respawn_time_seconds as u32, 8)?;
        bitstream.write_integer(self.m_respawn_time_seconds as u32, 8)?;
        bitstream.write_integer(self.m_suicide_penalty_seconds as u32, 8)?;
        bitstream.write_integer(self.m_betrayal_penalty_seconds as u32, 8)?;
        bitstream.write_integer(self.m_respawn_growth_seconds as u32, 4)?;
        bitstream.write_integer(self.m_loadout_cam_time, 4)?;
        bitstream.write_integer(self.m_respawn_player_traits_duration_seconds as u32, 6)?;
        self.m_respawn_player_traits.encode(bitstream)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_inherit_respawn_time = bitstream.read_bool("respawn-options-inherit-respawn-time")?;
        self.m_respawn_with_teammate = bitstream.read_bool("respawn-options-respawn-with-teammate")?;
        self.m_respawn_at_location = bitstream.read_bool("respawn-options-respawn-at-location")?;
        self.m_respawn_on_kills = bitstream.read_bool("respawn-options-respawn-on-kills")?;
        self.m_early_respawn_allowed =
            bitstream.read_bool("respawn-options-early-respawn-allowed")?;
        self.m_lives_per_round = bitstream.read_integer("respawn-options-lives-per-round", 6)?;
        self.m_team_lives_per_round =
            bitstream.read_integer("respawn-options-team-lives-per-round", 7)?;
        self.m_min_respawn_time_seconds =
            bitstream.read_integer("respawn-options-min-respawn-time", 8)?;
        self.m_respawn_time_seconds = bitstream.read_integer("respawn-options-respawn-time", 8)?;
        self.m_suicide_penalty_seconds =
            bitstream.read_integer("respawn-options-suicide-time", 8)?;
        self.m_betrayal_penalty_seconds =
            bitstream.read_integer("respawn-options-betrayal-time", 8)?;
        self.m_respawn_growth_seconds =
            bitstream.read_integer("respawn-options-respawn-growth-time", 4)?;
        self.m_loadout_cam_time =
            bitstream.read_integer("respawn-options-initial-loadout-selection-time", 4)?;
        self.m_respawn_player_traits_duration_seconds =
            bitstream.read_integer("respawn-options-player-traits-duration", 6)?;
        self.m_respawn_player_traits.decode(bitstream)?;

        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(1)]
pub enum e_trait_set_type {
    #[default]
    player_traits = 0,
    player_traits_dynamic = 1,
}

/// Halo 4 MaxStrings=148 → 8-bit string indices; trailing hidden + traits-type.
#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct s_player_trait_option {
    pub m_name_string_index: u8,
    pub m_description_string_index: u8,
    pub m_player_traits: c_player_traits,
    pub m_hidden: bool,
    pub m_traits_type: e_trait_set_type,
}

impl s_player_trait_option {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_integer(self.m_name_string_index, 8)?;
        bitstream.write_integer(self.m_description_string_index, 8)?;
        self.m_player_traits.encode(bitstream)?;
        bitstream.write_bool(self.m_hidden)?;
        bitstream.write_enum(self.m_traits_type)?;

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_name_string_index = bitstream.read_integer("name-string-index", 8)?;
        self.m_description_string_index =
            bitstream.read_integer("description-string-index", 8)?;
        self.m_player_traits.decode(bitstream)?;
        self.m_hidden = bitstream.read_bool("hidden")?;
        self.m_traits_type = bitstream.read_enum("traits-type")?;

        Ok(())
    }
}