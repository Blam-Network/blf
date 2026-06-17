import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";
import { c_object_type_reference } from "../../../v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_references";
import { c_custom_timer_reference } from "./megalogamengine_custom_timer_reference";
import { c_object_reference } from "./megalogamengine_object_reference";
import { c_player_reference } from "./megalogamengine_player_reference";
import { c_team_reference } from "./megalogamengine_team_reference";
import { s_variant_variable } from "./megalogamengine_variant_variable";

/** Matches `e_numeric_comparison` in blf_lib omaha_alpha `megalogamengine_conditions.rs`. */
export enum e_numeric_comparison {
  less_than = 0,
  greater_than = 1,
  equal_to = 2,
}

/** Matches `e_condition_type` in blf_lib omaha_alpha `megalogamengine_conditions.rs`. */
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
}

export class s_condition_if_parameters {
  @AutoMap(() => s_variant_variable)
  m_left = new s_variant_variable();
  @AutoMap(() => s_variant_variable)
  m_right = new s_variant_variable();
  @AutoMap(() => e_numeric_comparison)
  m_comparison: e_numeric_comparison = e_numeric_comparison.less_than;

  decode(bitstream: c_bitstream_reader): void {
    this.m_left.decode(bitstream);
    this.m_right.decode(bitstream);
    this.m_comparison = bitstream.read_enum(
      "comparison",
      2,
      e_numeric_comparison
    );
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_left.encode(bitstream);
    this.m_right.encode(bitstream);
    bitstream.write_enum(this.m_comparison, 2, e_numeric_comparison);
  }
}

export class s_condition_object_in_area_parameters {
  @AutoMap(() => c_object_reference)
  m_object_reference_1 = new c_object_reference();
  @AutoMap(() => c_object_reference)
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

export class s_condition_player_died_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => Number)
  m_killer_type = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_killer_type = bitstream.read_integer("killer-type", 5);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_integer(this.m_killer_type, 5);
  }
}

export class s_condition_team_disposition_parameters {
  @AutoMap(() => c_team_reference)
  m_team_1 = new c_team_reference();
  @AutoMap(() => c_team_reference)
  m_team_2 = new c_team_reference();
  @AutoMap(() => Number)
  m_disposition = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.m_team_1.decode(bitstream);
    this.m_team_2.decode(bitstream);
    this.m_disposition = bitstream.read_integer("disposition", 2);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_team_1.encode(bitstream);
    this.m_team_2.encode(bitstream);
    bitstream.write_integer(this.m_disposition, 2);
  }
}

export class s_condition_timer_expired_parameters {
  @AutoMap(() => c_custom_timer_reference)
  m_timer = new c_custom_timer_reference();

  decode(bitstream: c_bitstream_reader): void {
    this.m_timer.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_timer.encode(bitstream);
  }
}

export class s_condition_object_is_type_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => c_object_type_reference)
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

export class s_condition_team_is_active_parameters {
  @AutoMap(() => c_team_reference)
  m_team = new c_team_reference();

  decode(bitstream: c_bitstream_reader): void {
    this.m_team.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_team.encode(bitstream);
  }
}

export class s_condition_object_out_of_bounds_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();

  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
  }
}

export class s_condition_player_is_fire_team_leader_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();

  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
  }
}

export class s_condition_player_assisted_with_kill_parameters {
  @AutoMap(() => c_player_reference)
  m_player_1 = new c_player_reference();
  @AutoMap(() => c_player_reference)
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

export class s_condition_object_matches_filter_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => Number)
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

export class s_condition_player_is_active_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();

  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
  }
}

export class s_condition_equipment_is_active_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();

  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
  }
}

