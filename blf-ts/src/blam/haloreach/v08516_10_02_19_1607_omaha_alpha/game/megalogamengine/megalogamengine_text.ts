import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { BlfError } from "../../../../../error";
import { AutoMap } from "../../../../../helpers/automap";
import { e_player_filter_type } from "./megalogamengine_actions";
import { c_custom_timer_reference } from "./megalogamengine_custom_timer_reference";
import { c_custom_variable_reference } from "./megalogamengine_custom_variable_reference";
import { c_object_reference } from "./megalogamengine_object_reference";
import { c_player_reference } from "./megalogamengine_player_reference";
import { c_team_reference } from "./megalogamengine_team_reference";

function requireField<T>(value: T | undefined, message: string): T {
  if (value === undefined) {
    throw new BlfError(message);
  }
  return value;
}

export class c_player_filter_modifier {
  @AutoMap(() => e_player_filter_type)
  m_type: e_player_filter_type = e_player_filter_type.no_one;
  @AutoMap(() => c_player_reference)
  m_player?: c_player_reference;
  @AutoMap(() => c_custom_variable_reference)
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
    bitstream.write_enum(this.m_type, 3, e_player_filter_type);
    if (this.m_type === e_player_filter_type.specific_player) {
      requireField(this.m_player, "m_player does not exist.").encode(bitstream);
      requireField(this.m_variable, "m_variable does not exist.").encode(
        bitstream
      );
    }
  }
}

export enum e_replaceable_token_type {
  none = -1,
  player = 0,
  team = 1,
  object = 2,
  custom_variable = 3,
  custom_timer = 4,
}

export class c_replaceable_token {
  @AutoMap(() => e_replaceable_token_type)
  m_type: e_replaceable_token_type = e_replaceable_token_type.player;
  @AutoMap(() => c_player_reference)
  m_player?: c_player_reference;
  @AutoMap(() => c_object_reference)
  m_object?: c_object_reference;
  @AutoMap(() => c_team_reference)
  m_team?: c_team_reference;
  @AutoMap(() => c_custom_timer_reference)
  m_custom_timer?: c_custom_timer_reference;
  @AutoMap(() => c_custom_variable_reference)
  m_custom_variable?: c_custom_variable_reference;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum(
      "token-type",
      3,
      e_replaceable_token_type
    );
    switch (this.m_type) {
      case e_replaceable_token_type.player: {
        const player = new c_player_reference();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case e_replaceable_token_type.team: {
        const team = new c_team_reference();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case e_replaceable_token_type.object: {
        const object = new c_object_reference();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      case e_replaceable_token_type.custom_variable: {
        const customVariable = new c_custom_variable_reference();
        customVariable.decode(bitstream);
        this.m_custom_variable = customVariable;
        break;
      }
      case e_replaceable_token_type.custom_timer: {
        const customTimer = new c_custom_timer_reference();
        customTimer.decode(bitstream);
        this.m_custom_timer = customTimer;
        break;
      }
      case e_replaceable_token_type.none:
        break;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 3, e_replaceable_token_type);
    switch (this.m_type) {
      case e_replaceable_token_type.player:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case e_replaceable_token_type.team:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        break;
      case e_replaceable_token_type.object:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        break;
      case e_replaceable_token_type.custom_variable:
        requireField(
          this.m_custom_variable,
          "m_custom_variable does not exist."
        ).encode(bitstream);
        break;
      case e_replaceable_token_type.custom_timer:
        requireField(
          this.m_custom_timer,
          "m_custom_timer does not exist."
        ).encode(bitstream);
        break;
      case e_replaceable_token_type.none:
        break;
      default:
        throw new BlfError(`Invalid c_replaceable_token: type ${this.m_type}`);
    }
  }
}

export class c_dynamic_string {
  @AutoMap(() => Number)
  m_string_index = 0;
  @AutoMap(() => [c_replaceable_token])
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
