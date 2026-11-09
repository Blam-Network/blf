import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { s_blf_chunk_game_local_players } from "./s_blf_chunk_game_local_players";
import { s_blf_chunk_game_results_events } from "./s_blf_chunk_game_results_events";
import { s_blf_chunk_game_results_game_description } from "./s_blf_chunk_game_results_game_description";
import { s_blf_chunk_game_results_machines } from "./s_blf_chunk_game_results_machines";
import { s_blf_chunk_game_results_matchmaking_options } from "./s_blf_chunk_game_results_matchmaking_options";
import { s_blf_chunk_game_results_player_statistics } from "./s_blf_chunk_game_results_player_statistics";
import { s_blf_chunk_game_results_player_vs_ai_statistics } from "./s_blf_chunk_game_results_player_vs_ai_statistics";
import { s_blf_chunk_game_results_player_vs_player_statistics } from "./s_blf_chunk_game_results_player_vs_player_statistics";
import { s_blf_chunk_game_results_statistics } from "./s_blf_chunk_game_results_statistics";
import { s_blf_chunk_game_results_team_statistics } from "./s_blf_chunk_game_results_team_statistics";
import { s_blf_chunk_game_results_teams } from "./s_blf_chunk_game_results_teams";
import { s_blf_chunk_local_info } from "./s_blf_chunk_local_info";
import { s_blf_chunk_multiplayer_players } from "./s_blf_chunk_multiplayer_players";
import { s_blf_chunk_network_banhammer_cheating_report } from "./s_blf_chunk_network_banhammer_cheating_report";

const struct_chunks = [
  [s_blf_chunk_game_results_game_description, 0xfa68],
  [s_blf_chunk_game_results_matchmaking_options, 0xb0],
  [s_blf_chunk_multiplayer_players, 0x1410],
  [s_blf_chunk_game_results_teams, 0x50],
  [s_blf_chunk_game_results_machines, 0x1910],
  [s_blf_chunk_game_results_player_statistics, 0x6b90],
  [s_blf_chunk_game_results_player_vs_player_statistics, 0x410],
  [s_blf_chunk_game_results_player_vs_ai_statistics, 0x1190],
  [s_blf_chunk_game_results_team_statistics, 0x270],
  [s_blf_chunk_game_results_events, 0x1d4d0],
  [s_blf_chunk_game_local_players, 0x2c70],
  [s_blf_chunk_local_info, 0x20],
  [s_blf_chunk_network_banhammer_cheating_report, 0x120],
] as const;

describe("Reach TU1 webstats game result chunks", () => {
  it("match the s_blffile_webstats_game_result chunk sizes", () => {
    for (const [chunk, chunk_length] of struct_chunks) {
      expect(c.sizeof(chunk.schema) + 12).toBe(chunk_length);
      expect(new chunk().write("big").length).toBe(chunk_length);
    }
    expect(new s_blf_chunk_game_results_statistics().write("big").length).toBe(
      0x83b0
    );
  });
});
