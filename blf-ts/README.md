# @Blam-Network/blf

TypeScript BLF chunk library (Halo Reach / Halo 3).

## Consumers (apps importing this package)

Install the package and import from published **`dist/`** exports only (do not point TypeScript `paths` at `src/`).

Decorators are compiled away at publish time (`__esDecorate` in emitted JS). **You do not need `experimentalDecorators` or any decorator flags** to use exported chunk classes, `search_for_chunk`, `find_chunk`, etc.

`npm install` runs `prepare` → `tsc` so `dist/` exists for `file:` workspace links.

## Chunk authoring (library / fork development)

Chunks are defined under `src/chunks/` (per implementation build). Each game **version** under `src/versions/` re-exports the chunk set that build uses (often mixing chunks from another title’s folder, e.g. Reach TU1 reusing Halo 3 ship start/end-of-file chunks).

**Import a game build** (chunks + blam types for that exe):

```ts
import {
  s_blf_chunk_author,
  s_blf_chunk_start_of_file,
  e_file_type,
  c_game_variant,
} from "@Blam-Network/blf/haloreach/v12065_11_08_24_1738_tu1actual";

const author = new s_blf_chunk_author();
const sof = new s_blf_chunk_start_of_file();
```

Add a new published build by creating `src/versions/<game>/<build>.ts` and wiring its chunk/blam re-exports. No per-version entries are needed in `package.json`, `vitest.config.ts`, or `tsconfig` — those use wildcards (`./haloreach/*`, `@Blam-Network/blf/haloreach/*`).

**Struct chunks** (`@blf.chunk` + `@c.struct`):

```ts
import { blf } from "@Blam-Network/blf";
import { blf_chunk_cstruct } from "@Blam-Network/blf";
import { c } from "@craftycodie/cstruct";

@blf.chunk("chdr", 10.2)
@c.struct()
export class s_blf_chunk_content_header extends blf_chunk_cstruct {
  @c.field("u16")
  build_number!: number;
}
```

`blf_chunk_cstruct` provides default `read_body` / `write_body` from the cstruct layout; `read` / `write` add the BLF header.

**Custom chunks** (bitstream or other I/O): extend `BLFChunkBase`, implement `read_body` / `write_body`, and use `@blf.chunk("four", 1.0)`.

Authoring new decorated chunks in **your app** requires the same TS 5 stage-3 decorator setup used to build this repo (or add chunks here and consume via `dist/`).

Decorator order: `@blf.chunk` on top, `@c.struct` below, `@c.field` on properties.

## Scripts

```bash
npm install
npm run build
npm test
```
