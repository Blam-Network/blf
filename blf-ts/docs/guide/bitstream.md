# Bitstream

The BLF package includes a reimplementation of Halo's bitstream, which is necessary to encode and decode some BLF chunks. The bitstream is exported for use outside of BLF chunks.

Import it from the package root only — version subpaths do not re-export bitstream types:

```ts
import { bitstream } from "@blamnetwork/blf";
```

## Example

```ts
import { bitstream } from "@blamnetwork/blf";

const bytes = new Uint8Array([0x3f, 0xff]);

const reader = bitstream.c_bitstream_reader.new(
  bytes,
  bitstream.e_bitstream_byte_order._bitstream_byte_order_big_endian
);
reader.begin_reading();

const first = reader.read_integer("first-field", 3);
const second = reader.read_integer("second-field", 13);

reader.finish_reading();
```

`c_bitstream_writer` mirrors the reader for building buffers. Gametype and other version-bundle types use these internally when a chunk body is not a plain struct layout.
