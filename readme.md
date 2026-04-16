# ash-formats

`ash-formats` provides Vulkan format metadata extensions for [`ash::vk::Format`].

The main crate adds the `FormatExt` trait with helpers such as:

- `block_size()`
- `block_extent()`
- `texels_per_block()`
- `packed()`
- `format_class()`
- `chroma()`
- `compression()`
- `components()`
- `planes()`
- `aspect_flags()`

The repository also contains a generator crate that produces the format metadata source from Vulkan XML.

## Workspace layout

- `crates/ash-formats`: library crate exported as `ash-formats`
- `crates/generator`: code generation crate (`ash-formats-generator` binary)

## Usage

Add the crate to your project:

```toml
[dependencies]
ash-formats = "0.1.0"
```

Example:

```rust
use ash::vk::Format;
use ash_formats::FormatExt;

let fmt = Format::R8G8B8A8_UNORM;
let size = fmt.block_size();
let aspects = fmt.aspect_flags();
```

## Regenerating `formats.rs`

The generator uses Vulkan headers XML from the `ash` submodule.

1. Initialize submodules:

   ```bash
   git submodule update --init --recursive
   ```

2. Run the generator:

   ```bash
   cargo run -p formats-generator
   ```

This updates generated source in `crates/ash-formats/src`.

## Notes

- The library crate is `#![no_std]`.
- The workspace currently depends on the `ash` submodule for generator builds.
