use std::cmp;
use blf_lib::blam::common::math::integer_math::int32_point3d;
use blf_lib::blam::common::math::real_math::{global_up3d, k_real_epsilon, real_point3d, real_rectangle3d};
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
    quantized.x = quantize_real_fast_guts::<false, false>(position.x, bounds.x.lower, bounds.x.upper, 1 << bits.x as usize);
    quantized.y = quantize_real_fast_guts::<false, false>(position.y, bounds.y.lower, bounds.y.upper, 1 << bits.y as usize);
    quantized.z = quantize_real_fast_guts::<false, false>(position.z, bounds.z.lower, bounds.z.upper, 1 << bits.z as usize);
}

pub fn quantize_real(
    value: impl Into<f32>,
    min_value: impl Into<f32>,
    max_value: impl Into<f32>,
    quantized_value_count: usize,
    exact_midpoint: bool,
    exact_endpoints: bool,
) -> i32 {
    let value = value.into();
    let min_value = min_value.into();
    let max_value = max_value.into();
    let quantized_value_count = quantized_value_count as i32;

    let mut adjusted_count = quantized_value_count;
    if exact_midpoint && quantized_value_count < 3 {
        panic!("adjust_quantized_value_count_to_attain_exact_midpoint assertion failed");
    }

    if exact_midpoint {
        adjusted_count -= 1;
    }

    // Placeholder for quantize_real_asserts, implement as needed
    // quantize_real_asserts(value, min_value, max_value, quantized_value_count, exact_midpoint, some_flag);

    let mut result: i32;

    if exact_endpoints {
        if value == min_value {
            result = 0;
        } else if value == max_value {
            result = adjusted_count - 1;
        } else {
            let divisor = (adjusted_count - 2) as f32;
            assert!(divisor > 0.0, "quantize_real divisor must be positive");
            let step = (max_value - min_value) / divisor;
            assert!(step > 0.0, "quantize_real step must be positive");
            let mut temp = ((value - min_value) / step).floor() as i32 + 1;
            if temp <= 1 {
                temp = 1;
            }
            result = if temp > adjusted_count - 2 { adjusted_count - 2 } else { temp };
        }
    } else {
        assert!(adjusted_count > 0, "quantize_real adjusted_count must be positive");
        let step = (max_value - min_value) / adjusted_count as f32;
        assert!(step > 0.0, "quantize_real step must be positive");
        let temp = ((value - min_value) / step).floor() as i32;
        result = (temp & temp) & temp; // mimic original bit-twiddling behavior
        if result > adjusted_count - 1 {
            result = adjusted_count - 1;
        }
    }

    assert!(result >= 0 && result < adjusted_count, "quantize_real result out of range");
    result
}

// The exact midpoints param is a guess, though in reach it doesn't seem to actually do anything.
pub fn quantize_real_fast<const EXACT_MIDPOINTS: bool, const EXACT_ENDPOINTS: bool>(
    value: f32,
    min: f32,
    max: f32,
    count: usize,
) -> i32 {
    quantize_real_fast_guts::<EXACT_MIDPOINTS, EXACT_ENDPOINTS>(value, min, max, count)
}

pub fn dequantize_real(
    quantized: i32,
    min_value: impl Into<f32>,
    max_value: impl Into<f32>,
    quantized_value_count: usize,
    exact_midpoints: bool,
    exact_endpoints: bool
) -> f32
{
    let min_value = min_value.into();
    let max_value = max_value.into();
    let mut quantized_value_count = quantized_value_count as i32;

    assert!(quantized_value_count >= 1);
    assert!(max_value > min_value);

    let mut value_count = quantized_value_count;

    if exact_midpoints
    {
        assert!(value_count >= 3);
        value_count -= 1;
    }

    assert!(value_count > 0);

    if exact_endpoints
    {
        if quantized == 0
        {
            return min_value;
        }
        else if quantized == value_count - 1
        {
            return max_value;
        }
        else
        {
            let denom = (value_count - 2) as f32;
            assert!(denom > 0.0);

            let step = (max_value - min_value) / denom;
            assert!(step > 0.0);

            let fquant = (quantized - 1) as f32;
            return min_value + step * (fquant + 0.5);
        }
    }
    else
    {
        let fquant = quantized as f32;
        let fcount = value_count as f32;

        let step = (max_value - min_value) / fcount;
        return min_value + step * (fquant + 0.5);
    }
}


// Octahedral Quantization
pub fn quantize_unit_vector3d_fast<const N: usize>(v: &real_vector3d) -> BLFLibResult<i32> {
    let constants = get_unit_vector_encoding_constants(N)?;

    let x = v.i.0 as f64;
    let y = v.j.0 as f64;
    let z = v.k.0 as f64;

    let ax = x.abs();
    let ay = y.abs();
    let az = z.abs();

    let face: i32;
    let u: f64;
    let w: f64;

    if ax > ay {
        if ax > az {
            face = if x <= 0.0 { 3 } else { 0 };
            u = y / ax;
            w = z / ax;
        } else {
            face = if z <= 0.0 { 5 } else { 2 };
            u = y / az;
            w = x / az;
        }
    } else {
        if ay > az {
            face = if y <= 0.0 { 4 } else { 1 };
            u = x / ay;
            w = z / ay;
        } else {
            face = if z <= 0.0 { 5 } else { 2 };
            u = x / az;
            w = y / az;
        }
    }

    let qu = quantize_real_fast::<true, false>(u as f32, -1.0, 1.0, (constants.quantized_value_count) as usize);
    let qw = quantize_real_fast::<true, false>(w as f32, -1.0, 1.0, (constants.quantized_value_count) as usize);

    Ok(qw + face * (constants.actual_per_axis_max_count as i32) + qu * (constants.quantized_value_count as i32))
}

