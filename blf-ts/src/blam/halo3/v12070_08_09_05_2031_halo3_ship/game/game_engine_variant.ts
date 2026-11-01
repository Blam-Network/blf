import { CFieldType, c } from "@craftycodie/cstruct";
import { c_game_engine_assault_variant } from "./game_engine_assault";
import { c_game_engine_ctf_variant } from "./game_engine_ctf";
import { c_game_engine_base_variant } from "./game_engine_default";
import { c_game_engine_infection_variant } from "./game_engine_infection";
import { c_game_engine_juggernaut_variant } from "./game_engine_juggernaut";
import { c_game_engine_king_variant } from "./game_engine_king";
import { c_game_engine_oddball_variant } from "./game_engine_oddball";
import { c_game_engine_sandbox_variant } from "./game_engine_sandbox";
import { c_game_engine_slayer_variant } from "./game_engine_slayer";
import { c_game_engine_territories_variant } from "./game_engine_territories";
import { c_game_engine_vip_variant } from "./game_engine_vip";

export enum e_game_engine {
  none = 0,
  ctf = 1,
  slayer = 2,
  oddball = 3,
  king = 4,
  sandbox = 5,
  vip = 6,
  juggernaut = 7,
  territories = 8,
  assault = 9,
  infection = 10,
}

/** Size of the unpacked `c_game_variant` (engine index, base variant and engine variant). */
export const k_game_variant_size = 0x264;

/** Engine-specific variant key and layout for each engine. */
const k_game_engine_variants = {
  [e_game_engine.ctf]: ["m_ctf_variant", c_game_engine_ctf_variant],
  [e_game_engine.slayer]: ["m_slayer_variant", c_game_engine_slayer_variant],
  [e_game_engine.oddball]: ["m_oddball_variant", c_game_engine_oddball_variant],
  [e_game_engine.king]: ["m_king_variant", c_game_engine_king_variant],
  [e_game_engine.sandbox]: ["m_sandbox_variant", c_game_engine_sandbox_variant],
  [e_game_engine.vip]: ["m_vip_variant", c_game_engine_vip_variant],
  [e_game_engine.juggernaut]: [
    "m_juggernaut_variant",
    c_game_engine_juggernaut_variant,
  ],
  [e_game_engine.territories]: [
    "m_territories_variant",
    c_game_engine_territories_variant,
  ],
  [e_game_engine.assault]: ["m_assault_variant", c_game_engine_assault_variant],
  [e_game_engine.infection]: [
    "m_infection_variant",
    c_game_engine_infection_variant,
  ],
} as const satisfies Record<
  Exclude<e_game_engine, e_game_engine.none>,
  readonly [string, c.Struct]
>;

/**
 * Unpacked Halo 3 game variant. Only the variant for `m_game_engine` is present,
 * as in blf_lib (the other `Option`s are `None`).
 */
export interface c_game_variant {
  m_assault_variant?: c_game_engine_assault_variant;
  m_base_variant: c_game_engine_base_variant;
  m_ctf_variant?: c_game_engine_ctf_variant;
  m_game_engine: e_game_engine;
  m_infection_variant?: c_game_engine_infection_variant;
  m_juggernaut_variant?: c_game_engine_juggernaut_variant;
  m_king_variant?: c_game_engine_king_variant;
  m_oddball_variant?: c_game_engine_oddball_variant;
  m_sandbox_variant?: c_game_engine_sandbox_variant;
  m_slayer_variant?: c_game_engine_slayer_variant;
  m_territories_variant?: c_game_engine_territories_variant;
  m_vip_variant?: c_game_engine_vip_variant;
}

const engine_storage = c.enum(e_game_engine, c.u32());
const k_base_variant_offset = 4;
const k_engine_variant_offset =
  k_base_variant_offset + c.sizeof(c_game_engine_base_variant);

/**
 * `c_game_variant` in its unpacked layout: a `u32` engine index, the base
 * variant, then the engine's own variant, in a fixed 0x264-byte slot.
 */
export class c_game_variant_field extends CFieldType<c_game_variant> {
  getSize(): number {
    return k_game_variant_size;
  }

  read(buffer: Uint8Array, offset: number, endian: c.Endian): c_game_variant {
    const m_game_engine = engine_storage.read(buffer, offset, endian);
    const variant: c_game_variant = {
      m_game_engine,
      m_base_variant: c_game_engine_base_variant.read(buffer, {
        offset: offset + k_base_variant_offset,
        endian,
      }),
    };

    if (m_game_engine !== e_game_engine.none) {
      const [key, schema] = k_game_engine_variants[m_game_engine];
      (variant as unknown as Record<string, unknown>)[key] = schema.read(
        buffer,
        { offset: offset + k_engine_variant_offset, endian }
      );
    }

    return variant;
  }

  write(
    buffer: Uint8Array,
    offset: number,
    value: c_game_variant,
    endian: c.Endian
  ): void {
    buffer.fill(0, offset, offset + k_game_variant_size);
    engine_storage.write(buffer, offset, value.m_game_engine, endian);
    c_game_engine_base_variant.writeInto(
      buffer,
      value.m_base_variant,
      offset + k_base_variant_offset,
      endian
    );

    if (value.m_game_engine === e_game_engine.none) {
      return;
    }

    const [key, schema] = k_game_engine_variants[value.m_game_engine];
    const engine_variant = (value as unknown as Record<string, unknown>)[key];
    if (engine_variant === undefined || engine_variant === null) {
      throw new Error(
        `c_game_variant: no ${key} to write for engine ${value.m_game_engine}`
      );
    }
    (schema as c.Struct).writeInto(
      buffer,
      engine_variant as never,
      offset + k_engine_variant_offset,
      endian
    );
  }
}

/** Field factory for {@link c_game_variant_field}. */
export const c_game_variant = () => new c_game_variant_field();
