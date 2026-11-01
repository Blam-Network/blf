import { c } from "@craftycodie/cstruct";
import { blf, CStructBLFChunk } from "../../../blf_chunk";
import { write_blffile } from "../../../blffile";
import { s_blf_chunk_author } from "./s_blf_chunk_author";
import { s_blf_chunk_end_of_file } from "./s_blf_chunk_end_of_file";
import { s_blf_chunk_start_of_file } from "./s_blf_chunk_start_of_file";

/** Per-session network quality history (0x20 bytes). */
export const s_network_quality_session_statistics = c.struct({
  client_badness_history: c.u64().array(2),
  host_badness_history: c.u64().array(2),
});
export type s_network_quality_session_statistics = c.infer<
  typeof s_network_quality_session_statistics
>;

/** Persisted QoS / bandwidth measurements (0x6C bytes). */
export const s_network_bandwidth_persistent_data = c.struct({
  qos_sample_count: c.i32(),
  qos_sample_bps: c.i32().array(8),
  bandwidth_measurement_count: c.i32(),
  bandwidth_measurement_successful_bps: c.i32().array(8),
  bandwidth_measurement_unsafe_bps: c.i32().array(8),
  bandwidth_dispute_count: c.i32(),
});
export type s_network_bandwidth_persistent_data = c.infer<
  typeof s_network_bandwidth_persistent_data
>;

/** Halo 3 machine network statistics (`funs` 1.1, 0xC0 bytes). */
@blf.chunk("funs", 1.1)
export class s_blf_chunk_user_network_statistics extends CStructBLFChunk(
  c.struct({
    session: s_network_quality_session_statistics.array(2),
    connection_history: c.u64().array(2),
    bandwidth_data: s_network_bandwidth_persistent_data.padAfter(4),
  })
) {}

/** Halo 3 machine file: `_blf` "halo3 machine", `athr`, `funs`, `_eof`. */
export function build_machine_file(
  funs: c.input<typeof s_blf_chunk_user_network_statistics.schema>
): Uint8Array {
  return write_blffile("big", [
    s_blf_chunk_start_of_file.create("halo3 machine"),
    s_blf_chunk_author.forBuild(),
    Object.assign(new s_blf_chunk_user_network_statistics(), funs),
    new s_blf_chunk_end_of_file(),
  ]);
}
