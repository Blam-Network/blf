import { c } from "@craftycodie/cstruct";
import {
  default_purchase_state,
  e_purchase_state_flags,
  s_persistent_per_commendation_state,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/player_rewards";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach TU1 rewards persistence download (`rpdl` 2.1, 0x21B-byte body). */
@blf.chunk("rpdl", 2.1)
@c.struct()
export class s_blf_chunk_rewards_persistance extends CStructBLFChunk {
  @c.field("u32")
  credits = 0;

  @c.field("u32")
  unknown1 = 0;

  @c.field(s_persistent_per_commendation_state, { count: 128 })
  commendations = Array.from(
    { length: 128 },
    () => new s_persistent_per_commendation_state()
  );

  @c.field(c.bitfield("u8", e_purchase_state_flags), { count: 256 })
  purchased_items = Array.from({ length: 256 }, () => default_purchase_state());

  @c.field("u16")
  unknown2 = 0;

  @c.field("u32")
  unknown3 = 0;

  @c.field(c.Time64())
  unknown4 = new Date(0);

  @c.field("u32")
  awarded_credits = 0;

  @c.field("u8")
  unknown6 = 0;
}
