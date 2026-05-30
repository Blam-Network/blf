import type { real_rectangle3d } from "../math/real_types";

const k_world_units_to_inches = 10 * 12;
const k_inches_to_world_units = 1 / k_world_units_to_inches;

export function adjust_axis_encoding_bit_count_to_match_error_goals(
  bit_count: number,
  bounds: real_rectangle3d,
  max_bit_count: number,
  per_axis_bit_counts: { x: number; y: number; z: number }
): void {
  const dimensions = {
    x: bounds.x.upper - bounds.x.lower,
    y: bounds.y.upper - bounds.y.lower,
    z: bounds.z.upper - bounds.z.lower,
  };

  per_axis_bit_counts.x = bit_count;
  per_axis_bit_counts.y = bit_count;
  per_axis_bit_counts.z = bit_count;

  const max_error =
    bit_count <= 16
      ? k_inches_to_world_units * (1 << (16 - bit_count))
      : k_inches_to_world_units / (1 << (bit_count - 16));

  const adjust_axis = (dimension: number, axis: "x" | "y" | "z"): void => {
    const max_value = Math.min(
      Math.ceil(dimension / (max_error * 2)),
      0x800000
    );
    const required_bits = Math.ceil(Math.log2(max_value));
    per_axis_bit_counts[axis] = Math.min(required_bits, max_bit_count);
  };

  adjust_axis(dimensions.x, "x");
  adjust_axis(dimensions.y, "y");
  adjust_axis(dimensions.z, "z");
}
