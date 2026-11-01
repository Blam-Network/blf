import { c } from "@craftycodie/cstruct";

/**
 * Halo 3: ODST `s_content_item_metadata` (0xF8 bytes), as stored in `chdr` 9.3.
 * Same as Halo 3's, with `hopper_id` replaced by the campaign insertion point
 * and survival flag.
 */
export const s_content_item_metadata = c.struct({
  unique_id: c.u64(),
  name: c.wstring(0x10),
  description: c.string(128, "utf8"),
  author: c.string(16, "utf8"),
  file_type: c.i32(),
  author_is_xuid_online: c.bool().padAfter(3),
  author_id: c.u64(),
  size_in_bytes: c.u64(),
  date: c.Time64(),
  length_seconds: c.u32(),
  campaign_id: c.i32(),
  map_id: c.i32(),
  game_engine_type: c.u32(),
  campaign_difficulty: c.i32(),
  campaign_insertion_point: c.i8(),
  campaign_survival_enabled: c.bool(),
  game_id: c.u64().padBefore(2),
});
export type s_content_item_metadata = c.infer<typeof s_content_item_metadata>;
