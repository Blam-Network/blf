import { c } from "@craftycodie/cstruct";
import { inflate } from "pako";
import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../bitstream";
import { zlib_compress } from "../../../blam/common/memory/data_compress";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";

import { BlfError } from "../../../error";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_start_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";

/** Reach custom hopper category entry in `mhcf`. */
export const s_game_hopper_custom_category = c.struct({
  category_identifier: c.u16(),
  category_name: c.string(32, "latin1"),
  category_image_index: c.u16(),
  category_image_description: c.string(32, "latin1"),
});
export type s_game_hopper_custom_category = c.infer<
  typeof s_game_hopper_custom_category
>;

/** Latency desirability row inside {@link s_hopper_query_configuration}. */
export const s_hopper_query_latency_desirability_configuration = c.struct({
  weight: c.f32(),
  minimum_latency_ms: c.u32(),
  maximum_latency_ms: c.u32(),
});
export type s_hopper_query_latency_desirability_configuration = c.infer<
  typeof s_hopper_query_latency_desirability_configuration
>;

export enum e_hopper_query_weight {
  average_party_mu = 0,
  min_party_skill = 1,
  max_party_skill = 2,
  average_party_skill = 3,
  social_motivation = 4,
  social_tone = 5,
  social_chattiness = 6,
  social_teamwork = 7,
  locality_x = 8,
  locality_y = 9,
  locality_z = 10,
  language = 11,
  average_grade = 12,
  dlc_1 = 13,
  dlc_2 = 14,
  dlc_3 = 15,
  has_guests = 16,
}

/** Matchmaking query slot on a hopper configuration. */
export const s_hopper_query_configuration = c.struct({
  skill_level_match_delta: c.u32(),
  average_skill_level_match_enabled: c.bool().padAfter(3),
  average_skill_level_match_delta: c.u32(),
  average_mu_match_enabled: c.bool().padAfter(3),
  average_mu_match_delta: c.f32(),
  locality_match_enabled: c.bool().padAfter(3),
  locality_match_delta_x: c.f32(),
  locality_match_delta_y: c.f32(),
  locality_match_delta_z: c.f32(),
  language_preference_required: c.bool(),
  language_preference_allowed: c.bool().padAfter(2),
  latency_threshold_ms: c.u32(),
  latency_threshold_increment_ms: c.u32(),
  latency_threshold_increment_queries: c.u32(),
  remove_latency_threshold_after_final_increment: c.bool().padAfter(3),
  latency_desirability_configurations:
    s_hopper_query_latency_desirability_configuration.array(2),
  weights: c.f32().array(17),
});
export type s_hopper_query_configuration = c.infer<
  typeof s_hopper_query_configuration
>;

/** Per-team limits inside {@link c_hopper_configuration}. */
export const s_hopper_configuration_per_team_data = c.struct({
  minimum_team_size: c.u32(),
  maximum_team_size: c.u32(),
  team_model_override: c.u32(),
  team_allegiance: c.u32(),
});
export type s_hopper_configuration_per_team_data = c.infer<
  typeof s_hopper_configuration_per_team_data
>;

/** Jackpot reward row on a hopper configuration. */
export const s_hopper_jackpot_configuration = c.struct({
  chance: c.u32(),
  minimum_cookies: c.u32(),
  maximum_cookies: c.u32(),
});
export type s_hopper_jackpot_configuration = c.infer<
  typeof s_hopper_jackpot_configuration
>;

/** Map voting settings on a hopper configuration. */
export const s_hopper_voting_configuration = c.struct({
  voting_categories: c.u32(),
  number_of_votes_per_peer: c.u32(),
  voting_rounds: c.u32(),
  veto_rounds: c.u32(),
  maximum_load_failures: c.u32(),
  voting_time_seconds: c.u32(),
  flags: c.u8().padAfter(3),
});
export type s_hopper_voting_configuration = c.infer<
  typeof s_hopper_voting_configuration
>;

