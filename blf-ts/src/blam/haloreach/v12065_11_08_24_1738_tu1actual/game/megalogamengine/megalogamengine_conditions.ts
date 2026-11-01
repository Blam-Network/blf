import { c } from "@craftycodie/cstruct";
import type {
  BitfieldOf,
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { bitfieldFromRaw, bitfieldToRaw } from "../../../../../bitstream";
import { class_union } from "../../memory/class_union";
/** Matches `e_numeric_comparison` in blf_lib `megalogamengine_conditions.rs`. */
export enum e_numeric_comparison {
  less_than = 0,
  greater_than = 1,
  equal_to = 2,
  less_than_or_equal_to = 3,
  greater_than_or_equal_to = 4,
  not_equal_to = 5,
}

/** Matches `e_condition_type` in blf_lib `megalogamengine_conditions.rs`. */
export enum e_condition_type {
  none = 0,
  if = 1,
  object_in_area = 2,
  player_died = 3,
  team_disposition = 4,
  timer_expired = 5,
  object_is_type = 6,
  team_is_active = 7,
  object_out_of_bounds = 8,
  player_is_fire_team_leader = 9,
  player_assisted_with_kill = 10,
  object_matches_filter = 11,
  player_is_active = 12,
  equipment_is_active = 13,
  player_is_spartan = 14,
  player_is_elite = 15,
  player_is_editor = 16,
  game_is_forge = 17,
}

/** Bit indices for `c_flags<e_player_death_killer_type, …, 5>`. */
export enum e_player_death_killer_type {
  environment = 0,
  suicide = 1,
  enemy = 2,
  betrayal = 3,
  quit_game = 4,
}

/** Wire type for `m_killer_type` (`c_flags<e_player_death_killer_type>`, 5 bits). */
export const k_player_death_killer_type_flags = [
  "environment",
  "suicide",
  "enemy",
  "betrayal",
  "quit_game",
] as const;

export type e_player_death_killer_type_flags = BitfieldOf<
  typeof k_player_death_killer_type_flags
>;

export function e_player_death_killer_type_flags_none(): e_player_death_killer_type_flags {
  return {
    environment: false,
    suicide: false,
    enemy: false,
    betrayal: false,
    quit_game: false,
  };
}

export function e_player_death_killer_type_flags_any(): e_player_death_killer_type_flags {
  return {
    environment: true,
    suicide: true,
    enemy: true,
    betrayal: true,
    quit_game: true,
  };
}

/** Matches `e_disposition` (`c_enum`, 2 bits, range 0..3). */
export enum e_disposition {
  neutral = 0,
  friendly = 1,
  enemy = 2,
}

import {
  c_custom_timer_reference,
  c_object_reference,
  c_object_type_reference,
  c_player_reference,
  c_team_reference,
  custom_timer_reference_field,
  object_reference_field,
  player_reference_field,
  team_reference_field,
} from "./megalogamengine_references";
import {
  s_variant_variable,
  variant_variable_field,
} from "./megalogamengine_variant_variable";

@c.class()
export class s_condition_if_parameters {
  @c.field(variant_variable_field())
  m_left = new s_variant_variable();
  @c.field(variant_variable_field())
  m_right = new s_variant_variable();
  @c.field(c.u8())
  m_comparison: e_numeric_comparison = e_numeric_comparison.less_than;
  decode(bitstream: c_bitstream_reader): void {
    this.m_left.decode(bitstream);
    this.m_right.decode(bitstream);
    this.m_comparison = bitstream.read_enum(
      "comparison",
      3,
      e_numeric_comparison
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_left.encode(bitstream);
    this.m_right.encode(bitstream);
    bitstream.write_enum(this.m_comparison, 3, e_numeric_comparison);
  }
}

@c.class()
export class s_condition_object_in_area_parameters {
  @c.field(object_reference_field())
  m_object_reference_1 = new c_object_reference();
  @c.field(object_reference_field())
  m_object_reference_2 = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object_reference_1.decode(bitstream);
    this.m_object_reference_2.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object_reference_1.encode(bitstream);
    this.m_object_reference_2.encode(bitstream);
  }
}

@c.class()
export class s_condition_player_died_parameters {
  @c.field(player_reference_field())
  m_player = new c_player_reference();
  @c.field(
    c.u8().transform(
      (raw: number) => bitfieldFromRaw(raw, k_player_death_killer_type_flags),
      (flags: e_player_death_killer_type_flags) =>
        bitfieldToRaw(flags, k_player_death_killer_type_flags)
    )
  )
  m_killer_type: e_player_death_killer_type_flags =
    e_player_death_killer_type_flags_none();
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_killer_type = bitstream.read_bitfield(
      "killer-type",
      5,
      k_player_death_killer_type_flags
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_bitfield(
      this.m_killer_type,
      5,
      k_player_death_killer_type_flags
    );
  }
}

@c.class()
export class s_condition_team_disposition_parameters {
  @c.field(team_reference_field())
  m_team_1 = new c_team_reference();
  @c.field(team_reference_field())
  m_team_2 = new c_team_reference();
  @c.field(c.u8())
  m_disposition: e_disposition = e_disposition.neutral;
  decode(bitstream: c_bitstream_reader): void {
    this.m_team_1.decode(bitstream);
    this.m_team_2.decode(bitstream);
    this.m_disposition = bitstream.read_enum("disposition", 2, e_disposition);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_team_1.encode(bitstream);
    this.m_team_2.encode(bitstream);
    bitstream.write_enum(this.m_disposition, 2, e_disposition);
  }
}

@c.class()
export class s_condition_timer_expired_parameters {
  @c.field(custom_timer_reference_field())
  m_timer = new c_custom_timer_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_timer.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_timer.encode(bitstream);
  }
}

