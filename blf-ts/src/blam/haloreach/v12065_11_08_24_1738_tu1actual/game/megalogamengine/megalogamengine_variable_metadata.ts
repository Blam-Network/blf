import { type c, CFieldType } from "@craftycodie/cstruct";
import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { e_multiplayer_team_designator } from "../game_engine_default";
import {
  c_custom_variable_reference,
  custom_variable_reference_field,
} from "./megalogamengine_references";

/** Variable replication mode (`network-state`, 2 bits). */
export enum e_megalo_variable_network_state {
  local = 0,
  networked = 1,
  networked_high = 2,
}

export class s_variable_metadata {
  m_numeric_variables: [
    c_custom_variable_reference,
    e_megalo_variable_network_state,
  ][] = [];
  m_timer_variables: c_custom_variable_reference[] = [];
  m_team_variables: [
    e_multiplayer_team_designator,
    e_megalo_variable_network_state,
  ][] = [];
  m_player_variables: e_megalo_variable_network_state[] = [];
  m_object_variables: e_megalo_variable_network_state[] = [];
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
      const network_state = bitstream.read_enum(
        "network-state",
        2,
        e_megalo_variable_network_state
      );
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
      const team_variable_value = bitstream.read_enum(
        "team-variable-value",
        4,
        e_multiplayer_team_designator
      );
      const network_state = bitstream.read_enum(
        "network-state",
        2,
        e_megalo_variable_network_state
      );
      this.m_team_variables.push([team_variable_value, network_state]);
    }
    const player_variable_count = bitstream.read_integer(
      "player-variable-count",
      this.player_variable_count_bits
    );
    for (let i = 0; i < player_variable_count; i++) {
      this.m_player_variables.push(
        bitstream.read_enum("network-state", 2, e_megalo_variable_network_state)
      );
    }
    const object_variable_count = bitstream.read_integer(
      "object-variable-count",
      this.object_variable_count_bits
    );
    for (let i = 0; i < object_variable_count; i++) {
      this.m_object_variables.push(
        bitstream.read_enum("network-state", 2, e_megalo_variable_network_state)
      );
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(
      this.m_numeric_variables.length,
      this.numeric_variable_count_bits
    );
    for (const [numeric_variable, network_state] of this.m_numeric_variables) {
      numeric_variable.encode(bitstream);
      bitstream.write_enum(network_state, 2, e_megalo_variable_network_state);
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
      bitstream.write_enum(team_variable, 4, e_multiplayer_team_designator);
      bitstream.write_enum(network_state, 2, e_megalo_variable_network_state);
    }
    bitstream.write_integer(
      this.m_player_variables.length,
      this.player_variable_count_bits
    );
    for (const network_state of this.m_player_variables) {
      bitstream.write_enum(network_state, 2, e_megalo_variable_network_state);
    }
    bitstream.write_integer(
      this.m_object_variables.length,
      this.object_variable_count_bits
    );
    for (const network_state of this.m_object_variables) {
      bitstream.write_enum(network_state, 2, e_megalo_variable_network_state);
    }
  }
}
export function s_variable_metadata_global(): s_variable_metadata {
  return new s_variable_metadata(4, 4, 4, 4, 5);
}
export function s_variable_metadata_player(): s_variable_metadata {
  return new s_variable_metadata(4, 3, 3, 3, 3);
}
export function s_variable_metadata_object(): s_variable_metadata {
  return new s_variable_metadata(4, 3, 2, 3, 3);
}
export function s_variable_metadata_team(): s_variable_metadata {
  return new s_variable_metadata(4, 3, 3, 3, 3);
}

interface s_variable_metadata_layout {
  numeric_variables: number;
  timer_variables: number;
  team_variables: number;
  team_variables_padding: number;
  player_variables: number;
  object_variables: number;
  team_variable_network_states_padding: number;
}

export class CVariableMetadataField extends CFieldType<s_variable_metadata> {
  private readonly reference = custom_variable_reference_field();
  private readonly offsets: {
    numeric_variables: number;
    timer_count: number;
    timer_variables: number;
    team_count: number;
    team_variables: number;
    numeric_variable_network_states: number;
    player_variable_network_states: number;
    object_variable_network_states: number;
    team_variable_network_states: number;
    player_count: number;
    object_count: number;
    size: number;
  };

  constructor(
    private readonly create: () => s_variable_metadata,
    private readonly layout: s_variable_metadata_layout
  ) {
    super();
    const numeric_variables = 4;
    const timer_count = numeric_variables + layout.numeric_variables * 3;
    const timer_variables = timer_count + 4;
    const team_count = timer_variables + layout.timer_variables * 3;
    const team_variables = team_count + 4;
    const numeric_variable_network_states =
      team_variables + layout.team_variables + layout.team_variables_padding;
    const player_variable_network_states =
      numeric_variable_network_states + layout.numeric_variables;
    const object_variable_network_states =
      player_variable_network_states + layout.player_variables;
    const team_variable_network_states =
      object_variable_network_states + layout.object_variables;
    const player_count =
      team_variable_network_states +
      layout.team_variables +
      layout.team_variable_network_states_padding;
    this.offsets = {
      numeric_variables,
      timer_count,
      timer_variables,
      team_count,
      team_variables,
      numeric_variable_network_states,
      player_variable_network_states,
      object_variable_network_states,
      team_variable_network_states,
      player_count,
      object_count: player_count + 4,
      size: player_count + 8,
    };
  }

