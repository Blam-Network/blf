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

## Fileshare metadata (`_fsm`)

MCC fileshare exports append a `_fsm` 1.1 chunk (464 bytes) with digests, a PlayFab-style item id, and an attestation blob. Use `search_for_chunk` when the buffer is a full BLF that ends with `_eof` then `_fsm`:

```ts
import { search_for_chunk } from "@blamnetwork/blf";
import { s_blf_chunk_fileshare_metadata } from "@blamnetwork/blf/mcc/v2025_08_16_178512_1_release";

const fsm = new s_blf_chunk_fileshare_metadata();
if (search_for_chunk(file, fsm, "big")) {
  console.log(fsm.unknown98); // 40-char hex item id
}
```

See [MCC `v2025_08_16_178512_1_release`](/guide/versions/mcc/v2025_08_16_178512_1_release) for the version bundle ([`s_blf_chunk_fileshare_metadata` source](https://github.com/Blam-Network/blf/blob/main/blf-ts/src/chunks/mcc/s_blf_chunk_fileshare_metadata.ts)).

## Errors

Layout and chunk failures throw `BlfError`.
