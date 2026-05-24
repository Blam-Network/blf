export enum e_bitstream_byte_order {
  _bitstream_byte_order_little_endian = 0,
  _bitstream_byte_order_big_endian = 1,
}

export namespace e_bitstream_byte_order {
  export function swap(order: e_bitstream_byte_order): e_bitstream_byte_order {
    switch (order) {
      case e_bitstream_byte_order._bitstream_byte_order_little_endian:
        return e_bitstream_byte_order._bitstream_byte_order_big_endian;
      case e_bitstream_byte_order._bitstream_byte_order_big_endian:
        return e_bitstream_byte_order._bitstream_byte_order_little_endian;
    }
  }
}

export enum e_bitstream_byte_fill_direction {
  _bitstream_byte_fill_direction_msb_to_lsb = 0,
  _bitstream_byte_fill_direction_lsb_to_msb = 1, // Used by pre-release h3
}

export enum e_bitstream_state {
  _bitstream_state_initial = 0,
  _bitstream_state_writing = 1,
  _bitstream_state_write_finished = 2,
  _bitstream_state_reading = 3,
  _bitstream_state_read_only_for_consistency = 4,
  _bitstream_state_read_finished = 5,

  k_bitstream_state_count = 6,
}
