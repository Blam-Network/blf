# Version bundles

Each game build is a separate npm subpath export. Import chunks and blam types from that path only — do not mix types across builds.

| Game | Import prefix | Guide |
|------|---------------|-------|
| Halo Reach | `@blamnetwork/blf/haloreach/` | [Reach](/guide/versions/haloreach/) |
| Halo Reach MCC | `@blamnetwork/blf/haloreach_mcc/` | [Reach MCC](/guide/versions/haloreach_mcc/) |
| Halo 3 | `@blamnetwork/blf/halo3/` | [Halo 3](/guide/versions/halo3/) |
| Halo 3: ODST | `@blamnetwork/blf/halo3odst/` | [ODST](/guide/versions/halo3odst/) |

Open a **build** page under a game for the exact import path and chunk list. Lists are generated from `src/versions/<game>/<build_id>.ts` (run `npm run docs:gen` after changing a barrel).

A bundle typically re-exports:

- Shared `blam/common` helpers for that integration
- Game-specific blam modules (e.g. Reach `c_game_variant`) via `export * from "../../blam/…"`
- Chunk classes for that build (`s_blf_chunk_*`) via explicit `export * from "../../chunks/…"` lines
