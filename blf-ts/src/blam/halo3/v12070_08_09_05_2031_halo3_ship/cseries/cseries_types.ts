import { c } from "@craftycodie/cstruct";

/**
 * `time32_t`: unsigned 32-bit seconds since the Unix epoch, exposed as a `Date`
 * (blf_lib `time32_t`). Writes truncate to whole seconds.
 */
export const time32_t = () =>
  c.u32().transform(
    (seconds) => new Date(seconds * 1000),
    (date: Date) => Math.floor(date.getTime() / 1000)
  );

/** IPv4 address stored as a 32-bit integer, exposed as dotted-quad text (blf_lib `Ipv4Addr`). */
export const ipv4_address = () =>
  c.u32().transform(
    (raw) =>
      `${raw >>> 24}.${(raw >>> 16) & 0xff}.${(raw >>> 8) & 0xff}.${raw & 0xff}`,
    (address: string) => {
      const parts = address.split(".").map((part) => Number.parseInt(part, 10));
      if (
        parts.length !== 4 ||
        parts.some((part) => !Number.isInteger(part) || part < 0 || part > 255)
      ) {
        throw new Error(`Invalid IPv4 address "${address}"`);
      }
      return (
        ((parts[0] << 24) | (parts[1] << 16) | (parts[2] << 8) | parts[3]) >>> 0
      );
    }
  );

/**
 * A 16-bit statistic stored shifted left by one bit, read as `raw >> 1`
 * (blf_lib's `#[br(map = |x: i16| x >> 1)]`). Writes `value << 1`.
 */
export const shifted_i16 = () =>
  c.i16().transform(
    (raw) => raw >> 1,
    (value: number) => ((value << 1) << 16) >> 16
  );
