export class BitstreamError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "BitstreamError";
  }
}

export function assert_ok(
  condition: unknown,
  message?: string,
): asserts condition {
  if (!condition) {
    throw new BitstreamError(message ?? "assert_ok failed");
  }
}
