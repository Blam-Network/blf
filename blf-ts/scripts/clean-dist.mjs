import fs from "node:fs/promises";

for (const dir of ["dist", "dist-cjs"]) {
  await fs.rm(dir, { recursive: true, force: true });
}
