import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { e_bitstream_byte_order } from "../bitstream";
import { c_bitstream_reader } from "../bitstream/reader";
import { c_game_engine_custom_variant } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/c_game_engine_custom_variant";
import { s_player_trait_option } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/c_game_engine_traits";
import { e_game_mode } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/c_game_variant";
import {
  c_action,
  e_chud_navpoint_icon_type,
} from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/megalogamengine_actions";
import { c_condition } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/megalogamengine_conditions";
import { s_user_defined_option } from "../blam/haloreach_mcc/v_untracked_25_08_16_1352/game/megalogamengine/s_user_defined_option";
import { find_chunk } from "../blf_chunk";
import { s_blf_chunk_game_variant } from "../chunks/haloreach_mcc/v_untracked_25_08_16_1352/s_blf_chunk_game_variant";

const HEADHUNTER =
  "c:/Program Files (x86)/Steam/steamapps/common/Halo The Master Chief Collection/haloreach/hopper_game_variants/headhunter_054.bin";

const MPVR_OFFSET = 0x2f0;

function openGametypeReader(): c_bitstream_reader {
  const file = new Uint8Array(readFileSync(HEADHUNTER));
  const payload = file.subarray(MPVR_OFFSET + 12);
  const header = c_bitstream_reader.new(
    payload,
    e_bitstream_byte_order._bitstream_byte_order_big_endian
  );
  header.begin_reading();
  header.read_raw_data(20 * 8);
  header.read_signed_integer("unknown04", 16);
  header.read_integer("unknown06", 16);
  header.read_integer("variant-length", 32);
  header.finish_reading();

  const [byteOffset] = header.get_current_offset();
  const gametypeBytes = payload.subarray(byteOffset);
  const reader = c_bitstream_reader.new(
    gametypeBytes,
    e_bitstream_byte_order._bitstream_byte_order_big_endian
  );
  reader.begin_reading();
  return reader;
}

function decodeCustomVariantThroughConditions(
  reader: c_bitstream_reader
): number {
  const custom = new c_game_engine_custom_variant();
  custom.m_encoding_version = reader.read_signed_integer(
    "encoding-version",
    32
  );
  custom.m_build_number = reader.read_signed_integer("version", 32);
  custom.m_base_variant.decode(reader);

  const playerTraitCount = reader.read_integer("player-trait-count", 5);
  for (let i = 0; i < playerTraitCount; i++) {
    new s_player_trait_option().decode(reader);
  }

  const userDefinedOptionCount = reader.read_integer(
    "user-defined-option-count",
    5
  );
  for (let i = 0; i < userDefinedOptionCount; i++) {
    new s_user_defined_option().decode(reader);
  }

  custom.m_script_strings.decode(reader);
  reader.read_integer("base-name-string-index", 7);
  custom.m_localized_name.decode(reader);
  custom.m_localized_description.decode(reader);
  custom.m_localized_category.decode(reader);
  reader.read_integer("engine-icon-index", 5);
  reader.read_integer("engine-category", 5);
  custom.m_map_permissions.decode(reader);
  custom.m_player_ratings.decode(reader);
  reader.read_signed_integer("score-to-win-round", 16);
  reader.read_bool("fire-teams-enabled");
  reader.read_bool("symmetric-gametype");
  for (let i = 0; i < 1280; i++) {
    reader.read_bool("base-variant-parameters-locked");
  }
  for (let i = 0; i < 1280; i++) {
    reader.read_bool("base-variant-parameters-hidden");
  }
  for (let i = 0; i < 32; i++) {
    reader.read_bool("user-defined-options-locked");
  }
  for (let i = 0; i < 32; i++) {
    reader.read_bool("user-defined-options-hidden");
  }

  const conditionCount = reader.read_integer("condition-count", 10);
  for (let i = 0; i < conditionCount; i++) {
    new c_condition().decode(reader);
  }

  return reader.read_integer("action-count", 11);
}

function decodeActionsThrough(count: number): c_bitstream_reader {
  const reader = openGametypeReader();
  reader.read_enum("game-engine", 4, e_game_mode);
  const actionCount = decodeCustomVariantThroughConditions(reader);
  expect(actionCount).toBeGreaterThanOrEqual(count);
  for (let i = 0; i < count; i++) {
    new c_action().decode(reader);
  }
  return reader;
}

describe("headhunter_054.bin MCC decode", () => {
  it("decodes full mpvr after navpoint icon wire offset fix", () => {
    const chunk = new s_blf_chunk_game_variant();
    expect(
      find_chunk(new Uint8Array(readFileSync(HEADHUNTER)), chunk, "big")
    ).toBe(true);

    const beforeIcon = decodeActionsThrough(39);
    const icon39 = new c_action();
    icon39.decode(beforeIcon);
    expect(icon39.m_navpoint_set_icon_parameters?.m_navpoint_icon).toBe(
      e_chud_navpoint_icon_type.none
    );

    const beforeIcon2 = decodeActionsThrough(40);
    const icon40 = new c_action();
    icon40.decode(beforeIcon2);
    expect(icon40.m_navpoint_set_icon_parameters?.m_navpoint_icon).toBe(
      e_chud_navpoint_icon_type.num
    );
    expect(
      icon40.m_navpoint_set_icon_parameters?.m_navpoint_number
    ).toBeDefined();

    const reader = openGametypeReader();
    reader.read_enum("game-engine", 4, e_game_mode);
    const actionCount = decodeCustomVariantThroughConditions(reader);
    for (let i = 0; i < actionCount; i++) {
      expect(() => new c_action().decode(reader)).not.toThrow();
    }
  });
});
