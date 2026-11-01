import { c } from "@craftycodie/cstruct";

export const s_online_file_summary_listing_entry = c.struct({
  share_id: c.u64(),
  screenshots_count: c.u32(),
  films_count: c.u32(),
  game_variants_count: c.u32(),
  map_variants_count: c.u32(),
  new_items_count: c.u32(),
  last_modified_timestamp: c.u32(),
  unknown20: c.u32(),
});
export type s_online_file_summary_listing_entry = c.infer<
  typeof s_online_file_summary_listing_entry
>;
