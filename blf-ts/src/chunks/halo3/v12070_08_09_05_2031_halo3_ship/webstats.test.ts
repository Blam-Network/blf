import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { e_game_engine } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_engine_variant";
import { game_options } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_options";
import { c_map_variant } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/saved_games/scenario_map_variant";
import type { BLFChunk } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";
import { s_blf_chunk_author } from "./s_blf_chunk_author";
import { s_blf_chunk_campaign_metagame_results } from "./s_blf_chunk_campaign_metagame_results";
import { s_blf_chunk_campaign_metagame_results_secondary } from "./s_blf_chunk_campaign_metagame_results_secondary";
import { s_blf_chunk_end_of_file } from "./s_blf_chunk_end_of_file";
import { s_blf_chunk_game_local_players } from "./s_blf_chunk_game_local_players";
import { s_blf_chunk_game_options } from "./s_blf_chunk_game_options";
import { s_blf_chunk_game_variant } from "./s_blf_chunk_game_variant";
import { s_blf_chunk_local_info } from "./s_blf_chunk_local_info";
import {
  e_game_results_data_type,
  s_blf_chunk_multiplayer_events,
} from "./s_blf_chunk_multiplayer_events";
import { s_blf_chunk_multiplayer_game_description } from "./s_blf_chunk_multiplayer_game_description";
import { s_blf_chunk_multiplayer_machines } from "./s_blf_chunk_multiplayer_machines";
import { s_blf_chunk_multiplayer_matchmaking_options } from "./s_blf_chunk_multiplayer_matchmaking_options";
import { s_blf_chunk_multiplayer_players } from "./s_blf_chunk_multiplayer_players";
import { s_blf_chunk_multiplayer_statistics } from "./s_blf_chunk_multiplayer_statistics";
import { s_blf_chunk_multiplayer_teams } from "./s_blf_chunk_multiplayer_teams";
import { s_blf_chunk_start_of_file } from "./s_blf_chunk_start_of_file";
import { read_campaign_pgcr, read_webstats } from "./webstats";

/** Offset of the first chunk with `signature`'s payload in a BLF file. */
function payloadOffset(bytes: Uint8Array, signature: string): number {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  let offset = 0;
  while (offset + 12 <= bytes.length) {
    const sig = String.fromCharCode(...bytes.subarray(offset, offset + 4));
    if (sig === signature) {
      return offset + 12;
    }
    offset += view.getUint32(offset + 4, false);
  }
  throw new Error(`no ${signature} chunk`);
}

function multiChunks() {
  const author = s_blf_chunk_author.forBuild({
    buildString: "12070.08.09.05.2031.halo3_s",
  });

  const mpgd = new s_blf_chunk_multiplayer_game_description();
  mpgd.team_game = true;
  mpgd.game_id = 0x0102_0304_0506_0708n;
  mpgd.map_id = 340;
  mpgd.scenario_path = {
    is_dlc: true,
    path: "maps\\dlc\\sandbox",
  };
  mpgd.map_variant_name = "Foundry";
  mpgd.start_time = new Date(1_220_000_000_000);
  mpgd.finished = true;
  mpgd.finish_time = new Date(1_220_000_600_000);

  const mpmo = new s_blf_chunk_multiplayer_matchmaking_options();
  mpmo.hopper_identifier = 101;
  mpmo.hopper_name = "Team Slayer";

  const mpvr = new s_blf_chunk_game_variant();
  mpvr.game_variant.m_game_engine = e_game_engine.slayer;
  mpvr.game_variant.m_base_variant.m_metadata.name = "Slayer";
  mpvr.game_variant.m_slayer_variant = {
    m_score_to_win: 50,
    m_kill_points: 1,
    m_assist_points: 0,
    m_death_points: 0,
    m_suicide_points: -1,
    m_betrayal_points: -1,
    m_leader_killed_points: 0,
    m_elimination_points: 0,
    m_assassination_points: 0,
    m_headshot_points: 0,
    m_melee_points: 0,
    m_sticky_points: 0,
    m_splatter_points: 0,
    m_killing_spree_points: 0,
    m_leader_traits: new s_blf_chunk_game_variant().game_variant.m_base_variant
      .m_respawn_options.m_respawn_player_traits,
  };

  const mppl = new s_blf_chunk_multiplayer_players();
  mppl.players[0].player_exists = true;
  mppl.players[0].player_identifier = 0xabcdn;
  mppl.players[0].player_configuration_from_client.player_name = "Player One";
  mppl.players[0].player_configuration_from_client.player_xuid =
    0x0009_0000_1234_5678n;
  mppl.players[0].player_configuration_from_host.player_name = "Player One";
  mppl.players[0].score = 50;

  const mptm = new s_blf_chunk_multiplayer_teams();
  mptm.teams[1] = { exists: true, standing: 0, score: 50 };

  const mpma = new s_blf_chunk_multiplayer_machines();
  mpma.machines[0].session_info.exists = true;
  mpma.machines[0].session_info.secure_address.ina = "10.0.0.2";

  const par = new s_blf_chunk_multiplayer_statistics();
  par.mps1.players[0].statistics.kills = 25;
  par.mps1.players[0].damage_statistics.battle_rifle.valid = 1;
  par.mps1.players[0].damage_statistics.battle_rifle.kills = 20;
  par.mps2.players[0][1] = { kills: 3, deaths: 2 };
  par.mps3.teams[1].kills = 25;

  const mpev = new s_blf_chunk_multiplayer_events();
  mpev.events[0] = {
    type_: e_game_results_data_type._kill,
    player_references: [0, 1],
    kill_data: {
      killer_position: { x: 1, y: 2, z: 3 },
      dead_position: { x: 4, y: 5, z: 6 },
      kill_type: 7,
    },
    time: 120,
  };
  mpev.events[1] = {
    type_: e_game_results_data_type._carry,
    player_references: [1, -1],
    carry_data: {
      position: { x: 0.5, y: 0, z: 0 },
      weapon_index: 2,
      carry_type: 1,
    },
    time: 130,
  };

  const clif = new s_blf_chunk_local_info();
  clif.screen_width = 1280;
  clif.screen_height = 720;

  return {
    _blf: s_blf_chunk_start_of_file.create("halo3 multi"),
    athr: author,
    mpgd,
    mpmo,
    mpvr,
    mppl,
    mptm,
    mpma,
    _par: par,
    mpev,
    glcp: new s_blf_chunk_game_local_players(),
    clif,
    _eof: new s_blf_chunk_end_of_file(),
  };
}

