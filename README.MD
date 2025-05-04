# @blamnetwork/blf

This repository contains a [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) of various [Rust](https://doc.rust-lang.org/stable/) [Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) which handle BLF files used by various Blam engine games such as Halo 3, Halo 3: ODST and Halo: Reach.

## BLF Crates
### [blf_lib](./blf_lib/README.MD)
This is the core crate of this workspace, it defines many types of BLF chunks for various Halo versions, and functions to interact with them. 

### [blf_cli](./blf_cli/README.MD)
This binary crate is a a very simple command-line-interface (CLI) for interacting with BLF files. This CLI is primarily used for managing Halo's "Title Storage", more information can be found in [it's readme](./blf_cli/README.MD).

### [blf_bnet](./blf_bnet/README.MD)
This crate contains BLF functions used by the client-side TypeScript code of [blam.network](https://www.blam.network/). It is compiled to [WebAssembly](https://webassembly.org/) using [wasm-pack](https://github.com/rustwasm/wasm-pack).

### [blf_lsp](./blf_lsp/README.MD)
This crate contains BLF functions used by the server-side Typescript code of Blam Network's web-APIs. It is compiled to [WebAssembly](https://webassembly.org/) using [wasm-pack](https://github.com/rustwasm/wasm-pack).

## What is a BLF?
BLF is a [chunk-based format](https://en.wikipedia.org/wiki/File_format#Chunk-based_formats), similar to Material Exchange Format (MXF). These files consist of **blf chunks**, which are regions of data labelled with a 12-byte header including the chunk name, version and length, allowing readers to seek through the file until they find the chunk they're looking for. 
Generally, BLF files include a `_blf` start-of-file chunk and a `_eof` end-of-file chunk, some readers will check/assert that these chunks are present, though the format is still readable without them.

To my knowledge, Bungie have never shared the meaning of the BLF acronym, but it probably means The name "BLF" is probably short for "Blam File". 

### BLF chunk header
| offset | length | name            |
|--------|--------|-----------------|
| 0x0    | 4      | chunk signature |
| 0x4    | 4      | chunk length    |
| 0x8    | 2      | major version   |
| 0xA    | 2      | minor version   |

This repository consists of a code library and command-line interface built for reading and writing Blam! Files, used by various Halo games.
