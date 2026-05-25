# Install & quick start

## Install

```bash
npm install @blamnetwork/blf
```

`@craftycodie/cstruct` is installed automatically as a dependency — you do not need to add it unless you are developing chunk layouts in this repository.

Use TypeScript 5.x. Consumers only need the published types under `dist/`; no decorator configuration is required for normal use.

## Read a BLF

Read a Halo Reach TU1 BLF and decode its content header and game variant (`mpvr`) chunks:

```ts
import { readFileSync } from "node:fs";
import { find_chunk } from "@blamnetwork/blf";
import {
  s_blf_chunk_content_header,
  s_blf_chunk_game_variant,
} from "@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual";

const file = new Uint8Array(readFileSync("variant.blf"));

const chdr = new s_blf_chunk_content_header();
find_chunk(file, chdr, "big");

const mpvr = new s_blf_chunk_game_variant();
find_chunk(file, mpvr, "big");
```

Pass `"big"` or `"little"` for BLF header endianness (Reach `mpvr` gametypes use big-endian bitstreams internally). Instantiate the chunk class, then call `find_chunk` — fields are filled in place.

Next: [Reading chunks](/guide/chunks) for discovery APIs, or [Version bundles](/guide/versions/) to pick the right import path.
