import { c } from "@craftycodie/cstruct";
import {
  s_player_configuration_from_client,
  s_player_configuration_from_host,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/players";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach TU1 carnage report / multiplayer results player slot (0x140 bytes). */
@c.struct()
export class s_blf_chunk_multiplayer_players_player {
  @c.field(c.Bool())
  player_exists = false;

  @c.field("u8", { count: 6 })
  machine_identifier = Array.from({ length: 6 }, () => 0);

  @c.field("u64")
  player_identifier = 0n;

  @c.field(s_player_configuration_from_client, { pad_before: 1 })
  player_configuration_from_client = new s_player_configuration_from_client();

  @c.field(s_player_configuration_from_host)
  player_configuration_from_host = new s_player_configuration_from_host();

  @c.field("i8", { pad_before: 3 })
  standing = 0;

  @c.field("u32", { count: 16 })
  player_ratings = Array.from({ length: 16 }, () => 0);

  @c.field("i8")
  result = 0;

  @c.field("i16", { pad_before: 1 })
  score = 0;
}

/** Reach TU1 carnage report player roster (`mppl` 8.1, 0x1404-byte body). */
@blf.chunk("mppl", 8.1)
@c.struct()
export class s_blf_chunk_multiplayer_players extends CStructBLFChunk {
  @c.field(s_blf_chunk_multiplayer_players_player, { count: 16, pad_before: 4 })
  players = Array.from(
    { length: 16 },
    () => new s_blf_chunk_multiplayer_players_player()
  );
}
