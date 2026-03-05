fn main() {
    // Re-run build script if C sources or this script change.
    println!("cargo:rerun-if-changed=c_src/math_helpers.c");
    println!("cargo:rerun-if-changed=build.rs");

    // Compile C source into a static library.
    cc::Build::new()
        .file("c_src/math_helpers.c")
        .compile("math_helpers");

    // Link against the produced static library.
    println!("cargo:rustc-link-lib=static=math_helpers");
}
