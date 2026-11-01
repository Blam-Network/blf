import { read_blffile } from "../../../read_blffile";
import { s_blf_chunk_end_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_start_of_file } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import { s_blf_chunk_content_header } from "./s_blf_chunk_content_header";
import { s_blf_chunk_screenshot_camera } from "./s_blf_chunk_screenshot_camera";
import { s_blf_chunk_screenshot_data } from "./s_blf_chunk_screenshot_data";
import { s_blf_chunk_server_signature } from "./s_blf_chunk_server_signature";

/** Reach screenshot: `_blf chdr scnc scnd ssig _eof`. */
export interface blind_screenshot {
  _blf: s_blf_chunk_start_of_file;
  _eof: s_blf_chunk_end_of_file;
  chdr: s_blf_chunk_content_header;
  scnc: s_blf_chunk_screenshot_camera;
  scnd: s_blf_chunk_screenshot_data;
  ssig: s_blf_chunk_server_signature;
}

/** Read a Reach (TU0/TU1) screenshot, or `null` if `buffer` isn't one. */
export function read_blind_screenshot(
  buffer: Uint8Array
): blind_screenshot | null {
  const file: blind_screenshot = {
    _blf: new s_blf_chunk_start_of_file(),
    chdr: new s_blf_chunk_content_header(),
    scnc: new s_blf_chunk_screenshot_camera(),
    scnd: new s_blf_chunk_screenshot_data(),
    ssig: new s_blf_chunk_server_signature(),
    _eof: new s_blf_chunk_end_of_file(),
  };

  try {
    read_blffile(buffer, "big", [
      file._blf,
      file.chdr,
      file.scnc,
      file.scnd,
      file.ssig,
      file._eof,
    ]);
  } catch {
    return null;
  }

  return file;
}
