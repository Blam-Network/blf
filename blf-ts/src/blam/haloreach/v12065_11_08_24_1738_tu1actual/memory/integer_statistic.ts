import { CFieldType, c } from "@craftycodie/cstruct";

export interface s_integer_statistic {
  changed: boolean;
  value: number;
}

class CIntegerStatistic extends CFieldType<s_integer_statistic> {
  getSize(): number {
    return 2;
  }

  read(
    bytes: Uint8Array,
    offset: number,
    endian: c.Endian
  ): s_integer_statistic {
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const raw = view.getUint16(offset, endian === "little");
    return { value: raw >> 1, changed: (raw & 1) === 1 };
  }

  write(
    bytes: Uint8Array,
    offset: number,
    value: s_integer_statistic,
    endian: c.Endian
  ): void {
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const raw = ((value.value << 1) | (value.changed ? 1 : 0)) & 0xffff;
    view.setUint16(offset, raw, endian === "little");
  }
}

export const integer_statistic = () => new CIntegerStatistic();

export const integer_statistic_array = (count: number) =>
  integer_statistic().array(count);

export const s_real_point3d = c.struct({
  x: c.f32(),
  y: c.f32(),
  z: c.f32(),
});
export type s_real_point3d = c.infer<typeof s_real_point3d>;
