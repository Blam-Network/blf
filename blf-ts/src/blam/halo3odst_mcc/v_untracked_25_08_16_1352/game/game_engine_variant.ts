import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { BlfError } from "../../../../error";
import { AutoMap } from "../../../../helpers/automap";
import { c_game_engine_base_variant } from "./game_engine_default";
import { c_game_engine_survival_variant } from "./game_engine_survival";

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
  survival = 11,
}

export const k_game_variant_size = 0x400;

export class c_game_variant {
  @AutoMap(() => Number)
  m_game_engine: e_game_engine = e_game_engine.none;
  @AutoMap(() => c_game_engine_base_variant)
  m_base_variant = new c_game_engine_base_variant();
  @AutoMap(() => c_game_engine_survival_variant)
  m_survival_variant: c_game_engine_survival_variant | null = null;

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_game_engine, 4);

    if (this.m_game_engine === e_game_engine.survival) {
      if (!this.m_survival_variant) {
        throw new BlfError("Can't write - Survival variant is null");
      }
      this.m_survival_variant.encode(bitstream);
      return;
    }

    throw new BlfError(
      `Non-survival game engine ${this.m_game_engine} is not implemented in TS yet`
    );
  }

  decode(bitstream: c_bitstream_reader): void {
    this.m_game_engine = bitstream.read_integer(
      "game-engine",
      4
    ) as e_game_engine;
    this.m_survival_variant = null;

    if (this.m_game_engine === e_game_engine.survival) {
      const survival = new c_game_engine_survival_variant();
      survival.decode(bitstream);
      this.m_base_variant = survival.m_base_variant;
      this.m_survival_variant = survival;
      return;
    }

    throw new BlfError(
      `Non-survival game engine ${this.m_game_engine} is not implemented in TS yet`
    );
  }
}
