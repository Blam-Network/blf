import { enumWireIndexFromMember, type NumericEnum } from "./enum";
import {
  e_bitstream_byte_fill_direction,
  e_bitstream_byte_order,
  e_bitstream_state,
} from "./enums";
import { assert_ok, BitstreamError } from "./errors";
import { quantize_real } from "./math";

const {
  _bitstream_byte_fill_direction_msb_to_lsb,
  _bitstream_byte_fill_direction_lsb_to_msb,
} = e_bitstream_byte_fill_direction;

export class c_bitstream_writer {
  private m_data: Uint8Array;
  private m_data_size_bytes: number;
  private m_state: e_bitstream_state;
  private m_byte_order: e_bitstream_byte_order;
  private m_packed_byte_order: e_bitstream_byte_order;
  private m_byte_pack_direction: e_bitstream_byte_fill_direction;
  private m_byte_unpack_direction: e_bitstream_byte_fill_direction;

  private current_stream_bit_position: number;
  private current_stream_byte_position: number;

  static new(
    size: number,
    byte_order: e_bitstream_byte_order
  ): c_bitstream_writer {
    return new c_bitstream_writer(size, byte_order, {
      packed_byte_order: byte_order,
      byte_pack_direction: _bitstream_byte_fill_direction_msb_to_lsb,
      byte_unpack_direction: _bitstream_byte_fill_direction_msb_to_lsb,
    });
  }

  /** Use this when dealing with bitstream data from the Halo 3 Beta or prior. */
  static new_with_legacy_settings(
    size: number,
    byte_order: e_bitstream_byte_order
  ): c_bitstream_writer {
    return new c_bitstream_writer(size, byte_order, {
      packed_byte_order: e_bitstream_byte_order.swap(byte_order),
      byte_pack_direction: _bitstream_byte_fill_direction_lsb_to_msb,
      byte_unpack_direction: _bitstream_byte_fill_direction_lsb_to_msb,
    });
  }

  static new_from_instance(
    size: number,
    instance: c_bitstream_writer
  ): c_bitstream_writer {
    return new c_bitstream_writer(size, instance.get_byte_order(), {
      packed_byte_order: instance.m_packed_byte_order,
      byte_pack_direction: instance.m_byte_pack_direction,
      byte_unpack_direction: instance.m_byte_unpack_direction,
    });
  }

  private constructor(
    size: number,
    byte_order: e_bitstream_byte_order,
    options: {
      packed_byte_order: e_bitstream_byte_order;
      byte_pack_direction: e_bitstream_byte_fill_direction;
      byte_unpack_direction: e_bitstream_byte_fill_direction;
    }
  ) {
    this.m_data = new Uint8Array(size);
    this.m_data_size_bytes = size;
    this.m_state = e_bitstream_state._bitstream_state_initial;
    this.m_byte_order = byte_order;
    this.m_packed_byte_order = options.packed_byte_order;
    this.m_byte_pack_direction = options.byte_pack_direction;
    this.m_byte_unpack_direction = options.byte_unpack_direction;
    this.current_stream_bit_position = 0;
    this.current_stream_byte_position = 0;
  }

  get_byte_order(): e_bitstream_byte_order {
    return this.m_byte_order;
  }

  get_current_offset(): [number, number] {
    return [
      this.current_stream_byte_position,
      this.current_stream_bit_position,
    ];
  }

  write_enum(value: number, size_in_bits: number, enumObj: NumericEnum): void {
    this.write_integer(
      enumWireIndexFromMember(enumObj, value, "enum"),
      size_in_bits
    );
  }

  write_integer(value: number, size_in_bits: number): void {
    assert_ok(this.writing());
    const bytes = new Uint8Array(4);
    const view = new DataView(bytes.buffer);
    const u = value >>> 0;
    switch (this.m_packed_byte_order) {
      case e_bitstream_byte_order._bitstream_byte_order_little_endian:
        view.setUint32(0, u, true);
        break;
      case e_bitstream_byte_order._bitstream_byte_order_big_endian:
        view.setUint32(0, u, false);
        break;
    }
    this.write_bits_internal(bytes, size_in_bits);
  }

