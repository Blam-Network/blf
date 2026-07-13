# @blamnetwork/blf

[![npm](https://img.shields.io/npm/v/@blamnetwork/blf)](https://www.npmjs.com/package/@blamnetwork/blf)

TypeScript library for reading and writing Halo **BLF** (binary layout file) chunks — Reach, Halo 3, Halo 3: ODST, and Destiny builds.

Built on [@craftycodie/cstruct](https://www.npmjs.com/package/@craftycodie/cstruct) for packed struct layouts and a Halo-style **bitstream** for custom chunk bodies.

## Features

- Chunk discovery with `find_chunk` and `search_for_chunk`
- Per-game **version bundles** (`haloreach/*`, `haloreach_mcc/*`, `halo3/*`, `halo3odst/*`, `destiny/*`) with chunk classes and blam types for a specific exe build
- **`@blamnetwork/blf/helpers`** for cross-version Reach gametype conversion (TU1 ↔ MCC)
- Struct-backed chunks via `CStructBLFChunk` and `@blf.chunk` decorators (compiled away in published `dist/`)
- Bitstream reader/writer exported from the package root
- Little- and big-endian BLF headers

## Install

```bash
npm install @blamnetwork/blf
```

Consumers import from published **`dist/`** exports only. Decorators are lowered at build time — **you do not need `experimentalDecorators`** to use exported chunk classes.

## Usage

### Find and read chunks

```ts
import { readFileSync } from "node:fs";
import { find_chunk } from "@blamnetwork/blf";
import {
  s_blf_chunk_content_header,
  s_blf_chunk_game_variant,
} from "@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual";

const file = new Uint8Array(readFileSync("map.blf"));

const chdr = new s_blf_chunk_content_header();
find_chunk(file, chdr, "big");

const mpvr = new s_blf_chunk_game_variant();
find_chunk(file, mpvr, "big");
```

### Bitstream (root export only)

```ts
import { bitstream } from "@blamnetwork/blf";

const reader = new bitstream.BitstreamReader(buffer, "little");
const mode = reader.read_enum("game-mode", 4, e_game_mode);
```

### Game build subpaths

Each implementation build is a single module:

| Import | Build |
|--------|--------|
| `@blamnetwork/blf/haloreach/v08516_10_02_19_1607_omaha_alpha` | Halo: Reach Private Alpha |
| `@blamnetwork/blf/haloreach/v09449_10_03_25_1545_omaha_beta` | Halo: Reach Private Beta |
| `@blamnetwork/blf/haloreach/v09664_10_04_06_2121_omaha_beta` | Halo: Reach Private Beta TU1 |
| `@blamnetwork/blf/haloreach/v09730_10_04_09_1309_omaha_delta` | Halo: Reach Public Beta |
| `@blamnetwork/blf/haloreach/v11860_10_07_24_0147_omaha_release` | Halo: Reach Release |
| `@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual` | Halo: Reach TU1 |
| `@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_16_1352` | Halo: Reach MCC |
| `@blamnetwork/blf/halo3/v12070_08_09_05_2031_halo3_ship` | Halo 3 TU2 |
| `@blamnetwork/blf/halo3odst/v13895_09_04_27_2201_atlas_release` | ODST TU0 |
| `@blamnetwork/blf/destiny/v36735_13_12_02_1953_alpha` | Destiny Alpha |

Add a build by creating `src/versions/<game>/<build_id>.ts` and re-exporting its chunks — wildcard `exports` in `package.json` pick it up automatically.

Reach TU1 ↔ MCC gametype conversion: `@blamnetwork/blf/helpers` — see [Converting Reach Gametypes](https://blam-network.github.io/blf/guide/converting-reach-gametypes). Version import paths: [docs](https://blam-network.github.io/blf/guide/versions/).

## Documentation

Full guide, changelog, and API notes: **[blam-network.github.io/blf](https://blam-network.github.io/blf/)**

From `blf-ts/`: `npm run docs` (dev), `npm run docs:build`.

## Development

From `blf-ts/`:

```bash
npm install
npm run validate   # lint, test, typecheck, docs build
npm run build
npm run release    # bump version, tag, push (CI publishes on tag)
```

Chunk authoring (forking the library) uses Stage 3 decorators (`@blf.chunk`, `@c.struct`) with the same SWC/Vitest setup as the repo. Decorator order: `@blf.chunk` on the class, `@c.struct` below, `@c.field` on properties.

## License

MIT
