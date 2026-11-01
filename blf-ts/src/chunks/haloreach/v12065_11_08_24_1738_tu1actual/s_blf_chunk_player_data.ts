import { c } from "@craftycodie/cstruct";
import { type BLFChunk, blf, CStructBLFChunk } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_start_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import { s_blf_chunk_player_data as s_blf_chunk_player_data_alpha } from "../v08516_10_02_19_1607_omaha_alpha/s_blf_chunk_player_data";
import type { s_blf_chunk_challenge_progress } from "./s_blf_chunk_challenge_progress";
import type { s_blf_chunk_file_transfers } from "./s_blf_chunk_file_transfers";
import type { s_blf_chunk_service_record } from "./s_blf_chunk_service_record";
import type { s_blf_chunk_user_messaging_data } from "./s_blf_chunk_user_messaging_data";

/**
 * Wire values for {@link s_blf_chunk_player_data.bungie_user_role}. The low
 * bits are flags; the `nameplate_*` values are a 4-bit index in bits 12-15,
 * so set at most one of them.
 */
export enum e_bungienet_user_flags {
  none = 0,
  registered = 0b0000_0000_0000_0001,
  pro_member = 0b0000_0000_0000_0010,
  /** Unlocks blue flames. */
  staff = 0b0000_0000_0000_0100,
  community = 0b0000_0000_0000_1000,
  community2 = 0b0000_0000_0001_0000,
  community3 = 0b0000_0000_0010_0000,
  /** Unlocks CLAW; may unlock some activities in pre-releases. */
  debug_enabled = 0b0000_0000_0100_0000,
  unknown7 = 0b0000_0000_1000_0000,
  unknown8 = 0b0000_0001_0000_0000,
  unknown9 = 0b0000_0010_0000_0000,
  unknown10 = 0b0000_0100_0000_0000,
  unknown11 = 0b0000_1000_0000_0000,
  nameplate_seventh_column = 0b0001_0000_0000_0000,
  nameplate_dmr = 0b0010_0000_0000_0000,
  /** Requires `staff` to take effect. */
  nameplate_bungie = 0b0011_0000_0000_0000,
  nameplate_marathon = 0b0100_0000_0000_0000,
  nameplate_halo1 = 0b0101_0000_0000_0000,
  nameplate_halo2 = 0b0110_0000_0000_0000,
  nameplate_halo3 = 0b0111_0000_0000_0000,
  nameplate_odst = 0b1000_0000_0000_0000,
  nameplate_assault_rifle = 0b1001_0000_0000_0000,
  nameplate_mk4_helmet = 0b1010_0000_0000_0000,
  nameplate_halo = 0b1011_0000_0000_0000,
  nameplate_allstar = 0b1100_0000_0000_0000,
}

/** Reach TU1 player data (`fupd` 7.0, 0x45 bytes). */
@blf.chunk("fupd", 7.0)
export class s_blf_chunk_player_data extends CStructBLFChunk(
  c.struct({
    hopper_access: c.u8().padAfter(1),
    /** OR of {@link e_bungienet_user_flags} values. */
    bungie_user_role: c.u16(),
    hopper_directory: c.string(32, "utf8").default("default_hoppers"),
    /** 59 achievements, capacity of 256 bits. */
    unlock_achievements: c.u8().array(32),
    extras_portal_debug: c.bool(),
  })
) {}

/**
 * Reach TU1 user file (`_blf` "omaha user", ...chunks, `_eof`).
 *
 * With `fupd`, it is written twice: as TU1 `fupd` 7.0 and as alpha `fupd` 6.0.
 * Without it, only a default TU1 `fupd` 7.0 is written. Then `chpr`, `umsg`,
 * `srid` and `filq` follow when given.
 */
export function build_user_file(
  fupd?: s_blf_chunk_player_data | null,
  chpr?: s_blf_chunk_challenge_progress | null,
  srid?: s_blf_chunk_service_record | null,
  umsg?: s_blf_chunk_user_messaging_data | null,
  filq?: s_blf_chunk_file_transfers | null
): Uint8Array {
  const chunks: BLFChunk[] = [s_blf_chunk_start_of_file.create("omaha user")];

  if (fupd) {
    const alpha_fupd = new s_blf_chunk_player_data_alpha();
    alpha_fupd.hopper_access = fupd.hopper_access;
    alpha_fupd.bungie_user_role = fupd.bungie_user_role;
    alpha_fupd.hopper_directory = fupd.hopper_directory;
    alpha_fupd.unlock_achievements = [...fupd.unlock_achievements];
    alpha_fupd.extras_portal_debug = fupd.extras_portal_debug;
    chunks.push(fupd, alpha_fupd);
  } else {
    chunks.push(new s_blf_chunk_player_data());
  }

  if (chpr) {
    chunks.push(chpr);
  }
  if (umsg) {
    chunks.push(umsg);
  }
  if (srid) {
    chunks.push(srid);
  }
  if (filq) {
    chunks.push(filq);
  }

  chunks.push(new s_blf_chunk_end_of_file());
  return write_blffile("big", chunks);
}
