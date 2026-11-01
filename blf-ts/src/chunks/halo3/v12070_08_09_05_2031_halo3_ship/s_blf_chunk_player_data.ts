import { c } from "@craftycodie/cstruct";
import { type BLFChunk, blf, CStructBLFChunk } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";
import { s_blf_chunk_odst_service_record } from "../../halo3odst/v13895_09_04_27_2201_atlas_release/s_blf_chunk_odst_service_record";
import { s_blf_chunk_player_data as s_blf_chunk_player_data_v1 } from "../v08172_07_03_08_2240_delta/s_blf_chunk_player_data";
import { s_blf_chunk_player_data as s_blf_chunk_player_data_v2 } from "../v10015_07_05_14_2217_delta/s_blf_chunk_player_data";
import { s_blf_chunk_author } from "./s_blf_chunk_author";
import { s_blf_chunk_end_of_file } from "./s_blf_chunk_end_of_file";
import {
  s_blf_chunk_file_transfers,
  type s_files_user_auto_download_queue_item,
} from "./s_blf_chunk_file_transfers";
import { s_blf_chunk_service_record } from "./s_blf_chunk_service_record";
import { s_blf_chunk_start_of_file } from "./s_blf_chunk_start_of_file";

/** Flags for {@link s_blf_chunk_player_data.bungie_user_role}. */
export enum e_bungienet_user_flags {
  none = 0,
  registered = 1 << 0,
  pro_member = 1 << 1,
  /** Unlocks the Bungie chestpiece with flames. */
  staff = 1 << 2,
  /** Unlocks Recon. */
  community = 1 << 3,
  /** Appears unused: unlock code exists but no armour uses it. */
  community2 = 1 << 4,
  /** Appears unused: unlock code exists but no armour uses it. */
  community3 = 1 << 5,
  // Bit 31 (is_blue_disk) is set by the game at runtime for Mythic and locks campaign.
}

/** Halo 3 player data (`fupd` 3.1). */
@blf.chunk("fupd", 3.1)
export class s_blf_chunk_player_data extends CStructBLFChunk(
  c.struct({
    hopper_access: c.u32(),
    /** {@link e_bungienet_user_flags} bits. */
    bungie_user_role: c.u32(),
    highest_skill: c.u32(),
    hopper_directory: c.string(32, "latin1").default("default_hoppers"),
  })
) {}

/**
 * Halo 3 user file: `_blf` "halo3 user", `athr`, `fupd` 3.1, `fupd` 2.0, `fupd` 1.0,
 * then `srid`, `filq` and `osri` when given, then `_eof`.
 * The older `fupd` chunks copy their fields from `fupd`; a missing `fupd` is written
 * with its defaults (`default_hoppers`).
 */
export function build_user_file(
  fupd: c.input<typeof s_blf_chunk_player_data.schema> | undefined,
  srid: c.input<typeof s_blf_chunk_service_record.schema> | undefined,
  filq: { transfers: s_files_user_auto_download_queue_item[] } | undefined,
  osri: c.input<typeof s_blf_chunk_odst_service_record.schema> | undefined
): Uint8Array {
  const player_data = Object.assign(new s_blf_chunk_player_data(), fupd);

  const player_data_v2 = new s_blf_chunk_player_data_v2();
  player_data_v2.hopper_access = player_data.hopper_access;
  player_data_v2.highest_skill = player_data.highest_skill;
  player_data_v2.hopper_directory = player_data.hopper_directory;

  const player_data_v1 = new s_blf_chunk_player_data_v1();
  player_data_v1.hopper_access = player_data.hopper_access;
  player_data_v1.highest_skill = player_data.highest_skill;

  const chunks: BLFChunk[] = [
    s_blf_chunk_start_of_file.create("halo3 user"),
    s_blf_chunk_author.forBuild(),
    player_data,
    player_data_v2,
    player_data_v1,
  ];

  if (srid) {
    chunks.push(Object.assign(new s_blf_chunk_service_record(), srid));
  }

  if (filq) {
    const file_transfers = new s_blf_chunk_file_transfers();
    file_transfers.transfers = filq.transfers;
    chunks.push(file_transfers);
  }

  if (osri) {
    chunks.push(Object.assign(new s_blf_chunk_odst_service_record(), osri));
  }

  chunks.push(new s_blf_chunk_end_of_file());

  return write_blffile("big", chunks);
}
