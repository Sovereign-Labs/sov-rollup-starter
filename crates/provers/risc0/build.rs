use std::collections::HashMap;

fn main() {
    if let Ok(rollup_elf_path) = std::env::var("ROLLUP_ELF_PATH") {
        println!("Using prebuilt rollup ELF bytes at {rollup_elf_path}");

        let out_dir = std::env::var_os("OUT_DIR").unwrap();
        let out_dir = std::path::Path::new(&out_dir);
        let methods_path = out_dir.join("methods.rs");

        let rollup_elf_bytes = std::fs::read(rollup_elf_path).unwrap();

        let elf = format!(
            r#"
            pub const ROLLUP_ELF: &[u8] = &{rollup_elf_bytes:?};
        "#
        );

        std::fs::write(methods_path, elf).expect("Failed to write rollup elf to methods.rs");
    } else if std::env::var("SKIP_GUEST_BUILD").is_ok() {
        println!("Skipping guest build for CI run");
        let out_dir = std::env::var_os("OUT_DIR").unwrap();
        let out_dir = std::path::Path::new(&out_dir);
        let methods_path = out_dir.join("methods.rs");

        let elf = r#"
            pub const ROLLUP_ELF: &[u8] = &[];
            pub const MOCK_DA_ELF: &[u8] = &[];
        "#;

        std::fs::write(methods_path, elf).expect("Failed to write mock rollup elf");
    } else {
        let guest_pkg_to_options = get_guest_options();
        risc0_build::embed_methods_with_options(guest_pkg_to_options);
    }
}

fn get_guest_options() -> HashMap<&'static str, risc0_build::GuestOptions> {
    HashMap::new()
}