  write_signed_integer(value: number, size_in_bits: number): void {
    assert_ok(this.writing());
    assert_ok(size_in_bits > 0 && size_in_bits <= 32);
    // Avoid `1 << 31` — JS bitwise ops are 32-bit signed and overflow.
    const maximum =
      size_in_bits === 32 ? 0x7fffffff : (1 << (size_in_bits - 1)) - 1;
    const minimum = size_in_bits === 32 ? -0x80000000 : ~maximum;
    assert_ok(value >= minimum, "value>=minimum");
    assert_ok(value <= maximum, "value<=maximum");
    this.write_integer(value >>> 0, size_in_bits);
  }

  write_bool(value: boolean): void {
    this.write_integer(value ? 1 : 0, 1);
  }

  seek_relative(bits: number): void {
    const zero_bytes = new Uint8Array(Math.ceil(bits / 8));
    this.write_bits_internal(zero_bytes, bits);
  }

  write_float(value: number, size_in_bits: number): void {
    assert_ok(this.writing());
    const bytes = new Uint8Array(4);
    const view = new DataView(bytes.buffer);
    switch (this.m_packed_byte_order) {
      case e_bitstream_byte_order._bitstream_byte_order_little_endian:
        view.setFloat32(0, value, true);
        break;
      case e_bitstream_byte_order._bitstream_byte_order_big_endian:
        view.setFloat32(0, value, false);
        break;
    }
    this.write_bits_internal(bytes, size_in_bits);
  }

  write_raw_data(value: Uint8Array, size_in_bits: number): void {
    assert_ok(value.length >= Math.ceil(size_in_bits / 8));
    this.write_bits_internal(value, size_in_bits);
  }

  write_qword(value: bigint, size_in_bits: number): void {
    assert_ok(this.writing());
    const bytes = new Uint8Array(8);
    const view = new DataView(bytes.buffer);
    switch (this.m_packed_byte_order) {
      case e_bitstream_byte_order._bitstream_byte_order_little_endian:
        view.setBigUint64(0, value, true);
        break;
      case e_bitstream_byte_order._bitstream_byte_order_big_endian:
        view.setBigUint64(0, value, false);
        break;
    }
    this.write_bits_internal(bytes, size_in_bits);
  }

  write_index(value: number, max_value: number, bit_size: number): void {
    assert_ok(value <= max_value);
    if (value === -1) {
      this.write_bool(true);
    } else {
      this.write_bool(false);
      this.write_integer(value, bit_size);
    }
  }

  write_quantized_real(
    value: number,
    min_value: number,
    max_value: number,
    size_in_bits: number,
    exact_midpoint: boolean,
    exact_endpoints: boolean
  ): void {
    const quantized = quantize_real(
      value,
      min_value,
      max_value,
      1 << size_in_bits,
      exact_midpoint,
      exact_endpoints
    );
    this.write_integer(quantized, size_in_bits);
  }

  write_point3d(
    point: { x: number; y: number; z: number },
    axis_encoding_size_in_bits: number
  ): void {
    assert_ok(
      axis_encoding_size_in_bits > 0 && axis_encoding_size_in_bits <= 32
    );
    assert_ok(point.x < 1 << axis_encoding_size_in_bits);
    assert_ok(point.y < 1 << axis_encoding_size_in_bits);
    assert_ok(point.z < 1 << axis_encoding_size_in_bits);
    this.write_integer(point.x, axis_encoding_size_in_bits);
    this.write_integer(point.y, axis_encoding_size_in_bits);
    this.write_integer(point.z, axis_encoding_size_in_bits);
  }