@c.class()
export class s_condition_object_is_type_parameters {
  @c.field(object_reference_field())
  m_object = new c_object_reference();
  @c.field(c_object_type_reference, { padBefore: 1 })
  m_object_type = new c_object_type_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_object_type.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    this.m_object_type.encode(bitstream);
  }
}

@c.class()
export class s_condition_team_is_active_parameters {
  @c.field(team_reference_field())
  m_team = new c_team_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_team.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_team.encode(bitstream);
  }
}

@c.class()
export class s_condition_object_out_of_bounds_parameters {
  @c.field(object_reference_field())
  m_object = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
  }
}

@c.class()
export class s_condition_player_is_fire_team_leader_parameters {
  @c.field(player_reference_field())
  m_player = new c_player_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
  }
}

@c.class()
export class s_condition_player_assisted_with_kill_parameters {
  @c.field(player_reference_field())
  m_player_1 = new c_player_reference();
  @c.field(player_reference_field())
  m_player_2 = new c_player_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_player_1.decode(bitstream);
    this.m_player_2.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player_1.encode(bitstream);
    this.m_player_2.encode(bitstream);
  }
}

@c.class()
export class s_condition_object_matches_filter_parameters {
  @c.field(object_reference_field())
  m_object = new c_object_reference();
  @c.field(c.i32(), { padBefore: 1 })
  m_filter_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_filter_index = bitstream.read_index("filter-index", 16, 4);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_index(this.m_filter_index, 16, 4);
  }
}

@c.class()
export class s_condition_player_is_active_parameters {
  @c.field(player_reference_field())
  m_player = new c_player_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
  }
}

@c.class()
export class s_condition_equipment_is_active_parameters {
  @c.field(object_reference_field())
  m_object = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
  }
}

@c.class()
export class s_condition_player_is_spartan_parameters {
  @c.field(player_reference_field())
  m_player = new c_player_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
  }
}

@c.class()
export class s_condition_player_is_elite_parameters {
  @c.field(player_reference_field())
  m_player = new c_player_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
  }
}

@c.class()
export class s_condition_player_is_editor_parameters {
  @c.field(player_reference_field())
  m_player = new c_player_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
  }
}

export class s_condition_game_is_forge_parameters {
  decode(_bitstream: c_bitstream_reader): void {
    /* no payload */
  }
  encode(_bitstream: c_bitstream_writer): void {
    /* no payload */
  }
}

