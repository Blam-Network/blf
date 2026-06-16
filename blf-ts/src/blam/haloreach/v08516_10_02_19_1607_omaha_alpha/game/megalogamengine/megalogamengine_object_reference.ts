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

/** Matches `e_object_reference_type` in blf_lib omaha_alpha `megalogamengine_object_reference.rs`. */
export enum e_object_reference_type {
  global_object = 0,
  player_object = 1,
  object_object = 2,
  team_object = 3,
  player_biped = 4,
  player_player_biped = 5,
  object_player_biped = 6,
  team_player_biped = 7,
}

export class c_object_reference {
  @AutoMap(() => e_object_reference_type)
  m_type: e_object_reference_type = e_object_reference_type.global_object;
  @AutoMap(() => c_explicit_player)
  m_player?: c_explicit_player;
  @AutoMap(() => c_explicit_object)
  m_object?: c_explicit_object;
  @AutoMap(() => c_explicit_team)
  m_team?: c_explicit_team;
  @AutoMap(() => Number)
  m_variable_index?: number;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum("type", 3, e_object_reference_type);
    switch (this.m_type) {
      case e_object_reference_type.global_object: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      case e_object_reference_type.player_object: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case e_object_reference_type.object_object: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case e_object_reference_type.team_object: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
        break;
      }
      case e_object_reference_type.player_biped: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case e_object_reference_type.player_player_biped: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer("variable-index", 1);
        break;
      }
      case e_object_reference_type.object_player_biped: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 1);
        break;
      }
      case e_object_reference_type.team_player_biped: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 3, e_object_reference_type);
    switch (this.m_type) {
      case e_object_reference_type.global_object:
        if (!this.m_object) {
          throw new BlfError("Invalid c_object_reference: missing m_object");
        }
        this.m_object.encode(bitstream);
        break;
      case e_object_reference_type.player_object:
        if (!this.m_player || this.m_variable_index === undefined) {
          throw new BlfError(
            "Invalid c_object_reference: missing m_player or m_variable_index"
          );
        }
        this.m_player.encode(bitstream);
        bitstream.write_integer(this.m_variable_index, 2);
        break;
      case e_object_reference_type.object_object:
        if (!this.m_object || this.m_variable_index === undefined) {
          throw new BlfError(
            "Invalid c_object_reference: missing m_object or m_variable_index"
          );
        }
        this.m_object.encode(bitstream);
        bitstream.write_integer(this.m_variable_index, 2);
        break;
      case e_object_reference_type.team_object:
        if (!this.m_team || this.m_variable_index === undefined) {
          throw new BlfError(
            "Invalid c_object_reference: missing m_team or m_variable_index"
          );
        }
        this.m_team.encode(bitstream);
        bitstream.write_integer(this.m_variable_index, 3);
        break;
      case e_object_reference_type.player_biped:
        if (!this.m_player) {
          throw new BlfError("Invalid c_object_reference: missing m_player");
        }
        this.m_player.encode(bitstream);
        break;
      case e_object_reference_type.player_player_biped:
        if (!this.m_player || this.m_variable_index === undefined) {
          throw new BlfError(
            "Invalid c_object_reference: missing m_player or m_variable_index"
          );
        }
        this.m_player.encode(bitstream);
        bitstream.write_integer(this.m_variable_index, 1);
        break;
      case e_object_reference_type.object_player_biped:
        if (!this.m_object || this.m_variable_index === undefined) {
          throw new BlfError(
            "Invalid c_object_reference: missing m_object or m_variable_index"
          );
        }
        this.m_object.encode(bitstream);
        bitstream.write_integer(this.m_variable_index, 1);
        break;
      case e_object_reference_type.team_player_biped:
        if (!this.m_team || this.m_variable_index === undefined) {
          throw new BlfError(
            "Invalid c_object_reference: missing m_team or m_variable_index"
          );
        }
        this.m_team.encode(bitstream);
        bitstream.write_integer(this.m_variable_index, 2);
        break;
    }
  }
}
