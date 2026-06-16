import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { BlfError } from "../../../../../error";
import { AutoMap } from "../../../../../helpers/automap";
import {
  c_explicit_object,
  c_explicit_player,
  c_explicit_team,
} from "../../../v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_references";

/** Matches `e_custom_timer_type` in blf_lib omaha_alpha `megalogamengine_custom_timer_reference.rs`. */
export enum e_custom_timer_type {
  global = 0,
  player = 1,
  team = 2,
  object = 3,
  round = 4,
  sudden_death = 5,
  grace_period = 6,
}

export class c_custom_timer_reference {
  @AutoMap(() => e_custom_timer_type)
  m_type: e_custom_timer_type = e_custom_timer_type.global;
  @AutoMap(() => c_explicit_player)
  m_player?: c_explicit_player;
  @AutoMap(() => c_explicit_object)
  m_object?: c_explicit_object;
  @AutoMap(() => c_explicit_team)
  m_team?: c_explicit_team;
  @AutoMap(() => Number)
  m_variable_index?: number;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum("type", 3, e_custom_timer_type);
    switch (this.m_type) {
      case e_custom_timer_type.global:
        this.m_variable_index = bitstream.read_integer(
          "global-variable-index",
          3
        );
        break;
      case e_custom_timer_type.player: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer(
          "player-variable-index",
          2
        );
        break;
      }
      case e_custom_timer_type.team: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer(
          "team-variable-index",
          1
        );
        break;
      }
      case e_custom_timer_type.object: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer(
          "object-variable-index",
          2
        );
        break;
      }
      case e_custom_timer_type.round:
      case e_custom_timer_type.sudden_death:
      case e_custom_timer_type.grace_period:
        break;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 3, e_custom_timer_type);
    switch (this.m_type) {
      case e_custom_timer_type.global:
        if (this.m_variable_index === undefined) {
          throw new BlfError(
            "Invalid c_custom_timer_reference: missing m_variable_index"
          );
        }
        bitstream.write_integer(this.m_variable_index, 3);
        break;
      case e_custom_timer_type.player:
        if (!this.m_player || this.m_variable_index === undefined) {
          throw new BlfError(
            "Invalid c_custom_timer_reference: missing m_player or m_variable_index"
          );
        }
        this.m_player.encode(bitstream);
        bitstream.write_integer(this.m_variable_index, 2);
        break;
      case e_custom_timer_type.team:
        if (!this.m_team || this.m_variable_index === undefined) {
          throw new BlfError(
            "Invalid c_custom_timer_reference: missing m_team or m_variable_index"
          );
        }
        this.m_team.encode(bitstream);
        bitstream.write_integer(this.m_variable_index, 1);
        break;
      case e_custom_timer_type.object:
        if (!this.m_object || this.m_variable_index === undefined) {
          throw new BlfError(
            "Invalid c_custom_timer_reference: missing m_object or m_variable_index"
          );
        }
        this.m_object.encode(bitstream);
        bitstream.write_integer(this.m_variable_index, 2);
        break;
      case e_custom_timer_type.round:
      case e_custom_timer_type.sudden_death:
      case e_custom_timer_type.grace_period:
        break;
    }
  }
}
