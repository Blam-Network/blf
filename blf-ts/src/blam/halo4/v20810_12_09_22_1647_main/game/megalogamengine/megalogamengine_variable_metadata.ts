import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";
import { e_multiplayer_team_designator } from "../game_engine_default";
import { c_custom_variable_reference } from "./megalogamengine_references";

/** Variable replication mode (`network-state`, 2 bits). Halo 4 count=3. */
export enum e_megalo_variable_network_state {
  local = 0,
  networked = 1,
  networked_high = 2,
}

/** Halo 4 `VariableFlags`: network-state + is-persistent. */
export class s_megalo_variable_flags {
  @AutoMap(() => Number)
  m_network_state: e_megalo_variable_network_state =
    e_megalo_variable_network_state.local;
  @AutoMap(() => Boolean)
  m_is_persistent = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_network_state = bitstream.read_enum(
      "network-state",
      2,
      e_megalo_variable_network_state
    );
    this.m_is_persistent = bitstream.read_bool("is-persistent");
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(
      this.m_network_state,
      2,
      e_megalo_variable_network_state
    );
    bitstream.write_bool(this.m_is_persistent);
  }
}

export class s_variable_metadata {
  @AutoMap(() => [c_custom_variable_reference])
  m_numeric_variables: [
    c_custom_variable_reference,
    s_megalo_variable_flags,
  ][] = [];
  @AutoMap(() => [c_custom_variable_reference])
  m_timer_variables: c_custom_variable_reference[] = [];
  @AutoMap(() => [Number])
  m_team_variables: [e_multiplayer_team_designator, s_megalo_variable_flags][] =
    [];
  @AutoMap(() => [s_megalo_variable_flags])
  m_player_variables: s_megalo_variable_flags[] = [];
  @AutoMap(() => [s_megalo_variable_flags])
  m_object_variables: s_megalo_variable_flags[] = [];
  constructor(
    private readonly numeric_variable_count_bits: number,
    private readonly timer_variable_count_bits: number,
    private readonly team_variable_count_bits: number,
    private readonly player_variable_count_bits: number,
    private readonly object_variable_count_bits: number
  ) {}
  decode(bitstream: c_bitstream_reader): void {
    const numeric_variable_count = bitstream.read_integer(
      "numeric-variable-count",
      this.numeric_variable_count_bits
    );
    for (let i = 0; i < numeric_variable_count; i++) {
      const numeric_variable = new c_custom_variable_reference();
      numeric_variable.decode(bitstream);
      const flags = new s_megalo_variable_flags();
      flags.decode(bitstream);
      this.m_numeric_variables.push([numeric_variable, flags]);
    }
    const timer_variable_count = bitstream.read_integer(
      "timer-count",
      this.timer_variable_count_bits
    );
    for (let i = 0; i < timer_variable_count; i++) {
      const timer_variable = new c_custom_variable_reference();
      timer_variable.decode(bitstream);
      this.m_timer_variables.push(timer_variable);
    }
    const team_variable_count = bitstream.read_integer(
      "team-variable-count",
      this.team_variable_count_bits
    );
    for (let i = 0; i < team_variable_count; i++) {
      const team_variable_value = bitstream.read_enum(
        "team-variable-value",
        4,
        e_multiplayer_team_designator
      );
      const flags = new s_megalo_variable_flags();
      flags.decode(bitstream);
      this.m_team_variables.push([team_variable_value, flags]);
    }
    const player_variable_count = bitstream.read_integer(
      "player-variable-count",
      this.player_variable_count_bits
    );
    for (let i = 0; i < player_variable_count; i++) {
      const flags = new s_megalo_variable_flags();
      flags.decode(bitstream);
      this.m_player_variables.push(flags);
    }
    const object_variable_count = bitstream.read_integer(
      "object-variable-count",
      this.object_variable_count_bits
    );
    for (let i = 0; i < object_variable_count; i++) {
      const flags = new s_megalo_variable_flags();
      flags.decode(bitstream);
      this.m_object_variables.push(flags);
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(
      this.m_numeric_variables.length,
      this.numeric_variable_count_bits
    );
    for (const [numeric_variable, flags] of this.m_numeric_variables) {
      numeric_variable.encode(bitstream);
      flags.encode(bitstream);
    }
    bitstream.write_integer(
      this.m_timer_variables.length,
      this.timer_variable_count_bits
    );
    for (const timer_variable of this.m_timer_variables) {
      timer_variable.encode(bitstream);
    }
    bitstream.write_integer(
      this.m_team_variables.length,
      this.team_variable_count_bits
    );
    for (const [team_variable, flags] of this.m_team_variables) {
      bitstream.write_enum(team_variable, 4, e_multiplayer_team_designator);
      flags.encode(bitstream);
    }
    bitstream.write_integer(
      this.m_player_variables.length,
      this.player_variable_count_bits
    );
    for (const flags of this.m_player_variables) {
      flags.encode(bitstream);
    }
    bitstream.write_integer(
      this.m_object_variables.length,
      this.object_variable_count_bits
    );
    for (const flags of this.m_object_variables) {
      flags.encode(bitstream);
    }
  }
}
export function s_variable_metadata_global(): s_variable_metadata {
  // MegaloLimits: num20, timer8, team8, player10, object18
  return new s_variable_metadata(5, 4, 4, 4, 5);
}
export function s_variable_metadata_player(): s_variable_metadata {
  // MegaloLimits: num10, timer4, team4, player4, object4
  return new s_variable_metadata(4, 3, 3, 3, 3);
}
export function s_variable_metadata_object(): s_variable_metadata {
  // MegaloLimits: num10, timer4, team2, player4, object6
  return new s_variable_metadata(4, 3, 2, 3, 3);
}
export function s_variable_metadata_team(): s_variable_metadata {
  // MegaloLimits: num10, timer4, team4, player4, object6
  return new s_variable_metadata(4, 3, 3, 3, 3);
}
