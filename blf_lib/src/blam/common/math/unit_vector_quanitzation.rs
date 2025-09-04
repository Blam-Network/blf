use lazy_static::lazy_static;
use blf_lib::assert_ok;
use blf_lib::blam::common::math::real_math::square_root;
use blf_lib_derivable::result::BLFLibResult;

pub struct s_encoding_constants {
    pub quantized_value_count: u32,
    pub actual_per_axis_max_count: u32,
}

impl s_encoding_constants {
    pub fn compute(bit_count: usize) -> Self {
        assert!(bit_count >= k_unit_vector_quantization_minimum_bit_count);
        assert!(bit_count <= k_unit_vector_quantization_maximum_bit_count);

        let mut mask = 0;
        for (i, bit) in (0..bit_count - 1).rev().enumerate() {
            if i % 2 == 1 { // every odd bit = 1
                mask |= 1 << bit;
            }
        }

        Self {
            actual_per_axis_max_count: mask,
            quantized_value_count: square_root(mask as f32) as u32 - 1,
        }
    }
}

const k_unit_vector_quantization_minimum_bit_count: usize = 6;
const k_unit_vector_quantization_maximum_bit_count: usize = 30;

lazy_static! {
    static ref g_unit_vector_generated_encoding_constants: [s_encoding_constants; k_unit_vector_quantization_maximum_bit_count - k_unit_vector_quantization_minimum_bit_count + 1] = [
        s_encoding_constants::compute(6),
        s_encoding_constants::compute(7),
        s_encoding_constants::compute(8),
        s_encoding_constants::compute(9),
        s_encoding_constants::compute(10),
        s_encoding_constants::compute(11),
        s_encoding_constants::compute(12),
        s_encoding_constants::compute(13),
        s_encoding_constants::compute(14),
        s_encoding_constants::compute(15),
        s_encoding_constants::compute(16),
        s_encoding_constants::compute(17),
        s_encoding_constants::compute(18),
        s_encoding_constants::compute(19),
        s_encoding_constants::compute(20),
        s_encoding_constants::compute(21),
        s_encoding_constants::compute(22),
        s_encoding_constants::compute(23),
        s_encoding_constants::compute(24),
        s_encoding_constants::compute(25),
        s_encoding_constants::compute(26),
        s_encoding_constants::compute(27),
        s_encoding_constants::compute(28),
        s_encoding_constants::compute(29),
        s_encoding_constants::compute(30)
    ];
}

pub fn get_unit_vector_encoding_constants(bit_count: usize) -> BLFLibResult<&'static s_encoding_constants> {
    assert_ok!(bit_count >= k_unit_vector_quantization_minimum_bit_count);
    assert_ok!(bit_count <= k_unit_vector_quantization_maximum_bit_count);
    Ok(&g_unit_vector_generated_encoding_constants[bit_count - k_unit_vector_quantization_minimum_bit_count])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_encoding_constants_includes_reach_constants() {
        let expected = [
            [0xA, 0x2],
            [0x15, 0x3],
            [0x2A, 0x5],
            [0x55, 0x8],
            [0xAA, 0xC],
            [0x155, 0x11],
            [0x2AA, 0x19],
            [0x555, 0x23],
            [0xAAA, 0x33],
            [0x1555, 0x48],
            [0x2AAA, 0x67],
            [0x5555, 0x92],
            [0xAAAA, 0xD0],
            [0x15555, 0x126],
            [0x2AAAA, 0x1A1],
            [0x55555, 0x24E],
            [0xAAAAA, 0x343],
            [0x155555, 0x49D],
            [0x2AAAAA, 0x687],
            [0x555555, 0x93B],
            [0xAAAAAA, 0xD0F],
            [0x1555555, 0x1278],
            [0x2AAAAAA, 0x1A1F],
            [0x5555555, 0x24F2],
            [0xAAAAAAA, 0x3440],
        ];

        let reach_min: usize = 6;
        let reach_max: usize = 30;

        for bit_count in reach_min..reach_max + 1 {
            let encoding_constants = s_encoding_constants::compute(bit_count);
            assert_eq!(encoding_constants.actual_per_axis_max_count, expected[bit_count - reach_min][0]);
            assert_eq!(encoding_constants.quantized_value_count, expected[bit_count - reach_min][1]);
        }
    }
}