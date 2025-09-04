use blf_lib_derivable::result::BLFLibResult;
use crate::blam::common::math::real_math::{dequantize_real, normalize3d, real_vector3d};

pub fn dequantize_unit_vector3d(value: i32, vector: &mut real_vector3d) -> BLFLibResult {
    let face = value & 7;
    let x = dequantize_real((value >> 3) as u8 as i32, -1.0, 1.0, 8, false, true);
    let y = dequantize_real((value >> 11) as u8 as i32, -1.0, 1.0, 8, false, true);

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
