import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";
import {
  Color,
  EliteArmour,
  Grade,
  PlayerModel,
  Rank,
  SpartanBody,
  SpartanHelmet,
  SpartanShoulder,
} from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_service_record";

/**
 * Halo 3: ODST service record (`osri` 2.0, 0x60 bytes).
 * Same as Halo 3's `srid`, plus `extras_portal_debug` and `vidmaster` after the service tag.
 * The enums are Halo 3's (blf_lib redeclares identical ones for ODST).
 */
@blf.chunk("osri", 2.0)
export class s_blf_chunk_odst_service_record extends CStructBLFChunk(
  c.struct({
    /** Wide, 16 characters max. */
    player_name: c.wstring(16),
    appearance_flags: c.u8(),
    primary_color: c.enum(Color, c.u8()),
    secondary_color: c.enum(Color, c.u8()),
    tertiary_color: c.enum(Color, c.u8()),
    is_elite: c.enum(PlayerModel, c.u8()),
    foreground_emblem: c.u8(),
    background_emblem: c.u8(),
    emblem_flags: c.u8(),
    emblem_primary_color: c.enum(Color, c.u8()).padBefore(1),
    emblem_secondary_color: c.enum(Color, c.u8()),
    emblem_background_color: c.enum(Color, c.u8()),
    spartan_helmet: c.enum(SpartanHelmet, c.u8()).padBefore(2),
    spartan_left_shoulder: c.enum(SpartanShoulder, c.u8()),
    spartan_right_shoulder: c.enum(SpartanShoulder, c.u8()),
    spartan_body: c.enum(SpartanBody, c.u8()),
    elite_helmet: c.enum(EliteArmour, c.u8()),
    elite_left_shoulder: c.enum(EliteArmour, c.u8()),
    elite_right_shoulder: c.enum(EliteArmour, c.u8()),
    elite_body: c.enum(EliteArmour, c.u8()),
    service_tag: c.wstring(5),
    extras_portal_debug: c.bool(),
    vidmaster: c.u8(),
    campaign_progress: c.i32().padBefore(2),
    highest_skill: c.i32(),
    total_exp: c.i32(),
    experience_base: c.i32(),
    rank: c.enum(Rank, c.u32()),
    grade: c.enum(Grade, c.u32()),
    games_completed: c.i32(),
  })
) {}
