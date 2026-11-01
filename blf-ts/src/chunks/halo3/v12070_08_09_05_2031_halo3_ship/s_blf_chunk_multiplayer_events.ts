import { c } from "@craftycodie/cstruct";
import { real_point3d } from "../../../blam/common/math/real_math";
import { BLFChunkBase, blf } from "../../../blf_chunk";
import { BlfError } from "../../../error";

export const k_game_results_event_count = 1000;

/** Size of one event as the game stores it. */
const k_game_results_event_size = 0x24;
/** Size of an event's data slot. */
const k_game_results_event_data_size = 0x1c;

export enum e_game_results_data_type {
  _no_data = 0,
  _kill = 1,
  _carry = 2,
  _score = 3,
}

/** 0x1C bytes. */
export const s_game_results_data_kill = c.struct({
  killer_position: real_point3d,
  dead_position: real_point3d,
  kill_type: c.i32(),
});
export type s_game_results_data_kill = c.infer<typeof s_game_results_data_kill>;

/** 0x1C bytes. */
export const s_game_results_data_carry = c.struct({
  position: real_point3d,
  weapon_index: c.i32(),
  carry_type: c.i32().padAfter(8),
});
export type s_game_results_data_carry = c.infer<
  typeof s_game_results_data_carry
>;

/** 0x1C bytes. */
export const s_game_results_data_score = c.struct({
  position: real_point3d,
  weapon_index: c.i32(),
  score_type: c.i32().padAfter(8),
});
export type s_game_results_data_score = c.infer<
  typeof s_game_results_data_score
>;

/** One game results event. Only the data for `type_` is present. */
export interface s_game_results_event {
  carry_data?: s_game_results_data_carry;
  kill_data?: s_game_results_data_kill;
  player_references: [number, number];
  score_data?: s_game_results_data_score;
  time: number;
  type_: e_game_results_data_type;
}

const k_event_data = {
  [e_game_results_data_type._kill]: ["kill_data", s_game_results_data_kill],
  [e_game_results_data_type._carry]: ["carry_data", s_game_results_data_carry],
  [e_game_results_data_type._score]: ["score_data", s_game_results_data_score],
} as const;

/**
 * Halo 3 game results events (`mpev` 2.1, 0x8CA4 bytes): 1000 events of 0x24
 * bytes after 4 bytes of padding.
 *
 * blf_lib (and so `@blam-network/blf_lsp`) reads a `_no_data` event's data as
 * 1 byte instead of 0x1C, misreading every later event; this reads the game's
 * fixed 0x24-byte layout instead. An unknown event type throws.
 */
@blf.chunk("mpev", 2.1)
export class s_blf_chunk_multiplayer_events extends BLFChunkBase {
  events: s_game_results_event[] = Array.from(
    { length: k_game_results_event_count },
    () => ({
      type_: e_game_results_data_type._no_data,
      player_references: [0, 0] as [number, number],
      time: 0,
    })
  );

  read_body(payload: Uint8Array, endian: c.Endian): void {
    const size = 4 + k_game_results_event_count * k_game_results_event_size;
    if (payload.length < size) {
      throw new BlfError(
        `Cannot read mpev chunk: need ${size} bytes, got ${payload.length}`
      );
    }
    const view = new DataView(
      payload.buffer,
      payload.byteOffset,
      payload.byteLength
    );
    const little = endian === "little";
    const events: s_game_results_event[] = [];

    for (let i = 0; i < k_game_results_event_count; i++) {
      const offset = 4 + i * k_game_results_event_size;
      const type_ = view.getUint8(offset) as e_game_results_data_type;
      const event: s_game_results_event = {
        type_,
        player_references: [view.getInt8(offset + 1), view.getInt8(offset + 2)],
        time: view.getUint32(
          offset + 4 + k_game_results_event_data_size,
          little
        ),
      };

      if (type_ in k_event_data) {
        const [key, schema] = k_event_data[type_ as keyof typeof k_event_data];
        (event as unknown as Record<string, unknown>)[key] = schema.read(
          payload,
          { offset: offset + 4, endian }
        );
      } else if (type_ !== e_game_results_data_type._no_data) {
        throw new BlfError(
          `Cannot read mpev chunk: event ${i} has unknown type ${type_}`
        );
      }

      events.push(event);
    }

    this.events = events;
  }

  write_body(endian: c.Endian): Uint8Array {
    const out = new Uint8Array(
      4 + k_game_results_event_count * k_game_results_event_size
    );
    const view = new DataView(out.buffer);
    const little = endian === "little";

    for (let i = 0; i < k_game_results_event_count; i++) {
      const event = this.events[i];
      if (!event) {
        continue;
      }
      const offset = 4 + i * k_game_results_event_size;
      view.setUint8(offset, event.type_);
      view.setInt8(offset + 1, event.player_references[0]);
      view.setInt8(offset + 2, event.player_references[1]);

      if (event.type_ !== e_game_results_data_type._no_data) {
        const [key, schema] = k_event_data[event.type_];
        const data = (event as unknown as Record<string, unknown>)[key];
        if (data === undefined || data === null) {
          throw new BlfError(
            `Cannot write mpev chunk: event ${i} has no ${key}`
          );
        }
        (schema as c.Struct).writeInto(out, data as never, offset + 4, endian);
      }

      view.setUint32(
        offset + 4 + k_game_results_event_data_size,
        event.time,
        little
      );
    }

    return out;
  }
}
