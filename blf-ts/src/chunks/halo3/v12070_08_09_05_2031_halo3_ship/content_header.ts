import { find_chunk } from "../../../blf_chunk";
import { s_blf_chunk_content_header } from "./s_blf_chunk_content_header";

/** Find and read the `chdr` 9.2 chunk of a Halo 3 BLF file, or `null` if there isn't a valid one. */
export function read_content_header(
  buffer: Uint8Array
): s_blf_chunk_content_header | null {
  const chdr = new s_blf_chunk_content_header();
  try {
    return find_chunk(buffer, chdr, "big") ? chdr : null;
  } catch {
    return null;
  }
}
