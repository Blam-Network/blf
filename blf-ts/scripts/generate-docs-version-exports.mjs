/**
 * Reads each src/versions/<game>/<build>.ts barrel and:
 * - writes docs/.vitepress/version-exports.json (sidebar + components)
 * - writes docs/guide/versions/<game>/<build_id>.md per bundle
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");
const versionsDir = path.join(root, "src", "versions");
const docsVersionsDir = path.join(root, "docs", "guide", "versions");
const outPath = path.join(root, "docs", ".vitepress", "version-exports.json");
const TS_EXT = /\.ts$/;
const AUTOGEN_MARKER = "<!-- autogen:version-build -->";

/** GitHub blob base for chunk source links in the docs. */
const SOURCE_REPO_BASE = "https://github.com/Blam-Network/blf/blob/main/blf-ts";

/** @type {Record<string, string>} */
const GAME_LABELS = {
  haloreach: "Halo Reach",
  haloreach_mcc: "Halo: MCC - Halo: Reach",
  mcc: "Halo: MCC",
  halo3: "Halo 3",
  halo3odst: "Halo 3: ODST",
};

/** @type {Record<string, string>} */
const BUILD_LABELS = {
  v08516_10_02_19_1607_omaha_alpha: "Private Alpha",
  v09449_10_03_25_1545_omaha_beta: "Private Beta",
  v09664_10_04_06_2121_omaha_beta: "Private Beta - Title Update 1",
  v09730_10_04_09_1309_omaha_delta: "Public Beta",
  v11860_10_07_24_0147_omaha_release: "Release",
  v12065_11_08_24_1738_tu1actual: "Title Update 1 (Latest)",
  v_untracked_25_08_16_1352: "16th Aug 2025 (Latest)",
  v2025_08_16_178512_1_release: "16th Aug 2025 (Latest)",
  v12070_08_09_05_2031_halo3_ship: "Title Update 2 (Latest)",
  v13895_09_04_27_2201_atlas_release: "Release (Latest)",
};

/** @type {Record<string, string>} */
const BUILD_DESCRIPTIONS = {
  v08516_10_02_19_1607_omaha_alpha:
    'The earliest available build of Halo: Reach. This build self-identifies as "Pre-Alpha" though it seems more likely to be a Private Alpha build.',
  v09449_10_03_25_1545_omaha_beta:
    "A leaked Private Beta build of Halo: Reach.",
  v09664_10_04_06_2121_omaha_beta:
    "A leaked Title Update for a leaked private beta version of Halo: Reach for the Xbox 360. This version was built days before the Public Beta build.",
  v09730_10_04_09_1309_omaha_delta:
    "The beta, it's super, the beta, it's super AWESOME",
  v11860_10_07_24_0147_omaha_release:
    "The initial release version of Halo: Reach on the Xbox 360.",
  v12065_11_08_24_1738_tu1actual:
    "Halo: Reach's first and only Xbox 360 Title Update.",
  v_untracked_25_08_16_1352: "Halo: Reach MCC - 16th August 2025",
  v2025_08_16_178512_1_release:
    "Halo: The Master Chief Collection - 16th August 2025",
  v12070_08_09_05_2031_halo3_ship: "Halo 3's second and final Title Update.",
  v13895_09_04_27_2201_atlas_release:
    "The initial release version of Halo 3: ODST on the Xbox 360.",
};

const CHUNK_DECORATOR_RE =
  /@blf\.chunk\(\s*["']([^"']+)["']\s*,\s*([\d.]+)\s*\)/;

/**
 * @param {string} chunkFilePath
 * @returns {{ signature: string; version: string } | null}
 */
function parseChunkHeader(chunkFilePath) {
  const text = fs.readFileSync(chunkFilePath, "utf8");
  const match = text.match(CHUNK_DECORATOR_RE);
  if (!match) {
    return null;
  }
  return { signature: match[1], version: match[2] };
}

/**
 * @param {string} barrelPath Absolute path to `src/versions/<game>/<build>.ts`
 * @returns {{ name: string; source: string; signature: string; version: string }[]}
 */
