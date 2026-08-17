import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
import { c_player_traits } from "./game_engine_player_traits";
import { c_game_engine_custom_variant } from "./game_variant";

/** Matches `e_sandbox_variant_flags` in blf_lib omaha_alpha `game_engine_sandbox.rs`. */
export class e_sandbox_variant_flags {
  @AutoMap(() => Boolean)
  open_channel_voice = false;
  @AutoMap(() => Boolean)
  requires_all_objects = false;
  to_raw(): number {
    return (
      (this.open_channel_voice ? 1 : 0) |
      (this.requires_all_objects ? 1 << 1 : 0)
    );
  }
  static from_raw(raw: number): e_sandbox_variant_flags {
    const flags = new e_sandbox_variant_flags();
    flags.open_channel_voice = (raw & 1) !== 0;
    flags.requires_all_objects = (raw & (1 << 1)) !== 0;
    return flags;
  }
}

/** Matches `e_sandbox_edit_mode_settings` in blf_lib omaha_alpha `game_engine_sandbox.rs`. */
export enum e_sandbox_edit_mode_settings {
  all_players = 0,
  only_leader = 1,
}

/** Matches `c_game_engine_sandbox_variant` in blf_lib omaha_alpha `game_engine_sandbox.rs`. */
export class c_game_engine_sandbox_variant {
  @AutoMap(() => c_game_engine_custom_variant)
  m_custom_variant = new c_game_engine_custom_variant();
  @AutoMap(() => e_sandbox_variant_flags)
  m_variant_flags = new e_sandbox_variant_flags();
  @AutoMap(() => e_sandbox_edit_mode_settings)
  m_edit_mode: e_sandbox_edit_mode_settings =
    e_sandbox_edit_mode_settings.all_players;
  @AutoMap(() => Number)
  m_respawn_time = 0;
  @AutoMap(() => c_player_traits)
  m_editor_traits = new c_player_traits();

  decode(bitstream: c_bitstream_reader): void {
    this.m_custom_variant.decode(bitstream);
    this.m_variant_flags = e_sandbox_variant_flags.from_raw(
      bitstream.read_integer("variant-flags", 2)
    );
    this.m_edit_mode = bitstream.read_enum(
      "edit-mode",
      2,
      e_sandbox_edit_mode_settings
    );
    this.m_respawn_time = bitstream.read_integer("respawn-time", 6);
    this.m_editor_traits.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_custom_variant.encode(bitstream);
    bitstream.write_integer(this.m_variant_flags.to_raw(), 2);
    bitstream.write_enum(this.m_edit_mode, 2, e_sandbox_edit_mode_settings);
    bitstream.write_integer(this.m_respawn_time, 6);
    this.m_editor_traits.encode(bitstream);
  }
}
