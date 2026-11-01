import { c } from "@craftycodie/cstruct";
import {
  e_purchase_state_flags,
  s_persistent_per_commendation_state,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/player_rewards";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach TU1 rewards upload to LSP (`rpul` 3.1, 0x778-byte body). */
@blf.chunk("rpul", 3.1)
export class s_blf_chunk_reward_persistence_upload_to_lsp extends CStructBLFChunk(
  c.struct({
    alltime_cookie_count: c.u32(),
    alltime_cookie_award_count: c.u32(),
    alltime_commendation_progress:
      s_persistent_per_commendation_state.array(128),
    alltime_purchased_items: c
      .Bitfield(e_purchase_state_flags, c.u8())
      .array(256),
    cookies_earned_today_online: c.u32(),
    cookie_award_count_today_online: c.u32(),
    commendation_progress_today_online:
      s_persistent_per_commendation_state.array(128),
    purchased_items_today_online: c
      .Bitfield(e_purchase_state_flags, c.u8())
      .array(256),
    cookies_earned_today_offline: c.u32(),
    cookie_award_count_today_offline: c.u32(),
    commendation_progress_today_offline:
      s_persistent_per_commendation_state.array(128),
    purchase_order_count: c.u32(),
    purchase_order: c.u16().array(256),
    day_index: c.u32(),
    user_timezone: c.u32(),
    commendation_definition_checksum: c.u32(),
    purchase_definition_checksum: c.u32(),
    last_modified_at: c.Time64(),
    secure_storage: c.bool(),
    player_name: c.string(16, "latin1"),
    last_hopper_id: c.i16(),
    last_game_end_time: c.i64(),
    last_game_set_entry_index: c.u32(),
    quit_last_game: c.u8(),
    jackpot_cookies_awarded: c.u32(),
    profile_cookies: c.u32(),
    profile_last_modification_time: c.Time64(),
    profile_security_hash: c.u64(),
    profile_owner_xuid: c.i64(),
    profile_cookie_checksum: c.u32(),
  })
) {}
