import { c } from "@craftycodie/cstruct";
import { time32_t } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/cseries/cseries_types";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

const k_scenario_path_length = 260;

/**
 * A `char[260]` scenario path whose first byte's top bit marks a DLC map
 * (DLC paths aren't otherwise valid text). The bit is cleared from `path`.
 */
export interface s_scenario_path {
  is_dlc: boolean;
  path: string;
}

const utf8_decoder = new TextDecoder("utf-8");
const utf8_encoder = new TextEncoder();

export const s_scenario_path = () =>
  c.bytes(k_scenario_path_length).transform(
    (raw): s_scenario_path => {
      const bytes = Uint8Array.from(raw);
      const is_dlc = (bytes[0] & 0x80) !== 0;
      bytes[0] &= 0x7f;
      const end = bytes.indexOf(0);
      return {
        is_dlc,
        path: utf8_decoder.decode(
          bytes.subarray(0, end === -1 ? bytes.length : end)
        ),
      };
    },
    (value: s_scenario_path) => {
      const encoded = utf8_encoder.encode(value.path);
      if (encoded.length > k_scenario_path_length) {
        throw new Error(
          `Scenario path is too long (${encoded.length} > ${k_scenario_path_length} bytes)`
        );
      }
      const bytes = new Uint8Array(k_scenario_path_length);
      bytes.set(encoded);
      if (value.is_dlc) {
        bytes[0] |= 0x80;
      }
      return bytes;
    }
  );

/** Halo 3 game results game description (`mpgd` 2.1, 0x174 bytes). */
@blf.chunk("mpgd", 2.1)
export class s_blf_chunk_multiplayer_game_description extends CStructBLFChunk(
  c.struct({
    team_game: c.bool(),
    game_id: c.u64(),
    map_id: c.u32(),
    scenario_path: s_scenario_path(),
    map_variant_name: c.wstring(0x20),
    game_variant_unique_id: c.u64(),
    map_variant_unique_id: c.u64(),
    started: c.bool(),
    start_time: time32_t(),
    finished: c.bool(),
    finish_time: time32_t(),
    migrated_to_group: c.bool(),
    migrated_solo: c.bool(),
    simulation_aborted: c.bool().padAfter(6),
  })
) {}
