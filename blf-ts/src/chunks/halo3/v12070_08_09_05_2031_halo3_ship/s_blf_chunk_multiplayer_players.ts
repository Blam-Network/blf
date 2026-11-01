import { c } from "@craftycodie/cstruct";
import { k_maximum_players } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_options";
import {
  s_player_configuration_from_client,
  s_player_configuration_from_host,
} from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/players";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** 0x11E bytes. */
export const s_blf_chunk_multiplayer_players_player = c.struct({
  player_exists: c.bool(),
  machine_index: c.i8(),
  player_identifier: c.u64(),
  player_configuration_from_client: s_player_configuration_from_client,
  player_configuration_from_host: s_player_configuration_from_host,
  standing: c.i8(),
  result: c.i8(),
  score: c.i16(),
});
export type s_blf_chunk_multiplayer_players_player = c.infer<
  typeof s_blf_chunk_multiplayer_players_player
>;

/** Halo 3 game results players (`mppl` 2.1, 0x11E4 bytes). */
@blf.chunk("mppl", 2.1)
export class s_blf_chunk_multiplayer_players extends CStructBLFChunk(
  c.struct({
    players: s_blf_chunk_multiplayer_players_player
      .array(k_maximum_players)
      .padBefore(4),
  })
) {}
