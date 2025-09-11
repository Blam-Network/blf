use std::cmp::min;
use num_traits::real::Real;
use blf_lib::blam::common::math::integer_math::int32_point3d;
use blf_lib::blam::common::math::real_math::{real_point3d, real_rectangle3d};
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};
use crate::io::bitstream::c_bitstream_writer;
use crate::blam::common::math::real_math::point_in_rectangle3d;

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