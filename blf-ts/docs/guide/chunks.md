# Reading chunks

## `find_chunk`

Walks the buffer from the start as a BLF file: read each chunk header in order until the requested signature matches, then decode that chunk in place.

```ts
import { find_chunk } from "@blamnetwork/blf";

const found = find_chunk(file, chunk, "big");
```

Use this when the buffer is a normal BLF (a `.blf` on disk, or bytes you know are laid out as consecutive BLF chunks).

## `search_for_chunk`

Scans every byte offset for the chunk signature, then decodes the first match in place.

```ts
import { search_for_chunk } from "@blamnetwork/blf";

const found = search_for_chunk(file, chunk, "big");
```

Use this when the buffer does not only contain a BLF — for example, reading a gametype variant out of a larger Xbox 360 console dump where the `mpvr` / `gvar` chunk is embedded among other data. `find_chunk` assumes valid BLF structure from offset 0; `search_for_chunk` does not.

## Writing BLFs

Use `write_blffile` from the package root when assembling a BLF from chunk instances and `s_blf_header`.

## Errors

Layout and chunk failures throw `BlfError`.
