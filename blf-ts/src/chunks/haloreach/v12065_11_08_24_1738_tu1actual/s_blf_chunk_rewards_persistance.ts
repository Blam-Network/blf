import { c } from "@craftycodie/cstruct";
import {
  e_purchase_state_flags,
  s_persistent_per_commendation_state,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/player_rewards";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach TU1 rewards persistence download (`rpdl` 2.1, 0x21B-byte body). */
@blf.chunk("rpdl", 2.1)
export class s_blf_chunk_rewards_persistance extends CStructBLFChunk(
  c.struct({
    cookies: c.u32(),
    cookie_award_count: c.u32(),
    commendations: s_persistent_per_commendation_state.array(128),
    purchased_items: c.Bitfield(e_purchase_state_flags, c.u8()).array(256),
    jackpot_hopper_identifier: c.u16(),
    jackpot_cookies: c.u32(),
    jackpot_expiration_time: c.Time64(),
    awarded_credits: c.u32(),
    unknown1: c.u8(),
  })
) {}
