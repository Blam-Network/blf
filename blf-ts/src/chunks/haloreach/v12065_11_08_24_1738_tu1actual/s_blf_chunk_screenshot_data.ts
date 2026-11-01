import { blf } from "../../../blf_chunk";
import { s_blf_chunk_screenshot_data as halo3_s_blf_chunk_screenshot_data } from "../../halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_screenshot_data";

/** Reach screenshot JPEG payload (`scnd` 1.2): same layout as Halo 3's `scnd` 1.1. */
@blf.chunk("scnd", 1.2)
export class s_blf_chunk_screenshot_data extends halo3_s_blf_chunk_screenshot_data {}
