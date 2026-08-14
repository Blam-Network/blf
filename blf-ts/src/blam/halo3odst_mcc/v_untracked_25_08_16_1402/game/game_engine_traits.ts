import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import { c_player_traits } from "./game_engine_player_traits";

function testBit(flags: number, bit: number): boolean {
  return (flags & (1 << bit)) !== 0;
}

function setBit(flags: number, bit: number, value: boolean): number {
  if (value) {
    return flags | (1 << bit);
  }
  return flags & ~(1 << bit);
}

export class c_game_engine_miscellaneous_options {
  @AutoMap(() => Number)
  m_flags = 0;
  @AutoMap(() => Number)
  m_round_time_limit_minutes = 0;
  @AutoMap(() => Number)
  m_round_limit = 0;
  @AutoMap(() => Number)
  m_early_victory_win_count = 0;

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(testBit(this.m_flags, 0));
    bitstream.write_bool(testBit(this.m_flags, 1));
    bitstream.write_bool(testBit(this.m_flags, 2));
    bitstream.write_integer(this.m_round_time_limit_minutes, 8);
    bitstream.write_integer(this.m_round_limit, 4);
    bitstream.write_integer(this.m_early_victory_win_count, 4);
  }

  decode(bitstream: c_bitstream_reader): void {
    this.m_flags = setBit(
      this.m_flags,
      0,
      bitstream.read_bool("miscellaneous-options-flag-0")
    );
    this.m_flags = setBit(
      this.m_flags,
      1,
      bitstream.read_bool("miscellaneous-options-flag-1")
    );
    this.m_flags = setBit(
      this.m_flags,
      2,
      bitstream.read_bool("miscellaneous-options-flag-2")
    );
    this.m_round_time_limit_minutes = bitstream.read_integer(
      "round-time-limit-minutes",
      8
    );
    this.m_round_limit = bitstream.read_integer("round-limit", 4);
    this.m_early_victory_win_count = bitstream.read_integer(
      "early-victory-win-count",
      4
    );
  }
}

export class c_game_engine_respawn_options {
  @AutoMap(() => Number)
  m_flags = 0;
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
  m_respawn_player_traits_duration_seconds = 0;
  @AutoMap(() => c_player_traits)
  m_respawn_player_traits = new c_player_traits();

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(testBit(this.m_flags, 0));
    bitstream.write_bool(testBit(this.m_flags, 1));
    bitstream.write_bool(testBit(this.m_flags, 2));
    bitstream.write_bool(testBit(this.m_flags, 3));
    bitstream.write_integer(this.m_lives_per_round, 6);
    bitstream.write_integer(this.m_team_lives_per_round, 7);
    bitstream.write_integer(this.m_respawn_time_seconds, 8);
    bitstream.write_integer(this.m_suicide_penalty_seconds, 8);
    bitstream.write_integer(this.m_betrayal_penalty_seconds, 8);
    bitstream.write_integer(this.m_respawn_growth_seconds, 4);
    bitstream.write_integer(this.m_respawn_player_traits_duration_seconds, 6);
    this.m_respawn_player_traits.encode(bitstream);
  }

  decode(bitstream: c_bitstream_reader): void {
    this.m_flags = setBit(
      this.m_flags,
      0,
      bitstream.read_bool("respawn-options-flag-0")
    );
    this.m_flags = setBit(
      this.m_flags,
      1,
      bitstream.read_bool("respawn-options-flag-1")
    );
    this.m_flags = setBit(
      this.m_flags,
      2,
      bitstream.read_bool("respawn-options-flag-2")
    );
    this.m_flags = setBit(
      this.m_flags,
      3,
      bitstream.read_bool("respawn-options-flag-3")
    );
    this.m_lives_per_round = bitstream.read_integer("lives-per-round", 6);
    this.m_team_lives_per_round = bitstream.read_integer(
      "team-lives-per-round",
      7
    );
    this.m_respawn_time_seconds = bitstream.read_integer(
      "respawn-time-seconds",
      8
    );
    this.m_suicide_penalty_seconds = bitstream.read_integer(
      "suicide-penalty-seconds",
      8
    );
    this.m_betrayal_penalty_seconds = bitstream.read_integer(
      "betrayal-penalty-seconds",
      8
    );
    this.m_respawn_growth_seconds = bitstream.read_integer(
      "respawn-growth-seconds",
      4
    );
    this.m_respawn_player_traits_duration_seconds = bitstream.read_integer(
      "respawn-player-traits-duration-seconds",
      6
    );
    this.m_respawn_player_traits.decode(bitstream);
  }
}
