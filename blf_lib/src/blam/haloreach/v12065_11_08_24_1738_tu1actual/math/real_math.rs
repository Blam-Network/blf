use blf_lib::blam::common::math::unit_vector_quanitzation::get_unit_vector_encoding_constants;
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::common::math::real_math::{dequantize_real, normalize3d, real_vector3d};

pub fn dequantize_unit_vector3d(value: i32, vector: &mut real_vector3d, bit_count: usize) -> BLFLibResult {
    let encoding_constants = get_unit_vector_encoding_constants(bit_count)?;

    let face = (value as f32 / encoding_constants.actual_per_axis_max_count as f32) as usize;
    let x = (value % encoding_constants.actual_per_axis_max_count as i32) as f32 / encoding_constants.quantized_value_count as f32;
    let y = (value % encoding_constants.actual_per_axis_max_count as i32) as f32 % encoding_constants.quantized_value_count as f32;

    let x = dequantize_real(x as i32, -1.0, 1.0, bit_count, true, false);
    let y = dequantize_real(y as i32, -1.0, 1.0, bit_count, true, false);

    match face {
        0 => {
            vector.i.0 = 1.0;
            vector.j.0 = x;
            vector.k.0 = y;
        }
        1 => {
            vector.i.0 = x;
            vector.j.0 = 1.0;
            vector.k.0 = y;
        }
        2 => {
            vector.i.0 = x;
            vector.j.0 = y;
            vector.k.0 = 1.0;
        }
        3 => {
            vector.i.0 = -1.0;
            vector.j.0 = x;
            vector.k.0 = y;
        }
        4 => {
            vector.i.0 = x;
            vector.j.0 = -1.0;
            vector.k.0 = y;
        }
        5 => {
            vector.i.0 = x;
            vector.j.0 = y;
            vector.k.0 = -1.0;
        }
        _ => {
            return Err(format!("dequantize_unit_vector3d: bad face value {face} when reading unit vector").into());
        }
    }

    normalize3d(vector);

    Ok(())
}
