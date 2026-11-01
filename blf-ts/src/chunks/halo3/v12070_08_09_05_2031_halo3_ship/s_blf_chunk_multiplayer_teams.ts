import { c } from "@craftycodie/cstruct";
import { k_maximum_teams } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_options";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

export const s_blf_chunk_multiplayer_teams_team = c.struct({
  exists: c.bool(),
  standing: c.i8(),
  score: c.i16(),
});
export type s_blf_chunk_multiplayer_teams_team = c.infer<
  typeof s_blf_chunk_multiplayer_teams_team
>;

/** Halo 3 game results teams (`mptm` 1.1, 0x44 bytes). */
@blf.chunk("mptm", 1.1)
export class s_blf_chunk_multiplayer_teams extends CStructBLFChunk(
  c.struct({
    teams: s_blf_chunk_multiplayer_teams_team
      .array(k_maximum_teams)
      .padBefore(4),
  })
) {}
