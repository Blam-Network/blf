import {
  dequantize_real,
  global_up3d,
  normalize3d,
  quantize_real,
  type real_vector3d,
} from "../../../../bitstream/math";
import type {
  real_point3d,
  real_rectangle3d,
} from "../../../common/math/real_types";
import { get_unit_vector_encoding_constants } from "../../../common/math/unit_vector_quantization";

export function quantize_real_point3d_per_axis(
  position: real_point3d,
  bounds: real_rectangle3d,
  bits: { x: number; y: number; z: number },
  quantized: { x: number; y: number; z: number }
): void {
  quantized.x = quantize_real(
    position.x,
    bounds.x.lower,
    bounds.x.upper,
    1 << bits.x,
    false,
    false
  );
  quantized.y = quantize_real(
    position.y,
    bounds.y.lower,
    bounds.y.upper,
    1 << bits.y,
    false,
    false
  );
  quantized.z = quantize_real(
    position.z,
    bounds.z.lower,
    bounds.z.upper,
    1 << bits.z,
    false,
    false
  );
}

export function dequantize_real_point3d_per_axis(
  quantized: { x: number; y: number; z: number },
  bounds: real_rectangle3d,
  bits: { x: number; y: number; z: number },
  position: real_point3d
): void {
  position.x = dequantize_real(
    quantized.x,
    bounds.x.lower,
    bounds.x.upper,
    1 << bits.x,
    false,
    false
  );
  position.y = dequantize_real(
    quantized.y,
    bounds.y.lower,
    bounds.y.upper,
    1 << bits.y,
    false,
    false
  );
  position.z = dequantize_real(
    quantized.z,
    bounds.z.lower,
    bounds.z.upper,
    1 << bits.z,
    false,
    false
  );
}

export function quantize_unit_vector3d_fast(
  vector: real_vector3d,
  bit_count: number
): number {
  const constants = get_unit_vector_encoding_constants(bit_count);

  const x = vector.i;
  const y = vector.j;
  const z = vector.k;

  const ax = Math.abs(x);
  const ay = Math.abs(y);
  const az = Math.abs(z);

  let face: number;
  let u: number;
  let w: number;

  if (ax > ay) {
    if (ax > az) {
      face = x <= 0 ? 3 : 0;
      u = y / ax;
      w = z / ax;
    } else {
      face = z <= 0 ? 5 : 2;
      u = x / az;
      w = y / az;
    }
  } else if (ay > az) {
    face = y <= 0 ? 4 : 1;
    u = x / ay;
    w = z / ay;
  } else {
    face = z <= 0 ? 5 : 2;
    u = x / az;
    w = y / az;
  }

  const qu = quantize_real(
    u,
    -1,
    1,
    constants.quantized_value_count,
    true,
    false
  );
  const qw = quantize_real(
    w,
    -1,
    1,
    constants.quantized_value_count,
    true,
    false
  );

  return (
    qw +
    face * constants.actual_per_axis_max_count +
    qu * constants.quantized_value_count
  );
}

export function dequantize_unit_vector3d(
  value: number,
  vector: real_vector3d,
  bit_count: number
): void {
  const encoding_constants = get_unit_vector_encoding_constants(bit_count);
  const actual_per_axis_max_count =
    encoding_constants.actual_per_axis_max_count;
  const quantized_value_count = encoding_constants.quantized_value_count;

  const face = Math.max(Math.floor(value / actual_per_axis_max_count), 0);

  const rem =
    ((value % actual_per_axis_max_count) + actual_per_axis_max_count) %
    actual_per_axis_max_count;
  const qu = Math.floor(rem / quantized_value_count);
  const qw = rem % quantized_value_count;

  if (
    qu < 0 ||
    qu >= actual_per_axis_max_count / quantized_value_count ||
    qw < 0 ||
    qw >= quantized_value_count
  ) {
    vector.i = global_up3d.i;
    vector.j = global_up3d.j;
    vector.k = global_up3d.k;
    throw new Error(
      `dequantize_unit_vector3d: bad quant indices qu=${qu} qw=${qw} face=${face}`
    );
  }

  const u = dequantize_real(qu, -1, 1, quantized_value_count, true, false);
  const w = dequantize_real(qw, -1, 1, quantized_value_count, true, false);

  switch (face) {
    case 0:
      vector.i = 1;
      vector.j = u;
      vector.k = w;
      break;
    case 1:
      vector.i = u;
      vector.j = 1;
      vector.k = w;
      break;
    case 2:
      vector.i = u;
      vector.j = w;
      vector.k = 1;
      break;
    case 3:
      vector.i = -1;
      vector.j = u;
      vector.k = w;
      break;
    case 4:
      vector.i = u;
      vector.j = -1;
      vector.k = w;
      break;
    case 5:
      vector.i = u;
      vector.j = w;
      vector.k = -1;
      break;
    default:
      vector.i = global_up3d.i;
      vector.j = global_up3d.j;
      vector.k = global_up3d.k;
      throw new Error(`dequantize_unit_vector3d: bad face value ${face}`);
  }

  normalize3d(vector);
}
