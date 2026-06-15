import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";
import { c_player_traits } from "../c_player_traits";
import {
  e_game_engine_timer_rate,
  e_weapon_pickup_priority,
} from "../game_engine_enums";
import { e_megalogamengine_hud_meter_input_type } from "./megalogamengine_hud_widgets";
/** Matches `e_action_team_or_player_target` in blf_lib `megalogamengine_actions.rs`. */
export enum e_action_team_or_player_target {
  team = 0,
  player = 1,
  all_players = 2,
}
/** Matches `e_math_operation` in blf_lib `megalogamengine_actions.rs`. */
export enum e_math_operation {
  add = 0,
  subtract = 1,
  multiply_by = 2,
  divide_by = 3,
  set_to = 4,
  modulo_by = 5,
  bitwise_and_with = 6,
  bitwise_or_with = 7,
  bitwise_xor_with = 8,
  bitwise_not_with = 9,
  shift_left_with = 10,
  shift_right_with = 11,
  set_to_absolute = 12,
}
export enum e_grenade_type {
  frag_grenade = 0,
  plasma_grenade = 1,
}
export enum e_biped_give_weapon_mode {
  as_primary_weapon = 0,
  normally = 1,
  silently = 2,
}
export enum e_player_filter_type {
  no_one = 0,
  everyone = 1,
  allies = 2,
  enemies = 3,
  specific_player = 4,
  normal = 5,
}
export enum e_chud_navpoint_icon_type {
  none = -1,
  speaker = 0,
  dead_teammate = 1,
  unused = 2,
  target = 3,
  destination = 4,
  bomb = 5,
  flag = 6,
  skull = 7,
  king = 8,
  vip = 9,
  lock = 10,
  num = 11,
  num_1 = 12,
  num_2 = 13,
  num_3 = 14,
  num_4 = 15,
  num_5 = 16,
  num_6 = 17,
  num_7 = 18,
  num_8 = 19,
  num_9 = 20,
  ordnance = 21,
  interface = 22,
  recon = 23,
  ammunition = 24,
  recover = 25,
  defend = 26,
  neutralize = 27,
  coop_spawning = 28,
}
export enum e_navpoint_priority {
  low = 0,
  normal = 1,
  high = 2,
  blink = 3,
}
/** Matches `e_action_type` in blf_lib `megalogamengine_actions.rs`. */
export enum e_action_type {
  none = 0,
  set_score = 1,
  create_object = 2,
  delete_object = 3,
  navpoint_set_visible = 4,
  navpoint_set_icon = 5,
  navpoint_set_priority = 6,
  navpoint_set_timer = 7,
  navpoint_set_visible_range = 8,
  set = 9,
  set_boundary = 10,
  apply_player_traits = 11,
  set_pickup_filter = 12,
  set_respawn_filter = 13,
  set_fireteam_respawn_filter = 14,
  set_progress_bar = 15,
  hud_post_message = 16,
  timer_set_rate = 17,
  print_variable = 18,
  get_player_holding_object = 19,
  for_each = 20,
  end_round = 21,
  boundary_set_visible = 22,
  object_destroy = 23,
  object_set_invincibility = 24,
  random = 25,
  break_into_debugger = 26,
  object_get_orientation = 27,
  object_get_velocity = 28,
  player_death_get_killing_player = 29,
  player_death_get_damage_type = 30,
  player_death_get_special_type = 31,
  debugging_enable_tracing = 32,
  object_attach = 33,
  object_detach = 34,
  player_get_place = 35,
  team_get_place = 36,
  player_get_killing_spree_count = 37,
  player_adjust_money = 38,
  player_enable_purchases = 39,
  player_get_vehicle = 40,
  player_set_vehicle = 41,
  player_set_unit = 42,
  timer_reset = 43,
  weapon_set_pickup_priority = 44,
  object_bounce = 45,
  hud_widget_set_text = 46,
  hud_widget_set_value = 47,
  hud_widget_set_meter = 48,
  hud_widget_set_icon = 49,
  hud_widget_set_visibility = 50,
  play_sound = 51,
  object_set_scale = 52,
  navpoint_set_text = 53,
  object_get_shield = 54,
  object_get_health = 55,
  player_set_objective = 56,
  player_set_objective_allegiance = 57,
  player_set_objective_allegiance_icon = 58,
  team_set_coop_spawning = 59,
  team_set_primary_respawn_object = 60,
  player_set_primary_respawn_object = 61,
  player_get_fireteam_index = 62,
  player_set_fireteam_index = 63,
  object_adjust_shield = 64,
  object_adjust_health = 65,
  object_get_distance = 66,
  object_adjust_maximum_shield = 67,
  object_adjust_maximum_health = 68,
  player_set_requisition_palette = 69,
  device_set_power = 70,
  device_get_power = 71,
  device_set_position = 72,
  device_get_position = 73,
  adjust_grenades = 74,
  submit_incident = 75,
  submit_incident_with_custom_value = 76,
  set_loadout_palette = 77,
  device_set_position_track = 78,
  device_animate_position = 79,
  device_set_position_immediate = 80,
  saved_film_insert_marker = 81,
  respawn_zone_enable = 82,
  player_get_weapon = 83,
  player_get_equipment = 84,
  object_set_never_garbage = 85,
  player_get_target_object = 86,
  create_tunnel = 87,
  debug_force_player_view_count = 88,
  player_pick_up_weapon = 89,
  player_set_coop_spawning = 90,
  object_set_orientation = 91,
  object_face_object = 92,
  biped_give_weapon = 93,
  biped_drop_weapon = 94,
  set_scenario_interpolator_state = 95,
  get_random_object = 96,
  game_grief_record_custom_penalty = 97,
  boundary_set_player_color = 98,
  begin = 99,
  hs_function_call = 100,
  get_button_time = 101,
  team_set_vehicle_spawning = 102,
  player_set_vehicle_spawning = 103,
  set_player_respawn_vehicle = 104,
  set_team_respawn_vehicle = 105,
  hide_object = 106,
}
/** Matches `e_create_object_flags` in blf_lib `megalogamengine_actions.rs`. */
export class e_create_object_flags {
  @AutoMap(() => Boolean)
  never_garbage_collect = false;
  @AutoMap(() => Boolean)
  suppress_effect = false;
  @AutoMap(() => Boolean)
  absolute_orientation = false;
  to_raw(): number {
    return (
      (this.never_garbage_collect ? 1 : 0) |
      (this.suppress_effect ? 1 << 1 : 0) |
      (this.absolute_orientation ? 1 << 2 : 0)
    );
  }
  static from_raw(raw: number): e_create_object_flags {
    const flags = new e_create_object_flags();
    flags.never_garbage_collect = (raw & 1) !== 0;
    flags.suppress_effect = (raw & (1 << 1)) !== 0;
    flags.absolute_orientation = (raw & (1 << 2)) !== 0;
    return flags;
  }
}

import {
  c_custom_timer_reference,
  c_custom_variable_reference,
  c_object_reference,
  c_object_type_reference,
  c_player_reference,
  c_team_reference,
} from "./megalogamengine_references";
import {
  c_dynamic_string,
  c_player_filter_modifier,
} from "./megalogamengine_text";
import { s_variant_variable } from "./s_variant_variable";

