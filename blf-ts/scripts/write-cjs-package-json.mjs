import { writeFileSync } from "node:fs";
import { join } from "node:path";

writeFileSync(
  join(import.meta.dirname, "..", "dist-cjs", "package.json"),
  JSON.stringify({ type: "commonjs" }),
);
