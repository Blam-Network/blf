import { c } from "@craftycodie/cstruct";
import {
  s_online_file_metadata,
  s_online_file_tag,
} from "../../../blam/haloreach/v12065_11_08_24_1738_tu1actual/networking/online/files/online_file_metadata";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";

/** The fixed part of `fitm`, before the entries. */
const s_online_file_listing_header = c.struct({
  xuid: c.u64(),
  gamertag: c.string(16),
  unknown16: c.u8(),
  unknown17: c.u8(),
  unknown18: c.u8(),
  unknown19: c.u8(),
  quota_byte_count: c.u32(),
  quota_slot_count: c.u8().padAfter(1),
  slot_count: c.u16(),
  message_length: c.u8().padAfter(3),
});

/** A listing's message: `length` UTF-16 characters and a NUL. */
const message_struct = (length: number) =>
  c.struct({ message: c.wstring(length + 1) });

/**
 * Reach TU1 fileshare listing (`fitm` 4.0): a user's files, or one file's
 * details.
 *
 * A listing ends with `message` as a NUL-terminated UTF-16 string, written even
 * when empty. File details set `message` to `null` and instead end with the
 * single entry's screenshot preview (`entries[0].screenshot_length` bytes) and
 * tags (`entries[0].general.tag_count` of them).
 */
@blf.chunk("fitm", 4.0)
export class s_blf_chunk_online_file_listing extends BLFChunkBase {
  xuid = 0n;
  gamertag = "";
  unknown16 = 0;
  unknown17 = 0;
  unknown18 = 0;
  unknown19 = 0;
  quota_byte_count = 0;
  quota_slot_count = 0;
  entries: s_online_file_metadata[] = [];
  message: string | null = "";
  screenshot: number[] = [];
  tags: s_online_file_tag[] = [];

  read_body(payload: Uint8Array, endian: c.Endian): void {
    const header_size = s_online_file_listing_header.getSize();
    if (payload.length < header_size) {
      throw new BlfError(
        `fitm chunk payload too short: need at least ${header_size} bytes, got ${payload.length}`
      );
    }
    const header = s_online_file_listing_header.read(payload, endian);
    this.xuid = header.xuid;
    this.gamertag = header.gamertag;
    this.unknown16 = header.unknown16;
    this.unknown17 = header.unknown17;
    this.unknown18 = header.unknown18;
    this.unknown19 = header.unknown19;
    this.quota_byte_count = header.quota_byte_count;
    this.quota_slot_count = header.quota_slot_count;

    const entry_size = s_online_file_metadata.getSize();
    let offset = header_size;
    this.entries = [];
    for (let i = 0; i < header.slot_count; i++) {
      this.entries.push(s_online_file_metadata.read(payload, offset, endian));
      offset += entry_size;
    }

    this.message = null;
    this.screenshot = [];
    this.tags = [];
    const [first] = this.entries;
    const details =
      header.slot_count === 1 &&
      header.message_length === 0 &&
      first !== undefined &&
      offset +
        first.screenshot_length +
        first.general.tag_count * s_online_file_tag.getSize() ===
        payload.length;
    if (details) {
      this.screenshot = Array.from(
        payload.subarray(offset, offset + first.screenshot_length)
      );
      offset += first.screenshot_length;
      this.tags = c.read(
        s_online_file_tag.array(first.general.tag_count),
        payload,
        offset,
        endian
      );
      return;
    }
    this.message = message_struct(header.message_length).read(
      payload,
      offset,
      endian
    ).message;
  }

  write_body(endian: c.Endian): Uint8Array {
    const header = s_online_file_listing_header.write(
      {
        xuid: this.xuid,
        gamertag: this.gamertag,
        unknown16: this.unknown16,
        unknown17: this.unknown17,
        unknown18: this.unknown18,
        unknown19: this.unknown19,
        quota_byte_count: this.quota_byte_count,
        quota_slot_count: this.quota_slot_count,
        slot_count: this.entries.length,
        message_length: this.message?.length ?? 0,
      },
      endian
    );
    const parts: Uint8Array[] = [
      header,
      ...this.entries.map((entry) =>
        s_online_file_metadata.write(entry, endian)
      ),
    ];
    if (this.message === null) {
      parts.push(Uint8Array.from(this.screenshot));
      parts.push(
        c.write(s_online_file_tag.array(this.tags.length), this.tags, endian)
      );
    } else {
      parts.push(
        message_struct(this.message.length).write(
          { message: this.message },
          endian
        )
      );
    }

    const out = new Uint8Array(
      parts.reduce((size, part) => size + part.length, 0)
    );
    let at = 0;
    for (const part of parts) {
      out.set(part, at);
      at += part.length;
    }
    return out;
  }
}
