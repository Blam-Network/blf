import { type c_bitstream_reader, c_bitstream_writer } from "../../../../../bitstream";
import { BlfError } from "../../../../../error";

function requireField<T>(value: T | undefined, message: string): T {
  if (value === undefined) {
    throw new BlfError(message);
  }
  return value;
}
import { c_player_traits } from "../c_player_traits";
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
  m_target = 0;
  m_team?: c_team_reference;
  m_player?: c_player_reference;



  decode(bitstream: c_bitstream_reader): void {
    this.m_target = bitstream.read_integer("target", 2);
    switch (this.m_target) {
      case 0: {
        const team = new c_team_reference();
        team.decode(bitstream);
        this.m_team = team;
        break;
      }
      case 1: {
        const player = new c_player_reference();
        player.decode(bitstream);
        this.m_player = player;
        break;
      }
      default:
        break;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_target, 2);
    switch (this.m_target) {
      case 0:
        this.m_team?.encode(bitstream);
        break;
      case 1:
        this.m_player?.encode(bitstream);
        break;
    }
  }


}

export class s_action_set_score_parameters {
  m_target = new s_team_or_player_target();
  m_operation = 0;
  m_variable = new c_custom_variable_reference();



  decode(bitstream: c_bitstream_reader): void {
    this.m_target.decode(bitstream);
    this.m_operation = bitstream.read_integer("operation", 4);
    this.m_variable.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_target.encode(bitstream);
            bitstream.write_integer(this.m_operation, 4);
            this.m_variable.encode(bitstream);
  }



}

export class s_action_create_object_parameters {
  m_object_type = new c_object_type_reference();
  m_object_reference_1 = new c_object_reference();
  m_object_reference_2 = new c_object_reference();
  m_filter_index = 0;
  m_flags = 0;
  m_offset = 0;
  m_variant_name_index = 0;



  decode(bitstream: c_bitstream_reader): void {
    this.m_object_type.decode(bitstream);
    this.m_object_reference_1.decode(bitstream);
    this.m_object_reference_2.decode(bitstream);
    this.m_filter_index = bitstream.read_index("filter_index", 16, 4);
    this.m_flags = bitstream.read_integer("flags", 3);
    this.m_offset = bitstream.read_integer("offset", 24);
    this.m_variant_name_index = bitstream.read_integer("variant-name-index", 8);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_object_type.encode(bitstream);
            this.m_object_reference_1.encode(bitstream);
            this.m_object_reference_2.encode(bitstream);
            bitstream.write_index(this.m_filter_index, 16, 4);
            bitstream.write_integer(this.m_flags, 3);
            bitstream.write_integer(this.m_offset, 24);
            bitstream.write_integer(this.m_variant_name_index, 8);
  }



}

export class s_action_navpoint_set_icon_parameters {
  m_object = new c_object_reference();
  m_navpoint_icon = 0;
  m_variable?: c_custom_variable_reference;



  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_navpoint_icon = bitstream.read_integer("navpoint-icon", 5);
    if (this.m_navpoint_icon === 12) {
      const variable = new c_custom_variable_reference();
      variable.decode(bitstream);
      this.m_variable = variable;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
    bitstream.write_integer(this.m_navpoint_icon, 5);
    if (this.m_navpoint_icon === 12) {
      this.m_variable!.encode(bitstream);
    }
  }


}

export class s_action_navpoint_set_priority_parameters {
  m_object = new c_object_reference();
  m_priority = 0;



  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_priority = bitstream.read_integer("priority", 2);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
            bitstream.write_integer(this.m_priority, 2);
  }



}

export class s_action_navpoint_set_timer_parameters {
  m_object = new c_object_reference();
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
  m_object = new c_object_reference();
  m_variable_1 = new c_custom_variable_reference();
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
  m_variable_1 = new s_variant_variable();
  m_variable_2 = new s_variant_variable();
  m_operation = 0;



  decode(bitstream: c_bitstream_reader): void {
    this.m_variable_1.decode(bitstream);
    this.m_variable_2.decode(bitstream);
    this.m_operation = bitstream.read_integer("operation", 4);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_variable_1.encode(bitstream);
            this.m_variable_2.encode(bitstream);
            bitstream.write_integer(this.m_operation, 4);
  }



}