export class c_condition {
  m_type: e_condition_type = e_condition_type.none;
  m_negated = false;
  m_union_group = 0;
  m_execute_before_action = 0;
  m_if_parameters?: s_condition_if_parameters;
  m_object_in_area_parameters?: s_condition_object_in_area_parameters;
  m_player_died_parameters?: s_condition_player_died_parameters;
  m_team_disposition_parameters?: s_condition_team_disposition_parameters;
  m_timer_expired_parameters?: s_condition_timer_expired_parameters;
  m_object_is_type_parameters?: s_condition_object_is_type_parameters;
  m_team_is_active_parameters?: s_condition_team_is_active_parameters;
  m_object_out_of_bounds_parameters?: s_condition_object_out_of_bounds_parameters;
  m_player_is_fire_team_leader_parameters?: s_condition_player_is_fire_team_leader_parameters;
  m_player_assisted_with_kill_parameters?: s_condition_player_assisted_with_kill_parameters;
  m_object_matches_filter_parameters?: s_condition_object_matches_filter_parameters;
  m_player_is_active_parameters?: s_condition_player_is_active_parameters;
  m_equipment_is_active_parameters?: s_condition_equipment_is_active_parameters;
  m_player_is_spartan_parameters?: s_condition_player_is_spartan_parameters;
  m_player_is_elite_parameters?: s_condition_player_is_elite_parameters;
  m_player_is_editor_parameters?: s_condition_player_is_editor_parameters;
  m_game_is_forge_parameters?: s_condition_game_is_forge_parameters;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum("condition-type", 5, e_condition_type);
    if (this.m_type === e_condition_type.none) {
      return;
    }
    this.m_negated = bitstream.read_bool("negated");
    this.m_union_group = bitstream.read_integer("union-group", 9);
    this.m_execute_before_action = bitstream.read_integer(
      "execute-before-action",
      10
    );
    switch (this.m_type) {
      case e_condition_type.if: {
        const params = new s_condition_if_parameters();
        params.decode(bitstream);
        this.m_if_parameters = params;
        break;
      }
      case e_condition_type.object_in_area: {
        const params = new s_condition_object_in_area_parameters();
        params.decode(bitstream);
        this.m_object_in_area_parameters = params;
        break;
      }
      case e_condition_type.player_died: {
        const params = new s_condition_player_died_parameters();
        params.decode(bitstream);
        this.m_player_died_parameters = params;
        break;
      }
      case e_condition_type.team_disposition: {
        const params = new s_condition_team_disposition_parameters();
        params.decode(bitstream);
        this.m_team_disposition_parameters = params;
        break;
      }
      case e_condition_type.timer_expired: {
        const params = new s_condition_timer_expired_parameters();
        params.decode(bitstream);
        this.m_timer_expired_parameters = params;
        break;
      }
      case e_condition_type.object_is_type: {
        const params = new s_condition_object_is_type_parameters();
        params.decode(bitstream);
        this.m_object_is_type_parameters = params;
        break;
      }
      case e_condition_type.team_is_active: {
        const params = new s_condition_team_is_active_parameters();
        params.decode(bitstream);
        this.m_team_is_active_parameters = params;
        break;
      }
      case e_condition_type.object_out_of_bounds: {
        const params = new s_condition_object_out_of_bounds_parameters();
        params.decode(bitstream);
        this.m_object_out_of_bounds_parameters = params;
        break;
      }
      case e_condition_type.player_is_fire_team_leader: {
        const params = new s_condition_player_is_fire_team_leader_parameters();
        params.decode(bitstream);
        this.m_player_is_fire_team_leader_parameters = params;
        break;
      }
      case e_condition_type.player_assisted_with_kill: {
        const params = new s_condition_player_assisted_with_kill_parameters();
        params.decode(bitstream);
        this.m_player_assisted_with_kill_parameters = params;
        break;
      }
      case e_condition_type.object_matches_filter: {
        const params = new s_condition_object_matches_filter_parameters();
        params.decode(bitstream);
        this.m_object_matches_filter_parameters = params;
        break;
      }
      case e_condition_type.player_is_active: {
        const params = new s_condition_player_is_active_parameters();
        params.decode(bitstream);
        this.m_player_is_active_parameters = params;
        break;
      }
      case e_condition_type.equipment_is_active: {
        const params = new s_condition_equipment_is_active_parameters();
        params.decode(bitstream);
        this.m_equipment_is_active_parameters = params;
        break;
      }
      case e_condition_type.player_is_spartan: {
        const params = new s_condition_player_is_spartan_parameters();
        params.decode(bitstream);
        this.m_player_is_spartan_parameters = params;
        break;
      }
      case e_condition_type.player_is_elite: {
        const params = new s_condition_player_is_elite_parameters();
        params.decode(bitstream);
        this.m_player_is_elite_parameters = params;
        break;
      }
      case e_condition_type.player_is_editor: {
        const params = new s_condition_player_is_editor_parameters();
        params.decode(bitstream);
        this.m_player_is_editor_parameters = params;
        break;
      }
      case e_condition_type.game_is_forge: {
        const params = new s_condition_game_is_forge_parameters();
        params.decode(bitstream);
        this.m_game_is_forge_parameters = params;
        break;
      }
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 5, e_condition_type);
    if (this.m_type === e_condition_type.none) {
      return;
    }
    bitstream.write_bool(this.m_negated);
    bitstream.write_integer(this.m_union_group, 9);
    bitstream.write_integer(this.m_execute_before_action, 10);
    switch (this.m_type) {
      case e_condition_type.if:
        this.m_if_parameters!.encode(bitstream);
        break;
      case e_condition_type.object_in_area:
        this.m_object_in_area_parameters!.encode(bitstream);
        break;
      case e_condition_type.player_died:
        this.m_player_died_parameters!.encode(bitstream);
        break;
      case e_condition_type.team_disposition:
        this.m_team_disposition_parameters!.encode(bitstream);
        break;
      case e_condition_type.timer_expired:
        this.m_timer_expired_parameters!.encode(bitstream);
        break;
      case e_condition_type.object_is_type:
        this.m_object_is_type_parameters!.encode(bitstream);
        break;
      case e_condition_type.team_is_active:
        this.m_team_is_active_parameters!.encode(bitstream);
        break;
      case e_condition_type.object_out_of_bounds:
        this.m_object_out_of_bounds_parameters!.encode(bitstream);
        break;
      case e_condition_type.player_is_fire_team_leader:
        this.m_player_is_fire_team_leader_parameters!.encode(bitstream);
        break;
      case e_condition_type.player_assisted_with_kill:
        this.m_player_assisted_with_kill_parameters!.encode(bitstream);
        break;
      case e_condition_type.object_matches_filter:
        this.m_object_matches_filter_parameters!.encode(bitstream);
        break;
      case e_condition_type.player_is_active:
        this.m_player_is_active_parameters!.encode(bitstream);
        break;
      case e_condition_type.equipment_is_active:
        this.m_equipment_is_active_parameters!.encode(bitstream);
        break;
      case e_condition_type.player_is_spartan:
        this.m_player_is_spartan_parameters!.encode(bitstream);
        break;
      case e_condition_type.player_is_elite:
        this.m_player_is_elite_parameters!.encode(bitstream);
        break;
      case e_condition_type.player_is_editor:
        this.m_player_is_editor_parameters!.encode(bitstream);
        break;
      case e_condition_type.game_is_forge:
        this.m_game_is_forge_parameters!.encode(bitstream);
        break;
    }
  }
}

