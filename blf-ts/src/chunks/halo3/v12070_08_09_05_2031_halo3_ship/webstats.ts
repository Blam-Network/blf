import { read_blffile } from "../../../read_blffile";
import { s_blf_chunk_author } from "./s_blf_chunk_author";
import { s_blf_chunk_campaign_metagame_results } from "./s_blf_chunk_campaign_metagame_results";
import { s_blf_chunk_campaign_metagame_results_secondary } from "./s_blf_chunk_campaign_metagame_results_secondary";
import { s_blf_chunk_end_of_file } from "./s_blf_chunk_end_of_file";
import { s_blf_chunk_game_local_players } from "./s_blf_chunk_game_local_players";
import { s_blf_chunk_game_options } from "./s_blf_chunk_game_options";
import { s_blf_chunk_game_variant } from "./s_blf_chunk_game_variant";
import { s_blf_chunk_local_info } from "./s_blf_chunk_local_info";
import { s_blf_chunk_multiplayer_events } from "./s_blf_chunk_multiplayer_events";
import { s_blf_chunk_multiplayer_game_description } from "./s_blf_chunk_multiplayer_game_description";
import { s_blf_chunk_multiplayer_machines } from "./s_blf_chunk_multiplayer_machines";
import { s_blf_chunk_multiplayer_matchmaking_options } from "./s_blf_chunk_multiplayer_matchmaking_options";
import { s_blf_chunk_multiplayer_players } from "./s_blf_chunk_multiplayer_players";
import { s_blf_chunk_multiplayer_statistics } from "./s_blf_chunk_multiplayer_statistics";
import { s_blf_chunk_multiplayer_teams } from "./s_blf_chunk_multiplayer_teams";
import { s_blf_chunk_start_of_file } from "./s_blf_chunk_start_of_file";

/**
 * Multiplayer game results ("webstats") uploaded after a game:
 * `_blf athr mpgd mpmo mpvr mppl mptm mpma _par mpev glcp clif _eof`.
 */
export interface multi {
  _blf: s_blf_chunk_start_of_file;
  _eof: s_blf_chunk_end_of_file;
  _par: s_blf_chunk_multiplayer_statistics;
  athr: s_blf_chunk_author;
  clif: s_blf_chunk_local_info;
  glcp: s_blf_chunk_game_local_players;
  mpev: s_blf_chunk_multiplayer_events;
  mpgd: s_blf_chunk_multiplayer_game_description;
  mpma: s_blf_chunk_multiplayer_machines;
  mpmo: s_blf_chunk_multiplayer_matchmaking_options;
  mppl: s_blf_chunk_multiplayer_players;
  mptm: s_blf_chunk_multiplayer_teams;
  mpvr: s_blf_chunk_game_variant;
}

/**
 * Campaign carnage report uploaded after a mission:
 * `_blf athr gmop glcp cmrp cmrs clif _eof`.
 */
export interface campaign {
  _blf: s_blf_chunk_start_of_file;
  _eof: s_blf_chunk_end_of_file;
  athr: s_blf_chunk_author;
  clif: s_blf_chunk_local_info;
  cmrp: s_blf_chunk_campaign_metagame_results;
  cmrs: s_blf_chunk_campaign_metagame_results_secondary;
  glcp: s_blf_chunk_game_local_players;
  gmop: s_blf_chunk_game_options;
}

/** Read Halo 3 multiplayer game results, or `null` if `buffer` isn't a valid one. */
export function read_webstats(buffer: Uint8Array): multi | null {
  const file: multi = {
    _blf: new s_blf_chunk_start_of_file(),
    athr: new s_blf_chunk_author(),
    mpgd: new s_blf_chunk_multiplayer_game_description(),
    mpmo: new s_blf_chunk_multiplayer_matchmaking_options(),
    mpvr: new s_blf_chunk_game_variant(),
    mppl: new s_blf_chunk_multiplayer_players(),
    mptm: new s_blf_chunk_multiplayer_teams(),
    mpma: new s_blf_chunk_multiplayer_machines(),
    _par: new s_blf_chunk_multiplayer_statistics(),
    mpev: new s_blf_chunk_multiplayer_events(),
    glcp: new s_blf_chunk_game_local_players(),
    clif: new s_blf_chunk_local_info(),
    _eof: new s_blf_chunk_end_of_file(),
  };

  try {
    read_blffile(buffer, "big", [
      file._blf,
      file.athr,
      file.mpgd,
      file.mpmo,
      file.mpvr,
      file.mppl,
      file.mptm,
      file.mpma,
      file._par,
      file.mpev,
      file.glcp,
      file.clif,
      file._eof,
    ]);
  } catch {
    return null;
  }

  return file;
}

/** Read a Halo 3 campaign carnage report, or `null` if `buffer` isn't a valid one. */
export function read_campaign_pgcr(buffer: Uint8Array): campaign | null {
  const file: campaign = {
    _blf: new s_blf_chunk_start_of_file(),
    athr: new s_blf_chunk_author(),
    gmop: new s_blf_chunk_game_options(),
    glcp: new s_blf_chunk_game_local_players(),
    cmrp: new s_blf_chunk_campaign_metagame_results(),
    cmrs: new s_blf_chunk_campaign_metagame_results_secondary(),
    clif: new s_blf_chunk_local_info(),
    _eof: new s_blf_chunk_end_of_file(),
  };

  try {
    read_blffile(buffer, "big", [
      file._blf,
      file.athr,
      file.gmop,
      file.glcp,
      file.cmrp,
      file.cmrs,
      file.clif,
      file._eof,
    ]);
  } catch {
    return null;
  }

  return file;
}