export class s_action_set_boundary_parameters {
  m_object = new c_object_reference();
  m_shape: e_boundary_shape = e_boundary_shape.unused;
  m_variable_1?: c_custom_variable_reference;
  m_variable_2?: c_custom_variable_reference;
  m_variable_3?: c_custom_variable_reference;
  m_variable_4?: c_custom_variable_reference;



  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_shape = bitstream.read_enum("shape", 2, e_boundary_shape, {
      within_bits: true,
    });

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
    bitstream.write_enum(this.m_shape, 2);
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
  m_player = new c_player_reference();
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
  m_object = new c_object_reference();
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
  m_object = new c_object_reference();
  m_player_filter_modifier = new c_player_filter_modifier();
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
  m_target = new s_team_or_player_target();
  m_sound_index = 0;
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
  m_timer = new c_custom_timer_reference();
  m_rate = 0;



  decode(bitstream: c_bitstream_reader): void {
    this.m_timer.decode(bitstream);
    this.m_rate = bitstream.read_integer("timer-rate", 5);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_timer.encode(bitstream);
            bitstream.write_integer(this.m_rate, 5);
  }



}

export class s_action_for_each_parameters {
  m_trigger_index = 0;



  decode(bitstream: c_bitstream_reader): void {
    this.m_trigger_index = bitstream.read_integer("trigger-index", 9);
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_trigger_index, 9);
  }



}

export class s_action_object_destroy_parameters {
  m_object = new c_object_reference();
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
  m_object_1 = new c_object_reference();
  m_object_2 = new c_object_reference();
  m_offset = 0;
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
  m_player = new c_player_reference();
  m_math_operation = 0;
  m_variable = new c_custom_variable_reference();



  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_math_operation = bitstream.read_integer("math-operation", 4);
    this.m_variable.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
            bitstream.write_integer(this.m_math_operation, 4);
            this.m_variable.encode(bitstream);
  }



}

export class s_action_player_enable_purchases_parameters {
  m_player = new c_player_reference();
  m_variable = new c_custom_variable_reference();
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
  m_object = new c_object_reference();
  m_weapon_pickup_priority = 0;



  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_weapon_pickup_priority = bitstream.read_integer(
      "weapon-pickup-priority",
      2,
    );
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
            bitstream.write_integer(this.m_weapon_pickup_priority, 2);
  }



}

export class s_action_hud_widget_text_base {
  m_widget_index = 0;
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
  m_type = 0;
  m_variable_1?: c_custom_variable_reference;
  m_variable_2?: c_custom_variable_reference;
  m_timer?: c_custom_timer_reference;



  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_integer("type", 2);
    switch (this.m_type) {
      case 1: {
        const variable1 = new c_custom_variable_reference();
        const variable2 = new c_custom_variable_reference();
        variable1.decode(bitstream);
        variable2.decode(bitstream);
        this.m_variable_1 = variable1;
        this.m_variable_2 = variable2;
        break;
      }
      case 2: {
        const timer = new c_custom_timer_reference();
        timer.decode(bitstream);
        this.m_timer = timer;
        break;
      }
      default:
        break;
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    switch (this.m_type) {
      case 1:
        bitstream.write_integer(1, 2);
        this.m_variable_1!.encode(bitstream);
        this.m_variable_2!.encode(bitstream);
        break;
      case 2:
        bitstream.write_integer(2, 2);
        this.m_timer!.encode(bitstream);
        break;
    }
  }


}

export class s_action_hud_widget_set_meter_parameters {
  m_widget_index = 0;
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
  m_widget_index = 0;
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
  m_widget_index = 0;
  m_player = new c_player_reference();
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
  m_sound_index = 0;
  m_immediate = false;
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
  m_player = new c_player_reference();
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
  m_team = new c_team_reference();
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
  m_object = new c_object_reference();
  m_operation = 0;
  m_variable = new c_custom_variable_reference();



  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_operation = bitstream.read_integer("operation", 4);
    this.m_variable.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
            bitstream.write_integer(this.m_operation, 4);
            this.m_variable.encode(bitstream);
  }



}

export class s_action_object_get_distance_parameters {
  m_object_1 = new c_object_reference();
  m_object_2 = new c_object_reference();
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
  m_player = new c_player_reference();
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
  m_player = new c_player_reference();
  m_grenade_type = 0;
  m_math_operation = 0;
  m_variable = new c_custom_variable_reference();



  decode(bitstream: c_bitstream_reader): void {
    this.m_player.decode(bitstream);
    this.m_grenade_type = bitstream.read_integer("grenade-type", 1);
    this.m_math_operation = bitstream.read_integer("math-operation", 4);
    this.m_variable.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_player.encode(bitstream);
            bitstream.write_integer(this.m_grenade_type, 1);
            bitstream.write_integer(this.m_math_operation, 4);
            this.m_variable.encode(bitstream);
  }



}