/** Single hopper entry in the Reach matchmaking configuration table (`mhcf`). */
export const c_hopper_configuration = c.struct({
  hopper_name: c.string(32, "latin1"),
  game_set_hash: c.u8().array(20),
  identifier: c.u16(),
  category_identifier: c.u16(),
  category_index: c.u8(),
  player_investment_category: c.u8().padAfter(2),
  image_index: c.u32(),
  xlast_index: c.u32(),
  equivalency_id: c.u8().padAfter(3),
  start_time: c.u64(),
  end_time: c.u64(),
  minimum_games_won: c.u32(),
  maximum_games_won: c.u32(),
  minimum_games_played: c.u32(),
  maximum_games_played: c.u32(),
  minimum_grade: c.u32(),
  maximum_grade: c.u32(),
  min_party_size: c.u32(),
  max_party_size: c.u32(),
  min_local_players: c.u32(),
  max_local_players: c.u32(),
  hopper_access_bit: c.i32(),
  account_type_access: c.u32(),
  require_all_party_members_meet_games_played_requirements: c.bool(),
  require_all_party_members_meet_games_won_requirements: c.bool(),
  require_all_party_members_meet_grade_requirements: c.bool(),
  require_all_party_members_meet_access_requirements: c.bool(),
  require_all_party_members_meet_live_account_access_requirements: c.bool(),
  hide_hopper_from_games_played_restricted_players: c.bool(),
  hide_hopper_from_games_won_restricted_players: c.bool(),
  hide_hopper_from_grade_restricted_players: c.bool(),
  hide_hopper_from_access_restricted_players: c.bool(),
  hide_hopper_from_live_account_access_restricted_players: c.bool(),
  hide_hopper_due_to_time_restriction: c.bool(),
  requires_hard_drive: c.bool(),
  requires_local_party: c.bool().padAfter(3),
  voting_configuration: s_hopper_voting_configuration,
  is_ranked: c.u8(),
  is_arbitrated: c.u8(),
  are_guests_allowed: c.u8(),
  are_opponents_visible: c.u8(),
  uses_arena_lsp_stats: c.u8(),
  arena_active_in_even_seasons: c.bool().padAfter(2),
  arena_first_season: c.u32(),
  arena_last_season: c.u32(),
  arena_seasons_disabled: c.bool(),
  uses_high_score_leaderboard: c.u8().padAfter(2),
  posse_formation: c.u32(),
  post_match_countdown_time_seconds: c.u32(),
  require_hosts_on_multiple_teams: c.bool().padAfter(3),
  repeated_opponents_to_consider_for_penalty: c.u32(),
  repeated_opponents_skill_throttle_start: c.u32(),
  repeated_opponents_skill_throttle_stop: c.u32(),
  is_team_matching_enabled: c.bool(),
  unknowne1: c.u8().padAfter(2),
  gather_start_threshold_seconds: c.u32(),
  get_gather_start_game_early_seconds: c.u32(),
  get_gather_give_up_seconds: c.u32(),
  chance_of_gathering: c.u8().array(16),
  use_games_played_for_skill: c.bool().padAfter(3),
  games_played_skill_minimum: c.u32(),
  games_played_skill_maximum: c.u32(),
  uses_ffa_scoring_for_leaderboard_writes: c.bool(),
  should_modify_skill_update_weight_with_game_quality: c.bool().padAfter(2),
  trueskill_sigma_multiplier: c.f32(),
  trueskill_beta_performance_variation: c.f32(),
  trueskill_tau_dynamics_factor: c.f32(),
  trueskill_draw_probability: c.u32(),
  pre_match_voice_configuration: c.u32(),
  in_match_voice_configuration: c.u32(),
  post_match_voice_configuration: c.u32(),
  restrict_open_channel: c.u8().padAfter(3),
  search_stage_best_duration_seconds: c.u32(),
  query_configurations: s_hopper_query_configuration.array(4),
  game_type: c.u8(),
  is_ffa: c.bool().padAfter(2),
  minimum_player_count: c.u32(),
  maximum_player_count: c.u32(),
  ffa_model_override: c.u32(),
  minimum_team_count: c.u32(),
  maximum_team_count: c.u32(),
  per_team_data: s_hopper_configuration_per_team_data.array(8),
  maximum_team_imbalance: c.u32(),
  big_squad_size_threshold: c.u32(),
  unknown424: c.u32(),
  unknown428: c.u32(),
  undersized_party_split_permissions: c.u32(),
  jackpot_minimum_time_seconds: c.u32(),
  jackpot_configurations: s_hopper_jackpot_configuration.array(3),
});
export type c_hopper_configuration = c.infer<typeof c_hopper_configuration>;

export const k_hopper_maximum_category_count = 16;
export const k_hopper_maximum_hopper_count = 32;

const MHCF_PACKED_CAPACITY = 0x8f48;
const MHCF_INNER_ENDIAN = "big" as const;
const BE_BITSTREAM = e_bitstream_byte_order._bitstream_byte_order_big_endian;

function pad_fixed_array<T>(
  items: T[],
  capacity: number,
  create_default: () => T
): T[] {
  if (items.length > capacity) {
    throw new BlfError(
      `Expected at most ${capacity} elements but got ${items.length}`
    );
  }
  const out = items.slice();
  while (out.length < capacity) {
    out.push(create_default());
  }
  return out;
}

