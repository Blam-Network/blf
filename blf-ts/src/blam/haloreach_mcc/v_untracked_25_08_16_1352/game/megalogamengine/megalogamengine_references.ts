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
      5,
      e_explicit_player_type
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(
      this.m_explicit_player_type,
      5,
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
  m_explicit_team_type: e_explicit_team_type = e_explicit_team_type.no_team;
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
          2
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
export class c_player_reference {
  @AutoMap(() => Number)
  m_type = 0;
  @AutoMap(() => c_explicit_player)
  m_player?: c_explicit_player;
  @AutoMap(() => c_explicit_object)
  m_object?: c_explicit_object;
  @AutoMap(() => c_explicit_team)
  m_team?: c_explicit_team;
  @AutoMap(() => Number)
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
  @AutoMap(() => Number)
  m_type = 0;
  @AutoMap(() => c_explicit_player)
  m_player?: c_explicit_player;
  @AutoMap(() => c_explicit_object)
  m_object?: c_explicit_object;
  @AutoMap(() => c_explicit_team)
  m_team?: c_explicit_team;
  @AutoMap(() => Number)
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
/** Matches `e_custom_variable_type` in blf_lib `megalogamengine_custom_variable_reference.rs`. */
export enum e_custom_variable_type {
  constant = 0,
  player_number = 1,
  object_number = 2,
  team_number = 3,
  global_number = 4,
  option = 5,
  spawn_object = 6,
  team_score = 7,
  player_score = 8,
  player_money = 9,
  player_rating = 10,
  player_stat = 11,
  team_stat = 12,
  round = 13,
  symmetry_unused = 14,
  symmetric_gametype = 15,
  score_to_win_round = 16,
  fire_teams_enabled = 17,
  teams_enabled = 18,
  round_time_limit = 19,
  round_count = 20,
  perfection_enabled = 21,
  early_victory_win_count = 22,
  sudden_death_time_limit = 23,
  grace_period_time_limit = 24,
  lives_per_round = 25,
  team_lives_per_round = 26,
  respawn_time = 27,
  suicide_respawn_penalty = 28,
  betrayal_respawn_penalty = 29,
  respawn_time_growth = 30,
  loadout_selection_time = 31,
  respawn_traits_duration = 32,
  friendly_fire_enabled = 33,
  betrayal_booting_enabled = 34,
  enemy_voice_enabled = 35,
  open_channel_voice_enabled = 36,
  dead_player_voice_enabled = 37,
  grenades_on_map = 38,
  indestructible_vehicles = 39,
  red_powerup_duration = 40,
  blue_powerup_duration = 41,
  yellow_powerup_duration = 42,
  object_death_damage_type = 43,
  temporary_number = 44,
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
    this.m_type = bitstream.read_enum("type", 6, e_custom_variable_type);
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
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
        break;
      }
      case e_custom_variable_type.object_number: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
        break;
      }
      case e_custom_variable_type.team_number: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        this.m_variable_index = bitstream.read_integer("variable-index", 3);
        break;
      }
      case e_custom_variable_type.global_number:
      case e_custom_variable_type.temporary_number:
        this.m_variable_index = bitstream.read_integer("variable-index", 4);
        break;
      case e_custom_variable_type.option:
        this.m_option_index = bitstream.read_integer("option-index", 4);
        break;
      case e_custom_variable_type.spawn_object: {
        const object = new c_explicit_object();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      case e_custom_variable_type.team_score: {
        const team = new c_explicit_team();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case e_custom_variable_type.player_score:
      case e_custom_variable_type.player_money:
      case e_custom_variable_type.player_rating: {
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
    bitstream.write_enum(this.m_type, 6, e_custom_variable_type);
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
          3
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
          3
        );
        break;
      case e_custom_variable_type.team_number:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        bitstream.write_integer(
          requireField(
            this.m_variable_index,
            "m_variable_index does not exist."
          ),
          3
        );
        break;
      case e_custom_variable_type.global_number:
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
      case e_custom_variable_type.spawn_object:
        requireField(this.m_object, "m_object does not exist.").encode(
          bitstream
        );
        break;
      case e_custom_variable_type.team_score:
        requireField(this.m_team, "m_team does not exist.").encode(bitstream);
        break;
      case e_custom_variable_type.player_score:
      case e_custom_variable_type.player_money:
      case e_custom_variable_type.player_rating:
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
