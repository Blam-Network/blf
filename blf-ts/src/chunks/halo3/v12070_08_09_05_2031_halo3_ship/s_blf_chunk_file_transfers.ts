import { c } from "@craftycodie/cstruct";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";

const k_max_transfers = 8;

/** One entry of the user's fileshare auto-download queue (0x50 bytes). */
export const s_files_user_auto_download_queue_item = c.struct({
  share_id: c.u64(),
  slot: c.u32(),
  title_index: c.u16().padAfter(2),
  server_id: c.u64(),
  file_name: c.wstring(16),
  file_type: c.i32(),
  campaign_id: c.u32(),
  map_id: c.u32(),
  game_engine_type: c.u32(),
  size_bytes: c.u64(),
});
export type s_files_user_auto_download_queue_item = c.infer<
  typeof s_files_user_auto_download_queue_item
>;

const s_file_transfers_body = c.struct({
  transfers: s_files_user_auto_download_queue_item.array(k_max_transfers),
});

/**
 * Halo 3 fileshare auto-download queue (`filq` 1.0, 0x280 bytes).
 * Holds up to 8 transfers; fewer are padded with empty entries on write
 * (as blf_lib's `StaticArray` does). Reading always yields 8 entries.
 */
@blf.chunk("filq", 1.0)
export class s_blf_chunk_file_transfers extends BLFChunkBase {
  transfers: s_files_user_auto_download_queue_item[] = [];

  read_body(payload: Uint8Array, endian: c.Endian): void {
    if (payload.length < s_file_transfers_body.getSize()) {
      throw new BlfError(
        `filq chunk payload too short: need ${s_file_transfers_body.getSize()} bytes, got ${payload.length}`
      );
    }
    this.transfers = s_file_transfers_body.read(payload, endian).transfers;
  }

  write_body(endian: c.Endian): Uint8Array {
    if (this.transfers.length > k_max_transfers) {
      throw new BlfError(
        `filq holds at most ${k_max_transfers} transfers, got ${this.transfers.length}`
      );
    }
    const entry_size = s_files_user_auto_download_queue_item.getSize();
    const out = new Uint8Array(s_file_transfers_body.getSize());
    this.transfers.forEach((transfer, i) => {
      s_files_user_auto_download_queue_item.writeInto(
        out,
        transfer,
        i * entry_size,
        endian
      );
    });
    return out;
  }
}
