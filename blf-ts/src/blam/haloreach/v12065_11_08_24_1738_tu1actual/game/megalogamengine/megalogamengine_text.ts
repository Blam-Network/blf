import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { BlfError } from "../../../../../error";
import { e_player_filter_type } from "./megalogamengine_enums";
import {
  c_custom_timer_reference,
  c_custom_variable_reference,
  c_object_reference,
  c_player_reference,
  c_team_reference,
} from "./megalogamengine_references";

function requireField<T>(value: T | undefined, message: string): T {
  if (value === undefined) {
    throw new BlfError(message);
  }
  return value;
}

export class c_player_filter_modifier {
  m_type: e_player_filter_type = e_player_filter_type.no_one;
  m_player?: c_player_reference;
  m_variable?: c_custom_variable_reference;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum("type", 3, e_player_filter_type);
    if (this.m_type === e_player_filter_type.specific_player) {
      const player = new c_player_reference();
      const variable = new c_custom_variable_reference();
      player.decode(bitstream);
      variable.decode(bitstream);
      this.m_player = player;
      this.m_variable = variable;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 3);
    if (this.m_type === e_player_filter_type.specific_player) {
      requireField(this.m_player, "m_player does not exist.").encode(bitstream);
      requireField(this.m_variable, "m_variable does not exist.").encode(
        bitstream
      );
    }
  }
}

export class c_replaceable_token {
  m_type = 0;
  m_player?: c_player_reference;
  m_object?: c_object_reference;
  m_team?: c_team_reference;
  m_custom_timer?: c_custom_timer_reference;
  m_custom_variable?: c_custom_variable_reference;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_integer("token-type", 3);

    switch (this.m_type) {
      case 1: {
        const player = new c_player_reference();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case 2: {
        const team = new c_team_reference();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case 3: {
        const object = new c_object_reference();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      case 4: {
        const customVariable = new c_custom_variable_reference();
        customVariable.decode(bitstream);
        this.m_custom_variable = customVariable;
        break;
      }
      case 5: {
        const customTimer = new c_custom_timer_reference();
        customTimer.decode(bitstream);
        this.m_custom_timer = customTimer;
        break;
      }
      default:
        break;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_type, 3);
    switch (this.m_type) {
      case 1:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case 2:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        break;
      case 3:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        break;
      case 4:
        requireField(
          this.m_custom_variable,
          "m_custom_variable does not exist."
        ).encode(bitstream);
        break;
      case 5:
        requireField(
          this.m_custom_timer,
          "m_custom_timer does not exist."
        ).encode(bitstream);
        break;
      default:
        throw new BlfError(`Invalid c_replaceable_token: type ${this.m_type}`);
    }
  }
}

export class c_dynamic_string {
  m_string_index = 0;
  m_tokens: c_replaceable_token[] = [];

  decode(bitstream: c_bitstream_reader): void {
    this.m_string_index = bitstream.read_integer("string-index", 7);
    const tokenCount = bitstream.read_integer("token-count", 2);
    for (let i = 0; i < tokenCount; i++) {
      const token = new c_replaceable_token();
      token.decode(bitstream);
      this.m_tokens.push(token);
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_string_index, 7);
    bitstream.write_integer(this.m_tokens.length, 2);
    for (const token of this.m_tokens) {
      token.encode(bitstream);
    }
  }
}
