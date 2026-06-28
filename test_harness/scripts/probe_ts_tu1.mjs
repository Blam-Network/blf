import fs from "node:fs";
import { BLFFile } from "../../blf-ts/dist/index.js";
import { s_blf_chunk_game_variant } from "../../blf-ts/dist/chunks/haloreach_mcc/v_untracked_25_08_16_1352/s_blf_chunk_game_variant.js";
import { bitfieldToRaw } from "../../blf-ts/dist/bitstream/index.js";

const path =
  process.env.REACH_HOPPER_BIN ??
  String.raw`C:\Program Files (x86)\Steam\steamapps\common\Halo The Master Chief Collection\haloreach\hopper_game_variants\assault_one_bomb_anniversary_054.bin`;

const bytes = fs.readFileSync(path);
const blf = BLFFile.read(bytes);
const mpvr = blf.chunks.find((c) => c instanceof s_blf_chunk_game_variant);
if (!mpvr) throw new Error("mpvr not found");

const custom = mpvr.game_variant.m_custom_variant;
if (!custom) throw new Error("no custom variant");

const tu1 = custom.m_tu1_settings;
console.log("encoding_version", custom.m_encoding_version);
console.log("tu1 flags raw", bitfieldToRaw(tu1.m_flags, [
  "always_spillover_damage",
  "armor_lock_stickies_remain",
  "attached_damage_bypass_shields",
  "active_camo_override_energy_curve",
  "sword_gun_clang_kills",
  "magnum_is_automatic",
  "unknown",
]));
console.log("precision_bloom", tu1.m_precision_bloom);
console.log("camo min/max", tu1.m_active_camo_energy_curve_min, tu1.m_active_camo_energy_curve_max);
console.log("magnum damage", tu1.m_magnum_damage);
