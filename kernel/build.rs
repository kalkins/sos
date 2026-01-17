use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Get the directory where the kernel crate lives
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // 2. Create the absolute path to linker.ld
    let linker_script = PathBuf::from(manifest_dir).join("linker.ld");

    // 3. Pass the ABSOLUTE path to the linker
    println!("cargo:rustc-link-arg=-T{}", linker_script.display());

    // 4. Ensure we rebuild if the linker script changes
    println!("cargo:rerun-if-changed={}", linker_script.display());
}
