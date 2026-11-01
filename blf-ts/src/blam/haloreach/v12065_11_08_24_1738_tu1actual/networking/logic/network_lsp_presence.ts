import { c } from "@craftycodie/cstruct";
import {
  e_player_configuration_from_client_flags,
  s_player_appearance,
} from "../../game/players";
import { s_network_session_privacy_mode } from "../online/online_guide_xenon";

export { s_network_session_privacy_mode } from "../online/online_guide_xenon";

/** 9 bytes — `s_static_array<...,16>` stride in Reach TU1. */
export const s_network_lsp_heartbeat_session_player_data = c.struct({
  player_xuid: c.u64(),
  team: c.u8(),
});
export type s_network_lsp_heartbeat_session_player_data = c.infer<
  typeof s_network_lsp_heartbeat_session_player_data
>;

/** 201 bytes — host session block inside the heartbeat payload. */
export const s_network_lsp_heartbeat_session_data = c.struct({
  gui_game_mode: c.u8(),
  session_game_mode: c.u8(),
  hopper_id: c.i16(),
  session_privacy_mode: s_network_session_privacy_mode,
  unknownfa: c.u8(),
  local_player_count: c.u8(),
  player_count: c.u8(),
  incoming_join_failed: c.u8(),
  unknownfe: c.u8(),
  session_players: s_network_lsp_heartbeat_session_player_data.array(16),
  unknown18f: c.u8().array(44),
});
export type s_network_lsp_heartbeat_session_data = c.infer<
  typeof s_network_lsp_heartbeat_session_data
>;

/** 56 bytes — local profile slots in the heartbeat payload. */
export const s_network_lsp_heartbeat_player_data = c.struct({
  player_xuid: c.u64(),
  flags: c.Bitfield(e_player_configuration_from_client_flags, c.u16()),
  bungienet_user_flags: c.i16(),
  player_grade: c.u8(),
  player_sub_grade: c.u8(),
  unknowne: c.i16(),
  player_appearance: s_player_appearance,
});
export type s_network_lsp_heartbeat_player_data = c.infer<
  typeof s_network_lsp_heartbeat_player_data
>;

/** 443-byte LSP presence heartbeat core (`phbt` 5.1 body; prefix of `phbt` 6.0). */
export const s_network_lsp_heartbeat_data = c.struct({
  has_players: c.u8(),
  local_player_count: c.u8(),
  players: s_network_lsp_heartbeat_player_data.array(4),
  machine_id: c.u64(),
  squad_session_id: c.u64(),
  session_data: s_network_lsp_heartbeat_session_data,
});
export type s_network_lsp_heartbeat_data = c.infer<
  typeof s_network_lsp_heartbeat_data
>;

export const e_network_lsp_heartbeat_response_user_file_flags = [
  "player_0_file_requires_download",
  "player_1_file_requires_download",
  "player_2_file_requires_download",
  "player_3_file_requires_download",
] as const;

export const network_lsp_heartbeat_response_user_file_bitfield = c.Bitfield(
  e_network_lsp_heartbeat_response_user_file_flags,
  c.u8()
);

export enum e_network_lsp_heartbeat_response_queued_join_result {
  not_in_progress = 0,
  already_joined = 1,
  success = 2,
  not_enough_slots = 3,
  player_not_found = 4,
}

/** 0x93-byte LSP presence heartbeat response body (`phbr` chunk payload). */
export const s_network_lsp_heartbeat_response_data = c.struct({
  machine_file_requires_download: c.bool(),
  user_file_requires_download:
    network_lsp_heartbeat_response_user_file_bitfield,
  xuid_count: c.i32(),
  xuids: c.u64().array(16),
  session_id: c.u64(),
  ack_number: c.i32(),
  join_result: c.u8(),
});
export type s_network_lsp_heartbeat_response_data = c.infer<
  typeof s_network_lsp_heartbeat_response_data
>;
