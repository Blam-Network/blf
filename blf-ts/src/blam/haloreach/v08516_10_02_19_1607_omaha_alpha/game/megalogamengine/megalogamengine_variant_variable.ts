import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { BlfError } from "../../../../../error";
import { AutoMap } from "../../../../../helpers/automap";
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

/** Matches `e_variable_type` in blf_lib omaha_alpha `megalogamengine_variant_variable.rs`. */
export enum e_variable_type {
  custom_variable = 0,
  player = 1,
  object = 2,
  team = 3,
  custom_timer = 4,
}

export class s_variant_variable {
  @AutoMap(() => e_variable_type)
  m_type: e_variable_type = e_variable_type.custom_variable;
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
    this.m_type = bitstream.read_enum("type", 3, e_variable_type);
    switch (this.m_type) {
      case e_variable_type.custom_variable: {
        const custom_variable = new c_custom_variable_reference();
        custom_variable.decode(bitstream);
        this.m_custom_variable = custom_variable;
        break;
      }
      case e_variable_type.player: {
        const player = new c_player_reference();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case e_variable_type.object: {
        const object = new c_object_reference();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      case e_variable_type.team: {
        const team = new c_team_reference();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case e_variable_type.custom_timer: {
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
    bitstream.write_enum(this.m_type, 3, e_variable_type);
    switch (this.m_type) {
      case e_variable_type.custom_variable:
        requireField(
          this.m_custom_variable,
          "m_custom_variable does not exist."
        ).encode(bitstream);
        break;
      case e_variable_type.player:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case e_variable_type.object:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        break;
      case e_variable_type.team:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        break;
      case e_variable_type.custom_timer:
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
