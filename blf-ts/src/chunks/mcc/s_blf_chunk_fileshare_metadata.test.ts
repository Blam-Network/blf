import { readFileSync } from "node:fs";
import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { reachmcc_zb_slayer_team_dmr_054_fixture } from "../../../tests/fixtures/paths";
import { search_for_chunk } from "../../blf_chunk";
import { BlfError } from "../../error";
import { s_blf_chunk_fileshare_metadata } from "./s_blf_chunk_fileshare_metadata";

/** Payload bytes after the 12-byte BLF `_fsm` header (fixture-verified). */
const FSM_PAYLOAD_SIZE = 452;

/** Full `_fsm` chunk on disk including header (fixture-verified). */
const FSM_CHUNK_SIZE = 464;

function load_fixture(): Uint8Array {
  return new Uint8Array(readFileSync(reachmcc_zb_slayer_team_dmr_054_fixture));
}

function extract_fsm_bytes(file: Uint8Array): Uint8Array {
  const chunk = new s_blf_chunk_fileshare_metadata();
  expect(search_for_chunk(file, chunk, "big")).toBe(true);
  return chunk.write("big");
}

describe("s_blf_chunk_fileshare_metadata", () => {
  it("has the expected struct size", () => {
    expect(c.sizeof(s_blf_chunk_fileshare_metadata)).toBe(FSM_PAYLOAD_SIZE);
  });

  it("rejects payloads shorter than the struct", () => {
    const chunk = new s_blf_chunk_fileshare_metadata();
    expect(() => chunk.read_body(new Uint8Array(0), "big")).toThrow(BlfError);
  });

  it("finds and decodes _fsm from reachmcc_zb_slayer_team_dmr_054.bin", () => {
    const file = load_fixture();
    const chunk = new s_blf_chunk_fileshare_metadata();
    expect(search_for_chunk(file, chunk, "big")).toBe(true);

    expect(chunk.unknown28.replace(/\0/g, "")).toBe("");
    expect(chunk.unknown74.replace(/\0/g, "")).toBe("");
    expect(Array.from(chunk.unknown54.slice(0, 4))).toEqual([
      0xa3, 0xa0, 0xfd, 0x8d,
    ]);
    expect(chunk.unknown98.replace(/\0/g, "")).toBe(
      "85E3D5677B0D64FD91FACB25D696BF315CB062A1"
    );
    expect(chunk.attestation_signature[255]).toBe(0x75);
    expect(chunk.signature).toBe("_fsm");
  });

  it("round-trips payload bytes from fixture", () => {
    const on_disk = extract_fsm_bytes(load_fixture());
    expect(on_disk.length).toBe(FSM_CHUNK_SIZE);

    const chunk = new s_blf_chunk_fileshare_metadata();
    chunk.read_body(on_disk.subarray(12), "big");

    expect(chunk.write_body("big")).toEqual(on_disk.subarray(12));
  });

  it("round-trips full chunk from fixture", () => {
    const on_disk = extract_fsm_bytes(load_fixture());
    const chunk = new s_blf_chunk_fileshare_metadata();
    chunk.read(on_disk, "big");
    expect(chunk.write("big")).toEqual(on_disk);
  });
});