/** Matches `e_boundary_shape` in blf_lib `scenario_map_variant.rs`. */
enum e_boundary_shape {
  unused = 0,
  sphere = 1,
  cylinder = 2,
  box = 3,
}
export class s_team_or_player_target {
  @AutoMap(() => e_action_team_or_player_target)
  m_target: e_action_team_or_player_target =
    e_action_team_or_player_target.team;
  @AutoMap(() => c_team_reference)
  m_team?: c_team_reference;
  @AutoMap(() => c_player_reference)
  m_player?: c_player_reference;
  decode(bitstream: c_bitstream_reader): void {
    this.m_target = bitstream.read_enum(
      "target",
      2,
      e_action_team_or_player_target
    );
    switch (this.m_target) {
      case e_action_team_or_player_target.team: {
        const team = new c_team_reference();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case e_action_team_or_player_target.player: {
        const player = new c_player_reference();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      case e_action_team_or_player_target.all_players:
        break;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_target, 2, e_action_team_or_player_target);
    switch (this.m_target) {
      case e_action_team_or_player_target.team:
        this.m_team?.encode(bitstream);
        break;
      case e_action_team_or_player_target.player:
        this.m_player?.encode(bitstream);
        break;
      case e_action_team_or_player_target.all_players:
        break;
    }
  }
}
export class s_action_set_score_parameters {
  @AutoMap(() => s_team_or_player_target)
  m_target = new s_team_or_player_target();
  @AutoMap(() => e_math_operation)
  m_operation: e_math_operation = e_math_operation.add;
  @AutoMap(() => c_custom_variable_reference)
  m_variable = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_target.decode(bitstream);
    this.m_operation = bitstream.read_enum("operation", 4, e_math_operation);
    this.m_variable.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_target.encode(bitstream);
    bitstream.write_enum(this.m_operation, 4, e_math_operation);
    this.m_variable.encode(bitstream);
  }
}
export class s_action_create_object_parameters {
  @AutoMap(() => c_object_type_reference)
  m_object_type = new c_object_type_reference();
  @AutoMap(() => c_object_reference)
  m_object_reference_1 = new c_object_reference();
  @AutoMap(() => c_object_reference)
  m_object_reference_2 = new c_object_reference();
  @AutoMap(() => Number)
  m_filter_index = 0;
  @AutoMap(() => e_create_object_flags)
  m_flags = new e_create_object_flags();
  @AutoMap(() => Number)
  m_offset = 0;
  @AutoMap(() => Number)
  m_variant_name_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object_type.decode(bitstream);
    this.m_object_reference_1.decode(bitstream);
    this.m_object_reference_2.decode(bitstream);
    this.m_filter_index = bitstream.read_index("filter_index", 16, 4);
    this.m_flags = e_create_object_flags.from_raw(
      bitstream.read_integer("flags", 3)
    );
    this.m_offset = bitstream.read_integer("offset", 24);
    this.m_variant_name_index = bitstream.read_integer("variant-name-index", 8);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object_type.encode(bitstream);
    this.m_object_reference_1.encode(bitstream);
    this.m_object_reference_2.encode(bitstream);
    bitstream.write_index(this.m_filter_index, 16, 4);
    bitstream.write_integer(this.m_flags.to_raw(), 3);
    bitstream.write_integer(this.m_offset, 24);
    bitstream.write_integer(this.m_variant_name_index, 8);
  }
}
export class s_action_navpoint_set_icon_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => e_chud_navpoint_icon_type)
  m_navpoint_icon: e_chud_navpoint_icon_type =
    e_chud_navpoint_icon_type.speaker;
  @AutoMap(() => c_custom_variable_reference)
  m_navpoint_number?: c_custom_variable_reference;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_navpoint_icon = bitstream.read_enum(
      "navpoint-icon",
      5,
      e_chud_navpoint_icon_type
    );
    if (this.m_navpoint_icon === e_chud_navpoint_icon_type.num) {
      const navpoint_number = new c_custom_variable_reference();
      navpoint_number.decode(bitstream);
      this.m_navpoint_number = navpoint_number;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_enum(this.m_navpoint_icon, 5, e_chud_navpoint_icon_type);
    if (this.m_navpoint_icon === e_chud_navpoint_icon_type.num) {
      this.m_navpoint_number!.encode(bitstream);
    }
  }
}
export class s_action_navpoint_set_priority_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => e_navpoint_priority)
  m_priority: e_navpoint_priority = e_navpoint_priority.low;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_priority = bitstream.read_enum("priority", 2, e_navpoint_priority);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_enum(this.m_priority, 2, e_navpoint_priority);
  }
}
export class s_action_navpoint_set_timer_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => Number)
  m_timer_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_timer_index = bitstream.read_index("timer-index", 4, 2);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_index(this.m_timer_index, 4, 2);
  }
}
export class s_action_navpoint_set_visible_range_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable_1 = new c_custom_variable_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable_2 = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_variable_1.decode(bitstream);
    this.m_variable_2.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    this.m_variable_1.encode(bitstream);
    this.m_variable_2.encode(bitstream);
  }
}
export class s_action_set_parameters {
  @AutoMap(() => s_variant_variable)
  m_variable_1 = new s_variant_variable();
  @AutoMap(() => s_variant_variable)
  m_variable_2 = new s_variant_variable();
  @AutoMap(() => e_math_operation)
  m_operation: e_math_operation = e_math_operation.add;
  decode(bitstream: c_bitstream_reader): void {
    this.m_variable_1.decode(bitstream);
    this.m_variable_2.decode(bitstream);
    this.m_operation = bitstream.read_enum("operation", 4, e_math_operation);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_variable_1.encode(bitstream);
    this.m_variable_2.encode(bitstream);
    bitstream.write_enum(this.m_operation, 4, e_math_operation);
  }
}
export class s_action_set_boundary_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => e_boundary_shape)
  m_shape: e_boundary_shape = e_boundary_shape.unused;
  @AutoMap(() => c_custom_variable_reference)
  m_variable_1?: c_custom_variable_reference;
  @AutoMap(() => c_custom_variable_reference)
  m_variable_2?: c_custom_variable_reference;
  @AutoMap(() => c_custom_variable_reference)
  m_variable_3?: c_custom_variable_reference;
  @AutoMap(() => c_custom_variable_reference)
  m_variable_4?: c_custom_variable_reference;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_shape = bitstream.read_enum("shape", 2, e_boundary_shape);
    switch (this.m_shape) {
      case e_boundary_shape.sphere: {
        const radius = new c_custom_variable_reference();
        radius.decode(bitstream);
        this.m_variable_1 = radius;
        break;
      }
      case e_boundary_shape.cylinder: {
        const variable1 = new c_custom_variable_reference();
        const variable2 = new c_custom_variable_reference();
        const variable3 = new c_custom_variable_reference();
        variable1.decode(bitstream);
        variable2.decode(bitstream);
        variable3.decode(bitstream);
        this.m_variable_1 = variable1;
        this.m_variable_2 = variable2;
        this.m_variable_3 = variable3;
        break;
      }
      case e_boundary_shape.box: {
        const variable1 = new c_custom_variable_reference();
        const variable2 = new c_custom_variable_reference();
        const variable3 = new c_custom_variable_reference();
        const variable4 = new c_custom_variable_reference();
        variable1.decode(bitstream);
        variable2.decode(bitstream);
        variable3.decode(bitstream);
        variable4.decode(bitstream);
        this.m_variable_1 = variable1;
        this.m_variable_2 = variable2;
        this.m_variable_3 = variable3;
        this.m_variable_4 = variable4;
        break;
      }
      default:
        break;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_enum(this.m_shape, 2, e_boundary_shape);
    switch (this.m_shape) {
      case e_boundary_shape.sphere:
        this.m_variable_1!.encode(bitstream);
        break;
      case e_boundary_shape.cylinder:
        this.m_variable_1!.encode(bitstream);
        this.m_variable_2!.encode(bitstream);
        this.m_variable_3!.encode(bitstream);
        break;
      case e_boundary_shape.box:
        this.m_variable_1!.encode(bitstream);
        this.m_variable_2!.encode(bitstream);
        this.m_variable_3!.encode(bitstream);
        this.m_variable_4!.encode(bitstream);
        break;
    }
  }
}
export class s_action_apply_player_traits_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => Number)
  m_trait_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_trait_index = bitstream.read_integer("player-trait-index", 4);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_integer(this.m_trait_index, 4);
  }
}
export class s_action_set_fireteam_respawn_filter_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => Number)
  m_fireteam_filter = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_fireteam_filter = bitstream.read_integer("fireteam-filter", 8);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_integer(this.m_fireteam_filter, 8);
  }
}
export class s_action_set_progress_bar_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => c_player_filter_modifier)
  m_player_filter_modifier = new c_player_filter_modifier();
  @AutoMap(() => Number)
  m_timer_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_player_filter_modifier.decode(bitstream);
    this.m_timer_index = bitstream.read_index("timer-index", 4, 2);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    this.m_player_filter_modifier.encode(bitstream);
    bitstream.write_index(this.m_timer_index, 4, 2);
  }
}
export class s_action_hud_post_message_parameters {
  @AutoMap(() => s_team_or_player_target)
  m_target = new s_team_or_player_target();
  @AutoMap(() => Number)
  m_sound_index = 0;
  @AutoMap(() => c_dynamic_string)
  m_string = new c_dynamic_string();
  decode(bitstream: c_bitstream_reader): void {
    this.m_target.decode(bitstream);
    this.m_sound_index = bitstream.read_integer("sound-index", 7);
    this.m_string.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_target.encode(bitstream);
    bitstream.write_integer(this.m_sound_index, 7);
    this.m_string.encode(bitstream);
  }
}
export class s_action_timer_set_rate_parameters {
  @AutoMap(() => c_custom_timer_reference)
  m_timer = new c_custom_timer_reference();
  @AutoMap(() => e_game_engine_timer_rate)
  m_rate: e_game_engine_timer_rate = e_game_engine_timer_rate.zero;
  decode(bitstream: c_bitstream_reader): void {
    this.m_timer.decode(bitstream);
    this.m_rate = bitstream.read_enum(
      "timer-rate",
      5,
      e_game_engine_timer_rate
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_timer.encode(bitstream);
    bitstream.write_enum(this.m_rate, 5, e_game_engine_timer_rate);
  }
}
export class s_action_for_each_parameters {
  @AutoMap(() => Number)
  m_trigger_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_trigger_index = bitstream.read_integer("trigger-index", 9);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_trigger_index, 9);
  }
}
export class s_action_object_destroy_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => Boolean)
  m_no_statistics = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_no_statistics = bitstream.read_bool("no-statistics");
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_bool(this.m_no_statistics);
  }
}
export class s_action_object_attach_parameters {
  @AutoMap(() => c_object_reference)
  m_object_1 = new c_object_reference();
  @AutoMap(() => c_object_reference)
  m_object_2 = new c_object_reference();
  @AutoMap(() => Number)
  m_offset = 0;
  @AutoMap(() => Boolean)
  m_absolute_orientation = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object_1.decode(bitstream);
    this.m_object_2.decode(bitstream);
    this.m_offset = bitstream.read_integer("offset", 24);
    this.m_absolute_orientation = bitstream.read_bool("absolute_orientation");
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object_1.encode(bitstream);
    this.m_object_2.encode(bitstream);
    bitstream.write_integer(this.m_offset, 24);
    bitstream.write_bool(this.m_absolute_orientation);
  }
}
export class s_action_player_adjust_money_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => e_math_operation)
  m_math_operation: e_math_operation = e_math_operation.add;
  @AutoMap(() => c_custom_variable_reference)
  m_variable = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_math_operation = bitstream.read_enum(
      "math-operation",
      4,
      e_math_operation
    );
    this.m_variable.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_enum(this.m_math_operation, 4, e_math_operation);
    this.m_variable.encode(bitstream);
  }
}
export class s_action_player_enable_purchases_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable = new c_custom_variable_reference();
  @AutoMap(() => Number)
  m_mode = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_variable.decode(bitstream);
    this.m_mode = bitstream.read_integer("mode", 5);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    this.m_variable.encode(bitstream);
    bitstream.write_integer(this.m_mode, 5);
  }
}
export class s_action_weapon_set_pickup_priority_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => e_weapon_pickup_priority)
  m_weapon_pickup_priority: e_weapon_pickup_priority =
    e_weapon_pickup_priority.normal;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_weapon_pickup_priority = bitstream.read_enum(
      "weapon-pickup-priority",
      2,
      e_weapon_pickup_priority
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_enum(
      this.m_weapon_pickup_priority,
      2,
      e_weapon_pickup_priority
    );
  }
}
export class s_action_hud_widget_text_base {
  @AutoMap(() => Number)
  m_widget_index = 0;
  @AutoMap(() => c_dynamic_string)
  m_string = new c_dynamic_string();
  decode(bitstream: c_bitstream_reader): void {
    this.m_widget_index = bitstream.read_index("widget-index", 4, 2);
    this.m_string.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_index(this.m_widget_index, 4, 2);
    this.m_string.encode(bitstream);
  }
}
export class c_megalogamengine_hud_meter_input {
  @AutoMap(() => e_megalogamengine_hud_meter_input_type)
  m_type: e_megalogamengine_hud_meter_input_type =
    e_megalogamengine_hud_meter_input_type.none;
  @AutoMap(() => c_custom_variable_reference)
  m_variable_1?: c_custom_variable_reference;
  @AutoMap(() => c_custom_variable_reference)
  m_variable_2?: c_custom_variable_reference;
  @AutoMap(() => c_custom_timer_reference)
  m_timer?: c_custom_timer_reference;
  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum(
      "type",
      2,
      e_megalogamengine_hud_meter_input_type
    );
    switch (this.m_type) {
      case e_megalogamengine_hud_meter_input_type.number: {
        const variable1 = new c_custom_variable_reference();
        const variable2 = new c_custom_variable_reference();
        variable1.decode(bitstream);
        variable2.decode(bitstream);
        this.m_variable_1 = variable1;
        this.m_variable_2 = variable2;
        break;
      }
      case e_megalogamengine_hud_meter_input_type.timer: {
        const timer = new c_custom_timer_reference();
        timer.decode(bitstream);
        this.m_timer = timer;
        break;
      }
      case e_megalogamengine_hud_meter_input_type.none:
        break;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    switch (this.m_type) {
      case e_megalogamengine_hud_meter_input_type.number:
        bitstream.write_enum(
          e_megalogamengine_hud_meter_input_type.number,
          2,
          e_megalogamengine_hud_meter_input_type
        );
        this.m_variable_1!.encode(bitstream);
        this.m_variable_2!.encode(bitstream);
        break;
      case e_megalogamengine_hud_meter_input_type.timer:
        bitstream.write_enum(
          e_megalogamengine_hud_meter_input_type.timer,
          2,
          e_megalogamengine_hud_meter_input_type
        );
        this.m_timer!.encode(bitstream);
        break;
      case e_megalogamengine_hud_meter_input_type.none:
        break;
    }
  }
}
export class s_action_hud_widget_set_meter_parameters {
  @AutoMap(() => Number)
  m_widget_index = 0;
  @AutoMap(() => c_megalogamengine_hud_meter_input)
  m_meter_input = new c_megalogamengine_hud_meter_input();
  decode(bitstream: c_bitstream_reader): void {
    this.m_widget_index = bitstream.read_index("widget-index", 4, 2);
    this.m_meter_input.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_index(this.m_widget_index, 4, 2);
    this.m_meter_input.encode(bitstream);
  }
}
export class s_action_hud_widget_set_icon_parameters {
  @AutoMap(() => Number)
  m_widget_index = 0;
  @AutoMap(() => Number)
  m_icon_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_widget_index = bitstream.read_index("widget-index", 4, 2);
    this.m_icon_index = bitstream.read_index("icon-index", 64, 6);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_index(this.m_widget_index, 4, 2);
    bitstream.write_index(this.m_icon_index, 64, 6);
  }
}
export class s_action_hud_widget_set_visibility_parameters {
  @AutoMap(() => Number)
  m_widget_index = 0;
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => Boolean)
  m_visible = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_widget_index = bitstream.read_index("widget-index", 4, 2);
    this.m_player.decode(bitstream);
    this.m_visible = bitstream.read_bool("visible");
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_index(this.m_widget_index, 4, 2);
    this.m_player.encode(bitstream);
    bitstream.write_bool(this.m_visible);
  }
}
export class s_action_play_sound_parameters {
  @AutoMap(() => Number)
  m_sound_index = 0;
  @AutoMap(() => Boolean)
  m_immediate = false;
  @AutoMap(() => s_team_or_player_target)
  m_target = new s_team_or_player_target();
  decode(bitstream: c_bitstream_reader): void {
    this.m_sound_index = bitstream.read_integer("sound-index", 7);
    this.m_immediate = bitstream.read_bool("immediate");
    this.m_target.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_sound_index, 7);
    bitstream.write_bool(this.m_immediate);
    this.m_target.encode(bitstream);
  }
}
export class s_action_player_set_objective_allegiance_icon_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => Number)
  m_icon_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_icon_index = bitstream.read_index("icon-index", 128, 7);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_index(this.m_icon_index, 128, 7);
  }
}
export class s_action_team_set_coop_spawning_parameters {
  @AutoMap(() => c_team_reference)
  m_team = new c_team_reference();
  @AutoMap(() => Boolean)
  m_enabled = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_team.decode(bitstream);
    this.m_enabled = bitstream.read_bool("enabled");
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_team.encode(bitstream);
    bitstream.write_bool(this.m_enabled);
  }
}
export class s_action_vitality_adjustment_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => e_math_operation)
  m_operation: e_math_operation = e_math_operation.add;
  @AutoMap(() => c_custom_variable_reference)
  m_variable = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_operation = bitstream.read_enum("operation", 4, e_math_operation);
    this.m_variable.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_enum(this.m_operation, 4, e_math_operation);
    this.m_variable.encode(bitstream);
  }
}
export class s_action_object_get_distance_parameters {
  @AutoMap(() => c_object_reference)
  m_object_1 = new c_object_reference();
  @AutoMap(() => c_object_reference)
  m_object_2 = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object_1.decode(bitstream);
    this.m_object_2.decode(bitstream);
    this.m_variable.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object_1.encode(bitstream);
    this.m_object_2.encode(bitstream);
    this.m_variable.encode(bitstream);
  }
}
export class s_action_player_set_requisition_palette_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => Number)
  m_new_palette = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_new_palette = bitstream.read_integer("new-palette", 4);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_integer(this.m_new_palette, 4);
  }
}
export class s_action_adjust_grenades_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => e_grenade_type)
  m_grenade_type: e_grenade_type = e_grenade_type.frag_grenade;
  @AutoMap(() => e_math_operation)
  m_math_operation: e_math_operation = e_math_operation.add;
  @AutoMap(() => c_custom_variable_reference)
  m_variable = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_grenade_type = bitstream.read_enum(
      "grenade-type",
      1,
      e_grenade_type
    );
    this.m_math_operation = bitstream.read_enum(
      "math-operation",
      4,
      e_math_operation
    );
    this.m_variable.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_enum(this.m_grenade_type, 1, e_grenade_type);
    bitstream.write_enum(this.m_math_operation, 4, e_math_operation);
    this.m_variable.encode(bitstream);
  }
}
export class s_action_submit_incident_parameters {
  @AutoMap(() => Number)
  m_incident_id = 0;
  @AutoMap(() => s_team_or_player_target)
  m_target_1 = new s_team_or_player_target();
  @AutoMap(() => s_team_or_player_target)
  m_target_2 = new s_team_or_player_target();
  decode(bitstream: c_bitstream_reader): void {
    this.m_incident_id = bitstream.read_integer("incident-id", 10);
    this.m_target_1.decode(bitstream);
    this.m_target_2.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_incident_id, 10);
    this.m_target_1.encode(bitstream);
    this.m_target_2.encode(bitstream);
  }
}
export class s_action_submit_incident_with_custom_value_parameters {
  @AutoMap(() => Number)
  m_incident_id = 0;
  @AutoMap(() => s_team_or_player_target)
  m_target_1 = new s_team_or_player_target();
  @AutoMap(() => s_team_or_player_target)
  m_target_2 = new s_team_or_player_target();
  @AutoMap(() => c_custom_variable_reference)
  m_variable = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_incident_id = bitstream.read_integer("incident-id", 10);
    this.m_target_1.decode(bitstream);
    this.m_target_2.decode(bitstream);
    this.m_variable.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_incident_id, 10);
    this.m_target_1.encode(bitstream);
    this.m_target_2.encode(bitstream);
    this.m_variable.encode(bitstream);
  }
}
export class s_action_set_loadout_palette_parameters {
  @AutoMap(() => s_team_or_player_target)
  m_target = new s_team_or_player_target();
  @AutoMap(() => Number)
  m_loadout_palette_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_target.decode(bitstream);
    this.m_loadout_palette_index = bitstream.read_integer(
      "loadout-palette-index",
      3
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_target.encode(bitstream);
    bitstream.write_integer(this.m_loadout_palette_index, 3);
  }
}
export class s_action_device_set_position_track_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => Number)
  m_animation_name_index = 0;
  @AutoMap(() => c_custom_variable_reference)
  m_variable = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_animation_name_index = bitstream.read_integer(
      "animation-name-index",
      8
    );
    this.m_variable.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_integer(this.m_animation_name_index, 8);
    this.m_variable.encode(bitstream);
  }
}
export class s_action_device_animate_position_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable_1 = new c_custom_variable_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable_2 = new c_custom_variable_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable_3 = new c_custom_variable_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable_4 = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_variable_1.decode(bitstream);
    this.m_variable_2.decode(bitstream);
    this.m_variable_3.decode(bitstream);
    this.m_variable_4.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    this.m_variable_1.encode(bitstream);
    this.m_variable_2.encode(bitstream);
    this.m_variable_3.encode(bitstream);
    this.m_variable_4.encode(bitstream);
  }
}
export class s_action_player_get_weapon_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => Boolean)
  m_primary = false;
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_primary = bitstream.read_bool("primary");
    this.m_object.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_bool(this.m_primary);
    this.m_object.encode(bitstream);
  }
}
export class s_action_create_tunnel_parameters {
  @AutoMap(() => c_player_reference)
  m_object_1 = new c_player_reference();
  @AutoMap(() => c_player_reference)
  m_object_2 = new c_player_reference();
  @AutoMap(() => c_object_reference)
  m_object_type = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable = new c_custom_variable_reference();
  @AutoMap(() => c_player_reference)
  m_object_3 = new c_player_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object_1.decode(bitstream);
    this.m_object_2.decode(bitstream);
    this.m_object_type.decode(bitstream);
    this.m_variable.decode(bitstream);
    this.m_object_3.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object_1.encode(bitstream);
    this.m_object_2.encode(bitstream);
    this.m_object_type.encode(bitstream);
    this.m_variable.encode(bitstream);
    this.m_object_3.encode(bitstream);
  }
}
export class s_action_player_set_coop_spawning_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => Boolean)
  m_enabled = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_enabled = bitstream.read_bool("enabled");
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_bool(this.m_enabled);
  }
}
export class s_action_object_set_orientation_parameters {
  @AutoMap(() => c_object_reference)
  m_object_1 = new c_object_reference();
  @AutoMap(() => c_object_reference)
  m_object_2 = new c_object_reference();
  @AutoMap(() => Boolean)
  m_absolute_orientation = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object_1.decode(bitstream);
    this.m_object_2.decode(bitstream);
    this.m_absolute_orientation = bitstream.read_bool("absolute-orientation");
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object_1.encode(bitstream);
    this.m_object_2.encode(bitstream);
    bitstream.write_bool(this.m_absolute_orientation);
  }
}
export class s_action_object_face_object_parameters {
  @AutoMap(() => c_object_reference)
  m_object_1 = new c_object_reference();
  @AutoMap(() => c_object_reference)
  m_object_2 = new c_object_reference();
  @AutoMap(() => Number)
  m_offset = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object_1.decode(bitstream);
    this.m_object_2.decode(bitstream);
    this.m_offset = bitstream.read_integer("offset", 24);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object_1.encode(bitstream);
    this.m_object_2.encode(bitstream);
    bitstream.write_integer(this.m_offset, 24);
  }
}
export class s_action_biped_give_weapon_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => c_object_type_reference)
  m_object_type = new c_object_type_reference();
  @AutoMap(() => e_biped_give_weapon_mode)
  m_mode: e_biped_give_weapon_mode = e_biped_give_weapon_mode.as_primary_weapon;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_object_type.decode(bitstream);
    this.m_mode = bitstream.read_enum("mode", 2, e_biped_give_weapon_mode);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    this.m_object_type.encode(bitstream);
    bitstream.write_enum(this.m_mode, 2, e_biped_give_weapon_mode);
  }
}
export class s_action_biped_drop_weapon_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => Boolean)
  m_primary = false;
  @AutoMap(() => Boolean)
  m_delete_on_drop = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_primary = bitstream.read_bool("primary");
    this.m_delete_on_drop = bitstream.read_bool("delete_on_drop");
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_bool(this.m_primary);
    bitstream.write_bool(this.m_delete_on_drop);
  }
}
export class s_action_get_random_object_parameters {
  @AutoMap(() => c_object_reference)
  m_object_1 = new c_object_reference();
  @AutoMap(() => c_object_reference)
  m_object_2 = new c_object_reference();
  @AutoMap(() => Number)
  m_filter_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object_1.decode(bitstream);
    this.m_object_2.decode(bitstream);
    this.m_filter_index = bitstream.read_index("filter-index", 16, 4);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object_1.encode(bitstream);
    this.m_object_2.encode(bitstream);
    bitstream.write_index(this.m_filter_index, 16, 4);
  }
}
export class s_action_boundary_set_player_color_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => Number)
  m_player_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_player_index = bitstream.read_index("player-index", 4, 2);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_index(this.m_player_index, 4, 2);
  }
}
export class s_action_hs_function_call_parameters {
  @AutoMap(() => Number)
  m_function_name_index = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_function_name_index =
      bitstream.read_integer("function-name-index", 8) - 1;
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_function_name_index + 1, 8);
  }
}
export class s_action_get_button_time_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => Number)
  m_buttons = 0;
  @AutoMap(() => c_custom_variable_reference)
  m_variable = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_buttons = bitstream.read_integer("buttons", 5);
    this.m_variable.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_integer(this.m_buttons, 5);
    this.m_variable.encode(bitstream);
  }
}
export class s_action_team_set_vehicle_spawning_parameters {
  @AutoMap(() => c_team_reference)
  m_team = new c_team_reference();
  @AutoMap(() => Boolean)
  m_enabled = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_team.decode(bitstream);
    this.m_enabled = bitstream.read_bool("enabled");
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_team.encode(bitstream);
    bitstream.write_bool(this.m_enabled);
  }
}
export class s_action_player_set_vehicle_spawning_parameters {
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  @AutoMap(() => Boolean)
  m_enabled = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_enabled = bitstream.read_bool("enabled");
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
    bitstream.write_bool(this.m_enabled);
  }
}
export class s_action_set_player_respawn_vehicle_parameters {
  @AutoMap(() => c_object_type_reference)
  m_object_type = new c_object_type_reference();
  @AutoMap(() => c_player_reference)
  m_player = new c_player_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object_type.decode(bitstream);
    this.m_player.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object_type.encode(bitstream);
    this.m_player.encode(bitstream);
  }
}
export class s_action_set_team_respawn_vehicle_parameters {
  @AutoMap(() => c_object_type_reference)
  m_object_type = new c_object_type_reference();
  @AutoMap(() => c_team_reference)
  m_team = new c_team_reference();
  decode(bitstream: c_bitstream_reader): void {
    this.m_object_type.decode(bitstream);
    this.m_team.decode(bitstream);
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object_type.encode(bitstream);
    this.m_team.encode(bitstream);
  }
}
export class s_action_hide_object_parameters {
  @AutoMap(() => c_object_reference)
  m_object = new c_object_reference();
  @AutoMap(() => Boolean)
  m_should_hide = false;
  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_should_hide = bitstream.read_bool("should hide");
  }
  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_bool(this.m_should_hide);
  }
}
export class s_action_delete_object_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);

        
    }
}
export class s_action_navpoint_set_visible_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_player_filter_modifier)
  m_player_filter_modifier: c_player_filter_modifier = new c_player_filter_modifier();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_player_filter_modifier.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_player_filter_modifier.encode(bitstream);

        
    }
}
export class s_action_set_pickup_filter_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_player_filter_modifier)
  m_player_filter_modifier: c_player_filter_modifier = new c_player_filter_modifier();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_player_filter_modifier.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_player_filter_modifier.encode(bitstream);

        
    }
}
export class s_action_set_respawn_filter_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_player_filter_modifier)
  m_player_filter_modifier: c_player_filter_modifier = new c_player_filter_modifier();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_player_filter_modifier.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_player_filter_modifier.encode(bitstream);

        
    }
}
export class s_action_print_variable_parameters {
  @AutoMap(() => c_dynamic_string)
  m_string: c_dynamic_string = new c_dynamic_string();
  decode(bitstream: c_bitstream_reader): void {
        this.m_string.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_string.encode(bitstream);

        
    }
}
export class s_action_get_player_holding_object_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_player.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_player.encode(bitstream);

        
    }
}
export class s_action_end_round_parameters {
  decode(_bitstream: c_bitstream_reader): void {
        
    }
  encode(_bitstream: c_bitstream_writer): void {
        
    }
}
export class s_action_boundary_set_visible_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_player_filter_modifier)
  m_player_filter_modifier: c_player_filter_modifier = new c_player_filter_modifier();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_player_filter_modifier.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_player_filter_modifier.encode(bitstream);

        
    }
}
export class s_action_object_set_invincibility_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_random_parameters {
  @AutoMap(() => c_custom_variable_reference)
  m_variable_1: c_custom_variable_reference = new c_custom_variable_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable_2: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_variable_1.decode(bitstream);
        this.m_variable_2.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_variable_1.encode(bitstream);
        this.m_variable_2.encode(bitstream);

        
    }
}
export class s_action_break_into_debugger_parameters {
  decode(_bitstream: c_bitstream_reader): void {
        
    }
  encode(_bitstream: c_bitstream_writer): void {
        
    }
}
export class s_action_object_get_orientation_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_object_get_velocity_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_player_death_get_killing_player_parameters {
  @AutoMap(() => c_player_reference)
  m_player_1: c_player_reference = new c_player_reference();
  @AutoMap(() => c_player_reference)
  m_player_2: c_player_reference = new c_player_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player_1.decode(bitstream);
        this.m_player_2.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player_1.encode(bitstream);
        this.m_player_2.encode(bitstream);

        
    }
}
export class s_action_player_death_get_damage_type_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_player_death_get_special_type_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_debugging_enable_tracing_parameters {
  @AutoMap(() => Boolean)
  m_tracing_enabled: boolean = false;
  decode(bitstream: c_bitstream_reader): void {
        this.m_tracing_enabled = bitstream.read_bool("tracing-enabled");

        
    }
  encode(bitstream: c_bitstream_writer): void {
        bitstream.write_bool(this.m_tracing_enabled);

        
    }
}
export class s_action_object_detach_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);

        
    }
}
export class s_action_player_get_place_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_team_get_place_parameters {
  @AutoMap(() => c_team_reference)
  m_team: c_team_reference = new c_team_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_team.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_team.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_player_get_killing_spree_count_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_player_get_vehicle_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_object.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_object.encode(bitstream);

        
    }
}
export class s_action_player_set_vehicle_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_object.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_object.encode(bitstream);

        
    }
}
export class s_action_player_set_unit_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_object.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_object.encode(bitstream);

        
    }
}
export class s_action_timer_reset_parameters {
  @AutoMap(() => c_custom_timer_reference)
  m_timer: c_custom_timer_reference = new c_custom_timer_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_timer.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_timer.encode(bitstream);

        
    }
}
export class s_action_object_bounce_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);

        
    }
}
export class s_action_hud_widget_set_text_parameters {
  @AutoMap(() => Number)
  m_widget_index: number = 0;
  @AutoMap(() => c_dynamic_string)
  m_string: c_dynamic_string = new c_dynamic_string();
  decode(bitstream: c_bitstream_reader): void {
        this.m_widget_index = bitstream.read_index("widget-index", 4, 2) ;
        this.m_string.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        bitstream.write_index(this.m_widget_index, 4, 2);
        this.m_string.encode(bitstream);

        
    }
}
export class s_action_hud_widget_set_value_parameters {
  @AutoMap(() => Number)
  m_widget_index: number = 0;
  @AutoMap(() => c_dynamic_string)
  m_string: c_dynamic_string = new c_dynamic_string();
  decode(bitstream: c_bitstream_reader): void {
        this.m_widget_index = bitstream.read_index("widget-index", 4, 2) ;
        this.m_string.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        bitstream.write_index(this.m_widget_index, 4, 2);
        this.m_string.encode(bitstream);

        
    }
}
export class s_action_object_set_scale_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_navpoint_set_text_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_dynamic_string)
  m_string: c_dynamic_string = new c_dynamic_string();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_string.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_string.encode(bitstream);

        
    }
}
export class s_action_object_get_shield_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_object_get_health_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_player_set_objective_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_dynamic_string)
  m_string: c_dynamic_string = new c_dynamic_string();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_string.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_string.encode(bitstream);

        
    }
}
export class s_action_player_set_objective_allegiance_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_dynamic_string)
  m_string: c_dynamic_string = new c_dynamic_string();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_string.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_string.encode(bitstream);

        
    }
}
export class s_action_team_set_primary_respawn_object_parameters {
  @AutoMap(() => c_team_reference)
  m_team: c_team_reference = new c_team_reference();
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_team.decode(bitstream);
        this.m_object.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_team.encode(bitstream);
        this.m_object.encode(bitstream);

        
    }
}
export class s_action_player_set_primary_respawn_object_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_object.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_object.encode(bitstream);

        
    }
}
export class s_action_player_get_fireteam_index_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_player_set_fireteam_index_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_object_adjust_shield_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => e_math_operation)
  m_operation: e_math_operation;
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_operation = bitstream.read_enum("operation", 4, e_math_operation);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        bitstream.write_enum(this.m_operation, 4, e_math_operation);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_object_adjust_health_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => e_math_operation)
  m_operation: e_math_operation;
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_operation = bitstream.read_enum("operation", 4, e_math_operation);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        bitstream.write_enum(this.m_operation, 4, e_math_operation);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_object_adjust_maximum_shield_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => e_math_operation)
  m_operation: e_math_operation;
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_operation = bitstream.read_enum("operation", 4, e_math_operation);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        bitstream.write_enum(this.m_operation, 4, e_math_operation);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_object_adjust_maximum_health_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => e_math_operation)
  m_operation: e_math_operation;
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_operation = bitstream.read_enum("operation", 4, e_math_operation);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        bitstream.write_enum(this.m_operation, 4, e_math_operation);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_device_set_power_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_device_get_power_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_device_set_position_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_device_get_position_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_device_set_position_immediate_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_saved_film_insert_marker_parameters {
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  @AutoMap(() => c_dynamic_string)
  m_string: c_dynamic_string = new c_dynamic_string();
  decode(bitstream: c_bitstream_reader): void {
        this.m_variable.decode(bitstream);
        this.m_string.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_variable.encode(bitstream);
        this.m_string.encode(bitstream);

        
    }
}
export class s_action_respawn_zone_enable_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_player_get_equipment_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_object.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_object.encode(bitstream);

        
    }
}
export class s_action_object_set_never_garbage_parameters {
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_object.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_object.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_player_get_target_object_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_object.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_object.encode(bitstream);

        
    }
}
export class s_action_debug_force_player_view_count_parameters {
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_variable.encode(bitstream);

        
    }
}
export class s_action_player_pick_up_weapon_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_object_reference)
  m_object: c_object_reference = new c_object_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_object.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_object.encode(bitstream);

        
    }
}
export class s_action_set_scenario_interpolator_state_parameters {
  @AutoMap(() => c_custom_variable_reference)
  m_variable_1: c_custom_variable_reference = new c_custom_variable_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable_2: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_variable_1.decode(bitstream);
        this.m_variable_2.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_variable_1.encode(bitstream);
        this.m_variable_2.encode(bitstream);

        
    }
}
export class s_action_game_grief_record_custom_penalty_parameters {
  @AutoMap(() => c_player_reference)
  m_player: c_player_reference = new c_player_reference();
  @AutoMap(() => c_custom_variable_reference)
  m_variable: c_custom_variable_reference = new c_custom_variable_reference();
  decode(bitstream: c_bitstream_reader): void {
        this.m_player.decode(bitstream);
        this.m_variable.decode(bitstream);

        
    }
  encode(bitstream: c_bitstream_writer): void {
        this.m_player.encode(bitstream);
        this.m_variable.encode(bitstream);

        
    }
}
export class c_action {
  @AutoMap(() => e_action_type)
  m_type: e_action_type = e_action_type.none;
  @AutoMap(() => s_action_set_score_parameters)
  m_set_score_parameters?: s_action_set_score_parameters;
  @AutoMap(() => s_action_create_object_parameters)
  m_create_object_parameters?: s_action_create_object_parameters;
  @AutoMap(() => s_action_delete_object_parameters)
  m_delete_object_parameters?: s_action_delete_object_parameters;
  @AutoMap(() => s_action_navpoint_set_visible_parameters)
  m_navpoint_set_visible_parameters?: s_action_navpoint_set_visible_parameters;
  @AutoMap(() => s_action_navpoint_set_icon_parameters)
  m_navpoint_set_icon_parameters?: s_action_navpoint_set_icon_parameters;
  @AutoMap(() => s_action_navpoint_set_priority_parameters)
  m_navpoint_set_priority_parameters?: s_action_navpoint_set_priority_parameters;
  @AutoMap(() => s_action_navpoint_set_timer_parameters)
  m_navpoint_set_timer_parameters?: s_action_navpoint_set_timer_parameters;
  @AutoMap(() => s_action_navpoint_set_visible_range_parameters)
  m_navpoint_set_visible_range_parameters?: s_action_navpoint_set_visible_range_parameters;
  @AutoMap(() => s_action_set_parameters)
  m_set_parameters?: s_action_set_parameters;
  @AutoMap(() => s_action_set_boundary_parameters)
  m_set_boundary_parameters?: s_action_set_boundary_parameters;
  @AutoMap(() => s_action_apply_player_traits_parameters)
  m_apply_player_traits_parameters?: s_action_apply_player_traits_parameters;
  @AutoMap(() => s_action_set_pickup_filter_parameters)
  m_set_pickup_filter_parameters?: s_action_set_pickup_filter_parameters;
  @AutoMap(() => s_action_set_respawn_filter_parameters)
  m_set_respawn_filter_parameters?: s_action_set_respawn_filter_parameters;
  @AutoMap(() => s_action_set_fireteam_respawn_filter_parameters)
  m_set_fireteam_respawn_filter_parameters?: s_action_set_fireteam_respawn_filter_parameters;
  @AutoMap(() => s_action_set_progress_bar_parameters)
  m_set_progress_bar_parameters?: s_action_set_progress_bar_parameters;
  @AutoMap(() => s_action_hud_post_message_parameters)
  m_hud_post_message_parameters?: s_action_hud_post_message_parameters;
  @AutoMap(() => s_action_timer_set_rate_parameters)
  m_timer_set_rate_parameters?: s_action_timer_set_rate_parameters;
  @AutoMap(() => s_action_print_variable_parameters)
  m_print_variable_parameters?: s_action_print_variable_parameters;
  @AutoMap(() => s_action_get_player_holding_object_parameters)
  m_get_player_holding_object_parameters?: s_action_get_player_holding_object_parameters;
  @AutoMap(() => s_action_for_each_parameters)
  m_for_each_parameters?: s_action_for_each_parameters;
  @AutoMap(() => s_action_end_round_parameters)
  m_end_round_parameters?: s_action_end_round_parameters;
  @AutoMap(() => s_action_boundary_set_visible_parameters)
  m_boundary_set_visible_parameters?: s_action_boundary_set_visible_parameters;
  @AutoMap(() => s_action_object_destroy_parameters)
  m_object_destroy_parameters?: s_action_object_destroy_parameters;
  @AutoMap(() => s_action_object_set_invincibility_parameters)
  m_object_set_invincibility_parameters?: s_action_object_set_invincibility_parameters;
  @AutoMap(() => s_action_random_parameters)
  m_random_parameters?: s_action_random_parameters;
  @AutoMap(() => s_action_break_into_debugger_parameters)
  m_break_into_debugger_parameters?: s_action_break_into_debugger_parameters;
  @AutoMap(() => s_action_object_get_orientation_parameters)
  m_object_get_orientation_parameters?: s_action_object_get_orientation_parameters;
  @AutoMap(() => s_action_object_get_velocity_parameters)
  m_object_get_velocity_parameters?: s_action_object_get_velocity_parameters;
  @AutoMap(() => s_action_player_death_get_killing_player_parameters)
  m_player_death_get_killing_player_parameters?: s_action_player_death_get_killing_player_parameters;
  @AutoMap(() => s_action_player_death_get_damage_type_parameters)
  m_player_death_get_damage_type_parameters?: s_action_player_death_get_damage_type_parameters;
  @AutoMap(() => s_action_player_death_get_special_type_parameters)
  m_player_death_get_special_type_parameters?: s_action_player_death_get_special_type_parameters;
  @AutoMap(() => s_action_debugging_enable_tracing_parameters)
  m_debugging_enable_tracing_parameters?: s_action_debugging_enable_tracing_parameters;
  @AutoMap(() => s_action_object_attach_parameters)
  m_object_attach_parameters?: s_action_object_attach_parameters;
  @AutoMap(() => s_action_object_detach_parameters)
  m_object_detach_parameters?: s_action_object_detach_parameters;
  @AutoMap(() => s_action_player_get_place_parameters)
  m_player_get_place_parameters?: s_action_player_get_place_parameters;
  @AutoMap(() => s_action_team_get_place_parameters)
  m_team_get_place_parameters?: s_action_team_get_place_parameters;
  @AutoMap(() => s_action_player_get_killing_spree_count_parameters)
  m_player_get_killing_spree_count_parameters?: s_action_player_get_killing_spree_count_parameters;
  @AutoMap(() => s_action_player_adjust_money_parameters)
  m_player_adjust_money_parameters?: s_action_player_adjust_money_parameters;
  @AutoMap(() => s_action_player_enable_purchases_parameters)
  m_player_enable_purchases_parameters?: s_action_player_enable_purchases_parameters;
  @AutoMap(() => s_action_player_get_vehicle_parameters)
  m_player_get_vehicle_parameters?: s_action_player_get_vehicle_parameters;
  @AutoMap(() => s_action_player_set_vehicle_parameters)
  m_player_set_vehicle_parameters?: s_action_player_set_vehicle_parameters;
  @AutoMap(() => s_action_player_set_unit_parameters)
  m_player_set_unit_parameters?: s_action_player_set_unit_parameters;
  @AutoMap(() => s_action_timer_reset_parameters)
  m_timer_reset_parameters?: s_action_timer_reset_parameters;
  @AutoMap(() => s_action_weapon_set_pickup_priority_parameters)
  m_weapon_set_pickup_priority_parameters?: s_action_weapon_set_pickup_priority_parameters;
  @AutoMap(() => s_action_object_bounce_parameters)
  m_object_bounce_parameters?: s_action_object_bounce_parameters;
  @AutoMap(() => s_action_hud_widget_set_text_parameters)
  m_hud_widget_set_text_parameters?: s_action_hud_widget_set_text_parameters;
  @AutoMap(() => s_action_hud_widget_set_value_parameters)
  m_hud_widget_set_value_parameters?: s_action_hud_widget_set_value_parameters;
  @AutoMap(() => s_action_hud_widget_set_meter_parameters)
  m_hud_widget_set_meter_parameters?: s_action_hud_widget_set_meter_parameters;
  @AutoMap(() => s_action_hud_widget_set_icon_parameters)
  m_hud_widget_set_icon_parameters?: s_action_hud_widget_set_icon_parameters;
  @AutoMap(() => s_action_hud_widget_set_visibility_parameters)
  m_hud_widget_set_visibility_parameters?: s_action_hud_widget_set_visibility_parameters;
  @AutoMap(() => s_action_play_sound_parameters)
  m_play_sound_parameters?: s_action_play_sound_parameters;
  @AutoMap(() => s_action_object_set_scale_parameters)
  m_object_set_scale_parameters?: s_action_object_set_scale_parameters;
  @AutoMap(() => s_action_navpoint_set_text_parameters)
  m_navpoint_set_text_parameters?: s_action_navpoint_set_text_parameters;
  @AutoMap(() => s_action_object_get_shield_parameters)
  m_object_get_shield_parameters?: s_action_object_get_shield_parameters;
  @AutoMap(() => s_action_object_get_health_parameters)
  m_object_get_health_parameters?: s_action_object_get_health_parameters;
  @AutoMap(() => s_action_player_set_objective_parameters)
  m_player_set_objective_parameters?: s_action_player_set_objective_parameters;
  @AutoMap(() => s_action_player_set_objective_allegiance_parameters)
  m_player_set_objective_allegiance_parameters?: s_action_player_set_objective_allegiance_parameters;
  @AutoMap(() => s_action_player_set_objective_allegiance_icon_parameters)
  m_player_set_objective_allegiance_icon_parameters?: s_action_player_set_objective_allegiance_icon_parameters;
  @AutoMap(() => s_action_team_set_coop_spawning_parameters)
  m_team_set_coop_spawning_parameters?: s_action_team_set_coop_spawning_parameters;
  @AutoMap(() => s_action_team_set_primary_respawn_object_parameters)
  m_team_set_primary_respawn_object_parameters?: s_action_team_set_primary_respawn_object_parameters;
  @AutoMap(() => s_action_player_set_primary_respawn_object_parameters)
  m_player_set_primary_respawn_object_parameters?: s_action_player_set_primary_respawn_object_parameters;
  @AutoMap(() => s_action_player_get_fireteam_index_parameters)
  m_player_get_fireteam_index_parameters?: s_action_player_get_fireteam_index_parameters;
  @AutoMap(() => s_action_player_set_fireteam_index_parameters)
  m_player_set_fireteam_index_parameters?: s_action_player_set_fireteam_index_parameters;
  @AutoMap(() => s_action_object_adjust_shield_parameters)
  m_object_adjust_shield_parameters?: s_action_object_adjust_shield_parameters;
  @AutoMap(() => s_action_object_adjust_health_parameters)
  m_object_adjust_health_parameters?: s_action_object_adjust_health_parameters;
  @AutoMap(() => s_action_object_get_distance_parameters)
  m_object_get_distance_parameters?: s_action_object_get_distance_parameters;
  @AutoMap(() => s_action_object_adjust_maximum_shield_parameters)
  m_object_adjust_maximum_shield_parameters?: s_action_object_adjust_maximum_shield_parameters;
  @AutoMap(() => s_action_object_adjust_maximum_health_parameters)
  m_object_adjust_maximum_health_parameters?: s_action_object_adjust_maximum_health_parameters;
  @AutoMap(() => s_action_player_set_requisition_palette_parameters)
  m_player_set_requisition_palette_parameters?: s_action_player_set_requisition_palette_parameters;
  @AutoMap(() => s_action_device_set_power_parameters)
  m_device_set_power_parameters?: s_action_device_set_power_parameters;
  @AutoMap(() => s_action_device_get_power_parameters)
  m_device_get_power_parameters?: s_action_device_get_power_parameters;
  @AutoMap(() => s_action_device_set_position_parameters)
  m_device_set_position_parameters?: s_action_device_set_position_parameters;
  @AutoMap(() => s_action_device_get_position_parameters)
  m_device_get_position_parameters?: s_action_device_get_position_parameters;
  @AutoMap(() => s_action_adjust_grenades_parameters)
  m_adjust_grenades_parameters?: s_action_adjust_grenades_parameters;
  @AutoMap(() => s_action_submit_incident_parameters)
  m_submit_incident_parameters?: s_action_submit_incident_parameters;
  @AutoMap(() => s_action_submit_incident_with_custom_value_parameters)
  m_submit_incident_with_custom_value_parameters?: s_action_submit_incident_with_custom_value_parameters;
  @AutoMap(() => s_action_set_loadout_palette_parameters)
  m_set_loadout_palette_parameters?: s_action_set_loadout_palette_parameters;
  @AutoMap(() => s_action_device_set_position_track_parameters)
  m_device_set_position_track_parameters?: s_action_device_set_position_track_parameters;
  @AutoMap(() => s_action_device_animate_position_parameters)
  m_device_animate_position_parameters?: s_action_device_animate_position_parameters;
  @AutoMap(() => s_action_device_set_position_immediate_parameters)
  m_device_set_position_immediate_parameters?: s_action_device_set_position_immediate_parameters;
  @AutoMap(() => s_action_saved_film_insert_marker_parameters)
  m_saved_film_insert_marker_parameters?: s_action_saved_film_insert_marker_parameters;
  @AutoMap(() => s_action_respawn_zone_enable_parameters)
  m_respawn_zone_enable_parameters?: s_action_respawn_zone_enable_parameters;
  @AutoMap(() => s_action_player_get_weapon_parameters)
  m_player_get_weapon_parameters?: s_action_player_get_weapon_parameters;
  @AutoMap(() => s_action_player_get_equipment_parameters)
  m_player_get_equipment_parameters?: s_action_player_get_equipment_parameters;
  @AutoMap(() => s_action_object_set_never_garbage_parameters)
  m_object_set_never_garbage_parameters?: s_action_object_set_never_garbage_parameters;
  @AutoMap(() => s_action_player_get_target_object_parameters)
  m_player_get_target_object_parameters?: s_action_player_get_target_object_parameters;
  @AutoMap(() => s_action_create_tunnel_parameters)
  m_create_tunnel_parameters?: s_action_create_tunnel_parameters;
  @AutoMap(() => s_action_debug_force_player_view_count_parameters)
  m_debug_force_player_view_count_parameters?: s_action_debug_force_player_view_count_parameters;
  @AutoMap(() => s_action_player_pick_up_weapon_parameters)
  m_player_pick_up_weapon_parameters?: s_action_player_pick_up_weapon_parameters;
  @AutoMap(() => s_action_player_set_coop_spawning_parameters)
  m_player_set_coop_spawning_parameters?: s_action_player_set_coop_spawning_parameters;
  @AutoMap(() => s_action_object_set_orientation_parameters)
  m_object_set_orientation_parameters?: s_action_object_set_orientation_parameters;
  @AutoMap(() => s_action_object_face_object_parameters)
  m_object_face_object_parameters?: s_action_object_face_object_parameters;
  @AutoMap(() => s_action_biped_give_weapon_parameters)
  m_biped_give_weapon_parameters?: s_action_biped_give_weapon_parameters;
  @AutoMap(() => s_action_biped_drop_weapon_parameters)
  m_biped_drop_weapon_parameters?: s_action_biped_drop_weapon_parameters;
  @AutoMap(() => s_action_set_scenario_interpolator_state_parameters)
  m_set_scenario_interpolator_state_parameters?: s_action_set_scenario_interpolator_state_parameters;
  @AutoMap(() => s_action_get_random_object_parameters)
  m_get_random_object_parameters?: s_action_get_random_object_parameters;
  @AutoMap(() => s_action_game_grief_record_custom_penalty_parameters)
  m_game_grief_record_custom_penalty_parameters?: s_action_game_grief_record_custom_penalty_parameters;
  @AutoMap(() => s_action_boundary_set_player_color_parameters)
  m_boundary_set_player_color_parameters?: s_action_boundary_set_player_color_parameters;
  @AutoMap(() => s_action_hs_function_call_parameters)
  m_hs_function_call_parameters?: s_action_hs_function_call_parameters;
  @AutoMap(() => s_action_get_button_time_parameters)
  m_get_button_time_parameters?: s_action_get_button_time_parameters;
  @AutoMap(() => s_action_team_set_vehicle_spawning_parameters)
  m_team_set_vehicle_spawning_parameters?: s_action_team_set_vehicle_spawning_parameters;
  @AutoMap(() => s_action_player_set_vehicle_spawning_parameters)
  m_player_set_vehicle_spawning_parameters?: s_action_player_set_vehicle_spawning_parameters;
  @AutoMap(() => s_action_set_player_respawn_vehicle_parameters)
  m_set_player_respawn_vehicle_parameters?: s_action_set_player_respawn_vehicle_parameters;
  @AutoMap(() => s_action_set_team_respawn_vehicle_parameters)
  m_set_team_respawn_vehicle_parameters?: s_action_set_team_respawn_vehicle_parameters;
  @AutoMap(() => s_action_hide_object_parameters)
  m_hide_object_parameters?: s_action_hide_object_parameters;
  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_enum("action-type", 7, e_action_type);
    switch (this.m_type) {
      case e_action_type.set_score: {
        const set_score_parameters = new s_action_set_score_parameters();
        set_score_parameters.decode(bitstream);
        this.m_set_score_parameters = set_score_parameters;
        break;
      }
      case e_action_type.create_object: {
        const create_object_parameters = new s_action_create_object_parameters();
        create_object_parameters.decode(bitstream);
        this.m_create_object_parameters = create_object_parameters;
        break;
      }
      case e_action_type.delete_object: {
        const delete_object_parameters = new s_action_delete_object_parameters();
        delete_object_parameters.decode(bitstream);
        this.m_delete_object_parameters = delete_object_parameters;
        break;
      }
      case e_action_type.navpoint_set_visible: {
        const navpoint_set_visible_parameters = new s_action_navpoint_set_visible_parameters();
        navpoint_set_visible_parameters.decode(bitstream);
        this.m_navpoint_set_visible_parameters = navpoint_set_visible_parameters;
        break;
      }
      case e_action_type.navpoint_set_icon: {
        const navpoint_set_icon_parameters = new s_action_navpoint_set_icon_parameters();
        navpoint_set_icon_parameters.decode(bitstream);
        this.m_navpoint_set_icon_parameters = navpoint_set_icon_parameters;
        break;
      }
      case e_action_type.navpoint_set_priority: {
        const navpoint_set_priority_parameters = new s_action_navpoint_set_priority_parameters();
        navpoint_set_priority_parameters.decode(bitstream);
        this.m_navpoint_set_priority_parameters = navpoint_set_priority_parameters;
        break;
      }
      case e_action_type.navpoint_set_timer: {
        const navpoint_set_timer_parameters = new s_action_navpoint_set_timer_parameters();
        navpoint_set_timer_parameters.decode(bitstream);
        this.m_navpoint_set_timer_parameters = navpoint_set_timer_parameters;
        break;
      }
      case e_action_type.navpoint_set_visible_range: {
        const navpoint_set_visible_range_parameters = new s_action_navpoint_set_visible_range_parameters();
        navpoint_set_visible_range_parameters.decode(bitstream);
        this.m_navpoint_set_visible_range_parameters = navpoint_set_visible_range_parameters;
        break;
      }
      case e_action_type.set: {
        const set_parameters = new s_action_set_parameters();
        set_parameters.decode(bitstream);
        this.m_set_parameters = set_parameters;
        break;
      }
      case e_action_type.set_boundary: {
        const set_boundary_parameters = new s_action_set_boundary_parameters();
        set_boundary_parameters.decode(bitstream);
        this.m_set_boundary_parameters = set_boundary_parameters;
        break;
      }
      case e_action_type.apply_player_traits: {
        const apply_player_traits_parameters = new s_action_apply_player_traits_parameters();
        apply_player_traits_parameters.decode(bitstream);
        this.m_apply_player_traits_parameters = apply_player_traits_parameters;
        break;
      }
      case e_action_type.set_pickup_filter: {
        const set_pickup_filter_parameters = new s_action_set_pickup_filter_parameters();
        set_pickup_filter_parameters.decode(bitstream);
        this.m_set_pickup_filter_parameters = set_pickup_filter_parameters;
        break;
      }
      case e_action_type.set_respawn_filter: {
        const set_respawn_filter_parameters = new s_action_set_respawn_filter_parameters();
        set_respawn_filter_parameters.decode(bitstream);
        this.m_set_respawn_filter_parameters = set_respawn_filter_parameters;
        break;
      }
      case e_action_type.set_fireteam_respawn_filter: {
        const set_fireteam_respawn_filter_parameters = new s_action_set_fireteam_respawn_filter_parameters();
        set_fireteam_respawn_filter_parameters.decode(bitstream);
        this.m_set_fireteam_respawn_filter_parameters = set_fireteam_respawn_filter_parameters;
        break;
      }
      case e_action_type.set_progress_bar: {
        const set_progress_bar_parameters = new s_action_set_progress_bar_parameters();
        set_progress_bar_parameters.decode(bitstream);
        this.m_set_progress_bar_parameters = set_progress_bar_parameters;
        break;
      }
      case e_action_type.hud_post_message: {
        const hud_post_message_parameters = new s_action_hud_post_message_parameters();
        hud_post_message_parameters.decode(bitstream);
        this.m_hud_post_message_parameters = hud_post_message_parameters;
        break;
      }
      case e_action_type.timer_set_rate: {
        const timer_set_rate_parameters = new s_action_timer_set_rate_parameters();
        timer_set_rate_parameters.decode(bitstream);
        this.m_timer_set_rate_parameters = timer_set_rate_parameters;
        break;
      }
      case e_action_type.print_variable: {
        const print_variable_parameters = new s_action_print_variable_parameters();
        print_variable_parameters.decode(bitstream);
        this.m_print_variable_parameters = print_variable_parameters;
        break;
      }
      case e_action_type.get_player_holding_object: {
        const get_player_holding_object_parameters = new s_action_get_player_holding_object_parameters();
        get_player_holding_object_parameters.decode(bitstream);
        this.m_get_player_holding_object_parameters = get_player_holding_object_parameters;
        break;
      }
      case e_action_type.for_each: {
        const for_each_parameters = new s_action_for_each_parameters();
        for_each_parameters.decode(bitstream);
        this.m_for_each_parameters = for_each_parameters;
        break;
      }
      case e_action_type.end_round:
        break;
      case e_action_type.boundary_set_visible: {
        const boundary_set_visible_parameters = new s_action_boundary_set_visible_parameters();
        boundary_set_visible_parameters.decode(bitstream);
        this.m_boundary_set_visible_parameters = boundary_set_visible_parameters;
        break;
      }
      case e_action_type.object_destroy: {
        const object_destroy_parameters = new s_action_object_destroy_parameters();
        object_destroy_parameters.decode(bitstream);
        this.m_object_destroy_parameters = object_destroy_parameters;
        break;
      }
      case e_action_type.object_set_invincibility: {
        const object_set_invincibility_parameters = new s_action_object_set_invincibility_parameters();
        object_set_invincibility_parameters.decode(bitstream);
        this.m_object_set_invincibility_parameters = object_set_invincibility_parameters;
        break;
      }
      case e_action_type.random: {
        const random_parameters = new s_action_random_parameters();
        random_parameters.decode(bitstream);
        this.m_random_parameters = random_parameters;
        break;
      }
      case e_action_type.break_into_debugger:
        break;
      case e_action_type.object_get_orientation: {
        const object_get_orientation_parameters = new s_action_object_get_orientation_parameters();
        object_get_orientation_parameters.decode(bitstream);
        this.m_object_get_orientation_parameters = object_get_orientation_parameters;
        break;
      }
      case e_action_type.object_get_velocity: {
        const object_get_velocity_parameters = new s_action_object_get_velocity_parameters();
        object_get_velocity_parameters.decode(bitstream);
        this.m_object_get_velocity_parameters = object_get_velocity_parameters;
        break;
      }
      case e_action_type.player_death_get_killing_player: {
        const player_death_get_killing_player_parameters = new s_action_player_death_get_killing_player_parameters();
        player_death_get_killing_player_parameters.decode(bitstream);
        this.m_player_death_get_killing_player_parameters = player_death_get_killing_player_parameters;
        break;
      }
      case e_action_type.player_death_get_damage_type: {
        const player_death_get_damage_type_parameters = new s_action_player_death_get_damage_type_parameters();
        player_death_get_damage_type_parameters.decode(bitstream);
        this.m_player_death_get_damage_type_parameters = player_death_get_damage_type_parameters;
        break;
      }
      case e_action_type.player_death_get_special_type: {
        const player_death_get_special_type_parameters = new s_action_player_death_get_special_type_parameters();
        player_death_get_special_type_parameters.decode(bitstream);
        this.m_player_death_get_special_type_parameters = player_death_get_special_type_parameters;
        break;
      }
      case e_action_type.debugging_enable_tracing: {
        const debugging_enable_tracing_parameters = new s_action_debugging_enable_tracing_parameters();
        debugging_enable_tracing_parameters.decode(bitstream);
        this.m_debugging_enable_tracing_parameters = debugging_enable_tracing_parameters;
        break;
      }
      case e_action_type.object_attach: {
        const object_attach_parameters = new s_action_object_attach_parameters();
        object_attach_parameters.decode(bitstream);
        this.m_object_attach_parameters = object_attach_parameters;
        break;
      }
      case e_action_type.object_detach: {
        const object_detach_parameters = new s_action_object_detach_parameters();
        object_detach_parameters.decode(bitstream);
        this.m_object_detach_parameters = object_detach_parameters;
        break;
      }
      case e_action_type.player_get_place: {
        const player_get_place_parameters = new s_action_player_get_place_parameters();
        player_get_place_parameters.decode(bitstream);
        this.m_player_get_place_parameters = player_get_place_parameters;
        break;
      }
      case e_action_type.team_get_place: {
        const team_get_place_parameters = new s_action_team_get_place_parameters();
        team_get_place_parameters.decode(bitstream);
        this.m_team_get_place_parameters = team_get_place_parameters;
        break;
      }
      case e_action_type.player_get_killing_spree_count: {
        const player_get_killing_spree_count_parameters = new s_action_player_get_killing_spree_count_parameters();
        player_get_killing_spree_count_parameters.decode(bitstream);
        this.m_player_get_killing_spree_count_parameters = player_get_killing_spree_count_parameters;
        break;
      }
      case e_action_type.player_adjust_money: {
        const player_adjust_money_parameters = new s_action_player_adjust_money_parameters();
        player_adjust_money_parameters.decode(bitstream);
        this.m_player_adjust_money_parameters = player_adjust_money_parameters;
        break;
      }
      case e_action_type.player_enable_purchases: {
        const player_enable_purchases_parameters = new s_action_player_enable_purchases_parameters();
        player_enable_purchases_parameters.decode(bitstream);
        this.m_player_enable_purchases_parameters = player_enable_purchases_parameters;
        break;
      }
      case e_action_type.player_get_vehicle: {
        const player_get_vehicle_parameters = new s_action_player_get_vehicle_parameters();
        player_get_vehicle_parameters.decode(bitstream);
        this.m_player_get_vehicle_parameters = player_get_vehicle_parameters;
        break;
      }
      case e_action_type.player_set_vehicle: {
        const player_set_vehicle_parameters = new s_action_player_set_vehicle_parameters();
        player_set_vehicle_parameters.decode(bitstream);
        this.m_player_set_vehicle_parameters = player_set_vehicle_parameters;
        break;
      }
      case e_action_type.player_set_unit: {
        const player_set_unit_parameters = new s_action_player_set_unit_parameters();
        player_set_unit_parameters.decode(bitstream);
        this.m_player_set_unit_parameters = player_set_unit_parameters;
        break;
      }
      case e_action_type.timer_reset: {
        const timer_reset_parameters = new s_action_timer_reset_parameters();
        timer_reset_parameters.decode(bitstream);
        this.m_timer_reset_parameters = timer_reset_parameters;
        break;
      }
      case e_action_type.weapon_set_pickup_priority: {
        const weapon_set_pickup_priority_parameters = new s_action_weapon_set_pickup_priority_parameters();
        weapon_set_pickup_priority_parameters.decode(bitstream);
        this.m_weapon_set_pickup_priority_parameters = weapon_set_pickup_priority_parameters;
        break;
      }
      case e_action_type.object_bounce: {
        const object_bounce_parameters = new s_action_object_bounce_parameters();
        object_bounce_parameters.decode(bitstream);
        this.m_object_bounce_parameters = object_bounce_parameters;
        break;
      }
      case e_action_type.hud_widget_set_text: {
        const hud_widget_set_text_parameters = new s_action_hud_widget_set_text_parameters();
        hud_widget_set_text_parameters.decode(bitstream);
        this.m_hud_widget_set_text_parameters = hud_widget_set_text_parameters;
        break;
      }
      case e_action_type.hud_widget_set_value: {
        const hud_widget_set_value_parameters = new s_action_hud_widget_set_value_parameters();
        hud_widget_set_value_parameters.decode(bitstream);
        this.m_hud_widget_set_value_parameters = hud_widget_set_value_parameters;
        break;
      }
      case e_action_type.hud_widget_set_meter: {
        const hud_widget_set_meter_parameters = new s_action_hud_widget_set_meter_parameters();
        hud_widget_set_meter_parameters.decode(bitstream);
        this.m_hud_widget_set_meter_parameters = hud_widget_set_meter_parameters;
        break;
      }
      case e_action_type.hud_widget_set_icon: {
        const hud_widget_set_icon_parameters = new s_action_hud_widget_set_icon_parameters();
        hud_widget_set_icon_parameters.decode(bitstream);
        this.m_hud_widget_set_icon_parameters = hud_widget_set_icon_parameters;
        break;
      }
      case e_action_type.hud_widget_set_visibility: {
        const hud_widget_set_visibility_parameters = new s_action_hud_widget_set_visibility_parameters();
        hud_widget_set_visibility_parameters.decode(bitstream);
        this.m_hud_widget_set_visibility_parameters = hud_widget_set_visibility_parameters;
        break;
      }
      case e_action_type.play_sound: {
        const play_sound_parameters = new s_action_play_sound_parameters();
        play_sound_parameters.decode(bitstream);
        this.m_play_sound_parameters = play_sound_parameters;
        break;
      }
      case e_action_type.object_set_scale: {
        const object_set_scale_parameters = new s_action_object_set_scale_parameters();
        object_set_scale_parameters.decode(bitstream);
        this.m_object_set_scale_parameters = object_set_scale_parameters;
        break;
      }
      case e_action_type.navpoint_set_text: {
        const navpoint_set_text_parameters = new s_action_navpoint_set_text_parameters();
        navpoint_set_text_parameters.decode(bitstream);
        this.m_navpoint_set_text_parameters = navpoint_set_text_parameters;
        break;
      }
      case e_action_type.object_get_shield: {
        const object_get_shield_parameters = new s_action_object_get_shield_parameters();
        object_get_shield_parameters.decode(bitstream);
        this.m_object_get_shield_parameters = object_get_shield_parameters;
        break;
      }
      case e_action_type.object_get_health: {
        const object_get_health_parameters = new s_action_object_get_health_parameters();
        object_get_health_parameters.decode(bitstream);
        this.m_object_get_health_parameters = object_get_health_parameters;
        break;
      }
      case e_action_type.player_set_objective: {
        const player_set_objective_parameters = new s_action_player_set_objective_parameters();
        player_set_objective_parameters.decode(bitstream);
        this.m_player_set_objective_parameters = player_set_objective_parameters;
        break;
      }
      case e_action_type.player_set_objective_allegiance: {
        const player_set_objective_allegiance_parameters = new s_action_player_set_objective_allegiance_parameters();
        player_set_objective_allegiance_parameters.decode(bitstream);
        this.m_player_set_objective_allegiance_parameters = player_set_objective_allegiance_parameters;
        break;
      }
      case e_action_type.player_set_objective_allegiance_icon: {
        const player_set_objective_allegiance_icon_parameters = new s_action_player_set_objective_allegiance_icon_parameters();
        player_set_objective_allegiance_icon_parameters.decode(bitstream);
        this.m_player_set_objective_allegiance_icon_parameters = player_set_objective_allegiance_icon_parameters;
        break;
      }
      case e_action_type.team_set_coop_spawning: {
        const team_set_coop_spawning_parameters = new s_action_team_set_coop_spawning_parameters();
        team_set_coop_spawning_parameters.decode(bitstream);
        this.m_team_set_coop_spawning_parameters = team_set_coop_spawning_parameters;
        break;
      }
      case e_action_type.team_set_primary_respawn_object: {
        const team_set_primary_respawn_object_parameters = new s_action_team_set_primary_respawn_object_parameters();
        team_set_primary_respawn_object_parameters.decode(bitstream);
        this.m_team_set_primary_respawn_object_parameters = team_set_primary_respawn_object_parameters;
        break;
      }
      case e_action_type.player_set_primary_respawn_object: {
        const player_set_primary_respawn_object_parameters = new s_action_player_set_primary_respawn_object_parameters();
        player_set_primary_respawn_object_parameters.decode(bitstream);
        this.m_player_set_primary_respawn_object_parameters = player_set_primary_respawn_object_parameters;
        break;
      }
      case e_action_type.player_get_fireteam_index: {
        const player_get_fireteam_index_parameters = new s_action_player_get_fireteam_index_parameters();
        player_get_fireteam_index_parameters.decode(bitstream);
        this.m_player_get_fireteam_index_parameters = player_get_fireteam_index_parameters;
        break;
      }
      case e_action_type.player_set_fireteam_index: {
        const player_set_fireteam_index_parameters = new s_action_player_set_fireteam_index_parameters();
        player_set_fireteam_index_parameters.decode(bitstream);
        this.m_player_set_fireteam_index_parameters = player_set_fireteam_index_parameters;
        break;
      }
      case e_action_type.object_adjust_shield: {
        const object_adjust_shield_parameters = new s_action_object_adjust_shield_parameters();
        object_adjust_shield_parameters.decode(bitstream);
        this.m_object_adjust_shield_parameters = object_adjust_shield_parameters;
        break;
      }
      case e_action_type.object_adjust_health: {
        const object_adjust_health_parameters = new s_action_object_adjust_health_parameters();
        object_adjust_health_parameters.decode(bitstream);
        this.m_object_adjust_health_parameters = object_adjust_health_parameters;
        break;
      }
      case e_action_type.object_get_distance: {
        const object_get_distance_parameters = new s_action_object_get_distance_parameters();
        object_get_distance_parameters.decode(bitstream);
        this.m_object_get_distance_parameters = object_get_distance_parameters;
        break;
      }
      case e_action_type.object_adjust_maximum_shield: {
        const object_adjust_maximum_shield_parameters = new s_action_object_adjust_maximum_shield_parameters();
        object_adjust_maximum_shield_parameters.decode(bitstream);
        this.m_object_adjust_maximum_shield_parameters = object_adjust_maximum_shield_parameters;
        break;
      }
      case e_action_type.object_adjust_maximum_health: {
        const object_adjust_maximum_health_parameters = new s_action_object_adjust_maximum_health_parameters();
        object_adjust_maximum_health_parameters.decode(bitstream);
        this.m_object_adjust_maximum_health_parameters = object_adjust_maximum_health_parameters;
        break;
      }
      case e_action_type.player_set_requisition_palette: {
        const player_set_requisition_palette_parameters = new s_action_player_set_requisition_palette_parameters();
        player_set_requisition_palette_parameters.decode(bitstream);
        this.m_player_set_requisition_palette_parameters = player_set_requisition_palette_parameters;
        break;
      }
      case e_action_type.device_set_power: {
        const device_set_power_parameters = new s_action_device_set_power_parameters();
        device_set_power_parameters.decode(bitstream);
        this.m_device_set_power_parameters = device_set_power_parameters;
        break;
      }
      case e_action_type.device_get_power: {
        const device_get_power_parameters = new s_action_device_get_power_parameters();
        device_get_power_parameters.decode(bitstream);
        this.m_device_get_power_parameters = device_get_power_parameters;
        break;
      }
      case e_action_type.device_set_position: {
        const device_set_position_parameters = new s_action_device_set_position_parameters();
        device_set_position_parameters.decode(bitstream);
        this.m_device_set_position_parameters = device_set_position_parameters;
        break;
      }
      case e_action_type.device_get_position: {
        const device_get_position_parameters = new s_action_device_get_position_parameters();
        device_get_position_parameters.decode(bitstream);
        this.m_device_get_position_parameters = device_get_position_parameters;
        break;
      }
      case e_action_type.adjust_grenades: {
        const adjust_grenades_parameters = new s_action_adjust_grenades_parameters();
        adjust_grenades_parameters.decode(bitstream);
        this.m_adjust_grenades_parameters = adjust_grenades_parameters;
        break;
      }
      case e_action_type.submit_incident: {
        const submit_incident_parameters = new s_action_submit_incident_parameters();
        submit_incident_parameters.decode(bitstream);
        this.m_submit_incident_parameters = submit_incident_parameters;
        break;
      }
      case e_action_type.submit_incident_with_custom_value: {
        const submit_incident_with_custom_value_parameters = new s_action_submit_incident_with_custom_value_parameters();
        submit_incident_with_custom_value_parameters.decode(bitstream);
        this.m_submit_incident_with_custom_value_parameters = submit_incident_with_custom_value_parameters;
        break;
      }
      case e_action_type.set_loadout_palette: {
        const set_loadout_palette_parameters = new s_action_set_loadout_palette_parameters();
        set_loadout_palette_parameters.decode(bitstream);
        this.m_set_loadout_palette_parameters = set_loadout_palette_parameters;
        break;
      }
      case e_action_type.device_set_position_track: {
        const device_set_position_track_parameters = new s_action_device_set_position_track_parameters();
        device_set_position_track_parameters.decode(bitstream);
        this.m_device_set_position_track_parameters = device_set_position_track_parameters;
        break;
      }
      case e_action_type.device_animate_position: {
        const device_animate_position_parameters = new s_action_device_animate_position_parameters();
        device_animate_position_parameters.decode(bitstream);
        this.m_device_animate_position_parameters = device_animate_position_parameters;
        break;
      }
      case e_action_type.device_set_position_immediate: {
        const device_set_position_immediate_parameters = new s_action_device_set_position_immediate_parameters();
        device_set_position_immediate_parameters.decode(bitstream);
        this.m_device_set_position_immediate_parameters = device_set_position_immediate_parameters;
        break;
      }
      case e_action_type.saved_film_insert_marker: {
        const saved_film_insert_marker_parameters = new s_action_saved_film_insert_marker_parameters();
        saved_film_insert_marker_parameters.decode(bitstream);
        this.m_saved_film_insert_marker_parameters = saved_film_insert_marker_parameters;
        break;
      }
      case e_action_type.respawn_zone_enable: {
        const respawn_zone_enable_parameters = new s_action_respawn_zone_enable_parameters();
        respawn_zone_enable_parameters.decode(bitstream);
        this.m_respawn_zone_enable_parameters = respawn_zone_enable_parameters;
        break;
      }
      case e_action_type.player_get_weapon: {
        const player_get_weapon_parameters = new s_action_player_get_weapon_parameters();
        player_get_weapon_parameters.decode(bitstream);
        this.m_player_get_weapon_parameters = player_get_weapon_parameters;
        break;
      }
      case e_action_type.player_get_equipment: {
        const player_get_equipment_parameters = new s_action_player_get_equipment_parameters();
        player_get_equipment_parameters.decode(bitstream);
        this.m_player_get_equipment_parameters = player_get_equipment_parameters;
        break;
      }
      case e_action_type.object_set_never_garbage: {
        const object_set_never_garbage_parameters = new s_action_object_set_never_garbage_parameters();
        object_set_never_garbage_parameters.decode(bitstream);
        this.m_object_set_never_garbage_parameters = object_set_never_garbage_parameters;
        break;
      }
      case e_action_type.player_get_target_object: {
        const player_get_target_object_parameters = new s_action_player_get_target_object_parameters();
        player_get_target_object_parameters.decode(bitstream);
        this.m_player_get_target_object_parameters = player_get_target_object_parameters;
        break;
      }
      case e_action_type.create_tunnel: {
        const create_tunnel_parameters = new s_action_create_tunnel_parameters();
        create_tunnel_parameters.decode(bitstream);
        this.m_create_tunnel_parameters = create_tunnel_parameters;
        break;
      }
      case e_action_type.debug_force_player_view_count: {
        const debug_force_player_view_count_parameters = new s_action_debug_force_player_view_count_parameters();
        debug_force_player_view_count_parameters.decode(bitstream);
        this.m_debug_force_player_view_count_parameters = debug_force_player_view_count_parameters;
        break;
      }
      case e_action_type.player_pick_up_weapon: {
        const player_pick_up_weapon_parameters = new s_action_player_pick_up_weapon_parameters();
        player_pick_up_weapon_parameters.decode(bitstream);
        this.m_player_pick_up_weapon_parameters = player_pick_up_weapon_parameters;
        break;
      }
      case e_action_type.player_set_coop_spawning: {
        const player_set_coop_spawning_parameters = new s_action_player_set_coop_spawning_parameters();
        player_set_coop_spawning_parameters.decode(bitstream);
        this.m_player_set_coop_spawning_parameters = player_set_coop_spawning_parameters;
        break;
      }
      case e_action_type.object_set_orientation: {
        const object_set_orientation_parameters = new s_action_object_set_orientation_parameters();
        object_set_orientation_parameters.decode(bitstream);
        this.m_object_set_orientation_parameters = object_set_orientation_parameters;
        break;
      }
      case e_action_type.object_face_object: {
        const object_face_object_parameters = new s_action_object_face_object_parameters();
        object_face_object_parameters.decode(bitstream);
        this.m_object_face_object_parameters = object_face_object_parameters;
        break;
      }
      case e_action_type.biped_give_weapon: {
        const biped_give_weapon_parameters = new s_action_biped_give_weapon_parameters();
        biped_give_weapon_parameters.decode(bitstream);
        this.m_biped_give_weapon_parameters = biped_give_weapon_parameters;
        break;
      }
      case e_action_type.biped_drop_weapon: {
        const biped_drop_weapon_parameters = new s_action_biped_drop_weapon_parameters();
        biped_drop_weapon_parameters.decode(bitstream);
        this.m_biped_drop_weapon_parameters = biped_drop_weapon_parameters;
        break;
      }
      case e_action_type.set_scenario_interpolator_state: {
        const set_scenario_interpolator_state_parameters = new s_action_set_scenario_interpolator_state_parameters();
        set_scenario_interpolator_state_parameters.decode(bitstream);
        this.m_set_scenario_interpolator_state_parameters = set_scenario_interpolator_state_parameters;
        break;
      }
      case e_action_type.get_random_object: {
        const get_random_object_parameters = new s_action_get_random_object_parameters();
        get_random_object_parameters.decode(bitstream);
        this.m_get_random_object_parameters = get_random_object_parameters;
        break;
      }
      case e_action_type.game_grief_record_custom_penalty: {
        const game_grief_record_custom_penalty_parameters = new s_action_game_grief_record_custom_penalty_parameters();
        game_grief_record_custom_penalty_parameters.decode(bitstream);
        this.m_game_grief_record_custom_penalty_parameters = game_grief_record_custom_penalty_parameters;
        break;
      }
      case e_action_type.boundary_set_player_color: {
        const boundary_set_player_color_parameters = new s_action_boundary_set_player_color_parameters();
        boundary_set_player_color_parameters.decode(bitstream);
        this.m_boundary_set_player_color_parameters = boundary_set_player_color_parameters;
        break;
      }
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 7, e_action_type);
    switch (this.m_type) {
      case e_action_type.set_score:
        this.m_set_score_parameters?.encode(bitstream);
        break;
      case e_action_type.create_object:
        this.m_create_object_parameters?.encode(bitstream);
        break;
      case e_action_type.delete_object:
        this.m_delete_object_parameters?.encode(bitstream);
        break;
      case e_action_type.navpoint_set_visible:
        this.m_navpoint_set_visible_parameters?.encode(bitstream);
        break;
      case e_action_type.navpoint_set_icon:
        this.m_navpoint_set_icon_parameters?.encode(bitstream);
        break;
      case e_action_type.navpoint_set_priority:
        this.m_navpoint_set_priority_parameters?.encode(bitstream);
        break;
      case e_action_type.navpoint_set_timer:
        this.m_navpoint_set_timer_parameters?.encode(bitstream);
        break;
      case e_action_type.navpoint_set_visible_range:
        this.m_navpoint_set_visible_range_parameters?.encode(bitstream);
        break;
      case e_action_type.set:
        this.m_set_parameters?.encode(bitstream);
        break;
      case e_action_type.set_boundary:
        this.m_set_boundary_parameters?.encode(bitstream);
        break;
      case e_action_type.apply_player_traits:
        this.m_apply_player_traits_parameters?.encode(bitstream);
        break;
      case e_action_type.set_pickup_filter:
        this.m_set_pickup_filter_parameters?.encode(bitstream);
        break;
      case e_action_type.set_respawn_filter:
        this.m_set_respawn_filter_parameters?.encode(bitstream);
        break;
      case e_action_type.set_fireteam_respawn_filter:
        this.m_set_fireteam_respawn_filter_parameters?.encode(bitstream);
        break;
      case e_action_type.set_progress_bar:
        this.m_set_progress_bar_parameters?.encode(bitstream);
        break;
      case e_action_type.hud_post_message:
        this.m_hud_post_message_parameters?.encode(bitstream);
        break;
      case e_action_type.timer_set_rate:
        this.m_timer_set_rate_parameters?.encode(bitstream);
        break;
      case e_action_type.print_variable:
        this.m_print_variable_parameters?.encode(bitstream);
        break;
      case e_action_type.get_player_holding_object:
        this.m_get_player_holding_object_parameters?.encode(bitstream);
        break;
      case e_action_type.for_each:
        this.m_for_each_parameters?.encode(bitstream);
        break;
      case e_action_type.end_round:
        break;
      case e_action_type.boundary_set_visible:
        this.m_boundary_set_visible_parameters?.encode(bitstream);
        break;
      case e_action_type.object_destroy:
        this.m_object_destroy_parameters?.encode(bitstream);
        break;
      case e_action_type.object_set_invincibility:
        this.m_object_set_invincibility_parameters?.encode(bitstream);
        break;
      case e_action_type.random:
        this.m_random_parameters?.encode(bitstream);
        break;
      case e_action_type.break_into_debugger:
        break;
      case e_action_type.object_get_orientation:
        this.m_object_get_orientation_parameters?.encode(bitstream);
        break;
      case e_action_type.object_get_velocity:
        this.m_object_get_velocity_parameters?.encode(bitstream);
        break;
      case e_action_type.player_death_get_killing_player:
        this.m_player_death_get_killing_player_parameters?.encode(bitstream);
        break;
      case e_action_type.player_death_get_damage_type:
        this.m_player_death_get_damage_type_parameters?.encode(bitstream);
        break;
      case e_action_type.player_death_get_special_type:
        this.m_player_death_get_special_type_parameters?.encode(bitstream);
        break;
      case e_action_type.debugging_enable_tracing:
        this.m_debugging_enable_tracing_parameters?.encode(bitstream);
        break;
      case e_action_type.object_attach:
        this.m_object_attach_parameters?.encode(bitstream);
        break;
      case e_action_type.object_detach:
        this.m_object_detach_parameters?.encode(bitstream);
        break;
      case e_action_type.player_get_place:
        this.m_player_get_place_parameters?.encode(bitstream);
        break;
      case e_action_type.team_get_place:
        this.m_team_get_place_parameters?.encode(bitstream);
        break;
      case e_action_type.player_get_killing_spree_count:
        this.m_player_get_killing_spree_count_parameters?.encode(bitstream);
        break;
      case e_action_type.player_adjust_money:
        this.m_player_adjust_money_parameters?.encode(bitstream);
        break;
      case e_action_type.player_enable_purchases:
        this.m_player_enable_purchases_parameters?.encode(bitstream);
        break;
      case e_action_type.player_get_vehicle:
        this.m_player_get_vehicle_parameters?.encode(bitstream);
        break;
      case e_action_type.player_set_vehicle:
        this.m_player_set_vehicle_parameters?.encode(bitstream);
        break;
      case e_action_type.player_set_unit:
        this.m_player_set_unit_parameters?.encode(bitstream);
        break;
      case e_action_type.timer_reset:
        this.m_timer_reset_parameters?.encode(bitstream);
        break;
      case e_action_type.weapon_set_pickup_priority:
        this.m_weapon_set_pickup_priority_parameters?.encode(bitstream);
        break;
      case e_action_type.object_bounce:
        this.m_object_bounce_parameters?.encode(bitstream);
        break;
      case e_action_type.hud_widget_set_text:
        this.m_hud_widget_set_text_parameters?.encode(bitstream);
        break;
      case e_action_type.hud_widget_set_value:
        this.m_hud_widget_set_value_parameters?.encode(bitstream);
        break;
      case e_action_type.hud_widget_set_meter:
        this.m_hud_widget_set_meter_parameters?.encode(bitstream);
        break;
      case e_action_type.hud_widget_set_icon:
        this.m_hud_widget_set_icon_parameters?.encode(bitstream);
        break;
      case e_action_type.hud_widget_set_visibility:
        this.m_hud_widget_set_visibility_parameters?.encode(bitstream);
        break;
      case e_action_type.play_sound:
        this.m_play_sound_parameters?.encode(bitstream);
        break;
      case e_action_type.object_set_scale:
        this.m_object_set_scale_parameters?.encode(bitstream);
        break;
      case e_action_type.navpoint_set_text:
        this.m_navpoint_set_text_parameters?.encode(bitstream);
        break;
      case e_action_type.object_get_shield:
        this.m_object_get_shield_parameters?.encode(bitstream);
        break;
      case e_action_type.object_get_health:
        this.m_object_get_health_parameters?.encode(bitstream);
        break;
      case e_action_type.player_set_objective:
        this.m_player_set_objective_parameters?.encode(bitstream);
        break;
      case e_action_type.player_set_objective_allegiance:
        this.m_player_set_objective_allegiance_parameters?.encode(bitstream);
        break;
      case e_action_type.player_set_objective_allegiance_icon:
        this.m_player_set_objective_allegiance_icon_parameters?.encode(bitstream);
        break;
      case e_action_type.team_set_coop_spawning:
        this.m_team_set_coop_spawning_parameters?.encode(bitstream);
        break;
      case e_action_type.team_set_primary_respawn_object:
        this.m_team_set_primary_respawn_object_parameters?.encode(bitstream);
        break;
      case e_action_type.player_set_primary_respawn_object:
        this.m_player_set_primary_respawn_object_parameters?.encode(bitstream);
        break;
      case e_action_type.player_get_fireteam_index:
        this.m_player_get_fireteam_index_parameters?.encode(bitstream);
        break;
      case e_action_type.player_set_fireteam_index:
        this.m_player_set_fireteam_index_parameters?.encode(bitstream);
        break;
      case e_action_type.object_adjust_shield:
        this.m_object_adjust_shield_parameters?.encode(bitstream);
        break;
      case e_action_type.object_adjust_health:
        this.m_object_adjust_health_parameters?.encode(bitstream);
        break;
      case e_action_type.object_get_distance:
        this.m_object_get_distance_parameters?.encode(bitstream);
        break;
      case e_action_type.object_adjust_maximum_shield:
        this.m_object_adjust_maximum_shield_parameters?.encode(bitstream);
        break;
      case e_action_type.object_adjust_maximum_health:
        this.m_object_adjust_maximum_health_parameters?.encode(bitstream);
        break;
      case e_action_type.player_set_requisition_palette:
        this.m_player_set_requisition_palette_parameters?.encode(bitstream);
        break;
      case e_action_type.device_set_power:
        this.m_device_set_power_parameters?.encode(bitstream);
        break;
      case e_action_type.device_get_power:
        this.m_device_get_power_parameters?.encode(bitstream);
        break;
      case e_action_type.device_set_position:
        this.m_device_set_position_parameters?.encode(bitstream);
        break;
      case e_action_type.device_get_position:
        this.m_device_get_position_parameters?.encode(bitstream);
        break;
      case e_action_type.adjust_grenades:
        this.m_adjust_grenades_parameters?.encode(bitstream);
        break;
      case e_action_type.submit_incident:
        this.m_submit_incident_parameters?.encode(bitstream);
        break;
      case e_action_type.submit_incident_with_custom_value:
        this.m_submit_incident_with_custom_value_parameters?.encode(bitstream);
        break;
      case e_action_type.set_loadout_palette:
        this.m_set_loadout_palette_parameters?.encode(bitstream);
        break;
      case e_action_type.device_set_position_track:
        this.m_device_set_position_track_parameters?.encode(bitstream);
        break;
      case e_action_type.device_animate_position:
        this.m_device_animate_position_parameters?.encode(bitstream);
        break;
      case e_action_type.device_set_position_immediate:
        this.m_device_set_position_immediate_parameters?.encode(bitstream);
        break;
      case e_action_type.saved_film_insert_marker:
        this.m_saved_film_insert_marker_parameters?.encode(bitstream);
        break;
      case e_action_type.respawn_zone_enable:
        this.m_respawn_zone_enable_parameters?.encode(bitstream);
        break;
      case e_action_type.player_get_weapon:
        this.m_player_get_weapon_parameters?.encode(bitstream);
        break;
      case e_action_type.player_get_equipment:
        this.m_player_get_equipment_parameters?.encode(bitstream);
        break;
      case e_action_type.object_set_never_garbage:
        this.m_object_set_never_garbage_parameters?.encode(bitstream);
        break;
      case e_action_type.player_get_target_object:
        this.m_player_get_target_object_parameters?.encode(bitstream);
        break;
      case e_action_type.create_tunnel:
        this.m_create_tunnel_parameters?.encode(bitstream);
        break;
      case e_action_type.debug_force_player_view_count:
        this.m_debug_force_player_view_count_parameters?.encode(bitstream);
        break;
      case e_action_type.player_pick_up_weapon:
        this.m_player_pick_up_weapon_parameters?.encode(bitstream);
        break;
      case e_action_type.player_set_coop_spawning:
        this.m_player_set_coop_spawning_parameters?.encode(bitstream);
        break;
      case e_action_type.object_set_orientation:
        this.m_object_set_orientation_parameters?.encode(bitstream);
        break;
      case e_action_type.object_face_object:
        this.m_object_face_object_parameters?.encode(bitstream);
        break;
      case e_action_type.biped_give_weapon:
        this.m_biped_give_weapon_parameters?.encode(bitstream);
        break;
      case e_action_type.biped_drop_weapon:
        this.m_biped_drop_weapon_parameters?.encode(bitstream);
        break;
      case e_action_type.set_scenario_interpolator_state:
        this.m_set_scenario_interpolator_state_parameters?.encode(bitstream);
        break;
      case e_action_type.get_random_object:
        this.m_get_random_object_parameters?.encode(bitstream);
        break;
      case e_action_type.game_grief_record_custom_penalty:
        this.m_game_grief_record_custom_penalty_parameters?.encode(bitstream);
        break;
      case e_action_type.boundary_set_player_color:
        this.m_boundary_set_player_color_parameters?.encode(bitstream);
        break;
      case e_action_type.begin:
        break;
      case e_action_type.hs_function_call:
        this.m_hs_function_call_parameters?.encode(bitstream);
        break;
      case e_action_type.get_button_time:
        this.m_get_button_time_parameters?.encode(bitstream);
        break;
      case e_action_type.team_set_vehicle_spawning:
        this.m_team_set_vehicle_spawning_parameters?.encode(bitstream);
        break;
      case e_action_type.player_set_vehicle_spawning:
        this.m_player_set_vehicle_spawning_parameters?.encode(bitstream);
        break;
      case e_action_type.set_player_respawn_vehicle:
        this.m_set_player_respawn_vehicle_parameters?.encode(bitstream);
        break;
      case e_action_type.set_team_respawn_vehicle:
        this.m_set_team_respawn_vehicle_parameters?.encode(bitstream);
        break;
      case e_action_type.hide_object:
        this.m_hide_object_parameters?.encode(bitstream);
        break;
    }
  }
}
