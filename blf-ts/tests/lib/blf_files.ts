import type { BLFChunk } from "../../src/blf_chunk";
import { find_chunk } from "../../src/blf_chunk";
import * as halo3 from "../../src/chunks/halo3/v12070_08_09_05_2031_halo3_ship";
import { s_blf_chunk_content_header as odst_s_blf_chunk_content_header } from "../../src/chunks/halo3odst/v13895_09_04_27_2201_atlas_release/s_blf_chunk_content_header";
import { s_blf_chunk_content_header as reach_s_blf_chunk_content_header } from "../../src/chunks/haloreach/v12065_11_08_24_1738_tu1actual/s_blf_chunk_content_header";
import { s_blf_chunk_screenshot_camera as reach_s_blf_chunk_screenshot_camera } from "../../src/chunks/haloreach/v12065_11_08_24_1738_tu1actual/s_blf_chunk_screenshot_camera";
import { s_blf_chunk_screenshot_data as reach_s_blf_chunk_screenshot_data } from "../../src/chunks/haloreach/v12065_11_08_24_1738_tu1actual/s_blf_chunk_screenshot_data";
import { s_blf_chunk_server_signature } from "../../src/chunks/haloreach/v12065_11_08_24_1738_tu1actual/s_blf_chunk_server_signature";
import { read_blffile } from "../../src/read_blffile";

// Whole files the chunk tests read back: exactly these chunks, in order. Null if it isn't one.
function read_file<T extends Record<string, BLFChunk>>(
  buffer: Uint8Array,
  chunks: T
): T | null {
  try {
    read_blffile(buffer, "big", Object.values(chunks));
  } catch {
    return null;
  }
  return chunks;
}

// The chunk anywhere in the file, or null.
export function find_file_chunk<T extends BLFChunk>(
  buffer: Uint8Array,
  chunk: T
): T | null {
  try {
    return find_chunk(buffer, chunk, "big") ? chunk : null;
  } catch {
    return null;
  }
}

export function read_halo3_blind_screenshot(buffer: Uint8Array) {
  return read_file(buffer, {
    _blf: new halo3.s_blf_chunk_start_of_file(),
    chdr: new halo3.s_blf_chunk_content_header(),
    scnc: new halo3.s_blf_chunk_screenshot_camera(),
    scnd: new halo3.s_blf_chunk_screenshot_data(),
    _eof: new halo3.s_blf_chunk_end_of_file(),
  });
}

export function read_halo3_11637_blind_screenshot(buffer: Uint8Array) {
  return read_file(buffer, {
    _blf: new halo3.s_blf_chunk_start_of_file(),
    chdr: new halo3.s_blf_chunk_content_header(),
    scnd: new halo3.s_blf_chunk_screenshot_data(),
    _eof: new halo3.s_blf_chunk_end_of_file(),
  });
}

export function read_halo3_fileshare_screenshot(buffer: Uint8Array) {
  return read_file(buffer, {
    _blf: new halo3.s_blf_chunk_start_of_file(),
    chdr: new halo3.s_blf_chunk_content_header(),
    scnc: new halo3.s_blf_chunk_screenshot_camera(),
    scnd: new halo3.s_blf_chunk_compressed_data(
      halo3.s_blf_chunk_screenshot_data
    ),
    _eof: new halo3.s_blf_chunk_end_of_file(),
  });
}

export function read_odst_blind_screenshot(buffer: Uint8Array) {
  return read_file(buffer, {
    _blf: new halo3.s_blf_chunk_start_of_file(),
    chdr: new odst_s_blf_chunk_content_header(),
    scnc: new halo3.s_blf_chunk_screenshot_camera(),
    scnd: new halo3.s_blf_chunk_screenshot_data(),
    _eof: new halo3.s_blf_chunk_end_of_file(),
  });
}

export function read_reach_blind_screenshot(buffer: Uint8Array) {
  return read_file(buffer, {
    _blf: new halo3.s_blf_chunk_start_of_file(),
    chdr: new reach_s_blf_chunk_content_header(),
    scnc: new reach_s_blf_chunk_screenshot_camera(),
    scnd: new reach_s_blf_chunk_screenshot_data(),
    ssig: new s_blf_chunk_server_signature(),
    _eof: new halo3.s_blf_chunk_end_of_file(),
  });
}

export function read_reach_09730_blind_screenshot(buffer: Uint8Array) {
  return read_file(buffer, {
    _blf: new halo3.s_blf_chunk_start_of_file(),
    chdr: new reach_s_blf_chunk_content_header(),
    _cmp: new halo3.s_blf_chunk_compressed_data(
      reach_s_blf_chunk_screenshot_data
    ),
    _eof: new halo3.s_blf_chunk_end_of_file(),
  });
}

export function read_reach_09449_blind_screenshot(buffer: Uint8Array) {
  return read_file(buffer, {
    _blf: new halo3.s_blf_chunk_start_of_file(),
    chdr: new reach_s_blf_chunk_content_header(),
    _cmp: new halo3.s_blf_chunk_compressed_data(
      halo3.s_blf_chunk_screenshot_data
    ),
    _eof: new halo3.s_blf_chunk_end_of_file(),
  });
}

export function read_halo3_webstats(buffer: Uint8Array) {
  return read_file(buffer, {
    _blf: new halo3.s_blf_chunk_start_of_file(),
    athr: new halo3.s_blf_chunk_author(),
    mpgd: new halo3.s_blf_chunk_multiplayer_game_description(),
    mpmo: new halo3.s_blf_chunk_multiplayer_matchmaking_options(),
    mpvr: new halo3.s_blf_chunk_game_variant(),
    mppl: new halo3.s_blf_chunk_multiplayer_players(),
    mptm: new halo3.s_blf_chunk_multiplayer_teams(),
    mpma: new halo3.s_blf_chunk_multiplayer_machines(),
    _par: new halo3.s_blf_chunk_multiplayer_statistics(),
    mpev: new halo3.s_blf_chunk_multiplayer_events(),
    glcp: new halo3.s_blf_chunk_game_local_players(),
    clif: new halo3.s_blf_chunk_local_info(),
    _eof: new halo3.s_blf_chunk_end_of_file(),
  });
}

export function read_halo3_campaign_pgcr(buffer: Uint8Array) {
  return read_file(buffer, {
    _blf: new halo3.s_blf_chunk_start_of_file(),
    athr: new halo3.s_blf_chunk_author(),
    gmop: new halo3.s_blf_chunk_game_options(),
    glcp: new halo3.s_blf_chunk_game_local_players(),
    cmrp: new halo3.s_blf_chunk_campaign_metagame_results(),
    cmrs: new halo3.s_blf_chunk_campaign_metagame_results_secondary(),
    clif: new halo3.s_blf_chunk_local_info(),
    _eof: new halo3.s_blf_chunk_end_of_file(),
  });
}
