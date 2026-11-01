import { read_blffile } from "../../../read_blffile";
import { s_blf_chunk_compressed_data } from "./s_blf_chunk_compressed_data";
import { s_blf_chunk_content_header } from "./s_blf_chunk_content_header";
import { s_blf_chunk_end_of_file } from "./s_blf_chunk_end_of_file";
import { s_blf_chunk_screenshot_camera } from "./s_blf_chunk_screenshot_camera";
import { s_blf_chunk_screenshot_data } from "./s_blf_chunk_screenshot_data";
import { s_blf_chunk_start_of_file } from "./s_blf_chunk_start_of_file";

/** Screenshot uploaded outside the file share: `_blf chdr scnc scnd _eof`. */
export interface blind_screenshot {
  _blf: s_blf_chunk_start_of_file;
  _eof: s_blf_chunk_end_of_file;
  chdr: s_blf_chunk_content_header;
  scnc: s_blf_chunk_screenshot_camera;
  scnd: s_blf_chunk_screenshot_data;
}

/** File share screenshot: `_blf chdr scnc _cmp(scnd) _eof`. */
export interface fileshare_screenshot {
  _blf: s_blf_chunk_start_of_file;
  _eof: s_blf_chunk_end_of_file;
  chdr: s_blf_chunk_content_header;
  scnc: s_blf_chunk_screenshot_camera;
  scnd: s_blf_chunk_compressed_data<s_blf_chunk_screenshot_data>;
}

/** Read a Halo 3 blind screenshot, or `null` if `buffer` isn't one. */
export function read_blind_screenshot(
  buffer: Uint8Array
): blind_screenshot | null {
  const file: blind_screenshot = {
    _blf: new s_blf_chunk_start_of_file(),
    chdr: new s_blf_chunk_content_header(),
    scnc: new s_blf_chunk_screenshot_camera(),
    scnd: new s_blf_chunk_screenshot_data(),
    _eof: new s_blf_chunk_end_of_file(),
  };

  try {
    read_blffile(buffer, "big", [
      file._blf,
      file.chdr,
      file.scnc,
      file.scnd,
      file._eof,
    ]);
  } catch {
    return null;
  }

  return file;
}

/** Read a Halo 3 file share screenshot, or `null` if `buffer` isn't one. */
export function read_fileshare_screenshot(
  buffer: Uint8Array
): fileshare_screenshot | null {
  const file: fileshare_screenshot = {
    _blf: new s_blf_chunk_start_of_file(),
    chdr: new s_blf_chunk_content_header(),
    scnc: new s_blf_chunk_screenshot_camera(),
    scnd: new s_blf_chunk_compressed_data(s_blf_chunk_screenshot_data),
    _eof: new s_blf_chunk_end_of_file(),
  };

  try {
    read_blffile(buffer, "big", [
      file._blf,
      file.chdr,
      file.scnc,
      file.scnd,
      file._eof,
    ]);
  } catch {
    return null;
  }

  return file;
}
