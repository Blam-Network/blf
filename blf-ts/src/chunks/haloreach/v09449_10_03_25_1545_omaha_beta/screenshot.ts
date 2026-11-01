import { read_blffile } from "../../../read_blffile";
import { s_blf_chunk_compressed_data } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_compressed_data";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_screenshot_data } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_screenshot_data";
import { s_blf_chunk_start_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import { s_blf_chunk_content_header } from "../v12065_11_08_24_1738_tu1actual/s_blf_chunk_content_header";

/** Reach private beta screenshot: `_blf chdr _cmp(scnd 1.1) _eof`. */
export interface blind_screenshot {
  _blf: s_blf_chunk_start_of_file;
  _cmp: s_blf_chunk_compressed_data<s_blf_chunk_screenshot_data>;
  _eof: s_blf_chunk_end_of_file;
  chdr: s_blf_chunk_content_header;
}

/** Read a Reach 09449 (private beta) screenshot, or `null` if `buffer` isn't one. */
export function read_blind_screenshot(
  buffer: Uint8Array
): blind_screenshot | null {
  const file: blind_screenshot = {
    _blf: new s_blf_chunk_start_of_file(),
    chdr: new s_blf_chunk_content_header(),
    _cmp: new s_blf_chunk_compressed_data(s_blf_chunk_screenshot_data),
    _eof: new s_blf_chunk_end_of_file(),
  };

  try {
    read_blffile(buffer, "big", [file._blf, file.chdr, file._cmp, file._eof]);
  } catch {
    return null;
  }

  return file;
}