  write_point3d_efficient(
    point: { x: number; y: number; z: number },
    axis_encoding_size_in_bits: { x: number; y: number; z: number }
  ): void {
    assert_ok(
      axis_encoding_size_in_bits.x > 0 && axis_encoding_size_in_bits.x <= 32
    );
    assert_ok(
      axis_encoding_size_in_bits.y > 0 && axis_encoding_size_in_bits.y <= 32
    );
    assert_ok(
      axis_encoding_size_in_bits.z > 0 && axis_encoding_size_in_bits.z <= 32
    );
    assert_ok(point.x >>> 0 < 1 << axis_encoding_size_in_bits.x);
    assert_ok(point.y >>> 0 < 1 << axis_encoding_size_in_bits.y);
    assert_ok(point.z >>> 0 < 1 << axis_encoding_size_in_bits.z);
    this.write_integer(point.x, axis_encoding_size_in_bits.x);
    this.write_integer(point.y, axis_encoding_size_in_bits.y);
    this.write_integer(point.z, axis_encoding_size_in_bits.z);
  }

  /** Writes raw UTF-8 bytes + NUL (matches blf_lib `String::as_bytes()`). */
  write_string_utf8(char_string: string, max_string_size: number): void {
    assert_ok(this.writing());
    assert_ok(max_string_size > 0);
    const bytes = new TextEncoder().encode(char_string);
    assert_ok(bytes.length <= max_string_size);
    for (const byte of bytes) {
      this.write_value_internal(new Uint8Array([byte]), 8);
    }
    this.write_value_internal(new Uint8Array([0]), 8);
  }

  write_string_extended_ascii(
    char_string: string,
    max_string_size: number
  ): void {
    assert_ok(this.writing());
    assert_ok(max_string_size > 0);
    assert_ok(char_string.length <= max_string_size);
    for (const ch of char_string) {
      const code = ch.codePointAt(0)!;
      if (code > 0xff) {
        throw new BitstreamError(`Cannot encode non-Latin-1 char: ${ch}`);
      }
      this.write_value_internal(new Uint8Array([code]), 8);
    }
    this.write_value_internal(new Uint8Array([0]), 8);
  }

  write_string_wchar(value: string, max_string_size: number): void {
    assert_ok(this.writing());
    assert_ok(max_string_size > 0);
    assert_ok(value.length <= max_string_size);

    for (let i = 0; i < value.length; i++) {
      const code = value.charCodeAt(i);
      const bytes = new Uint8Array(2);
      const view = new DataView(bytes.buffer);
      switch (this.m_packed_byte_order) {
        case e_bitstream_byte_order._bitstream_byte_order_little_endian:
          view.setUint16(0, code, true);
          break;
        case e_bitstream_byte_order._bitstream_byte_order_big_endian:
          view.setUint16(0, code, false);
          break;
      }
      this.write_value_internal(bytes, 16);
    }

    this.write_value_internal(new Uint8Array(2), 16);
  }

  begin_writing(): void {
    this.reset(e_bitstream_state._bitstream_state_writing);
  }

  writing(): boolean {
    return this.m_state === e_bitstream_state._bitstream_state_writing;
  }

  finish_writing(): void {
    this.m_state = e_bitstream_state._bitstream_state_write_finished;
    this.m_data_size_bytes = Math.ceil(
      (this.current_stream_byte_position * 8 +
        this.current_stream_bit_position) /
        8
    );
  }

  get_data(): Uint8Array {
    assert_ok(!this.writing());
    return this.m_data.slice(0, this.m_data_size_bytes);
  }

  private write_value_internal(data: Uint8Array, size_in_bits: number): void {
    this.write_bits_internal(data, size_in_bits);
  }

