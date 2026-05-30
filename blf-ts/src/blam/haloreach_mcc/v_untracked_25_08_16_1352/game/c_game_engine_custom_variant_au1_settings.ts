import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { AutoMap } from "../../../../helpers/automap";
export class c_game_engine_custom_variant_au1_settings {
  @AutoMap(() => Number)
  m_flags = 0;
  @AutoMap(() => Number)
  m_precision_bloom = 0;
  @AutoMap(() => Number)
  m_active_camo_energy_curve_min = 0;
  @AutoMap(() => Number)
  m_active_camo_energy_curve_max = 0;
  @AutoMap(() => Number)
  m_armor_lock_damage_drain = 0;
  @AutoMap(() => Number)
  m_armor_lock_damage_drain_limit = 0;
  @AutoMap(() => Number)
  m_magnum_damage = 0;
  @AutoMap(() => Number)
  m_magnum_fire_delay = 0;
  decode(bitstream: c_bitstream_reader): void {
    this.m_flags = bitstream.read_integer("flags", 32);
    this.m_precision_bloom = bitstream.read_quantized_real(
      0,
      2,
      8,
      false,
      true
    );
    this.m_active_camo_energy_curve_min = bitstream.read_quantized_real(
      0,
      2,
      8,
      false,
      true
    );
    this.m_active_camo_energy_curve_max = bitstream.read_quantized_real(
      0,
      2,
      8,
      false,
      true
    );
    this.m_armor_lock_damage_drain = bitstream.read_quantized_real(
      0,
      2,
      8,
      false,
      true
    );
    this.m_armor_lock_damage_drain_limit = bitstream.read_quantized_real(
      0,
      2,
      8,
      false,
      true
    );
    this.m_magnum_damage = bitstream.read_quantized_real(0, 10, 8, false, true);
    this.m_magnum_fire_delay = bitstream.read_quantized_real(
      0,
      10,
      8,
      false,
      true
    );
  }
  encode(bitstream: c_bitstream_writer): void {
    bitstream.write_integer(this.m_flags, 32);
    bitstream.write_quantized_real(
      this.m_precision_bloom,
      0,
      2,
      8,
      false,
      true
    );
    bitstream.write_quantized_real(
      this.m_active_camo_energy_curve_min,
      0,
      2,
      8,
      false,
      true
    );
    bitstream.write_quantized_real(
      this.m_active_camo_energy_curve_max,
      0,
      2,
      8,
      false,
      true
    );
    bitstream.write_quantized_real(
      this.m_armor_lock_damage_drain,
      0,
      2,
      8,
      false,
      true
    );
    bitstream.write_quantized_real(
      this.m_armor_lock_damage_drain_limit,
      0,
      2,
      8,
      false,
      true
    );
    bitstream.write_quantized_real(this.m_magnum_damage, 0, 10, 8, false, true);
    bitstream.write_quantized_real(
      this.m_magnum_fire_delay,
      0,
      10,
      8,
      false,
      true
    );
  }
}
