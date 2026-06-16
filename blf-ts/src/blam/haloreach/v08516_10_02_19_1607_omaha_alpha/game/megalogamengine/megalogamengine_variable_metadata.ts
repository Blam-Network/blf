import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";
import { c_custom_variable_reference } from "./megalogamengine_custom_variable_reference";

export class s_variable_metadata {
  @AutoMap(() => [c_custom_variable_reference])
  m_numeric_variables: [c_custom_variable_reference, number][] = [];
  @AutoMap(() => [c_custom_variable_reference])
  m_timer_variables: c_custom_variable_reference[] = [];
  @AutoMap(() => [Number])
  m_team_variables: [number, number][] = [];
  @AutoMap(() => [Number])
  m_player_variables: number[] = [];
  @AutoMap(() => [Number])
  m_object_variables: number[] = [];

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
      const network_state = bitstream.read_integer("network-state", 2);
      this.m_numeric_variables.push([numeric_variable, network_state]);
    }
    const timer_variable_count = bitstream.read_integer(
      "timer-variable-count",
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
      const team_variable_value = bitstream.read_integer(
        "team-variable-value",
        4
      );
      const network_state = bitstream.read_integer("network-state", 2);
      this.m_team_variables.push([team_variable_value, network_state]);
    }
    const player_variable_count = bitstream.read_integer(
      "player-variable-count",
      this.player_variable_count_bits
    );
    for (let i = 0; i < player_variable_count; i++) {
      this.m_player_variables.push(bitstream.read_integer("network-state", 2));
    }
    const object_variable_count = bitstream.read_integer(
      "object-variable-count",
      this.object_variable_count_bits
    );
    for (let i = 0; i < object_variable_count; i++) {
      this.m_object_variables.push(bitstream.read_integer("network-state", 2));
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(
      this.m_numeric_variables.length,
      this.numeric_variable_count_bits
    );
    for (const [numeric_variable, network_state] of this.m_numeric_variables) {
      numeric_variable.encode(bitstream);
      bitstream.write_integer(network_state, 2);
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
    for (const [team_variable, network_state] of this.m_team_variables) {
      bitstream.write_integer(team_variable, 4);
      bitstream.write_integer(network_state, 2);
    }
    bitstream.write_integer(
      this.m_player_variables.length,
      this.player_variable_count_bits
    );
    for (const network_state of this.m_player_variables) {
      bitstream.write_integer(network_state, 2);
    }
    bitstream.write_integer(
      this.m_object_variables.length,
      this.object_variable_count_bits
    );
    for (const network_state of this.m_object_variables) {
      bitstream.write_integer(network_state, 2);
    }
  }
}

export function s_variable_metadata_global(): s_variable_metadata {
  return new s_variable_metadata(4, 4, 4, 4, 5);
}
export function s_variable_metadata_player(): s_variable_metadata {
  return new s_variable_metadata(4, 3, 2, 2, 3);
}
export function s_variable_metadata_object(): s_variable_metadata {
  return new s_variable_metadata(4, 3, 2, 3, 3);
}
export function s_variable_metadata_team(): s_variable_metadata {
  return new s_variable_metadata(2, 2, 2, 2, 3);
}
