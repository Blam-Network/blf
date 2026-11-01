import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Player model in a Halo 3 service record. */
export enum PlayerModel {
  Spartan = 0,
  Elite = 1,
}

export enum SpartanHelmet {
  Default = 0,
  Cobra = 1,
  Intruder = 2,
  Ninja = 3,
  Regulator = 4,
  Ryu = 5,
  Marathon = 6,
  Scout = 7,
  Odst = 8,
  MarkV = 9,
  Rogue = 10,
}

export enum SpartanShoulder {
  Default = 0,
  Cobra = 1,
  Intruder = 2,
  Ninja = 3,
  Regulator = 4,
  Ryu = 5,
  Marathon = 6,
  Scout = 7,
}

export enum SpartanBody {
  Default = 0,
  Cobra = 1,
  Intruder = 2,
  Ninja = 3,
  Ryu = 4,
  Regulator = 5,
  Scout = 6,
  Katana = 7,
  Bungie = 8,
}

export enum EliteArmour {
  Default = 0,
  Predator = 1,
  Raptor = 2,
  Blades = 3,
  Scythe = 4,
}

export enum Rank {
  None = 0,
  Recruit = 1,
  Apprentice = 2,
  Private = 3,
  Corporal = 4,
  Sergeant = 5,
  GunnerySergeant = 6,
  Lieutenant = 7,
  Captain = 8,
  Major = 9,
  Commander = 10,
  Colonel = 11,
  Brigadier = 12,
  General = 13,
}

export enum Grade {
  Grade1 = 0,
  Grade2 = 1,
  Grade3 = 2,
  Grade4 = 3,
}

export enum Color {
  Steel = 0,
  Silver = 1,
  White = 2,
  Red = 3,
  Mauve = 4,
  Salmon = 5,
  Orange = 6,
  Coral = 7,
  Peach = 8,
  Gold = 9,
  Yellow = 10,
  Pale = 11,
  Sage = 12,
  Green = 13,
  Olive = 14,
  Teal = 15,
  Aqua = 16,
  Cyan = 17,
  Blue = 18,
  Cobalt = 19,
  Sapphire = 20,
  Violet = 21,
  Orchid = 22,
  Lavender = 23,
  Crimson = 24,
  RubyWine = 25,
  Pink = 26,
  Brown = 27,
  Ran = 28,
  Khaki = 29,
  /** Modders use this; it's whiter than the normal white. */
  INVALID_White = 30,
}

/** Halo 3 service record (`srid` 2.1, 0x5C bytes). */
@blf.chunk("srid", 2.1)
export class s_blf_chunk_service_record extends CStructBLFChunk(
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
    campaign_progress: c.i32(),
    highest_skill: c.i32(),
    total_exp: c.i32(),
    experience_base: c.i32(),
    rank: c.enum(Rank, c.u32()),
    grade: c.enum(Grade, c.u32()),
    games_completed: c.i32(),
  })
) {}
