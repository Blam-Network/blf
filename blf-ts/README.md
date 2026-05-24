# @blamnetwork/blf

[![npm](https://img.shields.io/npm/v/@blamnetwork/blf)](https://www.npmjs.com/package/@blamnetwork/blf)

TypeScript library for reading and writing Halo **BLF** (binary layout file) chunks — Reach, Halo 3, and Halo 3: ODST builds.

Built on [@craftycodie/cstruct](https://www.npmjs.com/package/@craftycodie/cstruct) for packed struct layouts and a Halo-style **bitstream** for custom chunk bodies.

## Features

- Chunk discovery with `search_for_chunk` and `find_chunk`
- Per-game **version bundles** (`haloreach/*`, `halo3/*`, `halo3odst/*`) with chunk classes and blam types for a specific exe build
- Struct-backed chunks via `CStructBLFChunk` and `@blf.chunk` decorators (compiled away in published `dist/`)
- Bitstream reader/writer exported from the package root
- Little- and big-endian BLF headers

## Install

```bash
npm install @blamnetwork/blf @craftycodie/cstruct
```

Consumers import from published **`dist/`** exports only. Decorators are lowered at build time — **you do not need `experimentalDecorators`** to use exported chunk classes.

## Usage

### Find and read chunks

```ts
import { readFileSync } from "node:fs";
import { search_for_chunk } from "@blamnetwork/blf";
import {
  s_blf_chunk_content_header,
  s_blf_chunk_game_variant,
} from "@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual";

const file = new Uint8Array(readFileSync("map.blf"));

const chdr = new s_blf_chunk_content_header();
search_for_chunk(file, chdr, "big");

const mpvr = new s_blf_chunk_game_variant();
search_for_chunk(file, mpvr, "big");
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
| `@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual` | Reach TU1 |
| `@blamnetwork/blf/halo3/v12070_08_09_05_2031_halo3_ship` | Halo 3 ship |
| `@blamnetwork/blf/halo3odst/v13895_09_04_27_2201_atlas_release` | ODST Atlas |

Add a build by creating `src/versions/<game>/<build_id>.ts` and re-exporting its chunks — wildcard `exports` in `package.json` pick it up automatically.

## Development

From `blf-ts/`:

```bash
npm install
npm run validate   # lint, test, typecheck
npm run build
npm run release    # bump version, tag, push (CI publishes on tag)
```

Chunk authoring (forking the library) uses Stage 3 decorators (`@blf.chunk`, `@c.struct`) with the same SWC/Vitest setup as the repo. Decorator order: `@blf.chunk` on the class, `@c.struct` below, `@c.field` on properties.

## License

MIT
