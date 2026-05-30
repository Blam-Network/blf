import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import {
  angle_to_axes_internal,
  assert_valid_real_normal3d,
  axes_to_angle_internal,
  global_up3d,
  k_pi,
  k_real_epsilon,
  type real_vector3d,
  real_vector3d_default,
} from "../../../../bitstream/math";
import {
  dequantize_unit_vector3d,
  quantize_unit_vector3d_fast,
} from "../math/real_math";

export function read_axes(
  bitstream: c_bitstream_reader,
  forward: real_vector3d,
  up: real_vector3d,
  forward_bits: number,
  up_bits: number
): void {
  if (bitstream.read_bool("up-is-global-up3d")) {
    up.i = global_up3d.i;
    up.j = global_up3d.j;
    up.k = global_up3d.k;
  } else {
    const quantized = bitstream.read_integer("up-quantization", up_bits);
    dequantize_unit_vector3d(quantized, up, up_bits);
  }

  const forward_angle = bitstream.read_quantized_real(
    -k_pi,
    k_pi,
    forward_bits,
    false,
    false
  );
  angle_to_axes_internal(up, forward_angle, forward);
}

export function write_axes(
  bitstream: c_bitstream_writer,
  forward: real_vector3d,
  up: real_vector3d,
  forward_bits: number,
  up_bits: number
): void {
  assert_valid_real_normal3d(up);
  assert_valid_real_normal3d(forward);

  const dequantized_up = real_vector3d_default();

  const i_abs = Math.abs(up.i - global_up3d.i);
  const j_abs = Math.abs(up.j - global_up3d.j);
  const k_abs = Math.abs(up.k - global_up3d.k);

  if (
    i_abs >= k_real_epsilon ||
    j_abs >= k_real_epsilon ||
    k_abs >= k_real_epsilon
  ) {
    const quantized_up = quantize_unit_vector3d_fast(up, up_bits);
    bitstream.write_bool(false);
    bitstream.write_integer(quantized_up, up_bits);
    dequantize_unit_vector3d(quantized_up, dequantized_up, up_bits);
  } else {
    bitstream.write_bool(true);
    dequantized_up.i = global_up3d.i;
    dequantized_up.j = global_up3d.j;
    dequantized_up.k = global_up3d.k;
  }

  const forward_angle = axes_to_angle_internal(forward, dequantized_up);
  bitstream.write_quantized_real(
    forward_angle,
    -k_pi,
    k_pi,
    forward_bits,
    false,
    false
  );
}
