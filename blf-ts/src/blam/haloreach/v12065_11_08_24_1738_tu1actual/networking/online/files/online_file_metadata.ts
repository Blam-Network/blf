import { c } from "@craftycodie/cstruct";
import {
  s_content_item_campaign_metadata,
  s_content_item_film_metadata,
  s_content_item_firefight_metadata,
  s_content_item_game_variant_metadata,
  s_content_item_history,
  s_content_item_matchmaking_metadata,
} from "../../../saved_games/saved_game_files";

/** Reach TU1 fileshare file summary (32 bytes). */
export const s_online_file_general_metadata = c.struct({
  id: c.u64(),
  file_type: c.u8(),
  tag_count: c.u8(),
  megalo_category_index: c.u8().padAfter(1),
  size_in_bytes: c.u32(),
  activity: c.u8(),
  game_mode: c.u8(),
  game_engine_type: c.u8().padAfter(1),
  unknown3: c.u8().array(8),
  map_id: c.i32(),
});
export type s_online_file_general_metadata = c.infer<
  typeof s_online_file_general_metadata
>;

/** An empty 16-byte union slot. */
const k_empty_slot = c.struct({ pad: c.pad(16) });

/**
 * Reach TU1 fileshare file metadata. Each union slot holds the member the
 * writer provides; reads decode every member.
 */
export const s_online_file_metadata = c.struct({
  general: s_online_file_general_metadata,
  created: s_content_item_history,
  modified: s_content_item_history,
  name: c.wstring(128),
  description: c.wstring(128),
  game_variant_or_film: c.union({
    game_variant: s_content_item_game_variant_metadata,
    film: s_content_item_film_metadata,
    pad: k_empty_slot,
  }),
  matchmaking: c.union({
    metadata: s_content_item_matchmaking_metadata,
    pad: k_empty_slot,
  }),
  campaign_or_firefight: c.union({
    campaign: s_content_item_campaign_metadata,
    firefight: s_content_item_firefight_metadata,
    pad: k_empty_slot,
  }),
  /** Bytes of screenshot preview after the entries (file details only). */
  screenshot_length: c.u32(),
});
export type s_online_file_metadata = c.infer<typeof s_online_file_metadata>;

/** A fileshare file tag. */
export const s_online_file_tag = c.struct({
  tag: c.string(23),
  unknown: c.u32(),
});
export type s_online_file_tag = c.infer<typeof s_online_file_tag>;
