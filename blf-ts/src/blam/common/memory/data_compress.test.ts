import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { c_bitstream_reader, e_bitstream_byte_order } from "../../../bitstream";
import { s_blf_chunk_game_variant } from "../../../chunks/haloreach/v12065_11_08_24_1738_tu1actual/s_blf_chunk_game_variant.ts";
import { c_game_engine_base_variant } from "../../haloreach/v12065_11_08_24_1738_tu1actual/game/game_engine_default";
import { s_player_trait_option } from "../../haloreach/v12065_11_08_24_1738_tu1actual/game/game_engine_traits";
import { s_user_defined_option } from "../../haloreach/v12065_11_08_24_1738_tu1actual/game/megalogamengine/megalogamengine_user_defined_options";
import { c_string_table } from "../../haloreach/v12065_11_08_24_1738_tu1actual/game/string_table";
import { runtime_data_decompress, zlib_compress } from "./data_compress";

const BE = e_bitstream_byte_order._bitstream_byte_order_big_endian;
const HOT_POTATO = "C:/Users/codie/Downloads/original (9).blf";

function skipToLocalizedDescription(reader: c_bitstream_reader): void {
  reader.read_integer("mode", 4);
  reader.read_signed_integer("enc", 32);
  reader.read_signed_integer("build", 32);
  new c_game_engine_base_variant().decode(reader);
  const ptc = reader.read_integer("ptc", 5);
  for (let i = 0; i < ptc; i++) {
    new s_player_trait_option().decode(reader);
  }
  const udc = reader.read_integer("udc", 5);
  for (let i = 0; i < udc; i++) {
    new s_user_defined_option().decode(reader);
  }
  new c_string_table(112, 0x4c00, 15, 15, 7).decode(reader);
  reader.read_integer("bni", 7);
  new c_string_table(1, 0x180, 9, 9, 1).decode(reader);
}

function readLocalizedDescriptionWire(gametype: Uint8Array): Uint8Array {
  const reader = c_bitstream_reader.new(gametype, BE);
  reader.begin_reading();
  skipToLocalizedDescription(reader);
  const stringCount = reader.read_integer("sc", 1);
  for (let j = 0; j < stringCount; j++) {
    for (let i = 0; i < 12; i++) {
      if (reader.read_bool("e")) {
        reader.read_integer("o", 12);
      }
    }
  }
  reader.read_integer("bs", 12);
  reader.read_bool("c");
  const clen = reader.read_integer("cl", 12);
  return reader.read_raw_data(clen * 8);
}

describe("runtime_data_compress", () => {
  it("matches Bungie zlib bytes for Hot Potato localized description", () => {
    let file: Uint8Array;
    try {
      file = new Uint8Array(readFileSync(HOT_POTATO));
    } catch {
      return;
    }

    const payload = file.subarray(0x2fc, 0x2fc + 20508);
    const gametype = payload.subarray(28, 28 + 8657);
    const origWire = readLocalizedDescriptionWire(gametype);
    const inner = runtime_data_decompress(origWire);
    const recompressed = zlib_compress(inner);

    expect(recompressed).toEqual(origWire.subarray(4));
  });

  it("round-trips Hot Potato gametype bytes", () => {
    let file: Uint8Array;
    try {
      file = new Uint8Array(readFileSync(HOT_POTATO));
    } catch {
      return;
    }

    const payload = file.subarray(0x2fc, 0x2fc + 20508);
    const orig = payload.subarray(28, 28 + 8657);
    const chunk = new s_blf_chunk_game_variant();
    chunk.read_body(payload, "big");
    const written = chunk.write_body("big");
    const enc = written.subarray(28, 28 + chunk.variant_length);

    expect(enc.length).toBe(orig.length);
    expect(enc).toEqual(orig);
  });
});
