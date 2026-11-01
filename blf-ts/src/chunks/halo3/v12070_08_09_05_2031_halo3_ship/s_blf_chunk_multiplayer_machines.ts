import { c } from "@craftycodie/cstruct";
import {
  k_maximum_machines,
  s_machine_identifier,
} from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/game/game_options";
import { s_transport_secure_address } from "../../../blam/halo3/v12070_08_09_05_2031_halo3_ship/networking/transport/transport_security";
import { blf, CStructBLFChunk } from "../../../blf_chunk";

export enum e_online_nat_type {
  Unknown = 0,
  Open = 1,
  Moderate = 2,
  Strict = 3,
}

/** 0x15 bytes. */
export const s_game_results_machine_data = c.struct({
  identifier: s_machine_identifier,
  exists: c.bool(),
  connected_to_host: c.bool(),
  host: c.bool(),
  initial_host: c.bool(),
  voluntary_quit: c.bool(),
  bandwidth_events: c.i16().array(5),
});
export type s_game_results_machine_data = c.infer<
  typeof s_game_results_machine_data
>;

/** 0x42 bytes. */
export const s_machine_session_info = c.struct({
  exists: c.bool(),
  has_hard_drive: c.bool(),
  party_nonce: c.u64(),
  secure_address: s_transport_secure_address,
  network_version_number: c.i32(),
  peer_estimated_downstream_bandwidth_bps: c.i32(),
  peer_estimated_upstream_bandwidth_bps: c.i32(),
  peer_nat_type: c.enum(e_online_nat_type, c.i32()),
  peer_to_peer_connectivity_mask: c.u16(),
  peer_to_peer_probed_mask: c.u16(),
});
export type s_machine_session_info = c.infer<typeof s_machine_session_info>;

/** 0x58 bytes. */
export const s_blf_chunk_multiplayer_machines_machine = c.struct({
  machine_data: s_game_results_machine_data,
  session_info: s_machine_session_info.padBefore(1),
});
export type s_blf_chunk_multiplayer_machines_machine = c.infer<
  typeof s_blf_chunk_multiplayer_machines_machine
>;

/** Halo 3 game results machines (`mpma` 2.1, 0x584 bytes). */
@blf.chunk("mpma", 2.1)
export class s_blf_chunk_multiplayer_machines extends CStructBLFChunk(
  c.struct({
    in_squad_session: c.bool(),
    in_group_session: c.bool(),
    machines: s_blf_chunk_multiplayer_machines_machine
      .array(k_maximum_machines)
      .padBefore(2),
  })
) {}
