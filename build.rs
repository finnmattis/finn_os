fn main() {
    // where rust expects output from build scripts
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir);
    // cargo artifact dependency
    let kernel = std::env::var_os("CARGO_BIN_FILE_KERNEL_kernel").unwrap();
    let kernel = std::path::Path::new(&kernel);

    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .unwrap();

    // pass the disk image paths as env variables to the `main.rs`
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}
