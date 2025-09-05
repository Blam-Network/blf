use std::cmp::min;
use num_traits::real::Real;
use blf_lib::blam::common::math::integer_math::int32_point3d;
use blf_lib::blam::common::math::real_math::{dequantize_real_point3d, dequantize_real_point3d_per_axis, real_point3d, real_rectangle3d};
use blf_lib::io::bitstream::c_bitstream_reader;
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::io::bitstream::c_bitstream_writer;
use crate::blam::common::math::real_math::{point_in_rectangle3d, quantize_real_point3d, quantize_real_point3d_per_axis};

pub fn simulation_write_quantized_position(
    bitstream: &mut c_bitstream_writer,
    position: &real_point3d,
    bits: usize,
    a4: bool,
    world_bounds: &real_rectangle3d
) -> BLFLibResult {

    let mut quantized_point = int32_point3d::default();
    quantize_real_point3d(position, world_bounds, bits, &mut quantized_point);

    if a4 {
        unimplemented!()
    }

    bitstream.write_point3d(&quantized_point, bits)?;

    Ok(())
}

const k_world_units_to_inches: f32 = 10.0f32 * 12.0f32;
const k_inches_to_world_units: f32 = 1.0f32 / k_world_units_to_inches;

pub fn adjust_axis_encoding_bit_count_to_match_error_goals(
    bit_count: usize,
    bounds: &real_rectangle3d,
    max_bit_count: usize,
    per_axis_bit_counts: &mut int32_point3d,
) {
    // compute extents
    let mut dimensions = real_point3d::default();
    dimensions.x = bounds.x.upper - bounds.x.lower;
    dimensions.y = bounds.y.upper - bounds.y.lower;
    dimensions.z =  bounds.z.upper - bounds.z.lower;

    per_axis_bit_counts.x = bit_count as i32;
    per_axis_bit_counts.y = bit_count as i32;
    per_axis_bit_counts.z = bit_count as i32;

    let max_error: f32 = if bit_count <= 16 {
        k_inches_to_world_units * ((1 << (16 - bit_count)) as f32)
    } else {
        k_inches_to_world_units / ((1 << (bit_count - 16)) as f32)
    };

    // X
    let max_value = min((dimensions.x / (max_error * 2.0f32)).ceil() as usize, 0x800000);
    let required_bits = (f32::ln(max_value as f32) / f32::ln(2f32)).ceil() as usize;
    per_axis_bit_counts.x = min(required_bits, max_bit_count) as i32;

    // Y
    let max_value = min((dimensions.y / (max_error * 2.0f32)).ceil() as usize, 0x800000);
    let required_bits = (f32::ln(max_value as f32) / f32::ln(2f32)).ceil() as usize;
    per_axis_bit_counts.y = min(required_bits, max_bit_count) as i32;

    // Z
    let max_value = min((dimensions.z / (max_error * 2.0f32)).ceil() as usize, 0x800000);
    let required_bits = (f32::ln(max_value as f32) / f32::ln(2f32)).ceil() as usize;
    per_axis_bit_counts.z = min(required_bits, max_bit_count) as i32;
}


pub fn simulation_read_position(
    bitstream: &mut c_bitstream_reader,
    position: &mut real_point3d,
    axis_encoding_size_in_bits: usize,
    exact_midpoints: bool,
    exact_endpoints: bool,
    world_bounds: &real_rectangle3d
) -> BLFLibResult {
    if bitstream.read_bool()? { // point-in-initial-bounds
        let mut per_axis_bit_counts = int32_point3d::default();
        adjust_axis_encoding_bit_count_to_match_error_goals(axis_encoding_size_in_bits, world_bounds, 26, &mut per_axis_bit_counts);

        let mut quantized_point = int32_point3d::default();
        bitstream.read_point3d_efficient(&mut quantized_point, per_axis_bit_counts)?;

        Ok(dequantize_real_point3d_per_axis(&quantized_point, world_bounds, &per_axis_bit_counts, position, exact_midpoints, exact_endpoints))
    }
    else {
        // This branch requires runtime game BSP data, we can't perform it.
        Err(BLFLibError::from("Tried to read a position outside of world bounds! Fallback behaviour is only supported in-engine."))
    }
}

pub fn simulation_write_position(
    bitstream: &mut c_bitstream_writer,
    position: &real_point3d,
    bits: usize,
    world_bounds: &real_rectangle3d,
) -> BLFLibResult<()> {
    let mut per_axis_bit_counts = int32_point3d { x: bits as i32, y: bits as i32, z: bits as i32 };
    let mut quantized_point = int32_point3d::default();

    let in_bounds = point_in_rectangle3d(position, world_bounds);
    bitstream.write_bool(in_bounds)?;

    if !in_bounds {
        // This branch requires runtime game BSP data, we can't perform it.
        return Err(BLFLibError::from(
            format!("Tried to write a position {position:?} outside of world bounds {world_bounds:?}! Fallback behaviour is only supported in-engine.")
        ))
    }

    adjust_axis_encoding_bit_count_to_match_error_goals(bits, world_bounds, 26, &mut per_axis_bit_counts);

    quantize_real_point3d_per_axis(
        position,
        world_bounds,
        &per_axis_bit_counts,
        &mut quantized_point,
    );

    bitstream.write_point3d_efficient(
        &quantized_point,
        &per_axis_bit_counts,
    )?;

    Ok(())
}

pub fn simulation_read_quantized_position(
    bitstream: &mut c_bitstream_reader,
    position: &mut real_point3d,
    axis_encoding_size_in_bits: usize,
    world_bounds: &real_rectangle3d
) -> BLFLibResult {
    let mut point = int32_point3d::default();
    bitstream.read_point3d(&mut point, axis_encoding_size_in_bits)?;
    dequantize_real_point3d(&point, world_bounds, axis_encoding_size_in_bits, position);

    Ok(())
}