export class s_action_submit_incident_parameters {
  m_incident_id = 0;
  m_target_1 = new s_team_or_player_target();
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
  m_incident_id = 0;
  m_target_1 = new s_team_or_player_target();
  m_target_2 = new s_team_or_player_target();
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
  m_target = new s_team_or_player_target();
  m_loadout_palette_index = 0;



  decode(bitstream: c_bitstream_reader): void {
    this.m_target.decode(bitstream);
    this.m_loadout_palette_index = bitstream.read_integer(
      "loadout-palette-index",
      3,
    );
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_target.encode(bitstream);
            bitstream.write_integer(this.m_loadout_palette_index, 3);
  }



}

export class s_action_device_set_position_track_parameters {
  m_object = new c_object_reference();
  m_animation_name_index = 0;
  m_variable = new c_custom_variable_reference();



  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_animation_name_index = bitstream.read_integer("animation-name-index", 8);
    this.m_variable.decode(bitstream);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
            bitstream.write_integer(this.m_animation_name_index, 8);
            this.m_variable.encode(bitstream);
  }



}

export class s_action_device_animate_position_parameters {
  m_object = new c_object_reference();
  m_variable_1 = new c_custom_variable_reference();
  m_variable_2 = new c_custom_variable_reference();
  m_variable_3 = new c_custom_variable_reference();
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
  m_player = new c_player_reference();
  m_primary = false;
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
  m_object_1 = new c_player_reference();
  m_object_2 = new c_player_reference();
  m_object_type = new c_object_reference();
  m_variable = new c_custom_variable_reference();
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
  m_player = new c_player_reference();
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
  m_object_1 = new c_object_reference();
  m_object_2 = new c_object_reference();
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
  m_object_1 = new c_object_reference();
  m_object_2 = new c_object_reference();
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
  m_object = new c_object_reference();
  m_object_type = new c_object_type_reference();
  m_mode = 0;



  decode(bitstream: c_bitstream_reader): void {
    this.m_object.decode(bitstream);
    this.m_object_type.decode(bitstream);
    this.m_mode = bitstream.read_integer("mode", 2);
  }

  encode(bitstream: c_bitstream_writer): void {
    this.m_object.encode(bitstream);
            this.m_object_type.encode(bitstream);
            bitstream.write_integer(this.m_mode, 2);
  }



}

