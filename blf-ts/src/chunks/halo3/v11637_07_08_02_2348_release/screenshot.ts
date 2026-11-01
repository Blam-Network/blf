import { read_blffile } from "../../../read_blffile";
import { s_blf_chunk_content_header } from "../v12070_08_09_05_2031_halo3_ship/s_blf_chunk_content_header";
import { s_blf_chunk_end_of_file } from "../v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_screenshot_data } from "../v12070_08_09_05_2031_halo3_ship/s_blf_chunk_screenshot_data";
import { s_blf_chunk_start_of_file } from "../v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";

/** Pre-release Halo 3 blind screenshot (no camera chunk): `_blf chdr scnd _eof`. */
export interface blind_screenshot {
  _blf: s_blf_chunk_start_of_file;
  _eof: s_blf_chunk_end_of_file;
  chdr: s_blf_chunk_content_header;
  scnd: s_blf_chunk_screenshot_data;
}

/** Read a Halo 3 11637 blind screenshot, or `null` if `buffer` isn't one. */
export function read_blind_screenshot(
  buffer: Uint8Array
): blind_screenshot | null {
  const file: blind_screenshot = {
    _blf: new s_blf_chunk_start_of_file(),
    chdr: new s_blf_chunk_content_header(),
    scnd: new s_blf_chunk_screenshot_data(),
    _eof: new s_blf_chunk_end_of_file(),
  };

  try {
    read_blffile(buffer, "big", [file._blf, file.chdr, file.scnd, file._eof]);
  } catch {
    return null;
  }

  return file;
}
