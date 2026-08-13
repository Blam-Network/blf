/** Datamine (`compressed.dat`) — header/event layouts v1–v3. */

import { c } from "@craftycodie/cstruct";

export enum e_datamine_parameter_type {
  _datamine_parameter_type_long = 0,
  _datamine_parameter_type_int64 = 1,
  _datamine_parameter_type_float = 2,
  _datamine_parameter_type_string = 3,
}

export type s_datamine_value_string = { string: string };

export type s_datamine_parameter = {
  name: string;
  parameter_type: e_datamine_parameter_type;
  value_long?: number;
  value_int64?: bigint;
  value_float?: number;
  value_string?: s_datamine_value_string;
};

@c.struct()
export class c_datamine_game_info {
  @c.field("u64")
  game_instance = 0n;

  @c.field(c.String(260))
  map = "";
}

@c.struct()
export class s_datamine_event_header {
  @c.field("u32")
  total_size = 0;

  @c.field(c.String(512))
  event_name = "";

  @c.field(c.String(512))
  parameter_signature = "";

  @c.field("u32")
  priority = 0;

  @c.field("u32")
  event_index = 0;

  @c.field(c_datamine_game_info)
  game_info = new c_datamine_game_info();

  @c.field("u64")
  event_date = 0n;
}

export type s_datamine_event = {
  header: s_datamine_event_header;
  categories: string[];
  parameters: s_datamine_parameter[];
};

/** File header v1: BOM + major=1, then session fields. */
@c.struct()
export class s_data_mine_header_v1 {
  readonly version = 1 as const;

  @c.field("u16")
  byte_order_marker_fffe = 0xfffe;

  @c.field("u16")
  version_major = 1;

  @c.field(c.String(128))
  sessionid = "";

  @c.field(c.String(32))
  build_string = "";

  @c.field("i32")
  build_number = 0;

  @c.field(c.String(160))
  systemid = "";

  @c.field(c.String(32))
  title = "";

  @c.field("u64")
  session_start_date = 0n;
}

/** Adds source_flag (backend bucketing). */
@c.struct()
export class s_data_mine_header_v2 {
  readonly version = 2 as const;

  @c.field("u16")
  byte_order_marker_fffe = 0xfffe;

  @c.field("u16")
  version_major = 2;

  @c.field(c.String(128))
  sessionid = "";

  @c.field(c.String(32))
  build_string = "";

  @c.field("i32")
  build_number = 0;

  @c.field(c.String(160))
  systemid = "";

  @c.field(c.String(32))
  title = "";

  @c.field("u64")
  session_start_date = 0n;

  @c.field("u8")
  source_flag = 0;
}

/** File header v3: major=3 + minor + application_name extras. */
@c.struct()
export class s_data_mine_header_v3 {
  readonly version = 3 as const;

  @c.field("u16")
  byte_order_marker_fffe = 0xfffe;

  @c.field("u16")
  version_major = 3;

  @c.field("u16")
  version_minor = 1;

  @c.field(c.String(128))
  sessionid = "";

  @c.field(c.String(32))
  build_string = "";

  @c.field("i32")
  build_number = 0;

  @c.field(c.String(160))
  systemid = "";

  @c.field(c.String(32))
  title = "";

  @c.field("u64")
  session_start_date = 0n;

  @c.field("u8")
  source_flag = 0;

  @c.field("u8")
  source_flag_pad = 0;

  @c.field(c.String(64))
  unknown0 = "";

  @c.field(c.String(32))
  application_name = "";

  @c.field("u32")
  unknown1 = 0;
}

export type s_data_mine_header =
  | s_data_mine_header_v1
  | s_data_mine_header_v2
  | s_data_mine_header_v3;

export type s_datamine_file = {
  header: s_data_mine_header;
  events: s_datamine_event[];
};

@c.struct()
class s_datamine_bom_major {
  @c.field("u16")
  byte_order_marker_fffe = 0;

  @c.field("u16")
  version_major = 0;
}

/** v3 event definition: size + format(1.x) + name/sig + priority (cats/params follow). */
@c.struct()
class s_datamine_v3_definition_prefix {
  @c.field("u32")
  total_size = 0;

  @c.field("u8")
  format_major = 1;

  @c.field("u8")
  format_minor = 0;

  @c.field(c.String(512))
  event_name = "";

  @c.field(c.String(512))
  parameter_signature = "";

  @c.field("u32")
  priority = 0;
}

/** v3 event occurrence: size + type=2 + index + priority + filetime (values follow). */
@c.struct()
class s_datamine_v3_occurrence_prefix {
  @c.field("u32")
  total_size = 0;

  @c.field("u8")
  record_type = 2;