export class s_action_biped_drop_weapon_parameters {
  m_object = new c_object_reference();
  m_primary = false;
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
  m_object_1 = new c_object_reference();
  m_object_2 = new c_object_reference();
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
  m_object = new c_object_reference();
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

export class c_action {
  m_type = 0;
  m_set_score_parameters?: s_action_set_score_parameters;
  m_create_object_parameters?: s_action_create_object_parameters;
  m_object?: c_object_reference;
  m_player_filter_modifier?: c_player_filter_modifier;
  m_navpoint_set_icon_parameters?: s_action_navpoint_set_icon_parameters;
  m_navpoint_set_priority_parameters?: s_action_navpoint_set_priority_parameters;
  m_navpoint_set_timer_parameters?: s_action_navpoint_set_timer_parameters;
  m_navpoint_set_visible_range_parameters?: s_action_navpoint_set_visible_range_parameters;
  m_set_parameters?: s_action_set_parameters;
  m_set_boundary_parameters?: s_action_set_boundary_parameters;
  m_apply_player_traits_parameters?: s_action_apply_player_traits_parameters;
  m_set_fireteam_respawn_filter_parameters?: s_action_set_fireteam_respawn_filter_parameters;
  m_set_progress_bar_parameters?: s_action_set_progress_bar_parameters;
  m_hud_post_message_parameters?: s_action_hud_post_message_parameters;
  m_timer_set_rate_parameters?: s_action_timer_set_rate_parameters;
  m_string?: c_dynamic_string;
  m_player_1?: c_player_reference;
  m_player_2?: c_player_reference;
  m_for_each_parameters?: s_action_for_each_parameters;
  m_object_destroy_parameters?: s_action_object_destroy_parameters;
  m_variable_1?: c_custom_variable_reference;
  m_variable_2?: c_custom_variable_reference;
  m_tracing_enabled?: boolean;
  m_object_attach_parameters?: s_action_object_attach_parameters;
  m_team?: c_team_reference;
  m_player_adjust_money_parameters?: s_action_player_adjust_money_parameters;
  m_player_enable_purchases_parameters?: s_action_player_enable_purchases_parameters;
  m_timer?: c_custom_timer_reference;
  m_weapon_set_pickup_priority_parameters?: s_action_weapon_set_pickup_priority_parameters;
  m_hud_widget_text_base?: s_action_hud_widget_text_base;
  m_hud_widget_set_meter_parameters?: s_action_hud_widget_set_meter_parameters;
  m_hud_widget_set_icon_parameters?: s_action_hud_widget_set_icon_parameters;
  m_hud_widget_set_visibility_parameters?: s_action_hud_widget_set_visibility_parameters;
  m_play_sound_parameters?: s_action_play_sound_parameters;
  m_player_set_objective_allegiance_icon_parameters?: s_action_player_set_objective_allegiance_icon_parameters;
  m_team_set_coop_spawning_parameters?: s_action_team_set_coop_spawning_parameters;
  m_vitality_adjustment_parameters?: s_action_vitality_adjustment_parameters;
  m_object_get_distance_parameters?: s_action_object_get_distance_parameters;
  m_player_set_requisition_palette_parameters?: s_action_player_set_requisition_palette_parameters;
  m_adjust_grenades_parameters?: s_action_adjust_grenades_parameters;
  m_submit_incident_parameters?: s_action_submit_incident_parameters;
  m_submit_incident_with_custom_value_parameters?: s_action_submit_incident_with_custom_value_parameters;
  m_set_loadout_palette_parameters?: s_action_set_loadout_palette_parameters;
  m_device_set_position_track_parameters?: s_action_device_set_position_track_parameters;
  m_device_animate_position_parameters?: s_action_device_animate_position_parameters;
  m_player_get_weapon_parameters?: s_action_player_get_weapon_parameters;
  m_create_tunnel_parameters?: s_action_create_tunnel_parameters;
  m_player_set_coop_spawning_parameters?: s_action_player_set_coop_spawning_parameters;
  m_object_set_orientation_parameters?: s_action_object_set_orientation_parameters;
  m_object_face_object_parameters?: s_action_object_face_object_parameters;
  m_biped_give_weapon_parameters?: s_action_biped_give_weapon_parameters;
  m_biped_drop_weapon_parameters?: s_action_biped_drop_weapon_parameters;
  m_get_random_object_parameters?: s_action_get_random_object_parameters;
  m_boundary_set_player_color_parameters?: s_action_boundary_set_player_color_parameters;
  m_player_traits?: c_player_traits;



