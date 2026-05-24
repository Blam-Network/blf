import { assert_ok } from "./errors";

export type real_vector3d = {
  i: number;
  j: number;
  k: number;
};

export const k_real_epsilon = 1e-6;
export const k_pi = Math.PI;

export const global_forward3d: real_vector3d = { i: 0, j: 0, k: 1 };
export const global_left3d: real_vector3d = { i: 1, j: 0, k: 0 };
export const global_up3d: real_vector3d = { i: 0, j: 1, k: 0 };

export function real_vector3d_default(): real_vector3d {
  return { i: 0, j: 0, k: 0 };
}

export function dot_product3d(a: real_vector3d, b: real_vector3d): number {
  return a.i * b.i + a.j * b.j + a.k * b.k;
}

export function cross_product3d(
  a: real_vector3d,
  b: real_vector3d,
  out: real_vector3d
): void {
  out.i = a.j * b.k - a.k * b.j;
  out.j = a.k * b.i - a.i * b.k;
  out.k = a.i * b.j - a.j * b.i;
}

export function magnitude3d(v: real_vector3d): number {
  return Math.sqrt(dot_product3d(v, v));
}

/** Matches blf_lib `quantize_real` for Reach bitstreams. */
export function quantize_real(
  value: number,
  min_value: number,
  max_value: number,
  quantized_value_count: number,
  exact_midpoint: boolean,
  exact_endpoints: boolean
): number {
  let adjusted_count = quantized_value_count;
  if (exact_midpoint) {
    adjusted_count -= 1;
  }

  if (exact_endpoints) {
    if (value === min_value) {
      return 0;
    }
    if (value === max_value) {
      return adjusted_count - 1;
    }
    const divisor = adjusted_count - 2;
    const step = (max_value - min_value) / divisor;
    let temp = Math.floor((value - min_value) / step) + 1;
    if (temp <= 1) {
      temp = 1;
    }
    return temp > adjusted_count - 2 ? adjusted_count - 2 : temp;
  }

  const step = (max_value - min_value) / adjusted_count;
  let result = Math.floor((value - min_value) / step);
  if (result > adjusted_count - 1) {
    result = adjusted_count - 1;
  }
  return result;
}

/** Matches blf_lib `dequantize_real` for Reach bitstreams. */
export function dequantize_real(
  quantized: number,
  min_value: number,
  max_value: number,
  quantized_value_count: number,
  exact_midpoints: boolean,
  exact_endpoints: boolean
): number {
  let value_count = quantized_value_count;
  if (exact_midpoints) {
    value_count -= 1;
  }

  if (exact_endpoints) {
    if (quantized === 0) {
      return min_value;
    }
    if (quantized === value_count - 1) {
      return max_value;
    }
    const denom = value_count - 2;
    const step = (max_value - min_value) / denom;
    const fquant = quantized - 1;
    return min_value + step * (fquant + 0.5);
  }

  const step = (max_value - min_value) / value_count;
  return min_value + step * (quantized + 0.5);
}

export function normalize3d(v: real_vector3d): number {
  const mag = magnitude3d(v);
  if (mag > k_real_epsilon) {
    v.i /= mag;
    v.j /= mag;
    v.k /= mag;
  }
  return mag;
}

export function arctangent(y: number, x: number): number {
  return Math.atan2(y, x);
}

export function assert_valid_real_normal3d(v: real_vector3d): boolean {
  const mag = magnitude3d(v);
  return Math.abs(mag - 1) <= k_real_epsilon;
}

export function valid_real_vector3d_axes2(
  a: real_vector3d,
  b: real_vector3d
): boolean {
  return Math.abs(dot_product3d(a, b)) <= k_real_epsilon;
}

export function valid_real_vector3d_axes3(
  a: real_vector3d,
  b: real_vector3d,
  c: real_vector3d
): boolean {
  return (
    valid_real_vector3d_axes2(a, b) &&
    valid_real_vector3d_axes2(a, c) &&
    valid_real_vector3d_axes2(b, c)
  );
}

export function rotate_vector_about_axis(
  vector: real_vector3d,
  axis: real_vector3d,
  sine: number,
  cosine: number
): void {
  const cross: real_vector3d = { i: 0, j: 0, k: 0 };
  cross_product3d(axis, vector, cross);
  const dot = dot_product3d(axis, vector);
  const one_minus_cosine = 1 - cosine;

  vector.i =
    vector.i * cosine + cross.i * sine + axis.i * dot * one_minus_cosine;
  vector.j =
    vector.j * cosine + cross.j * sine + axis.j * dot * one_minus_cosine;
  vector.k =
    vector.k * cosine + cross.k * sine + axis.k * dot * one_minus_cosine;
}

export function axes_compute_reference_internal(
  up: real_vector3d,
  forward_reference: real_vector3d,
  left_reference: real_vector3d
): void {
  assert_ok(assert_valid_real_normal3d(up));

  const v10 = Math.abs(dot_product3d(up, global_forward3d));
  const v9 = Math.abs(dot_product3d(up, global_left3d));

  if (v10 >= v9) {
    cross_product3d(global_left3d, up, forward_reference);
  } else {
    cross_product3d(up, global_forward3d, forward_reference);
  }

  const forward_magnitude = normalize3d(forward_reference);
  assert_ok(
    forward_magnitude > k_real_epsilon,
    "forward_magnitude>k_real_epsilon"
  );

  cross_product3d(up, forward_reference, left_reference);

  const left_magnitude = normalize3d(left_reference);
  assert_ok(left_magnitude > k_real_epsilon, "left_magnitude>k_real_epsilon");

  assert_ok(valid_real_vector3d_axes3(forward_reference, left_reference, up));
}

export function axes_to_angle_internal(
  forward: real_vector3d,
  up: real_vector3d
): number {
  const forward_reference = real_vector3d_default();
  const left_reference = real_vector3d_default();
  axes_compute_reference_internal(up, forward_reference, left_reference);
  return arctangent(
    dot_product3d(left_reference, forward),
    dot_product3d(forward_reference, forward)
  );
}

export function angle_to_axes_internal(
  up: real_vector3d,
  angle: number,
  forward: real_vector3d
): void {
  const forward_reference = real_vector3d_default();
  const left_reference = real_vector3d_default();
  axes_compute_reference_internal(up, forward_reference, left_reference);

  forward.i = forward_reference.i;
  forward.j = forward_reference.j;
  forward.k = forward_reference.k;

  let u: number;
  let v: number;

  if (angle === k_pi || angle === -k_pi) {
    u = 0;
    v = -1;
  } else {
    u = Math.sin(angle);
    v = Math.cos(angle);
  }

  rotate_vector_about_axis(forward, up, u, v);
  normalize3d(forward);

  assert_ok(valid_real_vector3d_axes2(forward, up));
}
