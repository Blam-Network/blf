use blf_lib::blam::common::math::integer_math::int32_point3d;
use blf_lib::blam::common::math::real_math::{real_point3d, real_rectangle3d};
use blf_lib::blam::common::math::unit_vector_quanitzation::get_unit_vector_encoding_constants;
use blf_lib::types::numbers::Float32;
use blf_lib_derivable::result::BLFLibResult;
use crate::blam::common::math::real_math::{normalize3d, real_vector3d};

pub fn quantize_real_point3d_per_axis(
    position: &real_point3d,
    bounds: &real_rectangle3d,
    bits: &int32_point3d,
    quantized: &mut int32_point3d,
) {
    quantized.x = quantize_real(position.x, bounds.x.lower, bounds.x.upper, bits.x as usize, false, false);
    quantized.y = quantize_real(position.y, bounds.y.lower, bounds.y.upper, bits.y as usize, false, false);
    quantized.z = quantize_real(position.z, bounds.z.lower, bounds.z.upper, bits.z as usize, false, false);
}

pub fn quantize_real(value: impl Into<f32>, min_value: impl Into<f32>, max_value: impl Into<f32>, quantized_value_count: usize, exact_midpoint: bool, a6: bool) -> i32 {
    let value = value.into();
    let min_value = min_value.into();
    let max_value = max_value.into();

    assert!(quantized_value_count > 3, "quantized_value_count>3");
    assert!(max_value > min_value, "max_value>min_value");


    let step = (max_value - min_value) / quantized_value_count as f32;
    assert!(step > 0.0, "step>0.0f");

    let normalized_value = (value - min_value) / step;

    let sign = if normalized_value < 0.0 { -1.0 } else { 1.0 };
    let quantized_value = (sign * 0.5 + normalized_value) as i32;

    assert!(quantized_value >= 0 && quantized_value <= quantized_value_count as i32, "quantized_value>=0 && quantized_value<=step_count");

    quantized_value
}

pub fn dequantize_real(
    quantized: i32,
    min_value: impl Into<f32>,
    max_value: impl Into<f32>,
    quantized_value_count: usize,
    exact_midpoint: bool,
    exact_endpoints: bool
) -> f32 {
    let min_value = min_value.into();
    let max_value = max_value.into();
    let quantized_value_count = quantized_value_count as u32;

    assert!(quantized_value_count > 3, "quantized_value_count>3");
    assert!(max_value > min_value, "max_value>min_value");

    let dequantized: f32;

    if exact_endpoints {
        if quantized != 0 {
            if quantized < quantized_value_count as i32 {
                dequantized = (((quantized_value_count as i32 - quantized) as f32 * min_value) + (quantized as f32 * max_value)) / quantized_value_count as f32;
            } else {
                dequantized = max_value;
            }
        } else {
            dequantized = min_value;
        }
    } else {
        let step = (max_value - min_value) / quantized_value_count as f32;
        assert!(step > 0.0, "step>0.0f");
        dequantized = ((quantized as f32 * step) + min_value) + (step / 2.0f32);
    }

    if exact_midpoint && 2 * quantized == quantized_value_count as i32 {
        assert!(dequantized == (min_value + max_value) / 2.0, "value==(max_value+min_value)/2");
    }

    dequantized
}

pub fn quantize_unit_vector3d(vector: &real_vector3d, bit_count: usize) -> BLFLibResult<i32> {
    let encoding_constants = get_unit_vector_encoding_constants(bit_count).unwrap();

    let x = vector.i;
    let y = vector.j;
    let z = vector.k;

    let ax = x.abs();
    let ay = y.abs();
    let az = z.abs();

    let (face, u, w): (i32, f32, f32);

    if ax > ay && ax > az {
        if x >= 0.0 {
            face = 0;
            u = y / ax;
            w = z / ax;
        } else {
            face = 3;
            u = y / ax;
            w = z / ax;
        }
    } else if ay > az {
        if y >= 0.0 {
            face = 1;
            u = x / ay;
            w = z / ay;
        } else {
            face = 4;
            u = x / ay;
            w = z / ay;
        }
    } else {
        if z >= 0.0 {
            face = 2;
            u = x / az;
            w = y / az;
        } else {
            face = 5;
            u = x / az;
            w = y / az;
        }
    }

    assert!((-1.0..=1.0).contains(&u));
    assert!((-1.0..=1.0).contains(&w));

    let qu = quantize_real(u, -1.0, 1.0, encoding_constants.quantized_value_count as usize, true, false);
    let qw = quantize_real(w, -1.0, 1.0, encoding_constants.quantized_value_count as usize, true, false);

    Ok(
        encoding_constants.quantized_value_count as i32 * qu
        + encoding_constants.actual_per_axis_max_count as i32 * face
        + qw
    )
}

pub fn dequantize_unit_vector3d(value: i32, vector: &mut real_vector3d, bit_count: usize) -> BLFLibResult {
    let encoding_constants = get_unit_vector_encoding_constants(bit_count)?;

    let face = (value as f32 / encoding_constants.actual_per_axis_max_count as f32) as usize;
    let x = (value % encoding_constants.actual_per_axis_max_count as i32) as f32 / encoding_constants.quantized_value_count as f32;
    let y = (value % encoding_constants.actual_per_axis_max_count as i32) as f32 % encoding_constants.quantized_value_count as f32;

    let x = dequantize_real(x as i32, -1.0, 1.0, encoding_constants.quantized_value_count as usize, true, false);
    let y = dequantize_real(y as i32, -1.0, 1.0, encoding_constants.quantized_value_count as usize, true, false);

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

pub fn dequantize_real_point3d_per_axis(
    quantized: &int32_point3d,
    bounds: &real_rectangle3d,
    bits: &int32_point3d,
    position: &mut real_point3d,
    exact_midpoints: bool,
    exact_endpoints: bool,
) {
    assert!(bits.x <= 32 && bits.y <= 32 && bits.z <= 32);

    position.x = Float32::from(dequantize_real(
        quantized.x,
        bounds.x.lower,
        bounds.x.upper,
        bits.x as usize,
        exact_midpoints,
        exact_endpoints,
    ));

    position.y = Float32::from(dequantize_real(
        quantized.y,
        bounds.y.lower,
        bounds.y.upper,
        bits.y as usize,
        exact_midpoints,
        exact_endpoints,
    ));

    position.z = Float32::from(dequantize_real(
        quantized.z,
        bounds.z.lower,
        bounds.z.upper,
        bits.z as usize,
        exact_midpoints,
        exact_endpoints,
    ));
}