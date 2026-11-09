import { readFileSync } from "node:fs";
import type { BLFChunk } from "@blamnetwork/blf";
import { s_blf_chunk_compressed_data } from "@blamnetwork/blf/halo3/v12070_08_09_05_2031_halo3_ship";
import * as release from "@blamnetwork/blf/haloreach/v11860_10_07_24_0147_omaha_release";
import * as tu1 from "@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual";
import * as mcc from "@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_16_1352";
import { describe, expect, it } from "vitest";
import {
  reach_11860_hot_potato_fixture,
  reach_12065_oddball_fixture,
  reach_mcc_3nvasion_dlc_fixture,
  reachmcc_zb_slayer_team_dmr_054_fixture,
} from "./fixtures/paths";
import { blfToJson } from "./lib/blf_json";

const ENDIAN = "big" as const;
const k_header_size = 12;
const k_hash_size = 20;

type t_game_variant_chunk = BLFChunk & { game_variant: any };

// The chunk with this signature, header included, walking the file's chunks from `_blf`.
function chunk_bytes(file: Uint8Array, signature: string): Uint8Array {
  const view = new DataView(file.buffer, file.byteOffset, file.byteLength);
  let offset = Buffer.from(file).indexOf("_blf");
  while (offset >= 0 && offset + k_header_size <= file.length) {
    const size = view.getUint32(offset + 4, false);
    if (
      String.fromCharCode(...file.subarray(offset, offset + 4)) === signature
    ) {
      return file.subarray(offset, offset + size);
    }
    offset += size;
  }
  throw new Error(`no ${signature} chunk`);
}

// The game variant chunk exactly as stored, decompressed from `_cmp` when the file wraps it.
function stored_mpvr(file: Uint8Array, compressed: boolean): Uint8Array {
  if (!compressed) {
    return chunk_bytes(file, "mpvr");
  }
  const cmp = chunk_bytes(file, "_cmp");
  return s_blf_chunk_compressed_data.decompress_payload(
    cmp.subarray(k_header_size)
  );
}

interface s_fixture {
  build: string;
  chunk: new () => t_game_variant_chunk;
  compressed: boolean;
  // MCC signs variants differently, so the hash blf-ts recomputes on write
  // isn't the file's.
  keeps_hash: boolean;
  path: string;
}

const k_fixtures: s_fixture[] = [
  {
    build: "TU1 (v12065)",
    path: reach_12065_oddball_fixture,
    chunk: tu1.s_blf_chunk_game_variant,
    compressed: false,
    keeps_hash: true,
  },
  {
    build: "release (v11860)",
    path: reach_11860_hot_potato_fixture,
    chunk: release.s_blf_chunk_game_variant,
    compressed: true,
    keeps_hash: true,
  },
  {
    build: "MCC (v_untracked_25_08_16_1352)",
    path: reach_mcc_3nvasion_dlc_fixture,
    chunk: mcc.s_blf_chunk_game_variant,
    compressed: false,
    keeps_hash: false,
  },
  {
    build: "MCC (v_untracked_25_08_16_1352)",
    path: reachmcc_zb_slayer_team_dmr_054_fixture,
    chunk: mcc.s_blf_chunk_game_variant,
    compressed: false,
    keeps_hash: false,
  },
];

function read_variant(fixture: s_fixture) {
  const stored = stored_mpvr(
    new Uint8Array(readFileSync(fixture.path)),
    fixture.compressed
  );
  const chunk = new fixture.chunk();
  chunk.read(stored, ENDIAN);
  return { chunk, stored, engine: chunk.game_variant.m_custom_variant };
}

function parameters(engine: any, key: string): any[] {
  return engine.m_game_engine.m_actions
    .map((action: any) => action[key])
    .filter((value: unknown) => value !== undefined);
}

