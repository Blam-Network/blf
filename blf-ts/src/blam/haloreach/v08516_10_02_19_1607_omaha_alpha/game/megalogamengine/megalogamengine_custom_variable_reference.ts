import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";
import {
  c_explicit_object,
  c_explicit_player,
  c_explicit_team,
} from "../../../v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_references";

/** Matches `e_custom_variable_type` in blf_lib omaha_alpha `megalogamengine_custom_variable_reference.rs`. */
export enum e_custom_variable_type {
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

export class c_custom_variable_reference {
  @AutoMap(() => e_custom_variable_type)
  m_type: e_custom_variable_type = e_custom_variable_type.constant;
  @AutoMap(() => Number)
  m_immediate_value?: number;
  @AutoMap(() => c_explicit_player)
  m_player?: c_explicit_player;
  @AutoMap(() => c_explicit_object)
  m_object?: c_explicit_object;
  @AutoMap(() => c_explicit_team)
  m_team?: c_explicit_team;
  @AutoMap(() => Number)
  m_variable_index?: number;
  @AutoMap(() => Number)
  m_option_index?: number;
  @AutoMap(() => Number)
  m_statistic_index?: number;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum("type", 6, e_custom_variable_type);
    switch (this.m_type) {
      case e_custom_variable_type.constant:
        this.m_immediate_value = bitstream.read_signed_integer(
          "immediate-value",
          16
        );
        break;
      case e_custom_variable_type.player_number: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
        break;
      }
      case e_custom_variable_type.object_number: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
        break;
      }
      case e_custom_variable_type.team_number: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("variable-index", 1);
        break;
      }
      case e_custom_variable_type.global_number:
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
        break;
      case e_custom_variable_type.option:
        this.m_option_index = bitstream.read_integer("option-index", 4);
        break;
      case e_custom_variable_type.spawn_object: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      case e_custom_variable_type.team_score: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case e_custom_variable_type.player_score:
      case e_custom_variable_type.player_money:
      case e_custom_variable_type.player_rating: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case e_custom_variable_type.player_stat: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_statistic_index = bitstream.read_integer("statistic-index", 2);
        break;
      }
      case e_custom_variable_type.team_stat: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_statistic_index = bitstream.read_integer("statistic-index", 2);
        break;
      }
      default:
        break;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 6, e_custom_variable_type);
    switch (this.m_type) {
      case e_custom_variable_type.constant:
        bitstream.write_signed_integer(this.m_immediate_value ?? 0, 16);
        break;
      case e_custom_variable_type.player_number:
        this.m_player?.encode(bitstream);
        bitstream.write_integer(this.m_variable_index ?? 0, 3);
        break;
      case e_custom_variable_type.object_number:
        this.m_object?.encode(bitstream);
        bitstream.write_integer(this.m_variable_index ?? 0, 3);
        break;
      case e_custom_variable_type.team_number:
        this.m_team?.encode(bitstream);
        bitstream.write_integer(this.m_variable_index ?? 0, 1);
        break;
      case e_custom_variable_type.global_number:
        bitstream.write_integer(this.m_variable_index ?? 0, 3);
        break;
      case e_custom_variable_type.option:
        bitstream.write_integer(this.m_option_index ?? 0, 4);
        break;
      case e_custom_variable_type.spawn_object:
        this.m_object?.encode(bitstream);
        break;
      case e_custom_variable_type.team_score:
        this.m_team?.encode(bitstream);
        break;
      case e_custom_variable_type.player_score:
      case e_custom_variable_type.player_money:
      case e_custom_variable_type.player_rating:
        this.m_player?.encode(bitstream);
        break;
      case e_custom_variable_type.player_stat:
        this.m_player?.encode(bitstream);
        bitstream.write_integer(this.m_statistic_index ?? 0, 2);
        break;
      case e_custom_variable_type.team_stat:
        this.m_team?.encode(bitstream);
        bitstream.write_integer(this.m_statistic_index ?? 0, 2);
        break;
      default:
        break;
    }
  }

  is_writeable(): boolean {
    switch (this.m_type) {
      case e_custom_variable_type.player_number:
      case e_custom_variable_type.object_number:
      case e_custom_variable_type.team_number:
      case e_custom_variable_type.global_number:
      case e_custom_variable_type.team_score:
      case e_custom_variable_type.player_score:
      case e_custom_variable_type.player_money:
      case e_custom_variable_type.player_stat:
      case e_custom_variable_type.team_stat:
      case e_custom_variable_type.symmetric_gametype_pregame:
        return true;
      default:
        return false;
    }
  }
}
