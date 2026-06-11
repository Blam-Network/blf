use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_explicit_player::c_explicit_player;
use blf_lib::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_explicit_team::c_explicit_team;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach_mcc::v_untracked_25_08_16_1352::game::megalogamengine::megalogamengine_explicit_object::c_explicit_object;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive)]
pub enum e_custom_variable_type {
    #[default]
    constant = 0,
    player = 1,
    object = 2,
    team = 3,
    global = 4,
    option = 5,
    explicit_object = 6,
    team_score = 7,
    player_score = 8,
    money = 9,
    rating = 10,
    player_statistic = 11,
    team_statistic = 12,
    round = 13,
    get_symmetric_gametype = 14,
    symmetric_gametype = 15,
    score_to_win_round = 16,
    unk_f7a6 = 17,
    teams_enabled = 18,
    round_time_limit = 19,
    round_count = 20,
    perfection_enabled = 21,
    early_victory_win_count = 22,
    sudden_death_time_limit = 23,
    grace_period_time_limit = 24,
    lives_per_round = 25,
    team_lives_per_round = 26,
    respawn_time = 27,
    suicide_respawn_penalty = 28,
    betrayal_respawn_penalty = 29,
    respawn_time_growth = 30,
    loadout_selection_time = 31,
    respawn_traits_duration = 32,
    friendly_fire_enabled = 33,
    betrayal_booting_enabled = 34,
    enemy_voice_enabled = 35,
    open_channel_voice_enabled = 36,
    dead_player_voice_enabled = 37,
    grenades_on_map = 38,
    indestructible_vehicles = 39,
    red_powerup_duration = 40,
    blue_powerup_duration = 41,
    yellow_powerup_duration = 42,
    object_death_damage_type = 43,
}

#[derive(Default, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct c_custom_variable_reference {
    pub m_type: e_custom_variable_type, // 6 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_immediate_value: Option<i16>, // 16 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_explicit_player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object: Option<c_explicit_object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_explicit_team>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_index: Option<u8>, // 3 or 4 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_option_index: Option<u8>, // 4 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_statistic_index: Option<u8>, // 2 bits
}

impl c_custom_variable_reference {
    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_type, 6)?;

        match self.m_type {
            e_custom_variable_type::constant => {
                if let Some(immediate_value) = self.m_immediate_value {
                    bitstream.write_signed_integer(immediate_value, 16)?;
                }
            }
            e_custom_variable_type::player => {
                if let Some(player) = &self.m_player {
                    player.encode(bitstream)?;
                }
                if let Some(variable_index) = self.m_variable_index {
                    bitstream.write_integer(variable_index, 3)?;
                }
            }
            e_custom_variable_type::object => {
                if let Some(object) = &self.m_object {
                    object.encode(bitstream)?;
                }
                if let Some(variable_index) = self.m_variable_index {
                    bitstream.write_integer(variable_index, 3)?;
                }
            }
            e_custom_variable_type::team => {
                if let Some(team) = &self.m_team {
                    team.encode(bitstream)?;
                }
                if let Some(variable_index) = self.m_variable_index {
                    bitstream.write_integer(variable_index, 3)?;
                }
            }
            e_custom_variable_type::global => {
                if let Some(variable_index) = self.m_variable_index {
                    bitstream.write_integer(variable_index, 4)?;
                }
            }
            e_custom_variable_type::option => {
                if let Some(option_index) = self.m_option_index {
                    bitstream.write_integer(option_index, 4)?;
                }
            }
            e_custom_variable_type::explicit_object => {
                if let Some(object) = &self.m_object {
                    object.encode(bitstream)?;
                }
            }
            e_custom_variable_type::team_score => {
                if let Some(team) = &self.m_team {
                    team.encode(bitstream)?;
                }
            }
            e_custom_variable_type::player_score | e_custom_variable_type::money | e_custom_variable_type::rating => {
                if let Some(player) = &self.m_player {
                    player.encode(bitstream)?;
                }
            }
            e_custom_variable_type::player_statistic => {
                if let Some(player) = &self.m_player {
                    player.encode(bitstream)?;
                }
                if let Some(statistic_index) = self.m_statistic_index {
                    bitstream.write_integer(statistic_index, 2)?;
                }
            }
            e_custom_variable_type::team_statistic => {
                if let Some(team) = &self.m_team {
                    team.encode(bitstream)?;
                }
                if let Some(statistic_index) = self.m_statistic_index {
                    bitstream.write_integer(statistic_index, 2)?;
                }
            }
            _ => {}
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_unnamed_enum(6)?;

        match self.m_type {
            e_custom_variable_type::constant => {
                self.m_immediate_value = Some(bitstream.read_signed_integer("immediate-value", 16)?);
            }
            e_custom_variable_type::player => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 3)?);
            }
            e_custom_variable_type::object => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 3)?);
            }
            e_custom_variable_type::team => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 3)?);
            }
            e_custom_variable_type::global => {
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 4)?);
            }
            e_custom_variable_type::option => {
                self.m_option_index = Some(bitstream.read_integer("option-index", 4)?);
            }
            e_custom_variable_type::explicit_object => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
            }
            e_custom_variable_type::team_score => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
            }
            e_custom_variable_type::player_score | e_custom_variable_type::money | e_custom_variable_type::rating => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            e_custom_variable_type::player_statistic => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_statistic_index = Some(bitstream.read_integer("statistic-index", 2)?);
            }
            e_custom_variable_type::team_statistic => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_statistic_index = Some(bitstream.read_integer("statistic-index", 2)?);
            }
            _ => {}
        }

        Ok(())
    }
}
