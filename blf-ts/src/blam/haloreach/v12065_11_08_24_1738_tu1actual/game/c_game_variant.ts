import {
  c_bitstream_reader,
  c_bitstream_writer,
  e_bitstream_byte_order,
} from "../../../../bitstream";
import { BlfError } from "../../../../error";
import type { s_content_item_metadata } from "../saved_games/saved_game_files";
import { c_game_engine_base_variant } from "./c_game_engine_default";
import { c_game_engine_custom_variant } from "./c_game_engine_custom_variant";
import { c_game_engine_survival_variant } from "./c_game_engine_survival_variant";

/** Matches `e_game_mode` in blf_lib. */
export enum e_game_mode {
  sandbox = 1,
  custom = 2,
  campaign = 3,
  survival = 4,
}

/**
 * Halo Reach game variant gametype bitstream body (after the `mpvr` hash header).
 * Mirrors `c_game_variant` in blf_lib — decode-only.
 */
export class c_game_variant {
  m_game_engine: e_game_mode = e_game_mode.custom;
  m_campaign_variant?: c_game_engine_base_variant;
  m_custom_variant?: c_game_engine_custom_variant;
  m_survival_variant?: c_game_engine_survival_variant;

  decode(bitstream: c_bitstream_reader): void {
    this.m_game_engine = bitstream.read_enum("game-engine", 4, e_game_mode);

    switch (this.m_game_engine) {
      case e_game_mode.sandbox:
        throw new BlfError(
          "Decoding forge (sandbox) game variants is not supported yet",
        );
      case e_game_mode.custom: {
        const custom = new c_game_engine_custom_variant();
        custom.decode(bitstream);
        this.m_custom_variant = custom;
        break;
      }
      case e_game_mode.campaign: {
        const campaign = new c_game_engine_base_variant();
        campaign.decode(bitstream);
        this.m_campaign_variant = campaign;
        break;
      }
      case e_game_mode.survival: {
        const survival = new c_game_engine_survival_variant();
        survival.decode(bitstream);
        this.m_survival_variant = survival;
        break;
      }
      default:
        throw new BlfError(`Unrecognized game engine ${this.m_game_engine}`);
    }
  }

  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_enum(this.m_game_engine, 4);
    switch (this.m_game_engine) {
      case e_game_mode.sandbox:
        throw new BlfError("Encoding forge variants is currently unsupported");
      case e_game_mode.custom:
        if (!this.m_custom_variant) {
          throw new BlfError("m_custom_variant does not exist");
        }
        this.m_custom_variant.encode(bitstream);
        break;
      case e_game_mode.campaign:
        if (!this.m_campaign_variant) {
          throw new BlfError("m_campaign_variant does not exist");
        }
        this.m_campaign_variant.encode(bitstream);
        break;
      case e_game_mode.survival:
        if (!this.m_survival_variant) {
          throw new BlfError("m_survival_variant does not exist");
        }
        this.m_survival_variant.encode(bitstream);
        break;
      default:
        throw new BlfError(`Unrecognized game engine ${this.m_game_engine}`);
    }
  }

  /** Decode gametype bytes from a standalone buffer (convenience wrapper). */
  static decode_bytes(gametype_bytes: Uint8Array): c_game_variant {
    const bitstream = c_bitstream_reader.new(
      gametype_bytes,
      e_bitstream_byte_order._bitstream_byte_order_big_endian,
    );
    bitstream.begin_reading();
    const variant = new c_game_variant();
    variant.decode(bitstream);
    bitstream.finish_reading();
    return variant;
  }

  get_metadata(): s_content_item_metadata {
    switch (this.m_game_engine) {
      case e_game_mode.sandbox:
        throw new BlfError("Forge variants are currently unsupported");
      case e_game_mode.custom:
        if (!this.m_custom_variant) {
          throw new BlfError("m_custom_variant does not exist");
        }
        return this.m_custom_variant.m_base_variant.m_metadata;
      case e_game_mode.campaign:
        if (!this.m_campaign_variant) {
          throw new BlfError("m_campaign_variant does not exist");
        }
        return this.m_campaign_variant.m_metadata;
      case e_game_mode.survival:
        if (!this.m_survival_variant) {
          throw new BlfError("m_survival_variant does not exist");
        }
        return this.m_survival_variant.m_base_variant.m_metadata;
      default:
        throw new BlfError(`Unrecognized game engine ${this.m_game_engine}`);
    }
  }
}
