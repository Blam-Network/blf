use num_traits::real::Real;
use blf_lib::blam::common::math::integer_math::int32_point3d;
use blf_lib::blam::common::math::real_math::{point_in_rectangle3d, real_point3d, real_rectangle3d};
use blf_lib::blam::common::simulation::simulation_encoding::adjust_axis_encoding_bit_count_to_match_error_goals;
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::math::real_math::{dequantize_real_point3d_per_axis, quantize_real_point3d_per_axis};
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::{BLFLibError, BLFLibResult};


pub fn simulation_read_position(
    bitstream: &mut c_bitstream_reader,
    position: &mut real_point3d,
    axis_encoding_size_in_bits: usize,
    exact_midpoints: bool,
    exact_endpoints: bool,
    world_bounds: &real_rectangle3d
) -> BLFLibResult {
    if bitstream.read_bool("point-in-initial-bounds")? {
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

