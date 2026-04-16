use std::path::Path;

fn main() {
    eprintln!("Hello, world!");

    ash_formats::write_source_code(
        Path::new("ash/generator/Vulkan-Headers"),
        "crates/ash-formats/src",
    );
}
