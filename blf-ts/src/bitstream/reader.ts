import {
  assertFitsInBits,
  type EnumNumber,
  enumMemberFromWireIndex,
  type NumericEnum,
} from "./enum";
import {
  e_bitstream_byte_fill_direction,
  e_bitstream_byte_order,
  e_bitstream_state,
} from "./enums";
import { assert_ok, BitstreamError } from "./errors";
import {
  angle_to_axes_internal as angle_to_axes,
  axes_compute_reference_internal as axes_compute_reference,
  axes_to_angle_internal as axes_to_angle,
  dequantize_real,
  type real_vector3d,
} from "./math";

const {
  _bitstream_byte_fill_direction_msb_to_lsb,
  _bitstream_byte_fill_direction_lsb_to_msb,
} = e_bitstream_byte_fill_direction;

export class c_bitstream_reader {
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
    data: Uint8Array,
    byte_order: e_bitstream_byte_order
  ): c_bitstream_reader {
    return new c_bitstream_reader(data, byte_order, {
      packed_byte_order: byte_order,
      byte_pack_direction:
        e_bitstream_byte_fill_direction._bitstream_byte_fill_direction_msb_to_lsb,
      byte_unpack_direction:
        e_bitstream_byte_fill_direction._bitstream_byte_fill_direction_msb_to_lsb,
    });
  }

  /** Use this when dealing with bitstream data from the Halo 3 Beta or prior. */
  static new_with_legacy_settings(
    data: Uint8Array,
    byte_order: e_bitstream_byte_order
  ): c_bitstream_reader {
    return new c_bitstream_reader(data, byte_order, {
      packed_byte_order: e_bitstream_byte_order.swap(byte_order),
      byte_pack_direction: _bitstream_byte_fill_direction_lsb_to_msb,
      byte_unpack_direction: _bitstream_byte_fill_direction_lsb_to_msb,
    });
  }

  static new_from_instance(
    data: Uint8Array,
    instance: c_bitstream_reader
  ): c_bitstream_reader {
    return new c_bitstream_reader(data, instance.get_byte_order(), {
      packed_byte_order: instance.m_packed_byte_order,
      byte_pack_direction: instance.m_byte_pack_direction,
      byte_unpack_direction: instance.m_byte_unpack_direction,
    });
  }

  private constructor(
    data: Uint8Array,
    byte_order: e_bitstream_byte_order,
    options: {
      packed_byte_order: e_bitstream_byte_order;
      byte_pack_direction: e_bitstream_byte_fill_direction;
      byte_unpack_direction: e_bitstream_byte_fill_direction;
    }
  ) {
    this.m_data = data;
    this.m_data_size_bytes = data.length;
    this.m_state = e_bitstream_state._bitstream_state_initial;
    this.m_byte_order = byte_order;
    this.m_packed_byte_order = options.packed_byte_order;
    this.m_byte_pack_direction = options.byte_pack_direction;
    this.m_byte_unpack_direction = options.byte_unpack_direction;
    this.current_stream_bit_position = 0;
    this.current_stream_byte_position = 0;
  }

  get_byte_order(): e_bitstream_byte_order {
    return this.m_packed_byte_order;
  }

  seek_relative(bits: number): void {
    const current_bit_position =
      this.current_stream_byte_position * 8 + this.current_stream_bit_position;
    const new_bit_position = current_bit_position + bits;
    assert_ok(new_bit_position / 8 < this.m_data_size_bytes);

    this.current_stream_bit_position = new_bit_position % 8;
    this.current_stream_byte_position = Math.floor(new_bit_position / 8);
  }

  seek(byte: number): void {
    assert_ok(byte < this.m_data_size_bytes);

    this.current_stream_byte_position = byte;
    this.current_stream_bit_position = 0;
  }

  seek_bit(byte: number, bit: number): void {
    assert_ok(bit < 8);

    this.seek(byte);
    this.current_stream_bit_position = bit;
  }

  get_current_offset(): [number, number] {
    return [
      this.current_stream_byte_position,
      this.current_stream_bit_position,
    ];
  }

  read_raw_data(size_in_bits: number): Uint8Array {
    const buffer = new Uint8Array(Math.ceil(size_in_bits / 8));
    this.read_bits_internal(buffer, size_in_bits);
    return buffer;
  }

  read_bool(_name: string): boolean {
    return this.read_integer(_name, 1) === 1;
  }

  read_bits_internal(output: Uint8Array, size_in_bits: number): void {
    const end_memory_position = output.length;
    const end_stream_position = this.m_data.length;
    const remaining_stream_bytes =
      end_stream_position - (this.current_stream_byte_position + 1);
    const remaining_stream_bits =
      8 - this.current_stream_bit_position + remaining_stream_bytes * 8;

    const size_in_bytes = Math.ceil(size_in_bits / 8);
    if (end_memory_position < size_in_bytes) {
      throw new BitstreamError(
        `Tried to read ${size_in_bytes} bytes (${size_in_bits} bits) into a ${end_memory_position} byte buffer!`
      );
    }

    if (remaining_stream_bits < size_in_bits) {
      throw new BitstreamError(
        `Tried to read ${size_in_bits} bits but the stream only has ${remaining_stream_bits} bits left!`
      );
    }

    if (size_in_bits === 0) {
      throw new BitstreamError("Tried to read zero bits.");
    }

    let remaining_bits_to_read = size_in_bits;
    let output_byte_index = 0;

    while (remaining_bits_to_read > 0) {
      let output_byte = 0;
      let bits_read = 0;

      if (this.current_stream_bit_position !== 0) {
        const remaining_bits_at_position = 8 - this.current_stream_bit_position;
        const reading_bits_at_position = Math.min(
          remaining_bits_at_position,
          remaining_bits_to_read
        );

        let bits = this.m_data[this.current_stream_byte_position]!;

        switch (this.m_byte_pack_direction) {
          case _bitstream_byte_fill_direction_msb_to_lsb:
            bits = (bits << this.current_stream_bit_position) & 0xff;
            break;
          case _bitstream_byte_fill_direction_lsb_to_msb:
            bits =
              (bits <<
                (remaining_bits_at_position - reading_bits_at_position)) &
              0xff;
            break;
        }

        bits &= (0xff << (8 - reading_bits_at_position)) & 0xff;

        if (
          this.m_byte_unpack_direction ===
          _bitstream_byte_fill_direction_lsb_to_msb
        ) {
          bits = (bits >> (8 - reading_bits_at_position)) & 0xff;
        }

        output_byte = bits;
        bits_read += reading_bits_at_position;

        if (reading_bits_at_position > remaining_bits_at_position) {
          throw new BitstreamError(
            "bitstream reader believes it has read more bits than available. This should never happen."
          );
        }
        if (reading_bits_at_position === remaining_bits_at_position) {
          this.current_stream_bit_position = 0;
          this.current_stream_byte_position += 1;
        } else {
          this.current_stream_bit_position += reading_bits_at_position;
        }

        remaining_bits_to_read -= reading_bits_at_position;
      }

      if (remaining_bits_to_read > 0) {
        const reading_bits_at_position = Math.min(
          8 - bits_read,
          remaining_bits_to_read
        );
        let bits = this.m_data[this.current_stream_byte_position]!;

        if (
          this.m_byte_pack_direction ===
          _bitstream_byte_fill_direction_lsb_to_msb
        ) {
          bits = (bits << (8 - reading_bits_at_position)) & 0xff;
        }

        bits &= (0xff << (8 - reading_bits_at_position)) & 0xff;

        switch (this.m_byte_unpack_direction) {
          case _bitstream_byte_fill_direction_msb_to_lsb:
            bits = (bits >> bits_read) & 0xff;
            break;
          case _bitstream_byte_fill_direction_lsb_to_msb:
            bits =
              (bits >> (8 - (bits_read + reading_bits_at_position))) & 0xff;
            break;
        }

        output_byte = (output_byte | bits) & 0xff;
        bits_read += reading_bits_at_position;
        remaining_bits_to_read -= reading_bits_at_position;

        if (reading_bits_at_position === 8) {
          this.current_stream_bit_position = 0;
          this.current_stream_byte_position += 1;
        } else {
          this.current_stream_bit_position += reading_bits_at_position;
        }
      }

      if (
        bits_read < 8 &&
        this.m_byte_unpack_direction ===
          _bitstream_byte_fill_direction_lsb_to_msb
      ) {
        output_byte = (output_byte << (8 - bits_read)) & 0xff;
      }

      switch (this.m_packed_byte_order) {
        case e_bitstream_byte_order._bitstream_byte_order_big_endian: {
          const surplus_bits = (8 - (size_in_bits % 8)) % 8;
          const shifted = ((output_byte & 0xff) << (8 - surplus_bits)) & 0xffff;
          const left_output = (shifted >> 8) & 0xff;
          const right_output = shifted & 0xff;
          output[output_byte_index]! |= left_output;
          if (right_output !== 0) {
            output[output_byte_index + 1]! |= right_output;
          }
          output_byte_index += 1;
          break;
        }
        case e_bitstream_byte_order._bitstream_byte_order_little_endian: {
          if (bits_read < 8) {
            output_byte = (output_byte >>> (8 - bits_read)) & 0xff;
          }
          output[output_byte_index] = output_byte;
          output_byte_index += 1;
          break;
        }
      }
    }
  }

  /**
   * Read a numeric enum. Pass the enum object; no per-enum parser required.
   *
   * - Default: value must be a declared enum member.
   * - `{ within_bits: true }`: any value that fits in `size_in_bits` (reserved slots).
   */
  read_enum<E extends NumericEnum>(
    name: string,
    size_in_bits: number,
    enumObj: E,
    options?: { within_bits?: boolean }
  ): EnumNumber<E>;
  read_enum<T extends number>(
    name: string,
    size_in_bits: number,
    from_u32: (value: number) => T | undefined
  ): T;
  read_enum(
    name: string,
    size_in_bits: number,
    enumObjOrParser: NumericEnum | ((value: number) => number | undefined),
    options?: { within_bits?: boolean }
  ): number {
    const integer = this.read_integer(name, size_in_bits);

    if (typeof enumObjOrParser === "function") {
      const value = enumObjOrParser(integer);
      if (value === undefined) {
        throw new BitstreamError(
          `Unexpected enum value for ${name}: ${integer}`
        );
      }
      return value;
    }

    if (options?.within_bits) {
      assertFitsInBits(name, integer, size_in_bits);
      return integer;
    }

    return enumMemberFromWireIndex(enumObjOrParser, integer, name);
  }

  read_integer(_name: string, size_in_bits: number): number {
    assert_ok(size_in_bits > 0);
    assert_ok(size_in_bits <= 32);
    const size_in_bytes = Math.ceil(size_in_bits / 8);
    const bytes_vec = new Uint8Array(size_in_bytes);
    this.read_bits_internal(bytes_vec, size_in_bits);

    const byte_array = new Uint8Array(4);

    switch (this.m_packed_byte_order) {
      case e_bitstream_byte_order._bitstream_byte_order_little_endian: {
        byte_array.set(bytes_vec, 0);
        const view = new DataView(byte_array.buffer);
        return view.getUint32(0, true);
      }
      case e_bitstream_byte_order._bitstream_byte_order_big_endian: {
        byte_array.set(bytes_vec, 4 - bytes_vec.length);
        const view = new DataView(byte_array.buffer);
        return view.getUint32(0, false);
      }
    }
  }

  read_index(name: string, max: number, size_in_bits: number): number {
    if (this.read_bool("index-exists")) {
      return -1;
    }
    const value = this.read_integer(name, size_in_bits);
    assert_ok(value < max);
    return value;
  }

  read_quantized_real(
    min_value: number,
    max_value: number,
    size_in_bits: number,
    exact_midpoint: boolean,
    exact_endpoints: boolean
  ): number {
    const quantized = this.read_integer("quantized-real", size_in_bits);
    return dequantize_real(
      quantized,
      min_value,
      max_value,
      1 << size_in_bits,
      exact_midpoint,
      exact_endpoints
    );
  }

  read_float(_name: string, size_in_bits: number): number {
    assert_ok(size_in_bits > 0);
    assert_ok(size_in_bits <= 32);
    const bytes = new Uint8Array(4);
    this.read_bits_internal(bytes, size_in_bits);

    const view = new DataView(bytes.buffer);
    switch (this.m_packed_byte_order) {
      case e_bitstream_byte_order._bitstream_byte_order_little_endian:
        return view.getFloat32(0, true);
      case e_bitstream_byte_order._bitstream_byte_order_big_endian:
        return view.getFloat32(0, false);
    }
  }

  read_i16(size_in_bits: number): number {
    assert_ok(size_in_bits > 0);
    assert_ok(size_in_bits <= 16);
    const bytes = new Uint8Array(2);
    this.read_bits_internal(bytes, size_in_bits);

    const view = new DataView(bytes.buffer);
    switch (this.m_packed_byte_order) {
      case e_bitstream_byte_order._bitstream_byte_order_little_endian:
        return view.getInt16(0, true);
      case e_bitstream_byte_order._bitstream_byte_order_big_endian:
        return view.getInt16(0, false);
    }
  }

  read_signed_integer(_name: string, size_in_bits: number): number {
    let result = this.read_integer(_name, size_in_bits);

    if (size_in_bits < 32 && (result & (1 << (size_in_bits - 1))) !== 0) {
      result |= ~((1 << size_in_bits) - 1);
    }

    return result | 0;
  }

  read_qword(size_in_bits: number): bigint {
    assert_ok(size_in_bits > 0);
    assert_ok(size_in_bits <= 64);
    const bytes = new Uint8Array(8);
    this.read_bits_internal(bytes, size_in_bits);

    const view = new DataView(bytes.buffer);
    switch (this.m_packed_byte_order) {
      case e_bitstream_byte_order._bitstream_byte_order_little_endian:
        return view.getBigUint64(0, true);
      case e_bitstream_byte_order._bitstream_byte_order_big_endian:
        return view.getBigUint64(0, false);
    }
  }

  read_identifier(_identifier: string): void {
    throw new Error("unimplemented");
  }

  read_point3d(
    point: { x: number; y: number; z: number },
    axis_encoding_size_in_bits: number
  ): void {
    assert_ok(
      axis_encoding_size_in_bits > 0 && axis_encoding_size_in_bits <= 32
    );

    point.x = this.read_integer("???", axis_encoding_size_in_bits);
    point.y = this.read_integer("???", axis_encoding_size_in_bits);
    point.z = this.read_integer("???", axis_encoding_size_in_bits);
  }

  read_point3d_efficient(
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

    point.x = this.read_integer("???", axis_encoding_size_in_bits.x);
    point.y = this.read_integer("???", axis_encoding_size_in_bits.y);
    point.z = this.read_integer("???", axis_encoding_size_in_bits.z);
  }

  read_qword_internal(_size_in_bits: number): bigint {
    throw new Error("unimplemented");
  }

  read_secure_address(_address: unknown): void {
    throw new Error("unimplemented");
  }

  angle_to_axes_internal(
    up: real_vector3d,
    angle: number,
    forward: real_vector3d
  ): void {
    angle_to_axes(up, angle, forward);
  }

  read_string(_string: { value: string }, _max_string_size: number): void {
    throw new Error("unimplemented");
  }

  read_string_utf8(max_string_size: number): string {
    assert_ok(this.reading());
    assert_ok(max_string_size > 0);

    const bytes = new Uint8Array(max_string_size);

    for (let i = 0; i < max_string_size; i++) {
      const byte = this.read_integer("???", 8);
      bytes[i] = byte;

      if (byte === 0) {
        return new TextDecoder("utf-8").decode(bytes.subarray(0, i));
      }
    }

    throw new BitstreamError("Exceeded max string size reading utf8 string.");
  }

  read_string_extended_ascii(max_string_size: number): string {
    assert_ok(this.reading());
    assert_ok(max_string_size > 0);

    const bytes = new Uint8Array(max_string_size);

    for (let i = 0; i < max_string_size; i++) {
      const byte = this.read_integer("???", 8);
      bytes[i] = byte;

      if (byte === 0) {
        return String.fromCharCode(...Array.from(bytes.subarray(0, i)));
      }
    }

    throw new BitstreamError("Exceeded max string size reading utf8 string.");
  }

  read_string_wchar(max_string_size: number): string {
    assert_ok(this.reading());
    assert_ok(max_string_size > 0);

    const characters: string[] = [];

    for (let i = 0; i < max_string_size; i++) {
      const character = this.read_integer("???", 16);

      if (character === 0) {
        return characters.join("");
      }
      characters.push(String.fromCharCode(character));
    }

    throw new BitstreamError("Exceeded max string size reading wchar string.");
  }

  read_unit_vector(_unit_vector: unknown, _size_in_bits: number): void {
    throw new Error("unimplemented");
  }

  read_vector(
    _vector: unknown,
    _min_value: number,
    _max_value: number,
    _step_count_size_in_bits: number,
    _size_in_bits: number
  ): void {
    throw new Error("unimplemented");
  }

  begin_reading(): void {
    this.reset(e_bitstream_state._bitstream_state_reading);
  }

  reading(): boolean {
    return (
      this.m_state === e_bitstream_state._bitstream_state_reading ||
      this.m_state ===
        e_bitstream_state._bitstream_state_read_only_for_consistency
    );
  }

  finish_reading(): void {
    this.m_state = e_bitstream_state._bitstream_state_read_finished;
  }

  get_data(): readonly [Uint8Array, number] {
    return [this.m_data, this.m_data_size_bytes];
  }

  private reset(state: e_bitstream_state): void {
    this.m_state = state;
    this.current_stream_bit_position = 0;
    this.current_stream_byte_position = 0;
  }

  private set_data(data: Uint8Array): void {
    this.m_data = data;
    this.m_data_size_bytes = data.length;
    this.reset(e_bitstream_state._bitstream_state_initial);
  }

  private skip(_bits_to_skip: number): void {
    throw new Error("unimplemented");
  }

  private would_overflow(_size_in_bits: number): boolean {
    throw new Error("unimplemented");
  }

  static axes_compute_reference_internal(
    up: real_vector3d,
    forward_reference: real_vector3d,
    left_reference: real_vector3d
  ): void {
    axes_compute_reference(up, forward_reference, left_reference);
  }

  private static axes_to_angle_internal(
    forward: real_vector3d,
    up: real_vector3d
  ): number {
    return axes_to_angle(forward, up);
  }
}
