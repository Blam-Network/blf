# Converting Reach Gametypes

Cross-build utilities live on a dedicated entry point (not the package root):

```ts
import { convert_reach_gametype } from "@blamnetwork/blf/helpers";
import { c_game_variant as tu1_variant } from "@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual";
import { c_game_variant as mcc_variant } from "@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_19_1352";
```

## `convert_reach_gametype(from, to)`

Helper for converting **game variants** between Halo Reach on **MCC** and **Xbox 360** (this library’s TU1 build). Pass two decoded `c_game_variant` instances from the matching version bundles; the function writes the result onto `to`.

The Master Chief Collection added megalo features that Xbox 360 builds do not have:

- **Temporary variables** for objects, players, and teams
- **Additional math operators** (`<<=`, `>>=`)
- **“Network Test 1”** firefight mode

`convert_reach_gametype(MCC, Xbox360)` will return false if any of the above features are being used by the gametype. In addition, Forge gametypes are not yet supported.

## Example (MCC → TU1)

Read an MCC `mpvr` from a buffer (use `find_chunk` instead of `search_for_chunk` when the file is a normal BLF), convert the decoded gametype, then write a TU1 BLF:

```ts
import { readFileSync, writeFileSync } from "node:fs";
import { convert_reach_gametype } from "@blamnetwork/blf/helpers";
import { search_for_chunk, write_blffile } from "@blamnetwork/blf";
import { c_game_variant as tu1_variant } from "@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual";
import {
  s_blf_chunk_end_of_file,
  s_blf_chunk_game_variant as tu1_mpvr,
  s_blf_chunk_start_of_file,
} from "@blamnetwork/blf/haloreach/v12065_11_08_24_1738_tu1actual";
import { s_blf_chunk_game_variant as mcc_mpvr } from "@blamnetwork/blf/haloreach_mcc/v_untracked_25_08_19_1352";

const buffer = new Uint8Array(readFileSync("mcc-variant.blf"));

const mccChunk = new mcc_mpvr();
if (!search_for_chunk(buffer, mccChunk, "big")) {
  throw new Error("mpvr chunk not found");
}

const from = mccChunk.game_variant;
const to = new tu1_variant();

if (!convert_reach_gametype(from, to)) {
  throw new Error("MCC gametype cannot be represented as TU1");
}

const tu1Blf = write_blffile("big", [
  s_blf_chunk_start_of_file.create("converted"),
  tu1_mpvr.create(to),
  new s_blf_chunk_end_of_file(),
]);

writeFileSync("tu1-variant.blf", tu1Blf);
```

After conversion, `to` holds the TU1 gametype; `tu1_mpvr.create(to)` encodes it for a TU1 `mpvr` chunk body.
