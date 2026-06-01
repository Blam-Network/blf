import { c } from "@craftycodie/cstruct";
/** Reach TU1 `s_player_appearance` (40 bytes). */
@c.struct()
export class s_player_appearance {
  @c.field("i8")
  voice = 0;

  @c.field("i8")
  primary_color = 0;

  @c.field("i8")
  secondary_color = 0;

  @c.field("i8")
  tertiary_color = 0;

  @c.field("i8", { pad_after: 3 })
  player_model_choice = 0;

  @c.field("i8")
  foreground_emblem = 0;

  @c.field("u8")
  background_emblem = 0;

  @c.field("u8")
  emblem_flags = 0;

  @c.field("i8")
  emblem_primary_color = 0;

  @c.field("i8")
  emblem_secondary_color = 0;

  @c.field("i8", { pad_after: 2 })
  emblem_background_color = 0;

  @c.field("u8", { count: 8 })
  model_permutations = Array.from({ length: 8 }, () => 0);

  @c.field("u8", { count: 4 })
  non_model_customization = Array.from({ length: 4 }, () => 0);

  @c.field(c.WString(5), { pad_after: 2 })
  service_tag = "";
}
