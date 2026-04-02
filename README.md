# libvisio-bridge

Rust bridge to [libvisio](https://git.libreoffice.org/libvisio/).

## System-Level Dependencies

The following libraries must be installed for the build to succeed:

* [libvisio](https://git.libreoffice.org/libvisio/) (unsurprisingly)
* `librevenge-stream` from [librevenge](https://sourceforge.net/p/libwpd/librevenge/)

## Functionality

`libvisio` itself offers an event-based API for processing Visio files, similar to SAX for XML. The
C++ library accepts a `librevenge::RVNGInputStream` and a `librevenge::RVNGDrawingInterface`, then
reads draw commands from the input stream and calls the methods of the drawing interface.

`libvisio-bridge` offers the traits `InputStream` and `Painter` as shims for `RVNGInputStream` and
`RVNGDrawingInterface`, respectively. It also provides `ReadStream`, provides an `InputStream`
implementation by wrapping an instance of a type which implements Rust's built-in `Read` and `Seek`
traits.

Since `RVNGInputStream` is also expected to perform CFB decoding, `libvisio-bridge` passes this
responsibility to the [cfb](https://github.com/mdsteele/rust-cfb) crate.

Finally, a utility named `stencil-metafiles` is provided, which extracts metafiles (mostly `.emf`)
from a Visio stencil file (`.vss`).