  decode(bitstream: c_bitstream_reader): void {
    this.m_type = bitstream.read_integer("action-type", 7);

    switch (this.m_type) {
      case 1: {
        const setScoreParameters = new s_action_set_score_parameters();
        setScoreParameters.decode(bitstream);
        this.m_set_score_parameters = setScoreParameters;
        break;
      }
      case 2: {
        const createObjectParameters = new s_action_create_object_parameters();
        createObjectParameters.decode(bitstream);
        this.m_create_object_parameters = createObjectParameters;
        break;
      }
      case 3:
      case 34:
      case 45: {
        const object = new c_object_reference();
        object.decode(bitstream);
        this.m_object = object;
        break;
      }
      case 4:
      case 12:
      case 13:
      case 22: {
        const object = new c_object_reference();
        const playerFilterModifier = new c_player_filter_modifier();
        object.decode(bitstream);
        playerFilterModifier.decode(bitstream);
        this.m_object = object;
        this.m_player_filter_modifier = playerFilterModifier;
        break;
      }
      case 5: {
        const parameters = new s_action_navpoint_set_icon_parameters();
        parameters.decode(bitstream);
        this.m_navpoint_set_icon_parameters = parameters;
        break;
      }
      case 6: {
        const parameters = new s_action_navpoint_set_priority_parameters();
        parameters.decode(bitstream);
        this.m_navpoint_set_priority_parameters = parameters;
        break;
      }
      case 7: {
        const parameters = new s_action_navpoint_set_timer_parameters();
        parameters.decode(bitstream);
        this.m_navpoint_set_timer_parameters = parameters;
        break;
      }
      case 8: {
        const parameters = new s_action_navpoint_set_visible_range_parameters();
        parameters.decode(bitstream);
        this.m_navpoint_set_visible_range_parameters = parameters;
        break;
      }
      case 9: {
        const setParameters = new s_action_set_parameters();
        setParameters.decode(bitstream);
        this.m_set_parameters = setParameters;
        break;
      }
      case 10: {
        const setBoundaryParameters = new s_action_set_boundary_parameters();
        setBoundaryParameters.decode(bitstream);
        this.m_set_boundary_parameters = setBoundaryParameters;
        break;
      }
      case 11: {
        const applyPlayerTraitsParameters = new s_action_apply_player_traits_parameters();
        applyPlayerTraitsParameters.decode(bitstream);
        this.m_apply_player_traits_parameters = applyPlayerTraitsParameters;
        break;
      }
      case 14: {
        const parameters = new s_action_set_fireteam_respawn_filter_parameters();
        parameters.decode(bitstream);
        this.m_set_fireteam_respawn_filter_parameters = parameters;
        break;
      }
      case 15: {
        const parameters = new s_action_set_progress_bar_parameters();
        parameters.decode(bitstream);
        this.m_set_progress_bar_parameters = parameters;
        break;
      }
      case 16: {
        const parameters = new s_action_hud_post_message_parameters();
        parameters.decode(bitstream);
        this.m_hud_post_message_parameters = parameters;
        break;
      }
      case 17: {
        const parameters = new s_action_timer_set_rate_parameters();
        parameters.decode(bitstream);
        this.m_timer_set_rate_parameters = parameters;
        break;
      }
      case 18: {
        const string = new c_dynamic_string();
        string.decode(bitstream);
        this.m_string = string;
        break;
      }
      case 19: {
        const object = new c_object_reference();
        const player = new c_player_reference();
        object.decode(bitstream);
        player.decode(bitstream);
        this.m_object = object;
        this.m_player_1 = player;
        break;
      }
      case 20: {
        const parameters = new s_action_for_each_parameters();
        parameters.decode(bitstream);
        this.m_for_each_parameters = parameters;
        break;
      }
      case 21:
        // end_round — no payload (matches blf_lib e_action_type::end_round)
        break;
      case 23: {
        const parameters = new s_action_object_destroy_parameters();
        parameters.decode(bitstream);
        this.m_object_destroy_parameters = parameters;
        break;
      }
      case 24:
      case 27:
      case 28:
      case 52:
      case 54:
      case 55:
      case 70:
      case 71:
      case 72:
      case 73:
      case 80:
      case 82:
      case 85: {
        const object = new c_object_reference();
        const variable = new c_custom_variable_reference();
        object.decode(bitstream);
        variable.decode(bitstream);
        this.m_object = object;
        this.m_variable_1 = variable;
        break;
      }
      case 25:
      case 95: {
        const variable1 = new c_custom_variable_reference();
        const variable2 = new c_custom_variable_reference();
        variable1.decode(bitstream);
        variable2.decode(bitstream);
        this.m_variable_1 = variable1;
        this.m_variable_2 = variable2;
        break;
      }
      case 29: {
        const player1 = new c_player_reference();
        const player2 = new c_player_reference();
        player1.decode(bitstream);
        player2.decode(bitstream);
        this.m_player_1 = player1;
        this.m_player_2 = player2;
        break;
      }
      case 30:
      case 31:
      case 35:
      case 37:
      case 62:
      case 63:
      case 97: {
        const player1 = new c_player_reference();
        const variable1 = new c_custom_variable_reference();
        player1.decode(bitstream);
        variable1.decode(bitstream);
        this.m_player_1 = player1;
        this.m_variable_1 = variable1;
        break;
      }
      case 32:
        this.m_tracing_enabled = bitstream.read_bool("tracing-enabled");
        break;
      case 33: {
        const parameters = new s_action_object_attach_parameters();
        parameters.decode(bitstream);
        this.m_object_attach_parameters = parameters;
        break;
      }
      case 36: {
        const team = new c_team_reference();
        const variable = new c_custom_variable_reference();
        team.decode(bitstream);
        variable.decode(bitstream);
        this.m_team = team;
        this.m_variable_1 = variable;
        break;
      }
      case 38: {
        const parameters = new s_action_player_adjust_money_parameters();
        parameters.decode(bitstream);
        this.m_player_adjust_money_parameters = parameters;
        break;
      }
      case 39: {
        const parameters = new s_action_player_enable_purchases_parameters();
        parameters.decode(bitstream);
        this.m_player_enable_purchases_parameters = parameters;
        break;
      }
      case 40:
      case 41:
      case 42:
      case 61:
      case 84:
      case 86:
      case 89: {
        const player = new c_player_reference();
        const object = new c_object_reference();
        player.decode(bitstream);
        object.decode(bitstream);
        this.m_player_1 = player;
        this.m_object = object;
        break;
      }
      case 43: {
        const timer = new c_custom_timer_reference();
        timer.decode(bitstream);
        this.m_timer = timer;
        break;
      }
      case 44: {
        const parameters = new s_action_weapon_set_pickup_priority_parameters();
        parameters.decode(bitstream);
        this.m_weapon_set_pickup_priority_parameters = parameters;
        break;
      }
      case 46:
      case 47: {
        const parameters = new s_action_hud_widget_text_base();
        parameters.decode(bitstream);
        this.m_hud_widget_text_base = parameters;
        break;
      }
      case 48: {
        const parameters = new s_action_hud_widget_set_meter_parameters();
        parameters.decode(bitstream);
        this.m_hud_widget_set_meter_parameters = parameters;
        break;
      }
      case 49: {
        const parameters = new s_action_hud_widget_set_icon_parameters();
        parameters.decode(bitstream);
        this.m_hud_widget_set_icon_parameters = parameters;
        break;
      }
      case 50: {
        const parameters = new s_action_hud_widget_set_visibility_parameters();
        parameters.decode(bitstream);
        this.m_hud_widget_set_visibility_parameters = parameters;
        break;
      }
      case 51: {
        const parameters = new s_action_play_sound_parameters();
        parameters.decode(bitstream);
        this.m_play_sound_parameters = parameters;
        break;
      }
      case 53: {
        const object = new c_object_reference();
        const string = new c_dynamic_string();
        object.decode(bitstream);
        string.decode(bitstream);
        this.m_object = object;
        this.m_string = string;
        break;
      }
      case 56:
      case 57: {
        const player = new c_player_reference();
        const string = new c_dynamic_string();
        player.decode(bitstream);
        string.decode(bitstream);
        this.m_player_1 = player;
        this.m_string = string;
        break;
      }
      case 58: {
        const parameters = new s_action_player_set_objective_allegiance_icon_parameters();
        parameters.decode(bitstream);
        this.m_player_set_objective_allegiance_icon_parameters = parameters;
        break;
      }
      case 59: {
        const parameters = new s_action_team_set_coop_spawning_parameters();
        parameters.decode(bitstream);
        this.m_team_set_coop_spawning_parameters = parameters;
        break;
      }
      case 60: {
        const team = new c_team_reference();
        const object = new c_object_reference();
        team.decode(bitstream);
        object.decode(bitstream);
        this.m_team = team;
        this.m_object = object;
        break;
      }
      case 64:
      case 65:
      case 67:
      case 68: {
        const parameters = new s_action_vitality_adjustment_parameters();
        parameters.decode(bitstream);
        this.m_vitality_adjustment_parameters = parameters;
        break;
      }
      case 66: {
        const parameters = new s_action_object_get_distance_parameters();
        parameters.decode(bitstream);
        this.m_object_get_distance_parameters = parameters;
        break;
      }
      case 69: {
        const parameters = new s_action_player_set_requisition_palette_parameters();
        parameters.decode(bitstream);
        this.m_player_set_requisition_palette_parameters = parameters;
        break;
      }
      case 74: {
        const parameters = new s_action_adjust_grenades_parameters();
        parameters.decode(bitstream);
        this.m_adjust_grenades_parameters = parameters;
        break;
      }
      case 75: {
        const parameters = new s_action_submit_incident_parameters();
        parameters.decode(bitstream);
        this.m_submit_incident_parameters = parameters;
        break;
      }
      case 76: {
        const parameters = new s_action_submit_incident_with_custom_value_parameters();
        parameters.decode(bitstream);
        this.m_submit_incident_with_custom_value_parameters = parameters;
        break;
      }
      case 77: {
        const parameters = new s_action_set_loadout_palette_parameters();
        parameters.decode(bitstream);
        this.m_set_loadout_palette_parameters = parameters;
        break;
      }
      case 78: {
        const parameters = new s_action_device_set_position_track_parameters();
        parameters.decode(bitstream);
        this.m_device_set_position_track_parameters = parameters;
        break;
      }
      case 79: {
        const parameters = new s_action_device_animate_position_parameters();
        parameters.decode(bitstream);
        this.m_device_animate_position_parameters = parameters;
        break;
      }
      case 81: {
        const variable = new c_custom_variable_reference();
        const string = new c_dynamic_string();
        variable.decode(bitstream);
        string.decode(bitstream);
        this.m_variable_1 = variable;
        this.m_string = string;
        break;
      }
      case 83: {
        const parameters = new s_action_player_get_weapon_parameters();
        parameters.decode(bitstream);
        this.m_player_get_weapon_parameters = parameters;
        break;
      }
      case 87: {
        const parameters = new s_action_create_tunnel_parameters();
        parameters.decode(bitstream);
        this.m_create_tunnel_parameters = parameters;
        break;
      }
      case 88: {
        const variable = new c_custom_variable_reference();
        variable.decode(bitstream);
        this.m_variable_1 = variable;
        break;
      }
      case 90: {
        const parameters = new s_action_player_set_coop_spawning_parameters();
        parameters.decode(bitstream);
        this.m_player_set_coop_spawning_parameters = parameters;
        break;
      }
      case 91: {
        const parameters = new s_action_object_set_orientation_parameters();
        parameters.decode(bitstream);
        this.m_object_set_orientation_parameters = parameters;
        break;
      }
      case 92: {
        const parameters = new s_action_object_face_object_parameters();
        parameters.decode(bitstream);
        this.m_object_face_object_parameters = parameters;
        break;
      }
      case 93: {
        const parameters = new s_action_biped_give_weapon_parameters();
        parameters.decode(bitstream);
        this.m_biped_give_weapon_parameters = parameters;
        break;
      }
      case 94: {
        const parameters = new s_action_biped_drop_weapon_parameters();
        parameters.decode(bitstream);
        this.m_biped_drop_weapon_parameters = parameters;
        break;
      }
      case 96: {
        const parameters = new s_action_get_random_object_parameters();
        parameters.decode(bitstream);
        this.m_get_random_object_parameters = parameters;
        break;
      }
      case 98: {
        const parameters = new s_action_boundary_set_player_color_parameters();
        parameters.decode(bitstream);
        this.m_boundary_set_player_color_parameters = parameters;
        break;
      }
      default:
        break;
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_type, 7);
    switch (this.m_type) {
      case 1: {
        this.m_set_score_parameters!.encode(bitstream);
        break;
      }
      case 2: {
        this.m_create_object_parameters!.encode(bitstream);
        break;
      }
      case 3:
      case 34:
      case 45: {
        this.m_object!.encode(bitstream);
        break;
      }
      case 4:
      case 12:
      case 13:
      case 22: {
        this.m_object!.encode(bitstream);
        this.m_player_filter_modifier!.encode(bitstream);
        break;
      }
      case 5: {
        this.m_navpoint_set_icon_parameters!.encode(bitstream);
        break;
      }
      case 6: {
        this.m_navpoint_set_priority_parameters!.encode(bitstream);
        break;
      }
      case 7: {
        this.m_navpoint_set_timer_parameters!.encode(bitstream);
        break;
      }
      case 8: {
        this.m_navpoint_set_visible_range_parameters!.encode(bitstream);
        break;
      }
      case 9: {
        this.m_set_parameters!.encode(bitstream);
        break;
      }
      case 10: {
        this.m_set_boundary_parameters!.encode(bitstream);
        break;
      }
      case 11: {
        this.m_apply_player_traits_parameters!.encode(bitstream);
        break;
      }
      case 14: {
        this.m_set_fireteam_respawn_filter_parameters!.encode(bitstream);
        break;
      }
      case 15: {
        this.m_set_progress_bar_parameters!.encode(bitstream);
        break;
      }
      case 16: {
        this.m_hud_post_message_parameters!.encode(bitstream);
        break;
      }
      case 17: {
        this.m_timer_set_rate_parameters!.encode(bitstream);
        break;
      }
      case 18: {
        this.m_string!.encode(bitstream);
        break;
      }
      case 19: {
        this.m_object!.encode(bitstream);
        this.m_player_1!.encode(bitstream);
        break;
      }
      case 20: {
        this.m_for_each_parameters!.encode(bitstream);
        break;
      }
      case 21:
        break;
      case 23: {
        this.m_object_destroy_parameters!.encode(bitstream);
        break;
      }
      case 24:
      case 27:
      case 28:
      case 52:
      case 54:
      case 55:
      case 70:
      case 71:
      case 72:
      case 73:
      case 80:
      case 82:
      case 85: {
        this.m_object!.encode(bitstream);
        this.m_variable_1!.encode(bitstream);
        break;
      }
      case 25:
      case 95: {
        this.m_variable_1!.encode(bitstream);
        this.m_variable_2!.encode(bitstream);
        break;
      }
      case 29: {
        this.m_player_1!.encode(bitstream);
        this.m_player_2!.encode(bitstream);
        break;
      }
      case 30:
      case 31:
      case 35:
      case 37:
      case 62:
      case 63:
      case 97: {
        this.m_player_1!.encode(bitstream);
        this.m_variable_1!.encode(bitstream);
        break;
      }
      case 32: {
        bitstream.write_bool(this.m_tracing_enabled!);
        break;
      }
      case 33: {
        this.m_object_attach_parameters!.encode(bitstream);
        break;
      }
      case 36: {
        this.m_team!.encode(bitstream);
        this.m_variable_1!.encode(bitstream);
        break;
      }
      case 38: {
        this.m_player_adjust_money_parameters!.encode(bitstream);
        break;
      }
      case 39: {
        this.m_player_enable_purchases_parameters!.encode(bitstream);
        break;
      }
      case 40:
      case 41:
      case 42:
      case 61:
      case 84:
      case 86:
      case 89: {
        this.m_player_1!.encode(bitstream);
        this.m_object!.encode(bitstream);
        break;
      }
      case 43: {
        this.m_timer!.encode(bitstream);
        break;
      }
      case 44: {
        this.m_weapon_set_pickup_priority_parameters!.encode(bitstream);
        break;
      }
      case 46:
      case 47: {
        this.m_hud_widget_text_base!.encode(bitstream);
        break;
      }
      case 48: {
        this.m_hud_widget_set_meter_parameters!.encode(bitstream);
        break;
      }
      case 49: {
        this.m_hud_widget_set_icon_parameters!.encode(bitstream);
        break;
      }
      case 50: {
        this.m_hud_widget_set_visibility_parameters!.encode(bitstream);
        break;
      }
      case 51: {
        this.m_play_sound_parameters!.encode(bitstream);
        break;
      }
      case 53: {
        this.m_object!.encode(bitstream);
        this.m_string!.encode(bitstream);
        break;
      }
      case 56:
      case 57: {
        this.m_player_1!.encode(bitstream);
        this.m_string!.encode(bitstream);
        break;
      }
      case 58: {
        this.m_player_set_objective_allegiance_icon_parameters!.encode(bitstream);
        break;
      }
      case 59: {
        this.m_team_set_coop_spawning_parameters!.encode(bitstream);
        break;
      }
      case 60: {
        this.m_team!.encode(bitstream);
        this.m_object!.encode(bitstream);
        break;
      }
      case 64:
      case 65:
      case 67:
      case 68: {
        this.m_vitality_adjustment_parameters!.encode(bitstream);
        break;
      }
      case 66: {
        this.m_object_get_distance_parameters!.encode(bitstream);
        break;
      }
      case 69: {
        this.m_player_set_requisition_palette_parameters!.encode(bitstream);
        break;
      }
      case 74: {
        this.m_adjust_grenades_parameters!.encode(bitstream);
        break;
      }
      case 75: {
        this.m_submit_incident_parameters!.encode(bitstream);
        break;
      }
      case 76: {
        this.m_submit_incident_with_custom_value_parameters!.encode(bitstream);
        break;
      }
      case 77: {
        this.m_set_loadout_palette_parameters!.encode(bitstream);
        break;
      }
      case 78: {
        this.m_device_set_position_track_parameters!.encode(bitstream);
        break;
      }
      case 79: {
        this.m_device_animate_position_parameters!.encode(bitstream);
        break;
      }
      case 81: {
        this.m_variable_1!.encode(bitstream);
        this.m_string!.encode(bitstream);
        break;
      }
      case 83: {
        this.m_player_get_weapon_parameters!.encode(bitstream);
        break;
      }
      case 87: {
        this.m_create_tunnel_parameters!.encode(bitstream);
        break;
      }
      case 88: {
        this.m_variable_1!.encode(bitstream);
        break;
      }
      case 90: {
        this.m_player_set_coop_spawning_parameters!.encode(bitstream);
        break;
      }
      case 91: {
        this.m_object_set_orientation_parameters!.encode(bitstream);
        break;
      }
      case 92: {
        this.m_object_face_object_parameters!.encode(bitstream);
        break;
      }
      case 93: {
        this.m_biped_give_weapon_parameters!.encode(bitstream);
        break;
      }
      case 94: {
        this.m_biped_drop_weapon_parameters!.encode(bitstream);
        break;
      }
      case 96: {
        this.m_get_random_object_parameters!.encode(bitstream);
        break;
      }
      case 98: {
        this.m_boundary_set_player_color_parameters!.encode(bitstream);
        break;
      }
      default:
        break;
    }
  }
  
  


}
