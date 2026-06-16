# Changelog

All notable changes to [@blamnetwork/blf](https://www.npmjs.com/package/@blamnetwork/blf) are documented here.

## [Unreleased]

### Pre-release Halo: Reach (Omaha)

Megalo blam types and matchmaking game variant (`gvar`) chunks for pre-release Xbox 360 builds:

- [Omaha Alpha](/guide/versions/haloreach/v08516_10_02_19_1607_omaha_alpha) — `s_blf_chunk_packed_game_variant` (`gvar` 34.1)
- [Omaha Beta (Mar 2010)](/guide/versions/haloreach/v09449_10_03_25_1545_omaha_beta) — `gvar` 38.1 (Delta codec)
- [Omaha Beta (Apr 2010)](/guide/versions/haloreach/v09664_10_04_06_2121_omaha_beta) — `gvar` 38.1 (Delta codec)
- [Omaha Delta](/guide/versions/haloreach/v09730_10_04_09_1309_omaha_delta) — `s_blf_chunk_packed_game_variant` (`gvar` 38.1)

Also exported as `s_blf_chunk_matchmaking_game_variant` to match [blf_lib](https://github.com/Blam-Network/blf/tree/main/blf_lib) naming.

## [1.0.0] — 2026-05-31

Initial release.

- `find_chunk` and `search_for_chunk` for locating chunks in BLF files
- `write_blffile` for writing back.
- Implementation of Halo's bitstream reader and writer.
- Helper function for converting Halo: Reach gametypes between Halo: The Master Chief Collection and Xbox 360 versions.

### BLF Chunks

Ported the following BLF chunks from the [blf_lib](https://github.com/Blam-Network/blf/tree/main/blf_lib) Rust project. Import paths: `@blamnetwork/blf/<game>/<build_id>`.

- [Halo Reach — Title Update 1](/guide/versions/haloreach/v12065_11_08_24_1738_tu1actual)
  - `s_blf_chunk_start_of_file` (`_blf` 1.2)
  - `s_blf_chunk_compressed_data` (`_cmp` 1.1)
  - `s_blf_chunk_end_of_file` (`_eof` 1.1)
  - `s_blf_chunk_author` (`athr` 3.1)
  - `s_blf_chunk_content_header` (`chdr` 10.2)
  - `s_blf_chunk_packed_game_variant` (`gvar` 54.1)
  - `s_blf_chunk_game_variant` (`mpvr` 54.1)
  - `s_blf_chunk_map_variant` (`mvar` 31.1)
- [Halo: MCC – Reach](/guide/versions/haloreach_mcc/v_untracked_25_08_16_1352)
  - `s_blf_chunk_start_of_file` (`_blf` 1.2)
  - `s_blf_chunk_compressed_data` (`_cmp` 1.1)
  - `s_blf_chunk_end_of_file` (`_eof` 1.1)
  - `s_blf_chunk_content_header` (`chdr` 10.2)
  - `s_blf_chunk_packed_game_variant` (`gvar` 54.1)
  - `s_blf_chunk_game_variant` (`mpvr` 54.1)
  - `s_blf_chunk_map_variant` (`mvar` 31.1)
- [Halo: MCC](/guide/versions/mcc/v2025_08_16_178512_1_release)
  - `s_blf_chunk_fileshare_metadata` (`_fsm` 1.1)
- [Halo 3 — Title Update 2](/guide/versions/halo3/v12070_08_09_05_2031_halo3_ship)
  - `s_blf_chunk_start_of_file` (`_blf` 1.2)
  - `s_blf_chunk_compressed_data` (`_cmp` 1.1)
  - `s_blf_chunk_end_of_file` (`_eof` 1.1)
- [Halo 3: ODST](/guide/versions/halo3odst/v13895_09_04_27_2201_atlas_release)
  - `s_blf_chunk_start_of_file` (`_blf` 1.2)
  - `s_blf_chunk_compressed_data` (`_cmp` 1.1)
  - `s_blf_chunk_end_of_file` (`_eof` 1.1)


### Helpers

- `@blamnetwork/blf/helpers` — Reach gametype conversion between Xbox 360 TU1 and MCC, including map variant (`mpvr`) handling

