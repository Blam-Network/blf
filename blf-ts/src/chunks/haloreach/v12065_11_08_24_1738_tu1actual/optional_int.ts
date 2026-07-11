import { c } from "@craftycodie/cstruct";

type SignedStorage = "i16" | "i32";

/** Signed integer that uses `-1` on the wire for `null` (Reach challenge overrides). */
export class OptionalSignedInt extends c.AdvancedType<number | null> {
  readonly byteSize: number;

  constructor(private readonly storage: SignedStorage) {
    super();
    this.byteSize = storage === "i16" ? 2 : 4;
  }

  read(
    bytes: Uint8Array,
    offset: number,
    endian: c.Endian,
    label: string
  ): number | null {
    if (offset + this.byteSize > bytes.length) {
      throw new c.CStructError(
        `Cannot read ${label}: need ${this.byteSize} bytes at offset ${offset}`
      );
    }
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const little = endian === "little";
    const value =
      this.storage === "i16"
        ? view.getInt16(offset, little)
        : view.getInt32(offset, little);
    return value === -1 ? null : value;
  }

  write(
    bytes: Uint8Array,
    offset: number,
    value: number | null,
    endian: c.Endian,
    label: string
  ): void {
    if (offset + this.byteSize > bytes.length) {
      throw new c.CStructError(
        `Cannot write ${label}: need ${this.byteSize} bytes at offset ${offset}`
      );
    }
    if (
      value !== null &&
      (!Number.isInteger(value) || typeof value !== "number")
    ) {
      throw new c.CStructError(`${label}: expected integer or null`);
    }
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const little = endian === "little";
    const wire = value ?? -1;
    if (this.storage === "i16") {
      view.setInt16(offset, wire, little);
    } else {
      view.setInt32(offset, wire, little);
    }
  }
}

export function optional_i16(): OptionalSignedInt {
  return new OptionalSignedInt("i16");
}

export function optional_i32(): OptionalSignedInt {
  return new OptionalSignedInt("i32");
}
