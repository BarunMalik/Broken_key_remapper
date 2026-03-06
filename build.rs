fn main() {
    #[cfg(target_os = "windows")]
    {
        cc::Build::new()
            .file("c_src/keyboard.c")
            .include("c_src")
            .flag_if_supported("/W4")
            .compile("keyboard_listener");

        println!("cargo:rerun-if-changed=c_src/keyboard.c");
        println!("cargo:rerun-if-changed=c_src/keyboard.h");
    }
}
