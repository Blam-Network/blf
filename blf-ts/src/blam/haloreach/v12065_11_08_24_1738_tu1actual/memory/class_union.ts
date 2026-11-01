import { CFieldType, c } from "@craftycodie/cstruct";

export type UnionTagType = "u8" | "i8" | "i32";

export type UnionArms =
  | Record<number, readonly UnionMember[]>
  | (() => Record<number, readonly UnionMember[]>);

export interface UnionMember {
  field: any;
  offset: number;
  property: string;
}

function view_of(bytes: Uint8Array): DataView {
  return new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
}

function read_member(
  member: UnionMember,
  bytes: Uint8Array,
  offset: number,
  endian: c.Endian
): unknown {
  const at = offset + member.offset;
  const { field } = member;
  if (typeof field === "function") {
    return c.read(field, bytes.subarray(at, at + c.sizeof(field)), endian);
  }
  return field.read(bytes, at, endian);
}

function write_member(
  member: UnionMember,
  bytes: Uint8Array,
  offset: number,
  value: unknown,
  endian: c.Endian
): void {
  if (value === undefined) {
    return;
  }
  const at = offset + member.offset;
  const { field } = member;
  if (typeof field === "function") {
    bytes.set(c.write(value as object, endian), at);
  } else if (field instanceof CFieldType) {
    field.write(bytes, at, value, endian);
  } else {
    bytes.set(field.write(value, endian), at);
  }
}

export class CClassUnionField<T extends object> extends CFieldType<T> {
  constructor(
    private readonly create: () => T,
    private readonly size: number,
    private readonly tag: {
      property: string;
      offset: number;
      type: UnionTagType;
    },
    private readonly arms_source: UnionArms,
    private readonly shared: readonly UnionMember[] = []
  ) {
    super();
  }

  private resolved?: Record<number, readonly UnionMember[]>;

  private get arms(): Record<number, readonly UnionMember[]> {
    this.resolved ??=
      typeof this.arms_source === "function"
        ? this.arms_source()
        : this.arms_source;
    return this.resolved;
  }

  getSize(): number {
    return this.size;
  }

  read(bytes: Uint8Array, offset: number, endian: c.Endian): T {
    const view = view_of(bytes);
    const at = offset + this.tag.offset;
    let tag: number;
    if (this.tag.type === "u8") {
      tag = view.getUint8(at);
    } else if (this.tag.type === "i8") {
      tag = view.getInt8(at);
    } else {
      tag = view.getInt32(at, endian === "little");
    }
    const value = this.create() as Record<string, unknown>;
    value[this.tag.property] = tag;
    for (const member of [...(this.arms[tag] ?? []), ...this.shared]) {
      value[member.property] = read_member(member, bytes, offset, endian);
    }
    return value as T;
  }

  write(bytes: Uint8Array, offset: number, value: T, endian: c.Endian): void {
    const record = value as Record<string, unknown>;
    const tag = Number(record[this.tag.property] ?? 0);
    for (const member of [...(this.arms[tag] ?? []), ...this.shared]) {
      write_member(member, bytes, offset, record[member.property], endian);
    }
    const view = view_of(bytes);
    const at = offset + this.tag.offset;
    if (this.tag.type === "u8") {
      view.setUint8(at, tag & 0xff);
    } else if (this.tag.type === "i8") {
      view.setInt8(at, tag);
    } else {
      view.setInt32(at, tag, endian === "little");
    }
  }
}

export const class_union = <T extends object>(
  create: () => T,
  size: number,
  tag: { property: string; offset: number; type: UnionTagType },
  arms: UnionArms,
  shared: readonly UnionMember[] = []
) => new CClassUnionField<T>(create, size, tag, arms, shared);