  @c.field("u32")
  event_index = 0;

  @c.field("u32")
  priority = 0;

  @c.field("u64")
  event_date = 0n;
}

@c.struct()
class s_datamine_parameter_schema {
  @c.field(c.String(32))
  name = "";

  @c.field("u32")
  parameter_type = e_datamine_parameter_type._datamine_parameter_type_long;
}

type Endian = c.Endian;

function readCString(buf: Buffer, offset: number, max: number): string {
  let end = offset;
  const limit = Math.min(buf.length, offset + max);
  while (end < limit && buf[end] !== 0) {
    end++;
  }
  return buf.toString("utf8", offset, end);
}

function readU32(buf: Buffer, offset: number, endian: Endian): number {
  return endian === "big" ? buf.readUInt32BE(offset) : buf.readUInt32LE(offset);
}

function readU64(buf: Buffer, offset: number, endian: Endian): bigint {
  return endian === "big"
    ? buf.readBigUInt64BE(offset)
    : buf.readBigUInt64LE(offset);
}

function readF32(buf: Buffer, offset: number, endian: Endian): number {
  return endian === "big" ? buf.readFloatBE(offset) : buf.readFloatLE(offset);
}

function parameterValue(
  param: s_datamine_parameter
): string | number | bigint | undefined {
  switch (param.parameter_type) {
    case e_datamine_parameter_type._datamine_parameter_type_long:
      return param.value_long;
    case e_datamine_parameter_type._datamine_parameter_type_int64:
      return param.value_int64;
    case e_datamine_parameter_type._datamine_parameter_type_float:
      return param.value_float;
    case e_datamine_parameter_type._datamine_parameter_type_string:
      return param.value_string?.string;
    default:
      return undefined;
  }
}

/**
 * Lightweight sprintf for datamine format strings (`%s`, `%d`, `%u`, `%f`, `%x`,
 * `%llx`, `%.*s`, `%%`). Named parameters are ignored as format args.
 */
