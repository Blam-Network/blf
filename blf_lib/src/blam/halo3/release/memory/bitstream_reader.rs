use blf_lib::assert_ok;
use blf_lib::blam::common::math::real_math::{assert_valid_real_normal3d, global_up3d, k_pi, k_real_epsilon, quantize_normalized_vector3d, real_vector3d};
use blf_lib::blam::halo3::release::math::real_math::dequantize_unit_vector3d;
use blf_lib::io::bitstream::{c_bitstream_reader, c_bitstream_writer};
use blf_lib_derivable::result::BLFLibResult;

pub trait c_bitstream_reader_extensions<'a>  {
    fn bitstream_reader(&mut self) -> &mut c_bitstream_reader<'a>;

    fn read_axes(&mut self, forward: &mut real_vector3d, up: &mut real_vector3d) -> BLFLibResult {
        let reader = self.bitstream_reader();

        if reader.read_bool()? {
            up.clone_from(&global_up3d);
        }
        else {
            let quantized = reader.read_signed_integer(19)?;
            dequantize_unit_vector3d(quantized, up)?;
        }

        let forward_angle = reader.read_quantized_real(-k_pi, k_pi, 8, true, true)?;
        c_bitstream_reader::angle_to_axes_internal(up, forward_angle, forward)?;
        Ok(())
    }
}

impl<'a> c_bitstream_reader_extensions<'a> for &mut c_bitstream_reader<'a> {
    fn bitstream_reader(&mut self) -> &mut c_bitstream_reader<'a> {
        self
    }
}
