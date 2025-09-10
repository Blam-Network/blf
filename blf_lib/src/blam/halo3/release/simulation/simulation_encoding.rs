use blf_lib::blam::common::math::integer_math::int32_point3d;
use blf_lib::blam::common::math::real_math::{real_point3d, real_rectangle3d};
use blf_lib::blam::halo3::release::math::real_math::dequantize_real_point3d;
use blf_lib::io::bitstream::c_bitstream_reader;
use blf_lib_derivable::result::BLFLibResult;


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