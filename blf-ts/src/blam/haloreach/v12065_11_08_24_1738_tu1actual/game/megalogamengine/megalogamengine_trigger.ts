import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";
export enum e_trigger_execution_mode {
  general = 0,
  player = 1,
  random_player = 2,
  team = 3,
  object = 4,
  object_with_label = 5,
}
export enum e_trigger_type {
  normal = 0,
  subroutine = 1,
  initialization = 2,
  local_initialization = 3,
  host_migration = 4,
  object_death = 5,
  local = 6,
  pregame = 7,
}

export class c_trigger {
  @AutoMap(() => e_trigger_execution_mode)
  m_execution_mode: e_trigger_execution_mode = e_trigger_execution_mode.general;
  @AutoMap(() => e_trigger_type)
  m_trigger_type: e_trigger_type = e_trigger_type.normal;
  @AutoMap(() => Number)
  m_object_filter_index = -1;
  @AutoMap(() => Number)
  m_first_condition = 0;
  @AutoMap(() => Number)
  m_condition_count = 0;
  @AutoMap(() => Number)
  m_first_action = 0;
  @AutoMap(() => Number)
  m_action_count = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_execution_mode = bitstream.read_enum(
      "execution-mode",
      3,
      e_trigger_execution_mode
    );
    this.m_trigger_type = bitstream.read_enum(
      "trigger-type",
      3,
      e_trigger_type
    );
    if (
      this.m_execution_mode ===
      e_trigger_execution_mode.object_with_label
    ) {
      this.m_object_filter_index = bitstream.read_index(
        "object-filter-index",
        16,
        4
      );
    } else {
      this.m_object_filter_index = -1;
    }
    this.m_first_condition = bitstream.read_integer("first-condition", 9);
    this.m_condition_count = bitstream.read_integer("condition-count", 10);
    this.m_first_action = bitstream.read_integer("first-action", 10);
    this.m_action_count = bitstream.read_integer("action-count", 11);
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_execution_mode, 3, e_trigger_execution_mode);
    bitstream.write_enum(this.m_trigger_type, 3, e_trigger_type);
    if (
      this.m_execution_mode ===
      e_trigger_execution_mode.object_with_label
    ) {
      bitstream.write_index(this.m_object_filter_index, 16, 4);
    }
    bitstream.write_integer(this.m_first_condition, 9);
    bitstream.write_integer(this.m_condition_count, 10);
    bitstream.write_integer(this.m_first_action, 10);
    bitstream.write_integer(this.m_action_count, 11);
  }
}
