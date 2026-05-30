import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { BlfError } from "../../../../../error";
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
  m_explicit_player_type: e_explicit_player_type =
    e_explicit_player_type.no_player;

  decode(bitstream: c_bitstream_reader): void {
    this.m_explicit_player_type = bitstream.read_enum(
      "explicit-player-type",
      5,
      e_explicit_player_type
    );
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_explicit_player_type, 5);
  }
}

export class c_explicit_object {
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
    bitstream.write_enum(this.m_explicit_object_type, 5);
  }
}

export class c_explicit_team {
  m_explicit_team_type: e_explicit_team_type = e_explicit_team_type.no_team;

  decode(bitstream: c_bitstream_reader): void {
    this.m_explicit_team_type = bitstream.read_enum(
      "explicit-team-type",
      5,
      e_explicit_team_type
    );
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_explicit_team_type, 5);
  }
}

export class c_object_reference {
  m_type = 0;
  m_player?: c_explicit_player;
  m_object?: c_explicit_object;
  m_team?: c_explicit_team;
  m_variable_index?: number;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_integer("type", 3);

    switch (this.m_type) {
      case 0: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      case 1: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case 2: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case 3: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
        break;
      }
      case 4: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case 5: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case 6: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case 7: {
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
    bitstream.write_integer(this.m_type, 3);
    switch (this.m_type) {
      case 0:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        break;
      case 1:
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
      case 2:
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
      case 3:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          3
        );
        break;
      case 4:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case 5:
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
      case 6:
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
      case 7:
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

export class c_player_reference {
  m_type = 0;
  m_player?: c_explicit_player;
  m_object?: c_explicit_object;
  m_team?: c_explicit_team;
  m_variable_index?: number;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_integer("type", 2);

    switch (this.m_type) {
      case 0: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case 1: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer("m_variable_index", 2);
        break;
      }
      case 2: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("m_variable_index", 2);
        break;
      }
      case 3: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("m_variable_index", 2);
        break;
      }
      default:
        break;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_type, 2);
    switch (this.m_type) {
      case 0:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case 1:
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
      case 2:
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
      case 3:
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

export class c_team_reference {
  m_type = 0;
  m_player?: c_explicit_player;
  m_object?: c_explicit_object;
  m_team?: c_explicit_team;
  m_variable_index?: number;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_integer("type", 3);

    switch (this.m_type) {
      case 0: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case 1: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case 2: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 1);
        break;
      }
      case 3: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("variable-index", 2);
        break;
      }
      case 4: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case 5: {
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
    bitstream.write_integer(this.m_type, 3);
    switch (this.m_type) {
      case 0:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        break;
      case 1:
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
      case 2:
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
      case 3:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          2
        );
        break;
      case 4:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case 5:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        break;
      default:
        throw new BlfError(`Invalid c_team_reference: type ${this.m_type}`);
    }
  }
}

export class c_custom_variable_reference {
  m_type = 0;
  m_immediate_value?: number;
  m_player?: c_explicit_player;
  m_object?: c_explicit_object;
  m_team?: c_explicit_team;
  m_variable_index?: number;
  m_option_index?: number;
  m_statistic_index?: number;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_integer("type", 6);

    switch (this.m_type) {
      case 0:
        this.m_immediate_value = bitstream.read_signed_integer(
          "immediate-value",
          16
        );
        break;
      case 1: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
        break;
      }
      case 2: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
        break;
      }
      case 3: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
        break;
      }
      case 4:
        this.m_variable_index = bitstream.read_integer("variable-index", 4);
        break;
      case 5:
        this.m_option_index = bitstream.read_integer("option-index", 4);
        break;
      case 6: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      case 7: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case 8:
      case 9:
      case 10: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case 11: {
        const player = new c_explicit_player();
        player.decode(bitstream);
        this.m_player = player;
        this.m_statistic_index = bitstream.read_integer("statistic-index", 2);
        break;
      }
      case 12: {
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
    bitstream.write_integer(this.m_type, 6);
    switch (this.m_type) {
      case 0:
        bitstream.write_signed_integer(
          requireField(
            this.m_immediate_value,
            "m_immediate_value does not exist."
          ),
          16
        );
        break;
      case 1:
        requireField(this.m_player, "m_player does not exist.").encode(
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
      case 2:
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
      case 3:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          3
        );
        break;
      case 4:
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          4
        );
        break;
      case 5:
        bitstream.write_integer(
          requireField(this.m_option_index, "m_option_index does not exist."),
          4
        );
        break;
      case 6:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        break;
      case 7:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        break;
      case 8:
      case 9:
      case 10:
        requireField(this.m_player, "m_player does not exist.").encode(
          bitstream
        );
        break;
      case 11:
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
      case 12:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_statistic_index,
            "m_statistic_index does not exist."
          ),
          2
        );
        break;
    }
  }
}

export class c_custom_timer_reference {
  m_type: e_custom_timer_type = e_custom_timer_type.global;
  m_player?: c_explicit_player;
  m_object?: c_explicit_object;
  m_team?: c_explicit_team;
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
    bitstream.write_enum(this.m_type, 3);
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
