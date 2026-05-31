# What is @blamnetwork/blf?

**@blamnetwork/blf** is a TypeScript implementation of the [blf_lib](https://github.com/Blam-Network/blf/tree/main/blf_lib) Rust crate for reading and writing Halo **BLF** chunks for things like Map Variants and Gametypes.

## What is a BLF?

BLF is a [chunk-based format](https://en.wikipedia.org/wiki/File_format#Chunk-based_formats), similar to Material Exchange Format (MXF). These files consist of **BLF chunks**: regions of data labelled with a 12-byte header including the chunk name, version, and length, allowing readers to seek through the file until they find the chunk they are looking for.

Generally, BLF files include a `_blf` start-of-file chunk and a `_eof` end-of-file chunk. Some readers check or assert that these chunks are present, though the format is still readable without them.

To my knowledge, Bungie have never shared the meaning of the BLF acronym; the name is probably short for **Blam file**.

### BLF chunk header

| Offset | Length | Name |
|--------|--------|------|
| `0x0` | 4 | Chunk signature |
| `0x4` | 4 | Chunk length |
| `0x8` | 2 | Major version |
| `0xA` | 2 | Minor version |

When using this library, pass `"big"` or `"little"` as the payload endian to `find_chunk` / `search_for_chunk`. BLF chunk headers are typically big-endian; the argument controls how the chunk body is read or written. Most of the time BLF chunks are big-endian but little-endian chunks have been observed on occasion, usually with PC halo builds.

## What this package provides

- **Version bundles** — import `@blamnetwork/blf/<game>/<build_id>` for structures for a given Title and Build.
- **Chunk I/O** — [Reading](/guide/reading) (`find_chunk`, `search_for_chunk`) and [writing](/guide/writing) (`write_blffile`, per-chunk `write`) BLF chunks and files.
- **Bitstream** — Halo-style reader/writer for reading bit-packed data ([Bitstream](/guide/bitstream)).
- **Helpers** — Helper functions implemented by Blam Network to assist in common BLF tasks such as converting chunks between Builds.

Struct layouts are built with [@craftycodie/cstruct](https://www.npmjs.com/package/@craftycodie/cstruct), which npm installs automatically as a dependency of `@blamnetwork/blf`.

## Get started

See [Install & quick start](/guide/quick-start).

Release history: [Changelog](/changelog).