export class c_condition {
  @AutoMap(() => e_condition_type)
  m_type: e_condition_type = e_condition_type.none;
  @AutoMap(() => Boolean)
  m_negated = false;
  @AutoMap(() => Number)
  m_union_group = 0;
  @AutoMap(() => Number)
  m_execute_before_action = 0;
  @AutoMap(() => s_condition_if_parameters)
  m_if_parameters?: s_condition_if_parameters;
  @AutoMap(() => s_condition_object_in_area_parameters)
  m_object_in_area_parameters?: s_condition_object_in_area_parameters;
  @AutoMap(() => s_condition_player_died_parameters)
  m_player_died_parameters?: s_condition_player_died_parameters;
  @AutoMap(() => s_condition_team_disposition_parameters)
  m_team_disposition_parameters?: s_condition_team_disposition_parameters;
  @AutoMap(() => s_condition_timer_expired_parameters)
  m_timer_expired_parameters?: s_condition_timer_expired_parameters;
  @AutoMap(() => s_condition_object_is_type_parameters)
  m_object_is_type_parameters?: s_condition_object_is_type_parameters;
  @AutoMap(() => s_condition_team_is_active_parameters)
  m_team_is_active_parameters?: s_condition_team_is_active_parameters;
  @AutoMap(() => s_condition_object_out_of_bounds_parameters)
  m_object_out_of_bounds_parameters?: s_condition_object_out_of_bounds_parameters;
  @AutoMap(() => s_condition_player_is_fire_team_leader_parameters)
  m_player_is_fire_team_leader_parameters?: s_condition_player_is_fire_team_leader_parameters;
  @AutoMap(() => s_condition_player_assisted_with_kill_parameters)
  m_player_assisted_with_kill_parameters?: s_condition_player_assisted_with_kill_parameters;
  @AutoMap(() => s_condition_object_matches_filter_parameters)
  m_object_matches_filter_parameters?: s_condition_object_matches_filter_parameters;
  @AutoMap(() => s_condition_player_is_active_parameters)
  m_player_is_active_parameters?: s_condition_player_is_active_parameters;
  @AutoMap(() => s_condition_equipment_is_active_parameters)
  m_equipment_is_active_parameters?: s_condition_equipment_is_active_parameters;

  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum("condition-type", 4, e_condition_type);
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
      default:
        break;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 4, e_condition_type);
    if (this.m_type === e_condition_type.none) {
      return;
    }
    bitstream.write_bool(this.m_negated);
    bitstream.write_integer(this.m_union_group, 9);
    bitstream.write_integer(this.m_execute_before_action, 10);
    switch (this.m_type) {
      case e_condition_type.if:
        this.m_if_parameters?.encode(bitstream);
        break;
      case e_condition_type.object_in_area:
        this.m_object_in_area_parameters?.encode(bitstream);
        break;
      case e_condition_type.player_died:
        this.m_player_died_parameters?.encode(bitstream);
        break;
      case e_condition_type.team_disposition:
        this.m_team_disposition_parameters?.encode(bitstream);
        break;
      case e_condition_type.timer_expired:
        this.m_timer_expired_parameters?.encode(bitstream);
        break;
      case e_condition_type.object_is_type:
        this.m_object_is_type_parameters?.encode(bitstream);
        break;
      case e_condition_type.team_is_active:
        this.m_team_is_active_parameters?.encode(bitstream);
        break;
      case e_condition_type.object_out_of_bounds:
        this.m_object_out_of_bounds_parameters?.encode(bitstream);
        break;
      case e_condition_type.player_is_fire_team_leader:
        this.m_player_is_fire_team_leader_parameters?.encode(bitstream);
        break;
      case e_condition_type.player_assisted_with_kill:
        this.m_player_assisted_with_kill_parameters?.encode(bitstream);
        break;
      case e_condition_type.object_matches_filter:
        this.m_object_matches_filter_parameters?.encode(bitstream);
        break;
      case e_condition_type.player_is_active:
        this.m_player_is_active_parameters?.encode(bitstream);
        break;
      case e_condition_type.equipment_is_active:
        this.m_equipment_is_active_parameters?.encode(bitstream);
        break;
      default:
        break;
    }
  }
}
