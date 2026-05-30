import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { BlfError } from "../../../../../error";
import { AutoMap } from "../../../../../helpers/automap";
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
export class s_variant_variable {
  @AutoMap(() => Number)
  m_type = 0;
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
    this.m_type = bitstream.read_integer("type", 3);
    switch (this.m_type) {
      case 0: {
        const custom_variable = new c_custom_variable_reference();
        custom_variable.decode(bitstream);
        this.m_custom_variable = custom_variable;
        break;
      }
      case 1: {
        const player = new c_player_reference();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case 2: {
        const object = new c_object_reference();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      case 3: {
        const team = new c_team_reference();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case 4: {
        const custom_timer = new c_custom_timer_reference();
        custom_timer.decode(bitstream);
        this.m_custom_timer = custom_timer;
        break;
      }
      default:
        break;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_type, 3);
    switch (this.m_type) {
      case 0:
        requireField(
          this.m_custom_variable,
          "m_custom_variable does not exist."
        ).encode(bitstream);
        break;
      case 1:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case 2:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        break;
      case 3:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        break;
      case 4:
        requireField(
          this.m_custom_timer,
          "m_custom_timer does not exist."
        ).encode(bitstream);
        break;
      default:
        throw new BlfError(`Invalid s_variant_variable: type ${this.m_type}`);
    }
  }
}
