import { c } from "@craftycodie/cstruct";
import { describe, expect, it } from "vitest";
import { getBlfChunkMeta } from "../../../blf_chunk";
import {
  s_blf_chunk_file_transfers,
  s_files_user_auto_download_queue_item,
} from "./s_blf_chunk_file_transfers";

function make_item(i: number): s_files_user_auto_download_queue_item {
  const item = s_files_user_auto_download_queue_item.zero();
  item.share_id = 0xffff_ffff_ffff_fff0n + BigInt(i);
  item.server_id = BigInt(1000 + i);
  item.file_type = 6;
  item.activity = 2;
  item.game_engine_type = -1;
  item.icon_index = -1;
  item.megalo_category_index = 3;
  item.size_in_bytes = 0xe0_00;
  item.map_id = 1035;
  item.modified_time = new Date(1_600_000_000_000 + i * 1000);
  item.modified_by = `gamer ${i}`;
  item.file_name = `Map Ünïcode ${i}`;
  return item;
}

describe("s_blf_chunk_file_transfers", () => {
  it("is filq 3.0 with 0x138-byte items", () => {
    const meta = getBlfChunkMeta(new s_blf_chunk_file_transfers());
    expect(meta.signature).toBe("filq");
    expect(meta.major).toBe(3);
    expect(meta.minor).toBe(0);
    expect(c.sizeof(s_files_user_auto_download_queue_item)).toBe(0x138);
  });

  it("writes a u32 count before the items", () => {
    const filq = new s_blf_chunk_file_transfers();
    expect(Array.from(filq.write_body("big"))).toEqual([0, 0, 0, 0]);

    filq.transfers = [make_item(0), make_item(1), make_item(2)];
    const payload = filq.write_body("big");
    expect(payload.length).toBe(4 + 3 * 0x138);
    expect(Buffer.from(payload).readUInt32BE(0)).toBe(3);
    // megalo_category_index is followed by 3 pad bytes.
    expect(Array.from(payload.subarray(4 + 0x14, 4 + 0x18))).toEqual([
      3, 0, 0, 0,
    ]);
  });

  it("round-trips transfers", () => {
    const filq = new s_blf_chunk_file_transfers();
    filq.transfers = [make_item(0), make_item(1)];

    const parsed = new s_blf_chunk_file_transfers();
    parsed.read_body(filq.write_body("big"), "big");
    expect(parsed.transfers).toEqual([make_item(0), make_item(1)]);
  });

  it("rejects truncated payloads", () => {
    const parsed = new s_blf_chunk_file_transfers();
    expect(() => parsed.read_body(new Uint8Array(3), "big")).toThrow(
      /filq chunk payload too short/
    );
    expect(() =>
      parsed.read_body(new Uint8Array([0, 0, 0, 1, 0, 0]), "big")
    ).toThrow(/too short for 1 transfers/);
  });
});
