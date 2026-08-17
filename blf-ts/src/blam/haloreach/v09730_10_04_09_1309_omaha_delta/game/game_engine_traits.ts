import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import { c_player_traits } from "./game_engine_player_traits";

/** Matches blf_lib omaha_delta `c_game_engine_miscellaneous_options`. */
export class c_game_engine_miscellaneous_options {
  @AutoMap(() => Boolean)
  m_teams_enabled = false;
  @AutoMap(() => Boolean)
  m_round_reset_players = false;
  @AutoMap(() => Boolean)
  m_round_reset_map = false;
  @AutoMap(() => Number)
  m_round_time_limit_minutes = 0;
  @AutoMap(() => Number)
  m_round_limit = 0;
  @AutoMap(() => Number)
  m_early_victory_win_count = 0;
  @AutoMap(() => Number)
  m_sudden_death_time = 0;
  @AutoMap(() => Number)
  m_grace_period = 0;
  initialize(): void {
    this.m_teams_enabled = false;
    this.m_round_reset_players = false;
    this.m_round_reset_map = true;
    this.m_round_time_limit_minutes = 8;
    this.m_round_limit = 1;
    this.m_early_victory_win_count = 2;
    this.m_sudden_death_time = 0;
    this.m_grace_period = 0;
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_teams_enabled = bitstream.read_bool("miscellaneous-options-teams");
    this.m_round_reset_players = bitstream.read_bool(
      "miscellaneous-options-round-reset-players"
    );
    this.m_round_reset_map = bitstream.read_bool(
      "miscellaneous-options-round-reset-map"
    );
    this.m_round_time_limit_minutes = bitstream.read_integer(
      "miscellaneous-options-round-time-limit-minutes",
      8
    );
    this.m_round_limit = bitstream.read_integer(
      "miscellaneous-options-round-limit",
      5
    );
    this.m_early_victory_win_count = bitstream.read_integer(
      "miscellaneous-options-early-victory-win-count",
      4
    );
    this.m_sudden_death_time =
      bitstream.read_integer("sudden-death-time-limit", 7) - 1;
    this.m_grace_period = bitstream.read_integer("grace-period", 5);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_teams_enabled);
    bitstream.write_bool(this.m_round_reset_players);
    bitstream.write_bool(this.m_round_reset_map);
    bitstream.write_integer(this.m_round_time_limit_minutes, 8);
    bitstream.write_integer(this.m_round_limit, 5);
    bitstream.write_integer(this.m_early_victory_win_count, 4);
    bitstream.write_integer(this.m_sudden_death_time + 1, 7);
    bitstream.write_integer(this.m_grace_period, 5);
  }
}

/** Matches blf_lib omaha_delta `c_game_engine_respawn_options`. */
export class c_game_engine_respawn_options {
  @AutoMap(() => Boolean)
  m_inherit_respawn_time = false;
  @AutoMap(() => Boolean)
  m_respawn_with_teammate = false;
  @AutoMap(() => Boolean)
  m_respawn_at_location = false;
  @AutoMap(() => Boolean)
  m_respawn_on_kills = false;
  @AutoMap(() => Number)
  m_lives_per_round = 0;
  @AutoMap(() => Number)
  m_team_lives_per_round = 0;
  @AutoMap(() => Number)
  m_respawn_time_seconds = 0;
  @AutoMap(() => Number)
  m_suicide_penalty_seconds = 0;
  @AutoMap(() => Number)
  m_betrayal_penalty_seconds = 0;
  @AutoMap(() => Number)
  m_respawn_growth_seconds = 0;
  @AutoMap(() => Number)
  m_loadout_cam_time = 0;
  @AutoMap(() => Number)
  m_respawn_player_traits_duration_seconds = 0;
  @AutoMap(() => c_player_traits)
  m_respawn_player_traits = new c_player_traits();
  initialize(): void {
    this.m_inherit_respawn_time = false;
    this.m_respawn_with_teammate = false;
    this.m_respawn_at_location = false;
    this.m_respawn_on_kills = false;
    this.m_lives_per_round = 0;
    this.m_team_lives_per_round = 0;
    this.m_respawn_time_seconds = 5;
    this.m_suicide_penalty_seconds = 5;
    this.m_betrayal_penalty_seconds = 5;
    this.m_respawn_growth_seconds = 0;
    this.m_loadout_cam_time = 10;
    this.m_respawn_player_traits_duration_seconds = 5;
    this.m_respawn_player_traits = new c_player_traits();
    this.m_respawn_player_traits.clear();
  }
  decode(bitstream: c_bitstream_reader): void {
    this.m_inherit_respawn_time = bitstream.read_bool(
      "respawn-options-inherit-respawn-time"
    );
    this.m_respawn_with_teammate = bitstream.read_bool(
      "respawn-options-respawn-with-teammate"
    );
    this.m_respawn_at_location = bitstream.read_bool(
      "respawn-options-respawn-at-location"
    );
    this.m_respawn_on_kills = bitstream.read_bool(
      "respawn-options-respawn-on-kills"
    );
    this.m_lives_per_round = bitstream.read_integer(
      "respawn-options-lives-per-round",
      6
    );
    this.m_team_lives_per_round = bitstream.read_integer(
      "respawn-options-team-lives-per-round",
      7
    );
    this.m_respawn_time_seconds = bitstream.read_integer(
      "respawn-options-respawn-time",
      8
    );
    this.m_suicide_penalty_seconds = bitstream.read_integer(
      "respawn-options-suicide-time",
      8
    );
    this.m_betrayal_penalty_seconds = bitstream.read_integer(
      "respawn-options-betrayal-time",
      8
    );
    this.m_respawn_growth_seconds = bitstream.read_integer(
      "respawn-options-respawn-growth-time",
      4
    );
    this.m_loadout_cam_time = bitstream.read_integer(
      "respawn-options-initial-loadout-selection-time",
      4
    );
    this.m_respawn_player_traits_duration_seconds = bitstream.read_integer(
      "respawn-options-player-traits-duration",
      6
    );
    this.m_respawn_player_traits.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(this.m_inherit_respawn_time);
    bitstream.write_bool(this.m_respawn_with_teammate);
    bitstream.write_bool(this.m_respawn_at_location);
    bitstream.write_bool(this.m_respawn_on_kills);
    bitstream.write_integer(this.m_lives_per_round, 6);
    bitstream.write_integer(this.m_team_lives_per_round, 7);
    bitstream.write_integer(this.m_respawn_time_seconds, 8);
    bitstream.write_integer(this.m_suicide_penalty_seconds, 8);
    bitstream.write_integer(this.m_betrayal_penalty_seconds, 8);
    bitstream.write_integer(this.m_respawn_growth_seconds, 4);
    bitstream.write_integer(this.m_loadout_cam_time, 4);
    bitstream.write_integer(this.m_respawn_player_traits_duration_seconds, 6);
    this.m_respawn_player_traits.encode(bitstream);
  }
}

/** Matches blf_lib omaha_delta `s_player_trait_option`. */
export class s_player_trait_option {
  @AutoMap(() => Number)
  m_name_string_index = 0;
  @AutoMap(() => Number)
  m_description_string_index = 0;
  @AutoMap(() => c_player_traits)
  m_player_traits = new c_player_traits();
  decode(bitstream: c_bitstream_reader): void {
    this.m_name_string_index = bitstream.read_integer("name-string-index", 7);
    this.m_description_string_index = bitstream.read_integer(
      "description-string-index",
      7
    );
    this.m_player_traits.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_name_string_index, 7);
    bitstream.write_integer(this.m_description_string_index, 7);
    this.m_player_traits.encode(bitstream);
  }
}
