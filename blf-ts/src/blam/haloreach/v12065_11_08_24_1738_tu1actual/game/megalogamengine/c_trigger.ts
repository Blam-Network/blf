import { type c_bitstream_reader, c_bitstream_writer } from "../../../../../bitstream";

export enum e_trigger_execution_mode {
  normal = 0,
  for_each_player = 1,
  for_each_player_randomly = 2,
  for_each_team = 3,
  for_each_object = 4,
  for_each_object_with_label = 5,
  unknown6 = 6,
  unknown7 = 7,
}

export enum e_trigger_type {
  normal = 0,
  subroutine = 1,
  on_init = 2,
  on_local_init = 3,
  on_host_migration = 4,
  on_object_death = 5,
  local = 6,
  pregame = 7,
  incident = 8,
}

export class c_trigger {
  m_execution_mode: e_trigger_execution_mode = e_trigger_execution_mode.normal;
  m_trigger_type: e_trigger_type = e_trigger_type.normal;
  m_object_filter_index = -1;
  m_first_condition = 0;
  m_condition_count = 0;
  m_first_action = 0;
  m_action_count = 0;

  decode(bitstream: c_bitstream_reader): void {
    this.m_execution_mode = bitstream.read_enum(
      "execution-mode",
      3,
      e_trigger_execution_mode,
      { within_bits: true },
    );
    this.m_trigger_type = bitstream.read_enum(
      "trigger-type",
      3,
      e_trigger_type,
      { within_bits: true },
    );
    if (
      this.m_execution_mode ===
      e_trigger_execution_mode.for_each_object_with_label
    ) {
      this.m_object_filter_index = bitstream.read_index(
        "object-filter-index",
        16,
        4,
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
    bitstream.write_enum(this.m_execution_mode, 3);
    bitstream.write_enum(this.m_trigger_type, 3);
    if (
      this.m_execution_mode ===
      e_trigger_execution_mode.for_each_object_with_label
    ) {
      bitstream.write_index(this.m_object_filter_index, 16, 4);
    }
    bitstream.write_integer(this.m_first_condition, 9);
    bitstream.write_integer(this.m_condition_count, 10);
    bitstream.write_integer(this.m_first_action, 10);
    bitstream.write_integer(this.m_action_count, 11);
  }
}