function parseChunkExports(barrelPath) {
  const text = fs.readFileSync(barrelPath, "utf8");
  const barrelDir = path.dirname(barrelPath);
  /** @type {Map<string, { name: string; source: string; signature: string; version: string }>} */
  const chunks = new Map();
  for (const match of text.matchAll(/export \* from ["']([^"']+)["']/g)) {
    const exportPath = match[1];
    if (!exportPath.includes("/chunks/")) {
      continue;
    }
    const name = path.basename(exportPath).replace(TS_EXT, "");
    let resolved = path.normalize(path.join(barrelDir, exportPath));
    if (!resolved.endsWith(".ts")) {
      resolved += ".ts";
    }
    const source = path.relative(root, resolved).replace(/\\/g, "/");
    const header = parseChunkHeader(resolved);
    chunks.set(name, {
      name,
      source,
      signature: header?.signature ?? "—",
      version: header?.version ?? "—",
    });
  }
  return [...chunks.values()].sort((a, b) => {
    const sig = a.signature.localeCompare(b.signature);
    if (sig !== 0) {
      return sig;
    }
    return a.name.localeCompare(b.name);
  });
}

/** @param {{ game: string; buildId: string; label: string; buildLabel: string; importPath: string; docLink: string; chunks: { name: string; source: string; signature: string; version: string }[] }} bundle */
function buildPageMarkdown(bundle) {
  const description =
    BUILD_DESCRIPTIONS[bundle.buildId] ?? `${bundle.label} build.`;
  const sampleChunk = bundle.chunks[0]?.name ?? "s_blf_chunk_start_of_file";

  return `${AUTOGEN_MARKER}

# \`${bundle.buildId}\`

${description}

\`\`\`ts
import { ${sampleChunk} } from "${bundle.importPath}";
\`\`\`

## Chunk exports

<VersionChunkExports game="${bundle.game}" build-id="${bundle.buildId}" />
`;
}

/** @type {{ game: string; buildId: string; label: string; buildLabel: string; importPath: string; docLink: string; chunks: { name: string; source: string; signature: string; version: string }[] }[]} */
const bundles = [];
/** @type {Set<string>} */
const expectedBuildPages = new Set();

for (const game of fs.readdirSync(versionsDir).sort()) {
  const gameDir = path.join(versionsDir, game);
  if (!fs.statSync(gameDir).isDirectory()) {
    continue;
  }
  for (const file of fs.readdirSync(gameDir).sort()) {
    if (!file.endsWith(".ts")) {
      continue;
    }
    const buildId = file.slice(0, -3);
    const chunks = parseChunkExports(path.join(gameDir, file));
    const docLink = `/guide/versions/${game}/${buildId}`;
    const bundle = {
      game,
      buildId,
      label: GAME_LABELS[game] ?? game,
      buildLabel: BUILD_LABELS[buildId] ?? buildId,
      importPath: `@blamnetwork/blf/${game}/${buildId}`,
      docLink,
      chunks,
    };
    bundles.push(bundle);

    const pageDir = path.join(docsVersionsDir, game);
    const pagePath = path.join(pageDir, `${buildId}.md`);
    fs.mkdirSync(pageDir, { recursive: true });
    fs.writeFileSync(pagePath, `${buildPageMarkdown(bundle)}\n`);
    expectedBuildPages.add(pagePath);
  }
}

// Remove stale autogenerated build pages.
for (const game of fs.readdirSync(docsVersionsDir)) {
  const gameDir = path.join(docsVersionsDir, game);
  if (!fs.statSync(gameDir).isDirectory()) {
    continue;
  }
  for (const file of fs.readdirSync(gameDir)) {
    if (file === "index.md" || !file.endsWith(".md")) {
      continue;
    }
    const pagePath = path.join(gameDir, file);
    const text = fs.readFileSync(pagePath, "utf8");
    if (text.startsWith(AUTOGEN_MARKER) && !expectedBuildPages.has(pagePath)) {
      fs.unlinkSync(pagePath);
    }
  }
}

fs.mkdirSync(path.dirname(outPath), { recursive: true });
fs.writeFileSync(
  outPath,
  `${JSON.stringify({ sourceRepoBase: SOURCE_REPO_BASE, bundles }, null, 2)}\n`
);
console.log(
  `Wrote ${path.relative(root, outPath)} (${bundles.length} bundles, ${expectedBuildPages.size} build pages)`
);