export function get_formatted_event_string(
  event: s_datamine_event
): string | undefined {
  const args = event.parameters
    .filter((p) => !p.name)
    .map(parameterValue)
    .filter((v): v is string | number | bigint => v !== undefined);

  let argIndex = 0;
  return event.header.event_name.replace(
    /%%|%(\.\*)?[-+0 #]*\d*(?:\.\d+)?(?:ll|l|h|I64)?[diuoxXfFeEgGsc]/g,
    (match) => {
      if (match === "%%") {
        return "%";
      }
      if (argIndex >= args.length) {
        return match;
      }
      const arg = args[argIndex++];
      if (match.includes(".*s") || match.endsWith("s") || match.endsWith("c")) {
        return String(arg);
      }
      if (
        match.endsWith("x") ||
        match.endsWith("X") ||
        match.includes("llx") ||
        match.includes("llX")
      ) {
        const n = typeof arg === "bigint" ? arg : BigInt(Number(arg));
        return match.endsWith("X") || match.includes("llX")
          ? n.toString(16).toUpperCase()
          : n.toString(16);
      }
      if (
        match.endsWith("f") ||
        match.endsWith("F") ||
        match.endsWith("e") ||
        match.endsWith("g")
      ) {
        return String(Number(arg));
      }
      return String(arg);
    }
  );
}

function readParameterWithValue(
  buf: Buffer,
  offset: number,
  endian: Endian,
  end: number
): { param: s_datamine_parameter; size: number } | undefined {
  const schemaSize = c.sizeof(s_datamine_parameter_schema);
  if (offset + schemaSize > end) {
    return undefined;
  }
  const schema = c.read(
    s_datamine_parameter_schema,
    buf.subarray(offset, offset + schemaSize),
    endian
  );
  let p = offset + schemaSize;
  const param: s_datamine_parameter = {
    name: schema.name,
    parameter_type: schema.parameter_type as e_datamine_parameter_type,
  };

  switch (param.parameter_type) {
    case e_datamine_parameter_type._datamine_parameter_type_long: {
      if (p + 4 > end) return undefined;
      param.value_long = readU32(buf, p, endian);
      p += 4;
      break;
    }
    case e_datamine_parameter_type._datamine_parameter_type_int64: {
      if (p + 8 > end) return undefined;
      param.value_int64 = readU64(buf, p, endian);
      p += 8;
      break;
    }
    case e_datamine_parameter_type._datamine_parameter_type_float: {
      if (p + 4 > end) return undefined;
      param.value_float = readF32(buf, p, endian);
      p += 4;
      break;
    }
    case e_datamine_parameter_type._datamine_parameter_type_string: {
      if (p + 4 > end) return undefined;
      const len = readU32(buf, p, endian);
      p += 4;
      if (p + len > end) return undefined;
      param.value_string = { string: buf.toString("utf8", p, p + len) };
      p += len;
      break;
    }
    default:
      return undefined;
  }

  return { param, size: p - offset };
}

function readParameterValues(
  buf: Buffer,
  offset: number,
  endian: Endian,
  end: number,
  schema: s_datamine_parameter[]
): s_datamine_parameter[] {
  let p = offset;
  const out: s_datamine_parameter[] = [];
  for (const def of schema) {
    const param: s_datamine_parameter = {
      name: def.name,
      parameter_type: def.parameter_type,
    };
    switch (def.parameter_type) {
      case e_datamine_parameter_type._datamine_parameter_type_long: {
        if (p + 4 > end) return out;
        param.value_long = readU32(buf, p, endian);
        p += 4;
        break;
      }
      case e_datamine_parameter_type._datamine_parameter_type_int64: {
        if (p + 8 > end) return out;
        param.value_int64 = readU64(buf, p, endian);
        p += 8;
        break;
      }
      case e_datamine_parameter_type._datamine_parameter_type_float: {
        if (p + 4 > end) return out;
        param.value_float = readF32(buf, p, endian);
        p += 4;
        break;
      }
      case e_datamine_parameter_type._datamine_parameter_type_string: {
        if (p + 4 > end) return out;
        const len = readU32(buf, p, endian);
        p += 4;
        if (p + len > end) return out;
        param.value_string = { string: buf.toString("utf8", p, p + len) };
        p += len;
        break;
      }
      default:
        return out;
    }
    out.push(param);
  }
  return out;
}

function readCountedStrings(
  buf: Buffer,
  offset: number,
  endian: Endian,
  end: number,
  stringSize: number
): { strings: string[]; next: number } | undefined {
  if (offset + 4 > end) {
    return undefined;
  }
  const count = readU32(buf, offset, endian);
  let p = offset + 4;
  if (count > 64 || p + count * stringSize > end) {
    return undefined;
  }
  const strings: string[] = [];
  for (let i = 0; i < count; i++) {
    strings.push(readCString(buf, p, stringSize));
    p += stringSize;
  }
  return { strings, next: p };
}

function readHeader(
  buf: Buffer,
  endian: Endian
): { header: s_data_mine_header; offset: number } | undefined {
  const probeSize = c.sizeof(s_datamine_bom_major);
  if (buf.length < probeSize) {
    return undefined;
  }

  const probe = c.read(
    s_datamine_bom_major,
    buf.subarray(0, probeSize),
    endian
  );

  try {
    if (probe.version_major === 1) {
      const size = c.sizeof(s_data_mine_header_v1);
      if (buf.length < size) return undefined;
      const header = c.read(s_data_mine_header_v1, buf.subarray(0, size), endian);
      return { header, offset: size };
    }
    if (probe.version_major === 2) {
      const size = c.sizeof(s_data_mine_header_v2);
      if (buf.length < size) return undefined;
      const header = c.read(s_data_mine_header_v2, buf.subarray(0, size), endian);
      return { header, offset: size };
    }
    if (probe.version_major === 3) {
      const size = c.sizeof(s_data_mine_header_v3);
      if (buf.length < size) return undefined;
      const header = c.read(s_data_mine_header_v3, buf.subarray(0, size), endian);
      return { header, offset: size };
    }
  } catch {
    return undefined;
  }

  return undefined;
}

type s_datamine_v3_definition = {
  priority: number;
  event_name: string;
  parameter_signature: string;
  categories: string[];
  parameters: s_datamine_parameter[];
};

function read_v3_definition(
  record: Buffer,
  endian: Endian
): s_datamine_v3_definition | undefined {
  const prefixSize = c.sizeof(s_datamine_v3_definition_prefix);
  if (record.length < prefixSize) {
    return undefined;
  }

  let prefix: s_datamine_v3_definition_prefix;
  try {
    prefix = c.read(
      s_datamine_v3_definition_prefix,
      record.subarray(0, prefixSize),
      endian
    );
  } catch {
    return undefined;
  }
  if (prefix.format_major !== 1) {
    return undefined;
  }

  const cats = readCountedStrings(
    record,
    prefixSize,
    endian,
    record.length,
    32
  );
  if (!cats) {
    return undefined;
  }

  if (cats.next + 4 > record.length) {
    return undefined;
  }
  const parameter_count = readU32(record, cats.next, endian);
  let p = cats.next + 4;
  const schemaSize = c.sizeof(s_datamine_parameter_schema);
  const parameters: s_datamine_parameter[] = [];
  for (let i = 0; i < parameter_count; i++) {
    if (p + schemaSize > record.length) {
      break;
    }
    const schema = c.read(
      s_datamine_parameter_schema,
      record.subarray(p, p + schemaSize),
      endian
    );
    parameters.push({
      name: schema.name,
      parameter_type: schema.parameter_type as e_datamine_parameter_type,
    });
    p += schemaSize;
  }

  return {
    priority: prefix.priority,
    event_name: prefix.event_name,
    parameter_signature: prefix.parameter_signature,
    categories: cats.strings,
    parameters,
  };
}

function read_v3_occurrence(
  record: Buffer,
  endian: Endian,
  definitions: Map<number, s_datamine_v3_definition>
): s_datamine_event | undefined {
  const prefixSize = c.sizeof(s_datamine_v3_occurrence_prefix);
  if (record.length < prefixSize) {
    return undefined;
  }

  let prefix: s_datamine_v3_occurrence_prefix;
  try {
    prefix = c.read(
      s_datamine_v3_occurrence_prefix,
      record.subarray(0, prefixSize),
      endian
    );
  } catch {
    return undefined;
  }
  if (prefix.record_type !== 2) {
    return undefined;
  }

  const def = definitions.get(prefix.priority);
  const parameters = def
    ? readParameterValues(
        record,
        prefixSize,
        endian,
        record.length,
        def.parameters
      )
    : [];

  const header = new s_datamine_event_header();
  header.total_size = prefix.total_size;
  header.event_name = def?.event_name ?? "";
  header.parameter_signature = def?.parameter_signature ?? "";
  header.priority = prefix.priority;
  header.event_index = prefix.event_index;
  header.event_date = prefix.event_date;

  return {
    header,
    categories: def?.categories ?? [],
    parameters,
  };
}

function read_v1_event(
  record: Buffer,
  endian: Endian
): s_datamine_event | undefined {
  const headerSize = c.sizeof(s_datamine_event_header);
  if (record.length < headerSize) {
    return undefined;
  }

  let header: s_datamine_event_header;
  try {
    header = c.read(
      s_datamine_event_header,
      record.subarray(0, headerSize),
      endian
    );
  } catch {
    return undefined;
  }

  const cats = readCountedStrings(
    record,
    headerSize,
    endian,
    record.length,
    32
  );
  if (!cats) {
    return undefined;
  }

  if (cats.next + 4 > record.length) {
    return undefined;
  }
  const parameter_count = readU32(record, cats.next, endian);
  let p = cats.next + 4;
  const parameters: s_datamine_parameter[] = [];
  for (let i = 0; i < parameter_count; i++) {
    const read = readParameterWithValue(record, p, endian, record.length);
    if (!read) {
      break;
    }
    parameters.push(read.param);
    p += read.size;
  }

  return {
    header,
    categories: cats.strings,
    parameters,
  };
}

function readRecords(
  buf: Buffer,
  offset: number,
  endian: Endian,
  version: 1 | 2 | 3
): s_datamine_event[] {
  const events: s_datamine_event[] = [];
  const definitions = new Map<number, s_datamine_v3_definition>();
  let p = offset;

  while (p + 4 <= buf.length) {
    const total_size = readU32(buf, p, endian);
    if (total_size < 5 || p + total_size > buf.length) {
      break;
    }
    const record = buf.subarray(p, p + total_size);

    if (version === 3) {
      const kind = record[4];
      if (kind === 1) {
        const def = read_v3_definition(record, endian);
        if (def) {
          definitions.set(def.priority, def);
        }
      } else if (kind === 2) {
        const event = read_v3_occurrence(record, endian, definitions);
        if (event) {
          events.push(event);
        }
      } else {
        break;
      }
    } else {
      const event = read_v1_event(record, endian);
      if (!event) {
        break;
      }
      events.push(event);
    }

    p += total_size;
  }

  return events;
}

/**
 * Parse a raw `compressed.dat` buffer (contents of the datamine ZIP entry).
 * Returns `undefined` if the header is unrecognized.
 */
export function read_datamine_file(
  buffer: Buffer | Uint8Array
): s_datamine_file | undefined {
  const buf = Buffer.isBuffer(buffer) ? buffer : Buffer.from(buffer);
  if (buf.length < c.sizeof(s_datamine_bom_major)) {
    return undefined;
  }

  let endian: Endian = "big";
  const bomBe = buf.readUInt16BE(0);
  if (bomBe === 0xfeff) {
    endian = "little";
  } else if (bomBe !== 0xfffe) {
    return undefined;
  }

  const parsed = readHeader(buf, endian);
  if (!parsed) {
    return undefined;
  }

  return {
    header: parsed.header,
    events: readRecords(buf, parsed.offset, endian, parsed.header.version),
  };
}