function writeMulti() {
  const chunks = multiChunks();
  return write_blffile("big", Object.values(chunks) as BLFChunk[]);
}

describe("Halo 3 webstats", () => {
  it("has blf_lib's sizes", () => {
    expect(c.sizeof(game_options)).toBe(0xf810);
    expect(c.sizeof(c_map_variant)).toBe(0xe090);

    const sizes: [BLFChunk, number][] = [
      [new s_blf_chunk_multiplayer_game_description(), 0x174],
      [new s_blf_chunk_multiplayer_matchmaking_options(), 0x5c],
      [new s_blf_chunk_game_variant(), 0x264],
      [new s_blf_chunk_multiplayer_players(), 0x11e4],
      [new s_blf_chunk_multiplayer_teams(), 0x44],
      [new s_blf_chunk_multiplayer_machines(), 0x584],
      [new s_blf_chunk_multiplayer_statistics(), 0x4c14],
      [new s_blf_chunk_multiplayer_events(), 0x8ca4],
      [new s_blf_chunk_game_local_players(), 0x2a64],
      [new s_blf_chunk_local_info(), 12],
      [new s_blf_chunk_game_options(), 0xf814],
      [new s_blf_chunk_campaign_metagame_results(), 0x243c],
      [new s_blf_chunk_campaign_metagame_results_secondary(), 0x6c],
    ];
    for (const [chunk, size] of sizes) {
      expect(chunk.write_body("big").length).toBe(size);
    }
  });

  it("reads back what it writes", () => {
    const multi = read_webstats(writeMulti());
    expect(multi).not.toBeNull();
    if (!multi) {
      return;
    }

    expect(multi.athr.build_string).toBe("12070.08.09.05.2031.halo3_s");
    expect(multi.mpgd.game_id).toBe(0x0102_0304_0506_0708n);
    expect(multi.mpgd.scenario_path).toEqual({
      is_dlc: true,
      path: "maps\\dlc\\sandbox",
    });
    expect(multi.mpgd.finish_time).toEqual(new Date(1_220_000_600_000));
    expect(multi.mpmo.hopper_name).toBe("Team Slayer");
    expect(multi.mpvr.game_variant.m_game_engine).toBe(e_game_engine.slayer);
    expect(multi.mpvr.game_variant.m_slayer_variant?.m_score_to_win).toBe(50);
    expect("m_ctf_variant" in multi.mpvr.game_variant).toBe(false);
    expect(
      multi.mppl.players[0].player_configuration_from_client.player_name
    ).toBe("Player One");
    expect(multi.mptm.teams[1]).toEqual({
      exists: true,
      standing: 0,
      score: 50,
    });
    expect(multi.mpma.machines[0].session_info.secure_address.ina).toBe(
      "10.0.0.2"
    );
    expect(multi._par.mps1.players[0].statistics.kills).toBe(25);
    expect(
      multi._par.mps1.players[0].damage_statistics.battle_rifle.kills
    ).toBe(20);
    expect(multi._par.mps2.players[0][1]).toEqual({ kills: 3, deaths: 2 });
    expect(multi._par.mps3.teams[1].kills).toBe(25);
    expect(multi.mpev.events.slice(0, 2)).toEqual(
      multiChunks().mpev.events.slice(0, 2)
    );
    expect(multi.clif.screen_width).toBe(1280);
  });

  it("stores statistics shifted left by one bit and big-endian fields", () => {
    const bytes = writeMulti();
    const view = new DataView(bytes.buffer);

    const mpgd = payloadOffset(bytes, "mpgd");
    expect(bytes[mpgd]).toBe(1);
    expect(view.getBigUint64(mpgd + 1, false)).toBe(0x0102_0304_0506_0708n);
    expect(bytes[mpgd + 13] & 0x80).toBe(0x80); // DLC flag

    // mps1 is the first chunk nested in _par, after its 4-byte chunk count.
    const mps1 = payloadOffset(bytes, "_par") + 4 + 12;
    expect(String.fromCharCode(...bytes.subarray(mps1 - 12, mps1 - 8))).toBe(
      "mps1"
    );
    // statistics.kills is the 9th i16 of player 0, after 4 bytes of padding
    expect(view.getInt16(mps1 + 4 + 8 * 2, false)).toBe(50);
  });

  it("reads events at the game's 0x24-byte stride after a _no_data event", () => {
    const bytes = write_blffile(
      "big",
      Object.values({
        ...multiChunks(),
        mpev: new s_blf_chunk_multiplayer_events(),
      }) as BLFChunk[]
    );
    const mpev = payloadOffset(bytes, "mpev");
    const view = new DataView(bytes.buffer);
    // Event 1 in the game's 0x24-byte layout: a kill.
    bytes[mpev + 4 + 0x24] = e_game_results_data_type._kill;
    view.setUint32(mpev + 4 + 0x24 + 0x20, 99, false);

    const events = read_webstats(bytes)?.mpev.events;
    expect(events?.[0].type_).toBe(e_game_results_data_type._no_data);
    expect(events?.[1].type_).toBe(e_game_results_data_type._kill);
    expect(events?.[1].kill_data).toBeDefined();
    expect(events?.[1].time).toBe(99);
    expect(events).toHaveLength(1000);
  });

  it("rejects an unknown event type, truncated files and other files", () => {
    const bytes = writeMulti();
    const bad = bytes.slice();
    bad[payloadOffset(bad, "mpev") + 4] = 7;
    expect(read_webstats(bad)).toBeNull();

    expect(read_webstats(bytes.subarray(0, bytes.length - 1))).toBeNull();
    expect(read_campaign_pgcr(bytes)).toBeNull();
  });
});

