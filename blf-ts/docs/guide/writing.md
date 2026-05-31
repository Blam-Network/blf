# Writing chunks

BLF chunk headers are always written **big-endian**. Pass `"big"` or `"little"` to `write` / `write_blffile` for the chunk **payload** only (same rule as [reading](/guide/reading)).

## Single chunk

Each chunk class implements `write(endian)` (12-byte header + payload) and `write_body(endian)` (payload only):

```ts
import { s_blf_chunk_author } from "@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual";

const author = s_blf_chunk_author.forBuild({
  programName: "GameData.Reach",
  authorName: "blf_ts",
});

const bytes = author.write("big"); // full chunk including header
```

Use `write_body` when you are assembling payloads manually or testing layout without a BLF header.

## `write_blffile`

Concatenate one or more chunks in order and return a single `Uint8Array`:

```ts
import { write_blffile } from "@blamnetwork/blf";
import { s_blf_chunk_end_of_file } from "@blamnetwork/blf/halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_end_of_file";
import { s_blf_chunk_start_of_file } from "@blamnetwork/blf/halo3/v12070_08_09_05_2031_halo3_ship/s_blf_chunk_start_of_file";
import { s_blf_chunk_author } from "@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual";

const blf = write_blffile("big", [
  s_blf_chunk_start_of_file.create("my-variant"),
  s_blf_chunk_author.forBuild({ programName: "GameData.Reach" }),
  chdr,
  mpvr,
  new s_blf_chunk_end_of_file(),
]);
```

Typical Reach gametype files include `_blf`, body chunks (`chdr`, `mpvr`, `athr`, …), then `_eof`. Import chunk classes from the [version bundle](/guide/versions/) that matches the target build.

### `_eof` `file_size`

When the last chunk before `_eof` is written, `write_blffile` sets `file_size` on `s_blf_chunk_end_of_file` to the byte offset where `_eof` starts (the total size of all preceding chunks). You do not need to compute this yourself if `_eof` is the final entry in the array.

## Errors

Layout and chunk failures throw `BlfError`.

See also: [Reading chunks](/guide/reading), [Converting Reach gametypes](/guide/converting-reach-gametypes) for a full MCC → TU1 read/convert/write walkthrough.
