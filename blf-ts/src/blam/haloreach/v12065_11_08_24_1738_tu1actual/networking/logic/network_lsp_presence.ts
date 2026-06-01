import { c } from "@craftycodie/cstruct";
import { s_player_appearance } from "../../game/players";
import { s_network_session_privacy_mode } from "../online/online_guide_xenon";

export { s_network_session_privacy_mode } from "../online/online_guide_xenon";

/** 9 bytes — `s_static_array<...,16>` stride in Reach TU1. */
@c.struct()
export class s_network_lsp_heartbeat_session_player_data {
  @c.field("u8", { count: 8 })
  unknown0 = Array.from({ length: 8 }, () => 0);

  @c.field("u8")
  team = 0;
}

/** 201 bytes — host session block inside the heartbeat payload. */
@c.struct()
export class s_network_lsp_heartbeat_session_data {
  @c.field("u8")
  gui_game_mode = 0;

  @c.field("u8")
  session_game_mode = 0;

  @c.field("i16")
  hopper_id = 0;

  @c.field(s_network_session_privacy_mode)
  session_piracy_mode = new s_network_session_privacy_mode();

  @c.field("u8")
  unknownfa = 0;

  @c.field("u8")
  local_player_count = 0;

  @c.field("u8")
  player_count = 0;

  @c.field("u8")
  incoming_join_failed = 0;

  @c.field("u8")
  unknownfe = 0;

  @c.field(s_network_lsp_heartbeat_session_player_data, { count: 16 })
  session_players = Array.from(
    { length: 16 },
    () => new s_network_lsp_heartbeat_session_player_data()
  );

  @c.field("u8", { count: 44 })
  unknown18f = Array.from({ length: 44 }, () => 0);
}

/** 56 bytes — local profile slots in the heartbeat payload. */
@c.struct()
export class s_network_lsp_heartbeat_player_data {
  @c.field("u64")
  player_xuid = 0n;

  @c.field("i16")
  flags = 0;

  @c.field("i16")
  bungienet_user_flags = 0;

  @c.field("u8")
  player_grade = 0;

  @c.field("u8")
  player_sub_grade = 0;

  @c.field("i16")
  unknowne = 0;

  @c.field(s_player_appearance)
  player_appearance = new s_player_appearance();
}

/** 443-byte LSP presence heartbeat body (`phbt` chunk payload). */
@c.struct()
export class s_network_lsp_heartbeat_data {
  @c.field("u8")
  has_players = 0;

  @c.field("u8")
  local_player_count = 0;

  @c.field(s_network_lsp_heartbeat_player_data, { count: 4 })
  players = Array.from(
    { length: 4 },
    () => new s_network_lsp_heartbeat_player_data()
  );

  @c.field("u64")
  machine_id = 0n;

  @c.field("u8", { count: 8 })
  unknowneeA = Array.from({ length: 8 }, () => 0);

  @c.field(s_network_lsp_heartbeat_session_data)
  session_data = new s_network_lsp_heartbeat_session_data();
}

/** 0x93-byte LSP presence heartbeat response body (`phbr` chunk payload). */
@c.struct()
export class s_network_lsp_heartbeat_response_data {
  @c.field("i8")
  machine_file_requires_download = 0;

  @c.field("i8")
  flags = 0;

  @c.field("i32")
  xuid_count = 0;

  @c.field("u64", { count: 16 })
  xuids = Array.from({ length: 16 }, () => 0n);

  @c.field("u64")
  session_id = 0n;

  @c.field("i32")
  ack_number = 0;

  @c.field("i8")
  join_result = 0;
}