describe.each(k_fixtures)("$build: $path", (fixture) => {
  it("re-encodes the game variant byte for byte", () => {
    const { chunk, stored } = read_variant(fixture);
    const written = chunk.write(ENDIAN);
    expect(written.length).toBe(stored.length);
    // A fixed-size mpvr can carry stale bytes after the variant, where we write zeros.
    const end = written.findLastIndex((byte) => byte !== 0) + 1;
    const start = fixture.keeps_hash ? 0 : k_header_size + k_hash_size;
    expect(
      Buffer.from(written.subarray(start, end)).equals(
        Buffer.from(stored.subarray(start, end))
      )
    ).toBe(true);
    expect(written.subarray(end).every((byte) => byte === 0)).toBe(true);
  });
});

describe("Reach variant fields the game and blf_lib define", () => {
  const tu1_variant = read_variant(k_fixtures[0]!);

  it("reads TU1 settings' active camo curve as min, then max", () => {
    const settings = tu1_variant.engine.m_tu1_settings;
    expect(settings.m_active_camo_energy_curve_min).toBeCloseTo(0.035_433, 5);
    expect(settings.m_active_camo_energy_curve_max).toBeCloseTo(0.066_929, 5);

    const defaults = new tu1.c_game_engine_custom_variant_tu1_settings();
    expect(defaults.m_active_camo_energy_curve_min).toBe(0);
    expect(defaults.m_active_camo_energy_curve_max).toBe(0);
    expect(defaults.m_armor_lock_damage_drain).toBe(0.2);
    expect(defaults.m_armor_lock_damage_drain_limit).toBe(0.7);
  });

  it("reads MCC TU1 settings' active camo curve as min, then max", () => {
    const { engine } = read_variant(k_fixtures[3]!);
    expect(engine.m_tu1_settings.m_active_camo_energy_curve_min).toBeCloseTo(
      0.035_433,
      5
    );
    expect(engine.m_tu1_settings.m_active_camo_energy_curve_max).toBeCloseTo(
      0.066_929,
      5
    );
  });

  it("holds set_loadout_palette's palette index, with the wire value one higher", () => {
    const palettes = parameters(
      tu1_variant.engine,
      "m_set_loadout_palette_parameters"
    ).map((p) => p.m_loadout_palette_index);
    expect(palettes).toEqual([1, 0]);
  });

  it("holds object filter label indexes, -1 for a filter without a label", () => {
    const labels = tu1_variant.engine.m_game_engine.m_object_filters.map(
      (filter: any) => filter.m_label_string_index
    );
    expect(labels).toContain(-1);
    expect(labels.some((label: number) => label >= 0)).toBe(true);
  });

  it("keeps a cylinder set_boundary in slots 1, 3 and 4", () => {
    const cylinders = parameters(
      tu1_variant.engine,
      "m_set_boundary_parameters"
    ).filter((p) => p.m_shape === tu1.e_boundary_shape.cylinder);
    expect(cylinders).toHaveLength(1);
    expect(blfToJson(cylinders[0])).toMatchObject({
      m_variable_1: { m_immediate_value: 20 },
      m_variable_2: { m_type: 0 },
      m_variable_3: { m_immediate_value: 10 },
      m_variable_4: { m_immediate_value: 10 },
    });
  });

  it("covers the same cases in the release and MCC fixtures", () => {
    for (const fixture of k_fixtures.slice(1)) {
      const { engine } = read_variant(fixture);
      const labels = engine.m_game_engine.m_object_filters.map(
        (filter: any) => filter.m_label_string_index
      );
      expect(labels).toContain(-1);
      expect(labels.some((label: number) => label >= 0)).toBe(true);
      expect(
        parameters(engine, "m_set_loadout_palette_parameters").length
      ).toBeGreaterThan(0);
    }
    const release_cylinders = parameters(
      read_variant(k_fixtures[1]!).engine,
      "m_set_boundary_parameters"
    ).filter((p) => p.m_shape === tu1.e_boundary_shape.cylinder);
    expect(release_cylinders).toHaveLength(1);
  });

  // No real files for these yet.
  it.todo("covers a cylinder set_boundary in an MCC variant");
  it.todo("covers alpha (v08516) variants");
  it.todo("covers delta (v09730) variants");
});
