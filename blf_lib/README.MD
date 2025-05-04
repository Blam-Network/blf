# blf_lib

## Technical Design
### Helper Crates
Most of this library's code is contained directly within the blf_lib crate, however there are two additional crates used by blf_lib. These crates are re-exported by blf_lib, so you don't need to install them separately.
#### blf_lib-derive
This crate contains [derive macros](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros) used by blf_lib. These macros must be kept in a separate crate in order to build blf_lib.
#### blf_lib-derivable
In order to implement some traits, blf_lib-derive needs access to some blf-related structures such as `s_blf_header`. To put it simply, any blf related code which blf_lib-derive needs access to is stored in blf_lib-derivable, in order to prevent circular dependencies.
### Reading and writing BLF files.
Halo games generally read and write their BLF files by copying them from memory to file, or visa-versa. That is to say, many BLF chunks are structured in the same manner that the game structures it's memory. This is fine for Xbox 360 era Halo which is generally compiled once for one architecture and has a few minor patches, however this method breaks memory-safety which is enforced byt rust. Even if we were to use [unsafe](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html) rust code, this would require using Halo's memory alignment for BLF chunks, at the cost of layout optimization, so blf_lib handles this differently:

1. Most blf chunks are serialized using [**binwr**](https://binrw.rs/) which writes structs field by field.
2. Some chunks are serialized using an approximate reimplementation of Halo's `c_bitstream`, this is usually used for chunks which aren't byte-aligned, containing 5-bit integers or 15-bit floats, for example.

Outside of BLF files, all chunks support [Serde](https://serde.rs/) serialization. This can be used to write a chunk to a JSON file for example, an example of this can be found in **blf_cli**.