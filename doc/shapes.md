# Shape (SHP) file format documentation

Notes taken from analysis and re-implementation of shape extraction
from https://github.com/tomjepp/NetStormSharp

## Hierarchy

The `_shapes.shp` file has the following hierarchy (note that all of the
headers are at the start of the file, and the frame data must be
retrieved from the offsets into the file):

- A collection of unit types (Sections in NetStormSharp), beginning with a header consisting of two
  32-bit values:

  - A version header (`0x31 0x2E 0x31 0x30` for my version,
    1.10 stored little-endian)
  - The count of frames that this unit type contains (e.g. `0x82`
    = 130 frames for the `dude` type, note this is unsigned `u32`.)

- Each unit type contains a collection of frames (Shapes in
  NetStormSharp). Each frame has a
  header consisting of two 32-bit values:
  - An offset (`0xB8 0xB0 0x56 0x00` for the first `dude` frame), this
    should be added to the offset for the start of the unit type header
    (not the frame header itself) to find the start of the frame data.
  - A color table - this should always be zero.

The frame data itself consists of:

- The height of the frame in pixels `u16`, i.e. number of rows, `15` for
  first frame of `dude` type
- The width of the frame in pixels `u16`, `9` for first frame of `dude`
  type
- The x-coordinate of the origin, `u16`, `14` for first frame of `dude`
- The y-coordinate of the origin, `u16`, `4` for first frame of `dude`
- The minimum x-coordinate, `i32`, `-3` for first frame of `dude`
- The minimum y-coordinate, `i32`, `-14` for first frame of `dude`
- The maximum x-coordinate, `i32`, `2` for first frame of `dude`
- The maximum y-coordinate, `i32`, `0` for first frame of `dude`
- Packets of pixel data (see below) to fill a `u8` `[height, width]` array of
  row-wise pixels.

Pixel data is stored row-wise. The file format contains packets of pixel
data, to allow for run-length encoding and a padding compression.

Each packet is prefixed with a `u8` packet type.

- If the packet type is 0, then the remaining bytes in the current row
  should be filled with `0xFF`. Then advance to next row.
- If the packet type is 1, then the next `u8` indicates how many `0xFF`
  should be written. This never advances the row, and should be ignored
  if it would write past the row.
- If the `packet_type & 1 != 0` then the next `packet_type >> 1` bytes
  should be written as is (i.e. `u8`s). This should never advance the
  row as above.
- Otherwise the packet is using runlength encoding and `u8` immediately
  following the packet type should be wrriten `packet_type >> 1` times.

It may be useful to store the start and end offsets of the full frame
for debugging.

The colour palette is defined in `d/GIFCLOUD.COL`, the first 8 bytes
should be skipped, and the remaining bytes are triplets of `u8`s
defining RGB values for each colour entry.

To render the pixel data, if the palette index is 255 then a white pixel
is drawn. Otherwise the entry in the palette is taken and its RGB value
is used.

Pixels are then drawn row-wise. Need to determine how to use origin,
maxX and maxY, etc. to correctly position sprite, and positioning for
spritesheet.
