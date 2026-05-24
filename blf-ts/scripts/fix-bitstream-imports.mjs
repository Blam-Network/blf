import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { join, relative, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const srcRoot = join(fileURLToPath(new URL("..", import.meta.url)), "src");
const bitstreamRoot = join(srcRoot, "bitstream");

function walk(dir) {
  for (const ent of readdirSync(dir)) {
    const p = join(dir, ent);
    if (statSync(p).isDirectory()) {
      walk(p);
      continue;
    }
    if (!p.endsWith(".ts")) continue;

    let content = readFileSync(p, "utf8");
    if (!content.includes("bitstream")) continue;

    const relPath = relative(dirname(p), bitstreamRoot).split("\\").join("/");
    const target = relPath.startsWith(".") ? relPath : `./${relPath}`;

    const next = content.replace(
      /from ["'](?:@Blam-Network\/blf\/bitstream|\.[^"']*bitstream)["']/g,
      `from "${target}"`,
    );

    if (next !== content) {
      writeFileSync(p, next);
      console.log(relative(srcRoot, p), "->", target);
    }
  }
}

walk(srcRoot);