  getSize(): number {
    return this.offsets.size;
  }

  read(
    bytes: Uint8Array,
    offset: number,
    endian: c.Endian
  ): s_variable_metadata {
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const little = endian === "little";
    const at = this.offsets;
    const count = (position: number, capacity: number) =>
      Math.max(0, Math.min(capacity, view.getInt32(offset + position, little)));
    const value = this.create();
    const numeric_count = count(0, this.layout.numeric_variables);
    for (let i = 0; i < numeric_count; i++) {
      value.m_numeric_variables.push([
        this.reference.read(
          bytes,
          offset + at.numeric_variables + i * 3,
          endian
        ),
        view.getUint8(offset + at.numeric_variable_network_states + i),
      ]);
    }
    const timer_count = count(at.timer_count, this.layout.timer_variables);
    for (let i = 0; i < timer_count; i++) {
      value.m_timer_variables.push(
        this.reference.read(bytes, offset + at.timer_variables + i * 3, endian)
      );
    }
    const team_count = count(at.team_count, this.layout.team_variables);
    for (let i = 0; i < team_count; i++) {
      value.m_team_variables.push([
        view.getInt8(offset + at.team_variables + i),
        view.getUint8(offset + at.team_variable_network_states + i),
      ]);
    }
    const player_count = count(at.player_count, this.layout.player_variables);
    for (let i = 0; i < player_count; i++) {
      value.m_player_variables.push(
        view.getUint8(offset + at.player_variable_network_states + i)
      );
    }
    const object_count = count(at.object_count, this.layout.object_variables);
    for (let i = 0; i < object_count; i++) {
      value.m_object_variables.push(
        view.getUint8(offset + at.object_variable_network_states + i)
      );
    }
    return value;
  }

  write(
    bytes: Uint8Array,
    offset: number,
    value: s_variable_metadata,
    endian: c.Endian
  ): void {
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const little = endian === "little";
    const at = this.offsets;
    const numeric = value.m_numeric_variables.slice(
      0,
      this.layout.numeric_variables
    );
    view.setInt32(offset, numeric.length, little);
    numeric.forEach(([reference, network_state], i) => {
      this.reference.write(
        bytes,
        offset + at.numeric_variables + i * 3,
        reference,
        endian
      );
      view.setUint8(
        offset + at.numeric_variable_network_states + i,
        network_state
      );
    });
    const timers = value.m_timer_variables.slice(
      0,
      this.layout.timer_variables
    );
    view.setInt32(offset + at.timer_count, timers.length, little);
    timers.forEach((reference, i) => {
      this.reference.write(
        bytes,
        offset + at.timer_variables + i * 3,
        reference,
        endian
      );
    });
    const teams = value.m_team_variables.slice(0, this.layout.team_variables);
    view.setInt32(offset + at.team_count, teams.length, little);
    teams.forEach(([team, network_state], i) => {
      view.setInt8(offset + at.team_variables + i, team);
      view.setUint8(
        offset + at.team_variable_network_states + i,
        network_state
      );
    });
    const players = value.m_player_variables.slice(
      0,
      this.layout.player_variables
    );
    view.setInt32(offset + at.player_count, players.length, little);
    players.forEach((network_state, i) => {
      view.setUint8(
        offset + at.player_variable_network_states + i,
        network_state
      );
    });
    const objects = value.m_object_variables.slice(
      0,
      this.layout.object_variables
    );
    view.setInt32(offset + at.object_count, objects.length, little);
    objects.forEach((network_state, i) => {
      view.setUint8(
        offset + at.object_variable_network_states + i,
        network_state
      );
    });
  }
}

export const variable_metadata_global_field = () =>
  new CVariableMetadataField(s_variable_metadata_global, {
    numeric_variables: 12,
    timer_variables: 8,
    team_variables: 8,
    team_variables_padding: 0,
    player_variables: 8,
    object_variables: 16,
    team_variable_network_states_padding: 0,
  });

export const variable_metadata_player_field = () =>
  new CVariableMetadataField(s_variable_metadata_player, {
    numeric_variables: 8,
    timer_variables: 4,
    team_variables: 4,
    team_variables_padding: 0,
    player_variables: 4,
    object_variables: 4,
    team_variable_network_states_padding: 0,
  });

export const variable_metadata_object_field = () =>
  new CVariableMetadataField(s_variable_metadata_object, {
    numeric_variables: 8,
    timer_variables: 4,
    team_variables: 2,
    team_variables_padding: 2,
    player_variables: 4,
    object_variables: 4,
    team_variable_network_states_padding: 2,
  });

export const variable_metadata_team_field = () =>
  new CVariableMetadataField(s_variable_metadata_team, {
    numeric_variables: 8,
    timer_variables: 4,
    team_variables: 4,
    team_variables_padding: 0,
    player_variables: 4,
    object_variables: 6,
    team_variable_network_states_padding: 2,
  });
