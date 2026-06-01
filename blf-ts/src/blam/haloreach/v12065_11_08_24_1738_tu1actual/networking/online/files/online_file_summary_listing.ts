import { c } from "@craftycodie/cstruct";

@c.struct()
export class s_online_file_summary_listing_entry {
  @c.field("u64")
  share_id!: bigint;

  @c.field("u32")
  screenshots_count!: number;

  @c.field("u32")
  films_count!: number;

  @c.field("u32")
  game_variants_count!: number;

  @c.field("u32")
  map_variants_count!: number;

  @c.field("u32")
  new_items_count!: number;

  @c.field("u32")
  unknown1C!: number;

  @c.field("u32")
  unknown20!: number;
}
