import type {
  c_bitstream_reader,
  c_bitstream_writer,
} from "../../../../bitstream";
import { BlfError } from "../../../../error";
import type {
  real_point3d,
  real_rectangle3d,
} from "../../../common/math/real_types";
import { point_in_rectangle3d } from "../../../common/math/real_types";
import { adjust_axis_encoding_bit_count_to_match_error_goals } from "../../../common/simulation/simulation_encoding";
import {
  dequantize_real_point3d_per_axis,
  quantize_real_point3d_per_axis,
} from "../math/real_math";

export function simulation_read_position(
  bitstream: c_bitstream_reader,
  position: real_point3d,
  axis_encoding_size_in_bits: number,
  world_bounds: real_rectangle3d
): void {
  if (bitstream.read_bool("point-in-initial-bounds")) {
    const per_axis_bit_counts = { x: 0, y: 0, z: 0 };
    adjust_axis_encoding_bit_count_to_match_error_goals(
      axis_encoding_size_in_bits,
      world_bounds,
      26,
      per_axis_bit_counts
    );

    const quantized_point = { x: 0, y: 0, z: 0 };
    bitstream.read_point3d_efficient(quantized_point, per_axis_bit_counts);

    dequantize_real_point3d_per_axis(
      quantized_point,
      world_bounds,
      per_axis_bit_counts,
      position
    );
  } else {
    throw new BlfError(
      "Tried to read a position outside of world bounds! Fallback behaviour is only supported in-engine."
    );
  }
}

export function simulation_write_position(
  bitstream: c_bitstream_writer,
  position: real_point3d,
  bits: number,
  world_bounds: real_rectangle3d
): void {
  const per_axis_bit_counts = { x: bits, y: bits, z: bits };
  const quantized_point = { x: 0, y: 0, z: 0 };

  const in_bounds = point_in_rectangle3d(position, world_bounds);
  bitstream.write_bool(in_bounds);

  if (!in_bounds) {
    throw new BlfError(
      `Tried to write a position (${position.x}, ${position.y}, ${position.z}) outside of world bounds! Fallback behaviour is only supported in-engine.`
    );
  }

  adjust_axis_encoding_bit_count_to_match_error_goals(
    bits,
    world_bounds,
    26,
    per_axis_bit_counts
  );

  quantize_real_point3d_per_axis(
    position,
    world_bounds,
    per_axis_bit_counts,
    quantized_point
  );

  bitstream.write_point3d_efficient(quantized_point, per_axis_bit_counts);
}
