import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../../bitstream";
import { AutoMap } from "../../../../../helpers/automap";

/** Halo 4 `e_trigger_execution_mode` (`c_enum<...,0,7>` → 0..6, 3 bits). */
export enum e_trigger_execution_mode {
  general = 0,
  player = 1,
  random_player = 2,
  team = 3,
  object = 4,
  object_with_label = 5,
  game_object = 6,
}

/** Halo 4 `e_trigger_type` (`c_enum<...,0,9>` → 0..8, 4 bits). */
export enum e_trigger_type {
  normal = 0,
  subroutine = 1,
  initialization = 2,
  local_initialization = 3,
  host_migration = 4,
  double_migration = 5,
  object_death = 6,
  local = 7,
  pregame = 8,
}

/** `MegaloGameObjectTypeEnum` (`c_enum<...,0,2>` → 0..1, 1 bit). */
export enum e_megalo_game_object_type {
  none = 0,
  candy_spawner = 1,
}

/**
 * Halo 4 `c_trigger` wire layout (IDA `c_trigger::encode` @ 0x831c4078):
 * execution-mode, trigger-type, optional filter indices, MegaloActionScope,
 * frame-update-frequency/offset.
 *
 * MegaloActionScope first-condition/action and frame-update indices use
 * biased `write_index` (+1), not the bool+value optional form.
 */
export class c_trigger {
  @AutoMap(() => e_trigger_execution_mode)
  m_execution_mode: e_trigger_execution_mode = e_trigger_execution_mode.general;
  @AutoMap(() => e_trigger_type)
  m_trigger_type: e_trigger_type = e_trigger_type.normal;
  @AutoMap(() => Number)
  m_object_filter_index = -1;
  @AutoMap(() => e_megalo_game_object_type)
  m_game_object_type: e_megalo_game_object_type =
    e_megalo_game_object_type.none;
  @AutoMap(() => Number)
  m_game_object_filter_index = -1;
  @AutoMap(() => Number)
  m_first_condition = 0;
  @AutoMap(() => Number)
  m_condition_count = 0;
  @AutoMap(() => Number)
  m_first_action = 0;
  @AutoMap(() => Number)
  m_action_count = 0;
  @AutoMap(() => Number)
  m_frame_update_frequency = -1;
  @AutoMap(() => Number)
  m_frame_update_offset = -1;

  decode(bitstream: c_bitstream_reader): void {
    this.m_execution_mode = bitstream.read_enum(
      "execution-mode",
      3,
      e_trigger_execution_mode
    );
    this.m_trigger_type = bitstream.read_enum(
      "trigger-type",
      4,
      e_trigger_type
    );
    this.m_object_filter_index = -1;
    this.m_game_object_filter_index = -1;
    if (this.m_execution_mode === e_trigger_execution_mode.object_with_label) {
      this.m_object_filter_index = bitstream.read_index(
        "object-filter-index",
        16,
        4
      );
    } else if (this.m_execution_mode === e_trigger_execution_mode.game_object) {
      this.m_game_object_type = bitstream.read_enum(
        "game-object-type",
        1,
        e_megalo_game_object_type
      );
      this.m_game_object_filter_index = bitstream.read_index(
        "game-object-filter-index",
        4,
        2
      );
    }
    // MegaloActionScope::Encode — biased indices
    this.m_first_condition = bitstream.read_index(
      "first-condition-index",
      576,
      10
    );
    this.m_condition_count = bitstream.read_integer("condition-count", 10);
    this.m_first_action = bitstream.read_index("first-action-index", 1088, 11);
    this.m_action_count = bitstream.read_integer("action-count", 11);
    this.m_frame_update_frequency = bitstream.read_index(
      "frame-update-frequency",
      255,
      8
    );
    this.m_frame_update_offset = bitstream.read_index(
      "frame-update-offset",
      255,
      8
    );
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_execution_mode, 3, e_trigger_execution_mode);
    bitstream.write_enum(this.m_trigger_type, 4, e_trigger_type);
    if (this.m_execution_mode === e_trigger_execution_mode.object_with_label) {
      bitstream.write_index(this.m_object_filter_index, 16, 4);
    } else if (this.m_execution_mode === e_trigger_execution_mode.game_object) {
      bitstream.write_enum(
        this.m_game_object_type,
        1,
        e_megalo_game_object_type
      );
      bitstream.write_index(this.m_game_object_filter_index, 4, 2);
    }
    bitstream.write_index(this.m_first_condition, 576, 10);
    bitstream.write_integer(this.m_condition_count, 10);
    bitstream.write_index(this.m_first_action, 1088, 11);
    bitstream.write_integer(this.m_action_count, 11);
    bitstream.write_index(this.m_frame_update_frequency, 255, 8);
    bitstream.write_index(this.m_frame_update_offset, 255, 8);
  }
}
