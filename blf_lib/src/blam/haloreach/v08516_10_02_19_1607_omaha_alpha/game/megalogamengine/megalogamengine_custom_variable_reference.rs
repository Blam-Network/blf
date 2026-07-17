use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_player::c_explicit_player;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_team::c_explicit_team;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::haloreach::v12065_11_08_24_1738_tu1actual::game::megalogamengine::megalogamengine_explicit_object::c_explicit_object;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, ToPrimitive, FromPrimitive, crate::derive::c_enum)]
#[bits(6)]
pub enum e_custom_variable_type {
    #[default]
    constant = 0,
    player_number = 1,
    object_number = 2,
    team_number = 3,
    global_number = 4,
    option = 5,
    spawn_object = 6,
    team_score = 7,
    player_score = 8,
    player_money = 9,
    player_rating = 10,
    player_stat = 11,
    team_stat = 12,
    round_index = 13,
    symmetric_gametype = 14,
    symmetric_gametype_pregame = 15,
    score_to_win_round = 16,
    fire_teams_enabled = 17,
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
    pub m_type: e_custom_variable_type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_immediate_value: Option<i16>, // 16 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_player: Option<c_explicit_player>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_object: Option<c_explicit_object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_team: Option<c_explicit_team>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_variable_index: Option<u8>, // 1, 3 or 4 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_option_index: Option<u8>, // 4 bits
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_statistic_index: Option<u8>, // 2 bits
}

impl c_custom_variable_reference {
    pub fn is_writeable(&self) -> bool {
        matches!(
            self.m_type,
            e_custom_variable_type::player_number
                | e_custom_variable_type::object_number
                | e_custom_variable_type::team_number
                | e_custom_variable_type::global_number
                | e_custom_variable_type::team_score
                | e_custom_variable_type::player_score
                | e_custom_variable_type::player_money
                | e_custom_variable_type::player_stat
                | e_custom_variable_type::team_stat
                | e_custom_variable_type::symmetric_gametype_pregame
        )
    }

    pub fn encode(&self, bitstream: &mut c_bitstream_writer) -> BLFLibResult {
        bitstream.write_enum(self.m_type)?;

        match (
            self.m_type,
            self.m_immediate_value,
            self.m_player.as_ref(),
            self.m_object.as_ref(),
            self.m_team.as_ref(),
            self.m_variable_index,
            self.m_option_index,
            self.m_statistic_index,
        ) {
            (e_custom_variable_type::constant, Some(immediate_value), None, None, None, None, None, None) => {
                bitstream.write_signed_integer(immediate_value, 16)?;
            }
            (e_custom_variable_type::player_number, None, Some(player), None, None, Some(variable_index), None, None) => {
                player.encode(bitstream)?;
                bitstream.write_integer(variable_index, 3)?;
            }
            (e_custom_variable_type::object_number, None, None, Some(object), None, Some(variable_index), None, None) => {
                object.encode(bitstream)?;
                bitstream.write_integer(variable_index, 3)?;
            }
            (e_custom_variable_type::team_number, None, None, None, Some(team), Some(variable_index), None, None) => {
                team.encode(bitstream)?;
                bitstream.write_integer(variable_index, 1)?;
            }
            (e_custom_variable_type::global_number, None, None, None, None, Some(variable_index), None, None) => {
                bitstream.write_integer(variable_index, 3)?;
            }
            (e_custom_variable_type::option, None, None, None, None, None, Some(option_index), None) => {
                bitstream.write_integer(option_index, 4)?;
            }
            (e_custom_variable_type::spawn_object, None, None, Some(object), None, None, None, None) => {
                object.encode(bitstream)?;
            }
            (e_custom_variable_type::team_score, None, None, None, Some(team), None, None, None) => {
                team.encode(bitstream)?;
            }
            (
                e_custom_variable_type::player_score
                | e_custom_variable_type::player_money
                | e_custom_variable_type::player_rating,
                None,
                Some(player),
                None,
                None,
                None,
                None,
                None,
            ) => {
                player.encode(bitstream)?;
            }
            (e_custom_variable_type::player_stat, None, Some(player), None, None, None, None, Some(statistic_index)) => {
                player.encode(bitstream)?;
                bitstream.write_integer(statistic_index, 2)?;
            }
            (e_custom_variable_type::team_stat, None, None, None, Some(team), None, None, Some(statistic_index)) => {
                team.encode(bitstream)?;
                bitstream.write_integer(statistic_index, 2)?;
            }
            _ => {
                // return Err(format!("Invalid c_custom_variable_reference: {self:?}").into())
            }
        };

        Ok(())
    }

    pub fn decode(&mut self, bitstream: &mut c_bitstream_reader) -> BLFLibResult {
        self.m_type = bitstream.read_enum("type")?;

        match self.m_type {
            e_custom_variable_type::constant => {
                self.m_immediate_value = Some(bitstream.read_signed_integer("immediate-value", 16)?);
            }
            e_custom_variable_type::player_number => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 3)?);
            }
            e_custom_variable_type::object_number => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 3)?);
            }
            e_custom_variable_type::team_number => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 1)?);
            }
            e_custom_variable_type::global_number => {
                self.m_variable_index = Some(bitstream.read_integer("variable-index", 3)?);
            }
            e_custom_variable_type::option => {
                self.m_option_index = Some(bitstream.read_integer("option-index", 4)?);
            }
            e_custom_variable_type::spawn_object => {
                let mut object = c_explicit_object::default();
                object.decode(bitstream)?;
                self.m_object = Some(object);
            }
            e_custom_variable_type::team_score => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
            }
            e_custom_variable_type::player_score
            | e_custom_variable_type::player_money
            | e_custom_variable_type::player_rating => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
            }
            e_custom_variable_type::player_stat => {
                let mut player = c_explicit_player::default();
                player.decode(bitstream)?;
                self.m_player = Some(player);
                self.m_statistic_index = Some(bitstream.read_integer("statistic-index", 2)?);
            }
            e_custom_variable_type::team_stat => {
                let mut team = c_explicit_team::default();
                team.decode(bitstream)?;
                self.m_team = Some(team);
                self.m_statistic_index = Some(bitstream.read_integer("statistic-index", 2)?);
            }
            _ => {
                // return Err(format!("Invalid c_custom_variable_reference: {self:?}").into())
            }
        }

        Ok(())
    }
}
