import { c } from "@craftycodie/cstruct";
import { BlfError } from "./error";

/** Fixed 4-byte BLF chunk signature; reverses byte order on little-endian. */
export class ChunkSignature extends c.AdvancedType<string> {
  readonly byteSize = 4;

  read(
    bytes: Uint8Array,
    offset: number,
    endian: c.Endian,
    _label: string,
  ): string {
    if (offset + this.byteSize > bytes.length) {
      throw new BlfError(
        `Cannot read chunk signature: need ${this.byteSize} bytes at offset ${offset}, have ${bytes.length - offset}`,
      );
    }
    const w0 = bytes[offset]!;
    const w1 = bytes[offset + 1]!;
    const w2 = bytes[offset + 2]!;
    const w3 = bytes[offset + 3]!;
    if (endian === "little") {
      return String.fromCharCode(w3, w2, w1, w0);
    }
    return String.fromCharCode(w0, w1, w2, w3);
  }

  write(
    bytes: Uint8Array,
    offset: number,
    value: string,
    endian: c.Endian,
    _label: string,
  ): void {
    if (value.length !== this.byteSize) {
      throw new BlfError(
        `BLF chunk signature must be exactly 4 characters, got ${value.length}: "${value}"`,
      );
    }
    if (offset + this.byteSize > bytes.length) {
      throw new BlfError(
        `Cannot write chunk signature: need ${this.byteSize} bytes at offset ${offset}, have ${bytes.length - offset}`,
      );
    }
    const a = value.charCodeAt(0);
    const b = value.charCodeAt(1);
    const c0 = value.charCodeAt(2);
    const d = value.charCodeAt(3);
    if (endian === "little") {
      bytes[offset] = d;
      bytes[offset + 1] = c0;
      bytes[offset + 2] = b;
      bytes[offset + 3] = a;
    } else {
      bytes[offset] = a;
      bytes[offset + 1] = b;
      bytes[offset + 2] = c0;
      bytes[offset + 3] = d;
    }
  }
}

/** BLF chunk signature field for `@c.field`. */
export function chunkSignature(): ChunkSignature {
  return new ChunkSignature();
}
