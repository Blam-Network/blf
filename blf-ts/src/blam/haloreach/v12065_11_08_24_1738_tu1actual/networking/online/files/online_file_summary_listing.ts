import { c } from "@craftycodie/cstruct";

@c.struct()
export class s_online_file_summary_listing_entry {
  @c.field("u64")
  share_id = 0n;

  @c.field("u32")
  screenshots_count = 0;

  @c.field("u32")
  films_count = 0;

  @c.field("u32")
  game_variants_count = 0;

  @c.field("u32")
  map_variants_count = 0;

  @c.field("u32")
  new_items_count = 0;

  @c.field("u32")
  unknown1C = 0;

  @c.field("u32")
  unknown20 = 0;
}
