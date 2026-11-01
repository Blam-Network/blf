import { CFieldType, c } from "@craftycodie/cstruct";

type CountType = "i8" | "u8" | "i16" | "u16" | "i32" | "u32";

const k_count_sizes: Record<CountType, number> = {
  i8: 1,
  u8: 1,
  i16: 2,
  u16: 2,
  i32: 4,
  u32: 4,
};

type Element = any;

function view_of(bytes: Uint8Array): DataView {
  return new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
}

function read_count(
  bytes: Uint8Array,
  offset: number,
  type: CountType,
  endian: c.Endian
): number {
  const view = view_of(bytes);
  const little = endian === "little";
  switch (type) {
    case "i8":
      return view.getInt8(offset);
    case "u8":
      return view.getUint8(offset);
    case "i16":
      return view.getInt16(offset, little);
    case "u16":
      return view.getUint16(offset, little);
    case "i32":
      return view.getInt32(offset, little);
    case "u32":
      return view.getUint32(offset, little);
  }
}

function write_count(
  bytes: Uint8Array,
  offset: number,
  type: CountType,
  value: number,
  endian: c.Endian
): void {
  const view = view_of(bytes);
  const little = endian === "little";
  switch (type) {
    case "i8":
      view.setInt8(offset, value);
      break;
    case "u8":
      view.setUint8(offset, value);
      break;
    case "i16":
      view.setInt16(offset, value, little);
      break;
    case "u16":
      view.setUint16(offset, value, little);
      break;
    case "i32":
      view.setInt32(offset, value, little);
      break;
    case "u32":
      view.setUint32(offset, value, little);
      break;
  }
}

export class CCountedArray<T> extends CFieldType<T[]> {
  private readonly element_size: number;

  constructor(
    private readonly element: Element,
    private readonly capacity: number,
    private readonly count_type: CountType,
    private readonly count_offset = -k_count_sizes[count_type],
    private readonly create: () => T = () => undefined as T
  ) {
    super();
    this.element_size = c.sizeof(element);
  }

  getSize(): number {
    return this.element_size * this.capacity;
  }

  read(bytes: Uint8Array, offset: number, endian: c.Endian): T[] {
    const count = Math.max(
      0,
      Math.min(
        this.capacity,
        read_count(bytes, offset + this.count_offset, this.count_type, endian)
      )
    );
    const out: T[] = [];
    for (let i = 0; i < count; i++) {
      const start = offset + i * this.element_size;
      if (typeof this.element === "function") {
        out.push(
          c.read(
            this.element,
            bytes.subarray(start, start + this.element_size),
            endian
          ) as T
        );
      } else {
        out.push(this.element.read(bytes, start, endian) as T);
      }
    }
    return out;
  }

  write(bytes: Uint8Array, offset: number, value: T[], endian: c.Endian): void {
    const count = Math.min(value.length, this.capacity);
    for (let i = 0; i < this.capacity; i++) {
      const entry = i < count ? value[i] : this.create();
      if (entry === undefined) {
        continue;
      }
      const at = offset + i * this.element_size;
      if (typeof this.element === "function") {
        bytes.set(c.write(entry as object, endian), at);
      } else if (this.element instanceof CFieldType) {
        this.element.write(bytes, at, entry, endian);
      } else {
        bytes.set(this.element.write(entry, endian), at);
      }
    }
    write_count(
      bytes,
      offset + this.count_offset,
      this.count_type,
      count,
      endian
    );
  }
}

export const counted_array = <T>(
  element: Element,
  capacity: number,
  count_type: CountType,
  options?: { count_offset?: number; create?: () => T }
) =>
  new CCountedArray<T>(
    element,
    capacity,
    count_type,
    options?.count_offset,
    options?.create
  );
