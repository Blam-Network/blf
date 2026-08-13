import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  get_formatted_event_string,
  read_datamine_file,
} from "./data_mine_management";

const here = dirname(fileURLToPath(import.meta.url));
const probePath = join(
  here,
  "../../../../../../web-tiger/datamine-drops/_probe.compressed.dat"
);

describe("read_datamine_file (v3)", () => {
  it("parses v3 compressed.dat header and joined occurrences", () => {
    let buf: Buffer;
    try {
      buf = readFileSync(probePath);
    } catch {
      // Fixture lives next to web-tiger; skip if not present in CI checkout.
      return;
    }

    const file = read_datamine_file(buf);
    expect(file).toBeDefined();
    expect(file?.header.version).toBe(3);
    if (file?.header.version !== 3) {
      return;
    }
    expect(file.header.application_name).toContain("tiger_release_internal");
    expect(file.header.build_string).toContain("36735");
    expect(file.events.length).toBeGreaterThan(10);

    const owc = file.events.find((e) =>
      e.header.event_name.includes("Using world controller type")
    );
    expect(owc).toBeDefined();
    expect(get_formatted_event_string(owc!)).toBe(
      "world_controller: Using world controller type 'OWC'."
    );
  });
});
