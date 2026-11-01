import { c } from "@craftycodie/cstruct";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";

/** One queued file share download (`s_files_user_auto_download_queue_item`, 0x138 bytes). */
export const s_files_user_auto_download_queue_item = c.struct({
  share_id: c.u64(),
  server_id: c.u64(),
  file_type: c.u8(),
  activity: c.u8(),
  game_engine_type: c.i8(),
  icon_index: c.i8(),
  megalo_category_index: c.i8().padAfter(3),
  size_in_bytes: c.u32(),
  map_id: c.i32(),
  modified_time: c.Time64(),
  modified_by: c.string(16, "utf8"),
  file_name: c.wstring(128),
});
export type s_files_user_auto_download_queue_item = c.infer<
  typeof s_files_user_auto_download_queue_item
>;

const s_filq_header = c.struct({ transfers_count: c.u32() });
const FILQ_HEADER_SIZE = c.sizeof(s_filq_header);

/** Reach TU1 user file share download queue (`filq` 3.0): a u32 count, then the items. */
@blf.chunk("filq", 3.0)
export class s_blf_chunk_file_transfers extends BLFChunkBase {
  transfers: s_files_user_auto_download_queue_item[] = [];

  read_body(payload: Uint8Array, endian: c.Endian): void {
    if (payload.length < FILQ_HEADER_SIZE) {
      throw new BlfError(
        `filq chunk payload too short: need at least ${FILQ_HEADER_SIZE} bytes, got ${payload.length}`
      );
    }

    const { transfers_count } = c.read(
      s_filq_header,
      payload.subarray(0, FILQ_HEADER_SIZE),
      endian
    );
    const item_size = c.sizeof(s_files_user_auto_download_queue_item);
    const expected_size = FILQ_HEADER_SIZE + transfers_count * item_size;
    if (payload.length < expected_size) {
      throw new BlfError(
        `filq chunk payload too short for ${transfers_count} transfers: need ${expected_size} bytes, got ${payload.length}`
      );
    }

    this.transfers = [];
    for (let i = 0; i < transfers_count; i++) {
      const offset = FILQ_HEADER_SIZE + i * item_size;
      this.transfers.push(
        c.read(
          s_files_user_auto_download_queue_item,
          payload.subarray(offset, offset + item_size),
          endian
        )
      );
    }
  }

  write_body(endian: c.Endian): Uint8Array {
    const item_size = c.sizeof(s_files_user_auto_download_queue_item);
    const out = new Uint8Array(
      FILQ_HEADER_SIZE + this.transfers.length * item_size
    );
    out.set(
      c.write(
        s_filq_header,
        { transfers_count: this.transfers.length },
        endian
      ),
      0
    );
    this.transfers.forEach((transfer, i) => {
      out.set(
        s_files_user_auto_download_queue_item.write(transfer, endian),
        FILQ_HEADER_SIZE + i * item_size
      );
    });
    return out;
  }
}
