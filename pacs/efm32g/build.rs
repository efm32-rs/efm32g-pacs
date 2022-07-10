use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let device_file = if env::var_os("CARGO_FEATURE_EFM32G200").is_some() {
            "src/efm32g200/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32G210").is_some() {
            "src/efm32g210/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32G222").is_some() {
            "src/efm32g222/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32G230").is_some() {
            "src/efm32g230/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32G232").is_some() {
            "src/efm32g232/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32G280").is_some() {
            "src/efm32g280/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32G290").is_some() {
            "src/efm32g290/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32G800").is_some() {
            "src/efm32g800/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32G840").is_some() {
            "src/efm32g840/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32G842").is_some() {
            "src/efm32g842/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32G880").is_some() {
            "src/efm32g880/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32G890").is_some() {
            "src/efm32g890/device.x"
        } else { panic!("No device features selected"); };

        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

