const k_unit_vector_quantization_minimum_bit_count = 6;
const k_unit_vector_quantization_maximum_bit_count = 30;

export interface s_encoding_constants {
  actual_per_axis_max_count: number;
  quantized_value_count: number;
}

function compute_encoding_constants(bit_count: number): s_encoding_constants {
  let mask = 0;
  for (let i = 0, bit = bit_count - 2; bit >= 0; bit--, i++) {
    if (i % 2 === 1) {
      mask |= 1 << bit;
    }
  }

  return {
    actual_per_axis_max_count: mask,
    quantized_value_count: Math.floor(Math.sqrt(mask)) - 1,
  };
}

const g_unit_vector_generated_encoding_constants: s_encoding_constants[] =
  Array.from(
    {
      length:
        k_unit_vector_quantization_maximum_bit_count -
        k_unit_vector_quantization_minimum_bit_count +
        1,
    },
    (_, index) =>
      compute_encoding_constants(
        index + k_unit_vector_quantization_minimum_bit_count
      )
  );

export function get_unit_vector_encoding_constants(
  bit_count: number
): s_encoding_constants {
  if (
    bit_count < k_unit_vector_quantization_minimum_bit_count ||
    bit_count > k_unit_vector_quantization_maximum_bit_count
  ) {
    throw new Error(`Invalid unit vector bit count: ${bit_count}`);
  }

  return g_unit_vector_generated_encoding_constants[
    bit_count - k_unit_vector_quantization_minimum_bit_count
  ];
}
