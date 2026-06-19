import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";
import {
  e_condition_type,
  e_numeric_comparison,
  s_condition_equipment_is_active_parameters,
  s_condition_game_is_forge_parameters,
  s_condition_object_in_area_parameters,
  s_condition_object_is_type_parameters,
  s_condition_object_matches_filter_parameters,
  s_condition_object_out_of_bounds_parameters,
  s_condition_player_assisted_with_kill_parameters,
  s_condition_player_died_parameters,
  s_condition_player_is_active_parameters,
  s_condition_player_is_editor_parameters,
  s_condition_player_is_elite_parameters,
  s_condition_player_is_fire_team_leader_parameters,
  s_condition_player_is_spartan_parameters,
  s_condition_team_disposition_parameters,
  s_condition_team_is_active_parameters,
  s_condition_timer_expired_parameters,
} from "../../../v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_conditions";
import { s_variant_variable } from "./megalogamengine_variant_variable";

export {
  e_condition_type,
  e_numeric_comparison,
  s_condition_equipment_is_active_parameters,
  s_condition_game_is_forge_parameters,
  s_condition_object_in_area_parameters,
  s_condition_object_is_type_parameters,
  s_condition_object_matches_filter_parameters,
  s_condition_object_out_of_bounds_parameters,
  s_condition_player_assisted_with_kill_parameters,
  s_condition_player_died_parameters,
  s_condition_player_is_active_parameters,
  s_condition_player_is_editor_parameters,
  s_condition_player_is_elite_parameters,
  s_condition_player_is_fire_team_leader_parameters,
  s_condition_player_is_spartan_parameters,
  s_condition_team_disposition_parameters,
  s_condition_team_is_active_parameters,
  s_condition_timer_expired_parameters,
};

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
  @AutoMap(() => s_condition_player_is_spartan_parameters)
  m_player_is_spartan_parameters?: s_condition_player_is_spartan_parameters;
  @AutoMap(() => s_condition_player_is_elite_parameters)
  m_player_is_elite_parameters?: s_condition_player_is_elite_parameters;
  @AutoMap(() => s_condition_player_is_editor_parameters)
  m_player_is_editor_parameters?: s_condition_player_is_editor_parameters;
  @AutoMap(() => s_condition_game_is_forge_parameters)
  m_game_is_forge_parameters?: s_condition_game_is_forge_parameters;

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
      case e_condition_type.player_is_spartan:
        this.m_player_is_spartan_parameters?.encode(bitstream);
        break;
      case e_condition_type.player_is_elite:
        this.m_player_is_elite_parameters?.encode(bitstream);
        break;
      case e_condition_type.player_is_editor:
        this.m_player_is_editor_parameters?.encode(bitstream);
        break;
      case e_condition_type.game_is_forge:
        this.m_game_is_forge_parameters?.encode(bitstream);
        break;
      default:
        break;
    }
  }
}