describe("Halo 3 campaign carnage report", () => {
  it("reads back what it writes", () => {
    const gmop = new s_blf_chunk_game_options();
    gmop.options.map_id = 3010;
    gmop.options.game_tick_rate = 30;
    gmop.options.campaign_difficulty = 3;
    gmop.options.scenario_path = "scenarios\\solo\\010_jungle\\010_jungle";
    gmop.options.players[0].valid = true;
    gmop.options.players[0].configuration.client.player_name = "Chief";
    gmop.options.map_variant.m_metadata.name = "Sierra 117";
    gmop.options.multiplayer_variant.m_game_engine = e_game_engine.none;

    const cmrp = new s_blf_chunk_campaign_metagame_results();
    cmrp.valid = true;
    cmrp.results.final_total_score = 12_345;
    cmrp.results.stats[0].kill_counts.grunt.infantry = 12;
    cmrp.results.damage_stack[0].metagame_bucket.campaign_metagame_bucket_type = 1;

    const cmrs = new s_blf_chunk_campaign_metagame_results_secondary();
    cmrs.valid = true;
    cmrs.results.total_elapsed_tick_count = 54_000;

    const bytes = write_blffile("big", [
      s_blf_chunk_start_of_file.create("halo3 campaign"),
      s_blf_chunk_author.forBuild(),
      gmop,
      new s_blf_chunk_game_local_players(),
      cmrp,
      cmrs,
      new s_blf_chunk_local_info(),
      new s_blf_chunk_end_of_file(),
    ]);

    const campaign = read_campaign_pgcr(bytes);
    expect(campaign).not.toBeNull();
    expect(campaign?.gmop.options.map_id).toBe(3010);
    expect(campaign?.gmop.options.scenario_path).toBe(
      "scenarios\\solo\\010_jungle\\010_jungle"
    );
    expect(
      campaign?.gmop.options.players[0].configuration.client.player_name
    ).toBe("Chief");
    expect(campaign?.gmop.options.map_variant.m_metadata.name).toBe(
      "Sierra 117"
    );
    expect(campaign?.cmrp.results.final_total_score).toBe(12_345);
    expect(campaign?.cmrp.results.stats[0].kill_counts.grunt.infantry).toBe(12);
    expect(campaign?.cmrs.results.total_elapsed_tick_count).toBe(54_000);
    expect(read_webstats(bytes)).toBeNull();
  });
});
