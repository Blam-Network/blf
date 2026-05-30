import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";
import { c_megalo_game_statistic } from "./c_megalo_game_statistic";
import { c_object_filter } from "./c_object_filter";
import { c_trigger } from "./c_trigger";
import { c_action } from "./megalogamengine_actions";
import { c_condition } from "./megalogamengine_conditions";
import {
  s_variable_metadata,
  s_variable_metadata_global,
  s_variable_metadata_object,
  s_variable_metadata_player,
  s_variable_metadata_team,
} from "./s_variable_metadata";
export class s_custom_game_engine_definition {
  @AutoMap(() => [c_condition])
  m_conditions: c_condition[] = [];
  @AutoMap(() => [c_action])
  m_actions: c_action[] = [];
  @AutoMap(() => [c_trigger])
  m_triggers: c_trigger[] = [];
  @AutoMap(() => [c_megalo_game_statistic])
  m_statistics: c_megalo_game_statistic[] = [];
  @AutoMap(() => s_variable_metadata)
  m_global_variable_metadata = s_variable_metadata_global();
  @AutoMap(() => s_variable_metadata)
  m_player_variable_metadata = s_variable_metadata_player();
  @AutoMap(() => s_variable_metadata)
  m_object_variable_metadata = s_variable_metadata_object();
  @AutoMap(() => s_variable_metadata)
  m_team_variable_metadata = s_variable_metadata_team();
  @AutoMap(() => [Number])
  m_hud_widgets: number[] = [];
  @AutoMap(() => Number)
  m_initialization_trigger_index = 0;
  @AutoMap(() => Number)
  m_local_initialization_trigger_index = 0;
  @AutoMap(() => Number)
  m_host_migration_trigger_index = 0;
  @AutoMap(() => Number)
  m_double_migration_trigger_index = 0;
  @AutoMap(() => Number)
  m_object_death_event_trigger_index = 0;
  @AutoMap(() => Number)
  m_local_trigger_index = 0;
  @AutoMap(() => Number)
  m_pregame_trigger_index = 0;
  @AutoMap(() => [Boolean])
  m_objects_used: boolean[] = Array.from({ length: 2048 }, () => false);
  @AutoMap(() => [c_object_filter])
  m_object_filters: c_object_filter[] = [];
  decode(bitstream: c_bitstream_reader): void {
    const condition_count = bitstream.read_integer("condition-count", 10);
    for (let i = 0; i < condition_count; i++) {
      const condition = new c_condition();
      condition.decode(bitstream);
      this.m_conditions.push(condition);
    }
    const action_count = bitstream.read_integer("action-count", 11);
    for (let i = 0; i < action_count; i++) {
      const action = new c_action();
      action.decode(bitstream);
      this.m_actions.push(action);
    }
    const trigger_count = bitstream.read_integer("trigger-count", 9);
    for (let i = 0; i < trigger_count; i++) {
      const trigger = new c_trigger();
      trigger.decode(bitstream);
      this.m_triggers.push(trigger);
    }
    const statistic_count = bitstream.read_integer("game-statistic-count", 3);
    for (let i = 0; i < statistic_count; i++) {
      const statistic = new c_megalo_game_statistic();
      statistic.decode(bitstream);
      this.m_statistics.push(statistic);
    }
    this.m_global_variable_metadata.decode(bitstream);
    this.m_player_variable_metadata.decode(bitstream);
    this.m_object_variable_metadata.decode(bitstream);
    this.m_team_variable_metadata.decode(bitstream);
    const widget_count = bitstream.read_integer("hud-widget-count", 3);
    for (let i = 0; i < widget_count; i++) {
      this.m_hud_widgets.push(bitstream.read_integer("position", 4));
    }
    this.m_initialization_trigger_index = bitstream.read_integer(
      "initial-trigger-index",
      9
    );
    this.m_local_initialization_trigger_index = bitstream.read_integer(
      "local-initialization-trigger-index",
      9
    );
    this.m_host_migration_trigger_index = bitstream.read_integer(
      "host-migration-trigger-index",
      9
    );
    this.m_double_migration_trigger_index = bitstream.read_integer(
      "double-migration-trigger-index",
      9
    );
    this.m_object_death_event_trigger_index = bitstream.read_integer(
      "death-event-trigger-index",
      9
    );
    this.m_local_trigger_index = bitstream.read_integer(
      "local-trigger-index",
      9
    );
    this.m_pregame_trigger_index = bitstream.read_integer(
      "pregame-trigger-index",
      9
    );
    for (let i = 0; i < 2048; i++) {
      this.m_objects_used[i] = bitstream.read_bool("object-types-used");
    }
    const object_filter_count = bitstream.read_integer(
      "object-filter-count",
      5
    );
    for (let i = 0; i < object_filter_count; i++) {
      const filter = new c_object_filter();
      filter.decode(bitstream);
      this.m_object_filters.push(filter);
    }
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_conditions.length, 10);
    for (const condition of this.m_conditions) {
      condition.encode(bitstream);
    }
    bitstream.write_integer(this.m_actions.length, 11);
    for (const action of this.m_actions) {
      action.encode(bitstream);
    }
    bitstream.write_integer(this.m_triggers.length, 9);
    for (const trigger of this.m_triggers) {
      trigger.encode(bitstream);
    }
    bitstream.write_integer(this.m_statistics.length, 3);
    for (const statistic of this.m_statistics) {
      statistic.encode(bitstream);
    }
    this.m_global_variable_metadata.encode(bitstream);
    this.m_player_variable_metadata.encode(bitstream);
    this.m_object_variable_metadata.encode(bitstream);
    this.m_team_variable_metadata.encode(bitstream);
    bitstream.write_integer(this.m_hud_widgets.length, 3);
    for (const widget of this.m_hud_widgets) {
      bitstream.write_integer(widget, 4);
    }
    bitstream.write_integer(this.m_initialization_trigger_index, 9);
    bitstream.write_integer(this.m_local_initialization_trigger_index, 9);
    bitstream.write_integer(this.m_host_migration_trigger_index, 9);
    bitstream.write_integer(this.m_double_migration_trigger_index, 9);
    bitstream.write_integer(this.m_object_death_event_trigger_index, 9);
    bitstream.write_integer(this.m_local_trigger_index, 9);
    bitstream.write_integer(this.m_pregame_trigger_index, 9);
    for (const used of this.m_objects_used) {
      bitstream.write_bool(used);
    }
    bitstream.write_integer(this.m_object_filters.length, 5);
    for (const filter of this.m_object_filters) {
      filter.encode(bitstream);
    }
  }
}
