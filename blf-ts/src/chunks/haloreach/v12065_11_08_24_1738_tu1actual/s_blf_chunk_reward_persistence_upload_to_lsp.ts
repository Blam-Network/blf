import { c } from "@craftycodie/cstruct";
import {
  default_purchase_state,
  e_purchase_state_flags,
  s_persistent_per_commendation_state,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/game/player_rewards";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Reach TU1 rewards upload to LSP (`rpul` 3.1, 0x778-byte body). */
@blf.chunk("rpul", 3.1)
@c.struct()
export class s_blf_chunk_reward_persistence_upload_to_lsp extends CStructBLFChunk {
  @c.field("u32")
  alltime_cookie_count = 0;

  @c.field("u32")
  alltime_cookie_award_count = 0;

  @c.field(s_persistent_per_commendation_state, { count: 128 })
  alltime_commendation_progress = Array.from(
    { length: 128 },
    () => new s_persistent_per_commendation_state()
  );

  @c.field(c.bitfield("u8", e_purchase_state_flags), { count: 256 })
  alltime_purchased_items = Array.from({ length: 256 }, () =>
    default_purchase_state()
  );

  @c.field("u32")
  cookies_earned_today_online = 0;

  @c.field("u32")
  cookie_award_count_today_online = 0;

  @c.field(s_persistent_per_commendation_state, { count: 128 })
  commendation_progress_today_online = Array.from(
    { length: 128 },
    () => new s_persistent_per_commendation_state()
  );

  @c.field(c.bitfield("u8", e_purchase_state_flags), { count: 256 })
  purchased_items_today_online = Array.from({ length: 256 }, () =>
    default_purchase_state()
  );

  @c.field("u32")
  cookies_earned_today_offline = 0;

  @c.field("u32")
  cookie_award_count_today_offline = 0;

  @c.field(s_persistent_per_commendation_state, { count: 128 })
  commendation_progress_today_offline = Array.from(
    { length: 128 },
    () => new s_persistent_per_commendation_state()
  );

  @c.field("u32")
  armour_purchases_count = 0;

  @c.field("u16", { count: 256 })
  armour_purchase_stack = Array.from({ length: 256 }, () => 0);

  @c.field("u32")
  day_index = 0;

  @c.field("u32")
  user_timezone = 0;

  @c.field("u32")
  purchase_definition_checksum = 0;

  @c.field("u32")
  unknown_728 = 0;

  @c.field(c.Time64())
  last_modified_at = new Date(0);

  @c.field("u8")
  unknown_flag = 0;

  @c.field(c.String(16))
  player_name = "";

  @c.field("i16")
  last_hopper_id = 0;

  @c.field("i64")
  last_game_time = 0n;

  @c.field("u32")
  unknown_last_game_results = 0;

  @c.field("u8")
  quit_last_game = 0;

  @c.field("u32")
  unknown754 = 0;

  @c.field("u32")
  profile_unknown758 = 0;

  @c.field(c.Time64())
  profile_time_75c = new Date(0);

  @c.field("i16")
  profile_unknown764 = 0;

  @c.field("i16")
  profile_unknown766 = 0;

  @c.field("u32")
  profile_unknown768 = 0;

  @c.field("i64")
  player_xuid = 0n;

  @c.field("u32")
  profile_unknown774 = 0;
}