export function condition_field() {
  return class_union(
    () => new c_condition(),
    16,
    { property: "m_type", offset: 12, type: "u8" },
    () => ({
      [e_condition_type.if]: [
        {
          property: "m_if_parameters",
          field: s_condition_if_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.object_in_area]: [
        {
          property: "m_object_in_area_parameters",
          field: s_condition_object_in_area_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.player_died]: [
        {
          property: "m_player_died_parameters",
          field: s_condition_player_died_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.team_disposition]: [
        {
          property: "m_team_disposition_parameters",
          field: s_condition_team_disposition_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.timer_expired]: [
        {
          property: "m_timer_expired_parameters",
          field: s_condition_timer_expired_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.object_is_type]: [
        {
          property: "m_object_is_type_parameters",
          field: s_condition_object_is_type_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.team_is_active]: [
        {
          property: "m_team_is_active_parameters",
          field: s_condition_team_is_active_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.object_out_of_bounds]: [
        {
          property: "m_object_out_of_bounds_parameters",
          field: s_condition_object_out_of_bounds_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.player_is_fire_team_leader]: [
        {
          property: "m_player_is_fire_team_leader_parameters",
          field: s_condition_player_is_fire_team_leader_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.player_assisted_with_kill]: [
        {
          property: "m_player_assisted_with_kill_parameters",
          field: s_condition_player_assisted_with_kill_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.object_matches_filter]: [
        {
          property: "m_object_matches_filter_parameters",
          field: s_condition_object_matches_filter_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.player_is_active]: [
        {
          property: "m_player_is_active_parameters",
          field: s_condition_player_is_active_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.equipment_is_active]: [
        {
          property: "m_equipment_is_active_parameters",
          field: s_condition_equipment_is_active_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.player_is_spartan]: [
        {
          property: "m_player_is_spartan_parameters",
          field: s_condition_player_is_spartan_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.player_is_elite]: [
        {
          property: "m_player_is_elite_parameters",
          field: s_condition_player_is_elite_parameters,
          offset: 0,
        },
      ],
      [e_condition_type.player_is_editor]: [
        {
          property: "m_player_is_editor_parameters",
          field: s_condition_player_is_editor_parameters,
          offset: 0,
        },
      ],
    }),
    [
      { property: "m_negated", field: c.bool(), offset: 13 },
      { property: "m_union_group", field: c.u8(), offset: 14 },
      { property: "m_execute_before_action", field: c.u8(), offset: 15 },
    ]
  );
}
