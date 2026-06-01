import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { expect } from "vitest";

export function shouldUpdateBinarySnapshots(): boolean {
  return (
    process.env.UPDATE_SNAPSHOTS === "1" ||
    process.env.VITEST_UPDATE_SNAPSHOTS === "true" ||
    process.env.VITEST_UPDATE_SNAPSHOTS === "1"
  );
}

/** Compare `actual` to a committed file, or rewrite it when `UPDATE_SNAPSHOTS=1`. */
export function expectBinarySnapshot(
  actual: Uint8Array,
  snapshotPath: string
): void {
  if (shouldUpdateBinarySnapshots()) {
    writeFileSync(snapshotPath, actual);
    return;
  }

  if (!existsSync(snapshotPath)) {
    throw new Error(
      `Missing binary snapshot: ${snapshotPath}\n` +
        "Run with UPDATE_SNAPSHOTS=1 to write it into the repo."
    );
  }

  expect(Buffer.from(actual)).toEqual(readFileSync(snapshotPath));
}
