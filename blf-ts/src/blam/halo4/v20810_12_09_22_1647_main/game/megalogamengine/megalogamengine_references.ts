import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { BlfError } from "../../../../../error";
import { AutoMap } from "../../../../../helpers/automap";
import { e_explicit_object_type } from "./megalogamengine_explicit_object";
import { e_explicit_player_type } from "./megalogamengine_explicit_player";
import { e_explicit_team_type } from "./megalogamengine_explicit_team";

export { e_explicit_object_type } from "./megalogamengine_explicit_object";
export { e_explicit_player_type } from "./megalogamengine_explicit_player";
export { e_explicit_team_type } from "./megalogamengine_explicit_team";

function requireField<T>(value: T | undefined, message: string): T {
  if (value === undefined) {
    throw new BlfError(message);
  }
  return value;
}
export enum e_custom_timer_type {
  global = 0,
  player = 1,
  team = 2,
  object = 3,
  round = 4,
  sudden_death = 5,
  grace_period = 6,
}
export class c_explicit_player {
  @AutoMap(() => e_explicit_player_type)
  m_explicit_player_type: e_explicit_player_type =
    e_explicit_player_type.no_player;
  decode(bitstream: c_bitstream_reader): void {
    this.m_explicit_player_type = bitstream.read_enum(
      "explicit-player-type",
      6,
      e_explicit_player_type
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(
      this.m_explicit_player_type,
      6,
      e_explicit_player_type
    );
  }
}
export class c_explicit_object {
  @AutoMap(() => e_explicit_object_type)
  m_explicit_object_type: e_explicit_object_type =
    e_explicit_object_type.no_object;
  decode(bitstream: c_bitstream_reader): void {
    this.m_explicit_object_type = bitstream.read_enum(
      "explicit-object-type",
      5,
      e_explicit_object_type
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(
      this.m_explicit_object_type,
      5,
      e_explicit_object_type
    );
  }
}
export class c_explicit_team {
  @AutoMap(() => e_explicit_team_type)
  m_explicit_team_type: e_explicit_team_type = e_explicit_team_type.none;
  decode(bitstream: c_bitstream_reader): void {
    this.m_explicit_team_type = bitstream.read_enum(
      "explicit-team-type",
      5,
      e_explicit_team_type
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_explicit_team_type, 5, e_explicit_team_type);
  }
}
/** Matches `e_object_reference_type` in blf_lib `megalogamengine_object_reference.rs`. */
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
        // MaxObjectVariableObjects=6 → 3 bits (IDA).
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
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
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case e_object_reference_type.object_player_biped: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case e_object_reference_type.team_player_biped: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      default:
        break;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 3, e_object_reference_type);
    switch (this.m_type) {
      case e_object_reference_type.global_object:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        break;
      case e_object_reference_type.player_object:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      case e_object_reference_type.object_object:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          3
        );
        break;
      case e_object_reference_type.team_object:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          3
        );
        break;
      case e_object_reference_type.player_biped:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case e_object_reference_type.player_player_biped:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      case e_object_reference_type.object_player_biped:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      case e_object_reference_type.team_player_biped:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      default:
        throw new BlfError(`Invalid c_object_reference: type ${this.m_type}`);
    }
  }
}
/** Matches `e_player_reference_type` in blf_lib `megalogamengine_player_reference.rs`. */
export enum e_player_reference_type {
  global_player = 0,
  player_player = 1,
  object_player = 2,
  team_player = 3,
}
export class c_player_reference {
  @AutoMap(() => e_player_reference_type)
  m_type: e_player_reference_type = e_player_reference_type.global_player;
  @AutoMap(() => c_explicit_player)
  m_player?: c_explicit_player;
  @AutoMap(() => c_explicit_object)
  m_object?: c_explicit_object;
  @AutoMap(() => c_explicit_team)
  m_team?: c_explicit_team;
  @AutoMap(() => Number)
  m_variable_index?: number;
  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum("type", 2, e_player_reference_type);
    switch (this.m_type) {
      case e_player_reference_type.global_player: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case e_player_reference_type.player_player: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case e_player_reference_type.object_player: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case e_player_reference_type.team_player: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      default:
        break;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 2, e_player_reference_type);
    switch (this.m_type) {
      case e_player_reference_type.global_player:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case e_player_reference_type.player_player:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      case e_player_reference_type.object_player:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      case e_player_reference_type.team_player:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      default:
        throw new BlfError(`Invalid c_player_reference: type ${this.m_type}`);
    }
  }
}
/** Matches `e_team_reference_type` in blf_lib `megalogamengine_team_reference.rs`. */
export enum e_team_reference_type {
  global_team = 0,
  player_team = 1,
  object_team = 2,
  team_team = 3,
  player_owner_team = 4,
  object_owner_team = 5,
}
export class c_team_reference {
  @AutoMap(() => e_team_reference_type)
  m_type: e_team_reference_type = e_team_reference_type.global_team;
  @AutoMap(() => c_explicit_player)
  m_player?: c_explicit_player;
  @AutoMap(() => c_explicit_object)
  m_object?: c_explicit_object;
  @AutoMap(() => c_explicit_team)
  m_team?: c_explicit_team;
  @AutoMap(() => Number)
  m_variable_index?: number;
  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum("type", 3, e_team_reference_type);
    switch (this.m_type) {
      case e_team_reference_type.global_team: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case e_team_reference_type.player_team: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case e_team_reference_type.object_team: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 1);
        break;
      }
      case e_team_reference_type.team_team: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case e_team_reference_type.player_owner_team: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case e_team_reference_type.object_owner_team: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      default:
        break;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 3, e_team_reference_type);
    switch (this.m_type) {
      case e_team_reference_type.global_team:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        break;
      case e_team_reference_type.player_team:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      case e_team_reference_type.object_team:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          1
        );
        break;
      case e_team_reference_type.team_team:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      case e_team_reference_type.player_owner_team:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case e_team_reference_type.object_owner_team:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        break;
      default:
        throw new BlfError(`Invalid c_team_reference: type ${this.m_type}`);
    }
  }
}
/**
 * Halo 4 `e_custom_variable_type` — IDA count 73 (0..72), 7 bits.
 * Payload layout from `c_custom_variable_reference::encode` (midnight).
 */
