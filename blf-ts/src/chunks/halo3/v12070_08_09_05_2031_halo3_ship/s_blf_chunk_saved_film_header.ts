import { c } from "@craftycodie/cstruct";
import { time32_t } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/cseries/cseries_types";
import { game_options } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_options";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

/** Halo 3 saved film header (`flmh` 10.1, 0xFE60 bytes). */
@blf.chunk("flmh", 10.1)
export class s_blf_chunk_saved_film_header extends CStructBLFChunk(
  c.struct({
    build_number: c.string(32, "latin1").padBefore(4),
    executable_type: c.i32(),
    network_executable_version: c.i32(),
    network_compatible_version: c.i32(),
    map_language: c.i32(),
    map_minor_version: c.i32(),
    map_minor_version_is_tracked: c.bool(),
    map_signature_size: c.i32().padBefore(11),
    map_signature_bytes: c.u8().array(60),
    is_host_film: c.bool(),
    contains_gamestate: c.bool(),
    is_snippet: c.bool(),
    session_id: c.u8().array(128).padBefore(5),
    options: game_options,
    recorded_time: time32_t(),
    length_in_ticks: c.i32(),
    snippet_start_tick: c.i32(),
    /** blf_lib guesses this size. */
    padding_to_align_for_utility_drive: c.u8().array(0x538),
  })
) {}
