use blf_lib::blam::common::math::real_math::{global_up3d, k_pi, real_vector3d};
use blf_lib::blam::haloreach::v12065_11_08_24_1738_tu1actual::math::real_math::dequantize_unit_vector3d;
use blf_lib::io::bitstream::c_bitstream_reader;
use blf_lib_derivable::result::BLFLibResult;

pub trait c_bitstream_reader_extensions<'a> {
    fn bitstream_reader(&mut self) -> &mut c_bitstream_reader<'a>;

    fn read_axes<const forward_bits: usize, const up_bits: usize>(&mut self, forward: &mut real_vector3d, up: &mut real_vector3d) -> BLFLibResult {
        let reader = self.bitstream_reader();

        if reader.read_bool("up-is-global-up3d")? {
            up.clone_from(&global_up3d);
        }
        else {
            let quantized = reader.read_integer("up-quantization", up_bits)?;
            dequantize_unit_vector3d(quantized, up, up_bits)?;
        }

        let forward_angle = reader.read_quantized_real(-k_pi, k_pi, forward_bits, false, false)?;
        c_bitstream_reader::angle_to_axes_internal(up, forward_angle, forward)?;
        Ok(())
    }
}

impl<'a> c_bitstream_reader_extensions<'a> for c_bitstream_reader<'a> {
    fn bitstream_reader(&mut self) -> &mut c_bitstream_reader<'a> {
        self
    }
}