function encode_hopper_table_inner(
  hopper_categories: s_game_hopper_custom_category[],
  hopper_configurations: c_hopper_configuration[]
): Uint8Array {
  const category_size = c.sizeof(s_game_hopper_custom_category);
  const configuration_size = c.sizeof(c_hopper_configuration);
  const encoded = new Uint8Array(
    8 +
      k_hopper_maximum_category_count * category_size +
      k_hopper_maximum_hopper_count * configuration_size
  );
  const view = new DataView(
    encoded.buffer,
    encoded.byteOffset,
    encoded.byteLength
  );
  view.setUint32(0, hopper_configurations.length, false);
  view.setUint32(4, hopper_categories.length, false);

  let offset = 8;
  for (const category of pad_fixed_array(
    hopper_categories,
    k_hopper_maximum_category_count,
    () => s_game_hopper_custom_category.zero()
  )) {
    encoded.set(
      s_game_hopper_custom_category.write(category, MHCF_INNER_ENDIAN),
      offset
    );
    offset += category_size;
  }

  for (const configuration of pad_fixed_array(
    hopper_configurations,
    k_hopper_maximum_hopper_count,
    () => c_hopper_configuration.zero()
  )) {
    encoded.set(
      c_hopper_configuration.write(configuration, MHCF_INNER_ENDIAN),
      offset
    );
    offset += configuration_size;
  }

  return encoded;
}

function decode_hopper_table_inner(data: Uint8Array): {
  hopper_categories: s_game_hopper_custom_category[];
  hopper_configurations: c_hopper_configuration[];
} {
  if (data.length < 8) {
    throw new BlfError(
      `mhcf decompressed table too short: need at least 8 bytes, got ${data.length}`
    );
  }

  const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
  const hopper_configuration_count = view.getUint32(0, false);
  const hopper_category_count = view.getUint32(4, false);

  const category_size = c.sizeof(s_game_hopper_custom_category);
  const configuration_size = c.sizeof(c_hopper_configuration);
  const expected_size =
    8 +
    k_hopper_maximum_category_count * category_size +
    hopper_configuration_count * configuration_size;

  if (data.length < expected_size) {
    throw new BlfError(
      `mhcf decompressed table too short for ${hopper_configuration_count} configurations: need ${expected_size} bytes, got ${data.length}`
    );
  }

  let offset = 8;
  const hopper_categories: s_game_hopper_custom_category[] = [];
  for (let i = 0; i < k_hopper_maximum_category_count; i++) {
    const category = c.read(
      s_game_hopper_custom_category,
      data.subarray(offset, offset + category_size),
      MHCF_INNER_ENDIAN
    );
    offset += category_size;
    if (i < hopper_category_count) {
      hopper_categories.push(category);
    }
  }

  const hopper_configurations: c_hopper_configuration[] = [];
  for (let i = 0; i < hopper_configuration_count; i++) {
    hopper_configurations.push(
      c.read(
        c_hopper_configuration,
        data.subarray(offset, offset + configuration_size),
        MHCF_INNER_ENDIAN
      )
    );
    offset += configuration_size;
  }

  return { hopper_categories, hopper_configurations };
}

/**
 * Reach matchmaking hopper configuration table (`mhcf` 27.1).
 *
 * Payload is bitstream-packed and zlib-compressed; inner records are big-endian.
 */
@blf.chunk("mhcf", 27.1)
export class s_blf_chunk_hopper_configuration_table extends BLFChunkBase {
  hopper_categories: s_game_hopper_custom_category[] = [];
  hopper_configurations: c_hopper_configuration[] = [];

  read_body(_payload: Uint8Array, _endian: c.Endian): void {
    const bitstream = c_bitstream_reader.new(_payload, BE_BITSTREAM);
    bitstream.begin_reading();

    const compressed_length =
      bitstream.read_integer("compressed_length", 14) - 4;
    const decompressed_length = bitstream.read_integer(
      "decompressed_length",
      32
    );
    const compressed = bitstream.read_raw_data(compressed_length * 8);
    bitstream.finish_reading();

    const decompressed = inflate(compressed);
    if (decompressed.length !== decompressed_length) {
      throw new BlfError(
        `mhcf zlib decompressed ${decompressed.length} bytes, expected ${decompressed_length}`
      );
    }

    const table = decode_hopper_table_inner(decompressed);
    this.hopper_categories = table.hopper_categories;
    this.hopper_configurations = table.hopper_configurations;
  }

  write_body(_endian: c.Endian): Uint8Array {
    const encoded = encode_hopper_table_inner(
      this.hopper_categories,
      this.hopper_configurations
    );
    const compressed = zlib_compress(encoded);
    const compressed_length = compressed.length;

    const bitstream = c_bitstream_writer.new(
      MHCF_PACKED_CAPACITY,
      BE_BITSTREAM
    );
    bitstream.begin_writing();
    bitstream.write_integer(compressed_length + 4, 14);
    bitstream.write_integer(encoded.length, 32);
    bitstream.write_raw_data(compressed, compressed_length * 8);
    bitstream.finish_writing();

    return bitstream.get_data();
  }
}

/** BLF with `_blf`, `mhcf`, and `_eof` for Reach title-storage hopper tables. */
export function build_hopper_configuration_file(
  mhcf: s_blf_chunk_hopper_configuration_table
): Uint8Array {
  return write_blffile("big", [
    s_blf_chunk_start_of_file.create("hopper-configuration"),
    mhcf,
    new s_blf_chunk_end_of_file(),
  ]);
}
