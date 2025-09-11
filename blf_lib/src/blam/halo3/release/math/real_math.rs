use blf_lib::blam::common::math::integer_math::int32_point3d;
use blf_lib::blam::common::math::real_math::{assert_valid_real_normal3d, real_point3d, real_rectangle3d};
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::common::math::real_math::{normalize3d, real_vector3d};

pub fn quantize_real(value: impl Into<f32>, min_value: impl Into<f32>, max_value: impl Into<f32>, size_in_bits: usize, exact_midpoint: bool) -> i32 {
    let value = value.into();
    let min_value = min_value.into();
    let max_value = max_value.into();

    assert!(size_in_bits > 0, "size_in_bits>0");
    assert!(max_value > min_value, "max_value>min_value");
    assert!(!exact_midpoint || size_in_bits > 1, "!exact_midpoint || size_in_bits>1");
    assert!(value >= min_value, "value>=min_value");
    assert!(value <= max_value, "value<=max_value");

    let mut step_count = (1 << size_in_bits) - 1; // Maximum index based on size in bits
    if exact_midpoint {
        step_count -= step_count % 2; // Adjust for even distribution if exact midpoint is required
    }
    assert!(step_count > 0, "step_count>0");

    let step = (max_value - min_value) / step_count as f32;
    assert!(step > 0.0, "step>0.0f");

    let normalized_value = (value - min_value) / step;

    let sign = if normalized_value < 0.0 { -1.0 } else { 1.0 };
    let quantized_value = (sign * 0.5 + normalized_value) as i32;

    assert!(quantized_value >= 0 && quantized_value <= step_count, "quantized_value>=0 && quantized_value<=step_count");

    quantized_value
}

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

pub fn quantize_real_point3d(
    point: &real_point3d,
    bounds: &real_rectangle3d,
    axis_encoding_bit_count: usize,
    quantized_point: &mut int32_point3d
) {
    assert!(axis_encoding_bit_count <= 32, "axis_encoding_bit_count<=SIZEOF_BITS(point->n[0])");

    let bounded_x =
        if point.x > bounds.x.upper { bounds.x.upper }
        else if point.x < bounds.x.lower { bounds.x.lower }
        else { point.x };
    let bounded_y =
        if point.y > bounds.y.upper { bounds.y.upper }
        else if point.y < bounds.y.lower { bounds.y.lower }
        else { point.y };
    let bounded_z =
        if point.z > bounds.z.upper { bounds.z.upper }
        else if point.z < bounds.z.lower { bounds.z.lower }
        else { point.z };

    quantized_point.x = quantize_real(bounded_x, bounds.x.lower, bounds.x.upper, axis_encoding_bit_count, false);
    quantized_point.y = quantize_real(bounded_y, bounds.y.lower, bounds.y.upper, axis_encoding_bit_count, false);
    quantized_point.z = quantize_real(bounded_z, bounds.z.lower, bounds.z.upper, axis_encoding_bit_count, false);
}

pub fn quantize_normalized_vector3d(vector: &real_vector3d) -> i32 {
    assert!(assert_valid_real_normal3d(vector));

    let mut axis_code: u8;
    let u: f32;
    let v: f32;
    let negative: bool;
    let positive_code: u8;

    let i_abs = vector.i.abs();
    let j_abs = vector.j.abs();
    let k_abs = vector.k.abs();
    let i = vector.i;
    let j = vector.j;
    let k = vector.k;

    if i_abs <= j_abs && j_abs > k_abs {
        axis_code = 4;
        negative = j <= 0.0;
        positive_code = 1;
        u = i / j_abs;
        v = k / j_abs;
    } else if i_abs > j_abs && i_abs > k_abs {
        positive_code = 0;
        axis_code = 3;
        negative = i <= 0.0;
        u = j / i_abs;
        v = k / i_abs;
    } else {
        negative = k <= 0.0;
        positive_code = 2;
        axis_code = 5;
        v = j / k_abs;
        u = i / k_abs;
    }

    if !negative {
        axis_code = positive_code;
    }

    assert!((-1.0..=1.0).contains(&u));
    assert!((-1.0..=1.0).contains(&v));

    let quantized_u = quantize_real(u, -1.0, 1.0, 8, true);
    let quantized_v = quantize_real(v, -1.0, 1.0, 8, true);

    axis_code as i32 | (quantized_u << 3) | (quantized_v << 11)
}