export enum e_custom_variable_type {
  constant = 0,
  player_number = 1,
  object_number = 2,
  team_number = 3,
  global_number = 4,
  temporary_number = 5,
  option = 6,
  unknown_7 = 7,
  unknown_8 = 8,
  unknown_9 = 9,
  unknown_10 = 10,
  unknown_11 = 11,
  unknown_12 = 12,
  unknown_13 = 13,
  player_stat = 14,
  team_stat = 15,
  unknown_16 = 16,
  unknown_17 = 17,
  unknown_18 = 18,
  unknown_19 = 19,
  unknown_20 = 20,
  unknown_21 = 21,
  unknown_22 = 22,
  unknown_23 = 23,
  unknown_24 = 24,
  unknown_25 = 25,
  unknown_26 = 26,
  unknown_27 = 27,
  unknown_28 = 28,
  unknown_29 = 29,
  unknown_30 = 30,
  unknown_31 = 31,
  unknown_32 = 32,
  unknown_33 = 33,
  unknown_34 = 34,
  unknown_35 = 35,
  unknown_36 = 36,
  unknown_37 = 37,
  unknown_38 = 38,
  unknown_39 = 39,
  unknown_40 = 40,
  unknown_41 = 41,
  unknown_42 = 42,
  unknown_43 = 43,
  unknown_44 = 44,
  unknown_45 = 45,
  unknown_46 = 46,
  unknown_47 = 47,
  unknown_48 = 48,
  unknown_49 = 49,
  unknown_50 = 50,
  unknown_51 = 51,
  unknown_52 = 52,
  unknown_53 = 53,
  unknown_54 = 54,
  unknown_55 = 55,
  unknown_56 = 56,
  unknown_57 = 57,
  unknown_58 = 58,
  unknown_59 = 59,
  unknown_60 = 60,
  unknown_61 = 61,
  unknown_62 = 62,
  unknown_63 = 63,
  unknown_64 = 64,
  unknown_65 = 65,
  unknown_66 = 66,
  unknown_67 = 67,
  unknown_68 = 68,
  unknown_69 = 69,
  unknown_70 = 70,
  unknown_71 = 71,
  unknown_72 = 72,
}
export class c_custom_variable_reference {
  @AutoMap(() => e_custom_variable_type)
  m_type: e_custom_variable_type = e_custom_variable_type.constant;
  @AutoMap(() => Number)
  m_immediate_value?: number;
  @AutoMap(() => c_explicit_player)
  m_player?: c_explicit_player;
  @AutoMap(() => c_explicit_object)
  m_object?: c_explicit_object;
  @AutoMap(() => c_explicit_team)
  m_team?: c_explicit_team;
  @AutoMap(() => Number)
  m_variable_index?: number;
  @AutoMap(() => Number)
  m_option_index?: number;
  @AutoMap(() => Number)
  m_statistic_index?: number;
  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum("type", 7, e_custom_variable_type);
    switch (this.m_type) {
      case e_custom_variable_type.constant:
        this.m_immediate_value = bitstream.read_signed_integer(
          "immediate-value",
          16
        );
        break;
      case e_custom_variable_type.player_number: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer("variable-index", 4);
        break;
      }
      case e_custom_variable_type.object_number: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 4);
        break;
      }
      case e_custom_variable_type.team_number: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("variable-index", 4);
        break;
      }
      case e_custom_variable_type.global_number:
        this.m_variable_index = bitstream.read_integer("variable-index", 5);
        break;
      case e_custom_variable_type.temporary_number:
        this.m_variable_index = bitstream.read_integer("variable-index", 4);
        break;
      case e_custom_variable_type.option:
        this.m_option_index = bitstream.read_integer("option-index", 4);
        break;
      case e_custom_variable_type.unknown_7:
      case e_custom_variable_type.unknown_8:
      case e_custom_variable_type.unknown_9:
      case e_custom_variable_type.unknown_31:
      case e_custom_variable_type.unknown_32: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      case e_custom_variable_type.unknown_10:
      case e_custom_variable_type.unknown_22:
      case e_custom_variable_type.unknown_28:
      case e_custom_variable_type.unknown_29:
      case e_custom_variable_type.unknown_30: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case e_custom_variable_type.unknown_11:
      case e_custom_variable_type.unknown_12:
      case e_custom_variable_type.unknown_13:
      case e_custom_variable_type.unknown_23:
      case e_custom_variable_type.unknown_24:
      case e_custom_variable_type.unknown_25:
      case e_custom_variable_type.unknown_26:
      case e_custom_variable_type.unknown_27: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case e_custom_variable_type.player_stat: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_statistic_index = bitstream.read_integer("statistic-index", 2);
        break;
      }
      case e_custom_variable_type.team_stat: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_statistic_index = bitstream.read_integer("statistic-index", 2);
        break;
      }
      default:
        break;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 7, e_custom_variable_type);
    switch (this.m_type) {
      case e_custom_variable_type.constant:
        bitstream.write_signed_integer(
          requireField(
            this.m_immediate_value,
            "m_immediate_value does not exist."
          ),
          16
        );
        break;
      case e_custom_variable_type.player_number:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          4
        );
        break;
      case e_custom_variable_type.object_number:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          4
        );
        break;
      case e_custom_variable_type.team_number:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          4
        );
        break;
      case e_custom_variable_type.global_number:
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          5
        );
        break;
      case e_custom_variable_type.temporary_number:
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          4
        );
        break;
      case e_custom_variable_type.option:
        bitstream.write_integer(
          requireField(this.m_option_index, "m_option_index does not exist."),
          4
        );
        break;
      case e_custom_variable_type.unknown_7:
      case e_custom_variable_type.unknown_8:
      case e_custom_variable_type.unknown_9:
      case e_custom_variable_type.unknown_31:
      case e_custom_variable_type.unknown_32:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        break;
      case e_custom_variable_type.unknown_10:
      case e_custom_variable_type.unknown_22:
      case e_custom_variable_type.unknown_28:
      case e_custom_variable_type.unknown_29:
      case e_custom_variable_type.unknown_30:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        break;
      case e_custom_variable_type.unknown_11:
      case e_custom_variable_type.unknown_12:
      case e_custom_variable_type.unknown_13:
      case e_custom_variable_type.unknown_23:
      case e_custom_variable_type.unknown_24:
      case e_custom_variable_type.unknown_25:
      case e_custom_variable_type.unknown_26:
      case e_custom_variable_type.unknown_27:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case e_custom_variable_type.player_stat:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_statistic_index,
            "m_statistic_index does not exist."
          ),
          2
        );
        break;
      case e_custom_variable_type.team_stat:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_statistic_index,
            "m_statistic_index does not exist."
          ),
          2
        );
        break;
      default:
        break;
    }
  }
  is_writeable(): boolean {
    switch (this.m_type) {
      case e_custom_variable_type.player_number:
      case e_custom_variable_type.object_number:
      case e_custom_variable_type.team_number:
      case e_custom_variable_type.global_number:
      case e_custom_variable_type.temporary_number:
      case e_custom_variable_type.unknown_10:
      case e_custom_variable_type.unknown_11:
      case e_custom_variable_type.unknown_12:
      case e_custom_variable_type.player_stat:
      case e_custom_variable_type.team_stat:
      case e_custom_variable_type.unknown_19:
      case e_custom_variable_type.unknown_20:
      case e_custom_variable_type.unknown_21:
      case e_custom_variable_type.unknown_22:
      case e_custom_variable_type.unknown_23:
      case e_custom_variable_type.unknown_24:
        return true;
      default:
        return false;
    }
  }
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
          2
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
      default:
        break;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 3, e_custom_timer_type);
    switch (this.m_type) {
      case e_custom_timer_type.global:
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          3
        );
        break;
      case e_custom_timer_type.player:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      case e_custom_timer_type.team:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      case e_custom_timer_type.object:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      case e_custom_timer_type.round:
      case e_custom_timer_type.sudden_death:
      case e_custom_timer_type.grace_period:
        break;
      default:
        throw new BlfError(
          `Invalid c_custom_timer_reference: type ${this.m_type}`
        );
    }
  }
}
export class c_object_type_reference {
  @AutoMap(() => Number)
  m_object_type_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object_type_index = bitstream.read_index(
      "object-type-index",
      2048,
      11
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_index(this.m_object_type_index, 2048, 11);
  }
}
