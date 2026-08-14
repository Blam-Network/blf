import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import { s_content_item_metadata } from "../saved_games/saved_game_files";
import { c_player_traits } from "./game_engine_player_traits";
import {
  c_game_engine_miscellaneous_options,
  c_game_engine_respawn_options,
} from "./game_engine_traits";

function testBit(flags: number, bit: number): boolean {
  return (flags & (1 << bit)) !== 0;
}

function setBit(flags: number, bit: number, value: boolean): number {
  if (value) {
    return flags | (1 << bit);
  }
  return flags & ~(1 << bit);
}

export const k_game_engine_type_count = 12;

/** Port of Halo 3 12070 `c_game_engine_social_options`. */
export class c_game_engine_social_options {
  @AutoMap(() => Number)
  m_flags = 0;
  @AutoMap(() => Number)
  m_team_changing = 0;

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(false);
    bitstream.write_integer(this.m_team_changing, 2);
    bitstream.write_bool(testBit(this.m_flags, 0));
    bitstream.write_bool(testBit(this.m_flags, 1));
    bitstream.write_bool(testBit(this.m_flags, 2));
    bitstream.write_bool(testBit(this.m_flags, 3));
    bitstream.write_bool(testBit(this.m_flags, 4));
  }

  decode(bitstream: c_bitstream_reader): void {
    bitstream.seek_relative(1);
    this.m_team_changing = bitstream.read_integer("team-changing", 2);
    this.m_flags = setBit(
      this.m_flags,
      0,
      bitstream.read_bool("social-options-flag-0")
    );
    this.m_flags = setBit(
      this.m_flags,
      1,
      bitstream.read_bool("social-options-flag-1")
    );
    this.m_flags = setBit(
      this.m_flags,
      2,
      bitstream.read_bool("social-options-flag-2")
    );
    this.m_flags = setBit(
      this.m_flags,
      3,
      bitstream.read_bool("social-options-flag-3")
    );
    this.m_flags = setBit(
      this.m_flags,
      4,
      bitstream.read_bool("social-options-flag-4")
    );
  }
}

/** Port of Halo 3 12070 `c_game_engine_map_override_options`. */
export class c_game_engine_map_override_options {
  @AutoMap(() => Number)
  m_flags = 0;
  @AutoMap(() => c_player_traits)
  m_base_player_traits = new c_player_traits();
  @AutoMap(() => Number)
  m_weapon_set_absolute_index = 0;
  @AutoMap(() => Number)
  m_vehicle_set_absolute_index = 0;
  @AutoMap(() => c_player_traits)
  m_red_powerup_traits = new c_player_traits();
  @AutoMap(() => c_player_traits)
  m_blue_powerup_traits = new c_player_traits();
  @AutoMap(() => c_player_traits)
  m_yellow_powerup_traits = new c_player_traits();
  @AutoMap(() => Number)
  m_red_powerup_duration_seconds = 0;
  @AutoMap(() => Number)
  m_blue_powerup_duration_seconds = 0;
  @AutoMap(() => Number)
  m_yellow_powerup_duration_seconds = 0;

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_bool(testBit(this.m_flags, 0));
    bitstream.write_bool(testBit(this.m_flags, 1));
    this.m_base_player_traits.encode(bitstream);
    bitstream.write_signed_integer(this.m_weapon_set_absolute_index, 8);
    bitstream.write_signed_integer(this.m_vehicle_set_absolute_index, 8);
    this.m_red_powerup_traits.encode(bitstream);
    this.m_blue_powerup_traits.encode(bitstream);
    this.m_yellow_powerup_traits.encode(bitstream);
    bitstream.write_integer(this.m_red_powerup_duration_seconds, 7);
    bitstream.write_integer(this.m_blue_powerup_duration_seconds, 7);
    bitstream.write_integer(this.m_yellow_powerup_duration_seconds, 7);
  }

  decode(bitstream: c_bitstream_reader): void {
    this.m_flags = setBit(
      this.m_flags,
      0,
      bitstream.read_bool("map-override-flag-0")
    );
    this.m_flags = setBit(
      this.m_flags,
      1,
      bitstream.read_bool("map-override-flag-1")
    );
    this.m_base_player_traits.decode(bitstream);
    this.m_weapon_set_absolute_index = bitstream.read_signed_integer(
      "weapon-set-absolute-index",
      8
    );
    this.m_vehicle_set_absolute_index = bitstream.read_signed_integer(
      "vehicle-set-absolute-index",
      8
    );
    this.m_red_powerup_traits.decode(bitstream);
    this.m_blue_powerup_traits.decode(bitstream);
    this.m_yellow_powerup_traits.decode(bitstream);
    this.m_red_powerup_duration_seconds = bitstream.read_integer(
      "red-powerup-duration-seconds",
      7
    );
    this.m_blue_powerup_duration_seconds = bitstream.read_integer(
      "blue-powerup-duration-seconds",
      7
    );
    this.m_yellow_powerup_duration_seconds = bitstream.read_integer(
      "yellow-powerup-duration-seconds",
      7
    );
  }
}

export class c_game_engine_base_variant {
  @AutoMap(() => Number)
  m_checksum = 0;
  @AutoMap(() => s_content_item_metadata)
  m_metadata = new s_content_item_metadata();
  @AutoMap(() => c_game_engine_miscellaneous_options)
  m_miscellaneous_options = new c_game_engine_miscellaneous_options();
  @AutoMap(() => c_game_engine_respawn_options)
  m_respawn_options = new c_game_engine_respawn_options();
  @AutoMap(() => c_game_engine_social_options)
  m_social_options = new c_game_engine_social_options();
  @AutoMap(() => c_game_engine_map_override_options)
  m_map_override_options = new c_game_engine_map_override_options();
  @AutoMap(() => Number)
  m_flags = 0;
  @AutoMap(() => Number)
  m_team_scoring_method = 0;

  encode(bitstream: c_bitstream_writer): void {
    this.m_metadata.encode(bitstream);
    bitstream.write_integer(this.m_flags, 1);
    this.m_miscellaneous_options.encode(bitstream);
    this.m_respawn_options.encode(bitstream);
    this.m_social_options.encode(bitstream);
    this.m_map_override_options.encode(bitstream);
    bitstream.write_integer(this.m_team_scoring_method, 3);
  }

  decode(bitstream: c_bitstream_reader): void {
    this.m_metadata.decode(bitstream);
    this.m_flags = bitstream.read_integer("base-variant-flags", 1);
    this.m_miscellaneous_options.decode(bitstream);
    this.m_respawn_options.decode(bitstream);
    this.m_social_options.decode(bitstream);
    this.m_map_override_options.decode(bitstream);
    this.m_team_scoring_method = bitstream.read_integer(
      "team-scoring-method",
      3
    );
  }
}