pub fn dequantize_unit_vector3d(
    value: i32,
    vector: &mut real_vector3d,
    bit_count: usize,
) -> BLFLibResult {
    let encoding_constants = get_unit_vector_encoding_constants(bit_count)?;
    let actual_per_axis_max_count = encoding_constants.actual_per_axis_max_count as i32;
    let quantized_value_count = encoding_constants.quantized_value_count as i32;

    let face = cmp::max(((value as f32) / (actual_per_axis_max_count as f32)).floor() as usize, 0);

    let rem = value.rem_euclid(actual_per_axis_max_count);
    let qu = rem / quantized_value_count;
    let qw = rem % quantized_value_count;

    if qu < 0
        || qu >= actual_per_axis_max_count / quantized_value_count
        || qw < 0
        || qw >= quantized_value_count
    {
        *vector = global_up3d.clone();
        return Err(format!(
            "dequantize_unit_vector3d: bad quant indices qu={} qw={} face={}",
            qu, qw, face
        )
            .into());
    }

    let u = dequantize_real(qu, -1.0f32, 1.0f32, (quantized_value_count - 1) as usize, true, false);
    let w = dequantize_real(qw, -1.0f32, 1.0f32, (quantized_value_count - 1) as usize, true, false);

    match face {
        0 => {
            vector.i = Float32(1.0);
            vector.j = Float32(u);
            vector.k = Float32(w);
        }
        1 => {
            vector.i = Float32(u);
            vector.j = Float32(1.0);
            vector.k = Float32(w);
        }
        2 => {
            vector.i = Float32(u);
            vector.j = Float32(w);
            vector.k = Float32(1.0);
        }
        3 => {
            vector.i = Float32(-1.0);
            vector.j = Float32(u);
            vector.k = Float32(w);
        }
        4 => {
            vector.i = Float32(u);
            vector.j = Float32(-1.0);
            vector.k = Float32(w);
        }
        5 => {
            vector.i = Float32(u);
            vector.j = Float32(w);
            vector.k = Float32(-1.0);
        }
        _ => {
            *vector = global_up3d.clone();
            return Err(format!("dequantize_unit_vector3d: bad face value {}", face).into());
        }
    }

    normalize3d(vector);

    Ok(())
}

pub fn quantize_real_fast_guts<const EXACT_MIDPOINTS: bool, const EXACT_ENDPOINTS: bool>(
    value: impl Into<f32>,
    min: impl Into<f32>,
    max: impl Into<f32>,
    step_count: usize,
) -> i32 {
    let value = value.into();
    let min = min.into();
    let max = max.into();
    let step_count = step_count as i32;

    assert!(step_count > 0, "step_count must be positive");

    if EXACT_ENDPOINTS {
        // Handle exact endpoints
        if value == min {
            return 0;
        }
        if value == max {
            return step_count - 1;
        }

        // Map intermediate values into [1, step_count - 2]
        let mut max_intermediate_index = step_count - 2;
        assert!(max_intermediate_index > 0, "step_count too small for EXACT_ENDPOINTS");

        let step_size = (max - min) / max_intermediate_index as f32;
        assert!(step_size > 0.0, "step_size must be positive");

        let mut computed_index = ((value - min) / step_size).floor() as i32 + 1;

        if computed_index <= 1 {
            computed_index = 1;
        }

        if computed_index <= max_intermediate_index {
            max_intermediate_index = computed_index;
        }

        assert!(max_intermediate_index >= 0 && max_intermediate_index < step_count, "index out of range");

        return max_intermediate_index;
    }

    // Fallback to <1,0> logic
    let step_size = (max - min) / step_count as f32;
    assert!(step_size > 0.0, "step_size must be positive");

    let v12 = ((value - min) / step_size) as i32;
    let mut index = (if v12 == 0 { 1 } else { 0 } + (v12 >> 31) - 1) & v12;

    if index > step_count - 1 {
        index = step_count - 1;
    }

    assert!(index >= 0 && index < step_count, "index out of range");

    index
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
        1 << bits.x as usize,
        exact_midpoints,
        exact_endpoints,
    ));

    position.y = Float32::from(dequantize_real(
        quantized.y,
        bounds.y.lower,
        bounds.y.upper,
        1 << bits.y as usize,
        exact_midpoints,
        exact_endpoints,
    ));

    position.z = Float32::from(dequantize_real(
        quantized.z,
        bounds.z.lower,
        bounds.z.upper,
        1 << bits.z as usize,
        exact_midpoints,
        exact_endpoints,
    ));
}