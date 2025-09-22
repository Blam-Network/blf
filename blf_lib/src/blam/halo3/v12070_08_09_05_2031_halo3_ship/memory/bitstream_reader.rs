use blf_lib::assert_ok;
use blf_lib::blam::common::math::real_math::{global_up3d, k_pi, real_vector3d};
use blf_lib::blam::halo3::v12070_08_09_05_2031_halo3_ship::math::real_math::{dequantize_real, dequantize_unit_vector3d};
use blf_lib::io::bitstream::c_bitstream_reader;
use blf_lib::types::numbers::Float32;
use blf_lib_derivable::result::BLFLibResult;

pub trait c_bitstream_reader_extensions<'a>  {
    fn bitstream_reader(&mut self) -> &mut c_bitstream_reader<'a>;

    fn read_quantized_real(&mut self, min_value: f32, max_value: f32, size_in_bits: usize, exact_midpoint: bool) -> BLFLibResult<Float32> {
        let reader = self.bitstream_reader();

        assert_ok!(reader.reading());
        let value: i32 = reader.read_unnamed_integer(size_in_bits)?;
        Ok(Float32(dequantize_real(value, min_value, max_value, size_in_bits, exact_midpoint)))
    }

    fn read_axes(&mut self, forward: &mut real_vector3d, up: &mut real_vector3d) -> BLFLibResult {
        let mut reader = self.bitstream_reader();

        if reader.read_bool("up-is-global-up3d")? {
            up.clone_from(&global_up3d);
        }
        else {
            let quantized = reader.read_integer("up-quantization", 19)?;
            dequantize_unit_vector3d(quantized, up)?;
        }

        let forward_angle = reader.read_quantized_real(-k_pi, k_pi, 8, true)?;
        c_bitstream_reader::angle_to_axes_internal(up, forward_angle, forward)?;
        Ok(())
    }
}

impl<'a> c_bitstream_reader_extensions<'a> for &mut c_bitstream_reader<'a> {
    fn bitstream_reader(&mut self) -> &mut c_bitstream_reader<'a> {
        self
    }
}
