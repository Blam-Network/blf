use blf_lib::assert_ok;
use blf_lib::blam::common::math::real_math::{assert_valid_real_normal3d, global_up3d, k_pi, k_real_epsilon, quantize_normalized_vector3d, real_vector3d};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::math::real_math::dequantize_unit_vector3d;
use blf_lib::io::bitstream::c_bitstream_writer;
use blf_lib_derivable::result::BLFLibResult;

pub trait c_bitstream_writer_extensions {
    fn bitstream_writer(&mut self) -> &mut c_bitstream_writer;

    fn write_axes<const forward_bits: usize, const up_bits: usize>(&mut self, forward: &real_vector3d, up: &real_vector3d) -> BLFLibResult {
        let mut writer = self.bitstream_writer();
        assert_ok!(assert_valid_real_normal3d(up));
        assert_ok!(assert_valid_real_normal3d(forward));

        let mut dequantized_up: real_vector3d = real_vector3d::default();

        let i_abs = (up.i - global_up3d.i).abs();
        let j_abs = (up.j - global_up3d.j).abs();
        let k_abs = (up.k - global_up3d.k).abs();

        if i_abs > k_real_epsilon || j_abs > k_real_epsilon || k_abs > k_real_epsilon {
            let quantized_up = quantize_normalized_vector3d(up);
            writer.write_bool(false)?; // up-is-global-up3d
            writer.write_integer(quantized_up as u32, up_bits)?;
            dequantize_unit_vector3d(quantized_up, &mut dequantized_up, 20)?;
        } else {
            writer.write_bool(true)?; // up-is-global-up3d
            dequantized_up = global_up3d;
        }

        let forward_angle = c_bitstream_writer::axes_to_angle_internal(forward, &dequantized_up)?;
        writer.write_quantized_real(forward_angle, -k_pi, k_pi, forward_bits, false, false)?;
        Ok(())
    }
}

impl c_bitstream_writer_extensions for &mut c_bitstream_writer {
    fn bitstream_writer(&mut self) -> &mut c_bitstream_writer {
        self
    }
}