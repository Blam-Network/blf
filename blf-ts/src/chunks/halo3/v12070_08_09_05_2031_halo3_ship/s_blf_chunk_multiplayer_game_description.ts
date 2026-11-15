import { c } from "@craftycodie/cstruct";
import { time32_t } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/cseries/cseries_types";
import { s_scenario_path } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_options";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

export { s_scenario_path };

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