  private write_bits_internal(data: Uint8Array, size_in_bits: number): void {
    const size_in_bytes = Math.ceil(size_in_bits / 8);
    if (data.length < size_in_bytes) {
      throw new BitstreamError(
        `Tried to write ${size_in_bits} bits but only ${data.length * 8} were provided`
      );
    }

    const surplus_bytes = data.length - size_in_bytes;
    const surplus_bits = (data.length * 8 - size_in_bits) % 8;

    let remaining_bits_to_write = size_in_bits;

    while (remaining_bits_to_write > 0) {
      const bytes_written = Math.floor(
        (size_in_bits - remaining_bits_to_write) / 8
      );
      let bits_written = 0;

      let writing_byte: number;
      switch (this.m_packed_byte_order) {
        case e_bitstream_byte_order._bitstream_byte_order_little_endian:
          if (remaining_bits_to_write < 8) {
            writing_byte =
              (data[bytes_written]! << (8 - remaining_bits_to_write)) & 0xff;
          } else {
            writing_byte = data[bytes_written]!;
          }
          break;
        case e_bitstream_byte_order._bitstream_byte_order_big_endian: {
          let bits =
            (data[bytes_written + surplus_bytes]! << surplus_bits) & 0xff;
          if (
            surplus_bits !== 0 &&
            remaining_bits_to_write > 8 - surplus_bits
          ) {
            bits |=
              (data[bytes_written + surplus_bytes + 1]! >> (8 - surplus_bits)) &
              0xff;
          }
          writing_byte = bits;
          break;
        }
      }

      writing_byte &= 0xff << (8 - Math.min(8, remaining_bits_to_write));

      const writing_bits_at_position = Math.min(
        8 - this.current_stream_bit_position,
        remaining_bits_to_write
      );
      const remaining_bits_at_position = 8 - this.current_stream_bit_position;

      let bits: number;
      switch (this.m_byte_unpack_direction) {
        case _bitstream_byte_fill_direction_lsb_to_msb:
          bits =
            (writing_byte <<
              (Math.min(8, remaining_bits_to_write) -
                writing_bits_at_position)) &
            0xff;
          break;
        case _bitstream_byte_fill_direction_msb_to_lsb:
          bits = writing_byte & (0xff << (8 - writing_bits_at_position));
          break;
      }

      switch (this.m_byte_pack_direction) {
        case _bitstream_byte_fill_direction_lsb_to_msb:
          bits >>>=
            8 - (this.current_stream_bit_position + writing_bits_at_position);
          break;
        case _bitstream_byte_fill_direction_msb_to_lsb:
          bits >>>= this.current_stream_bit_position;
          break;
      }

      this.ensure_byte_capacity(this.current_stream_byte_position);
      this.m_data[this.current_stream_byte_position]! |= bits;
      bits_written += writing_bits_at_position;

      if (writing_bits_at_position === remaining_bits_at_position) {
        this.current_stream_bit_position = 0;
        this.current_stream_byte_position += 1;
      } else {
        this.current_stream_bit_position += writing_bits_at_position;
      }

      remaining_bits_to_write -= writing_bits_at_position;

      if (remaining_bits_to_write > 0 && bits_written < 8) {
        const extra_bits_at_position = Math.min(
          remaining_bits_to_write,
          8 - bits_written
        );

        let extra_bits: number;
        switch (this.m_byte_unpack_direction) {
          case _bitstream_byte_fill_direction_lsb_to_msb:
            extra_bits = writing_byte & (0xff << (8 - extra_bits_at_position));
            break;
          case _bitstream_byte_fill_direction_msb_to_lsb:
            extra_bits = (writing_byte << bits_written) & 0xff;
            break;
        }

        if (
          this.m_byte_pack_direction ===
          _bitstream_byte_fill_direction_lsb_to_msb
        ) {
          extra_bits >>>= 8 - extra_bits_at_position;
        }

        this.ensure_byte_capacity(this.current_stream_byte_position);
        this.m_data[this.current_stream_byte_position]! |= extra_bits;
        bits_written += extra_bits_at_position;
        this.current_stream_bit_position = extra_bits_at_position;
        remaining_bits_to_write -= extra_bits_at_position;
      }
    }
  }

  private ensure_byte_capacity(byte_index: number): void {
    if (byte_index < this.m_data.length) {
      return;
    }
    const new_size = Math.max(this.m_data.length * 2, byte_index + 1);
    const grown = new Uint8Array(new_size);
    grown.set(this.m_data);
    this.m_data = grown;
    this.m_data_size_bytes = new_size;
  }

  private reset(state: e_bitstream_state): void {
    this.m_state = state;
    this.current_stream_bit_position = 0;
    this.current_stream_byte_position = 0;
    this.m_data.fill(0);
  }
}
