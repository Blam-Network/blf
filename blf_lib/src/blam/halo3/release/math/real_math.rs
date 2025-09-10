use blf_lib::blam::common::math::integer_math::int32_point3d;
use blf_lib::blam::common::math::real_math::{real_point3d, real_rectangle3d};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::common::math::real_math::{normalize3d, real_vector3d};

pub fn dequantize_real(quantized: i32, min_value: impl Into<f32>, max_value: impl Into<f32>, size_in_bits: usize, exact_midpoint: bool) -> f32 {
    let min_value = min_value.into();
    let max_value = max_value.into();

    assert!(size_in_bits > 0, "size_in_bits>0");
    assert!(max_value > min_value, "max_value>min_value");
    assert!(!exact_midpoint || size_in_bits > 1, "!exact_midpoint || size_in_bits>1");

    let mut step_count = (1 << size_in_bits) - 1;
    if exact_midpoint {
        step_count -= step_count % 2;
    }
    assert!(step_count > 0, "step_count>0");

    let dequantized: f32;

    if quantized != 0 {
        if quantized < step_count {
            dequantized = (((step_count - quantized) as f32 * min_value) + (quantized as f32 * max_value)) / step_count as f32;
        }
        else {
            dequantized = max_value;
        }
    } else {
        dequantized = min_value;
    }

    if exact_midpoint && 2 * quantized == step_count {
        assert!(dequantized == (min_value + max_value) / 2.0, "value==(max_value+min_value)/2");
    }

    dequantized
}


pub fn dequantize_unit_vector3d(value: i32, vector: &mut real_vector3d) -> BLFLibResult {
    let face = value & 7;
    let x = dequantize_real((value >> 3) as u8 as i32, -1.0, 1.0, 8, false);
    let y = dequantize_real((value >> 11) as u8 as i32, -1.0, 1.0, 8, false);

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

pub fn dequantize_real_point3d(
    point: &int32_point3d,
    bounds: &real_rectangle3d,
    axis_encoding_bit_count: usize,
    dequantized_point: &mut real_point3d
) {
    // I think there's a missing assert here.
    dequantized_point.x.0 = dequantize_real(point.x, bounds.x.lower, bounds.x.upper, axis_encoding_bit_count, false);
    dequantized_point.y.0 = dequantize_real(point.y, bounds.y.lower, bounds.y.upper, axis_encoding_bit_count, false);
    dequantized_point.z.0 = dequantize_real(point.z, bounds.z.lower, bounds.z.upper, axis_encoding_bit_count, false);
}