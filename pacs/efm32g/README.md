# EFM32G
    
[![crates.io](https://img.shields.io/crates/v/efm32g-pac?label=efm32g)](https://crates.io/crates/efm32g-pac)

This crate provides an autogenerated API for access to EFM32G peripherals.

## Usage

Each device supported by this crate is behind a feature gate so that you only
compile the device(s) you want. To use, in your Cargo.toml:

```toml
[dependencies.efm32g-pac]
version = "0.1.4"
features = ["efm32g200"]
```

The `rt` feature is enabled by default and brings in support for `cortex-m-rt`.
To disable, specify `default-features = false` in `Cargo.toml`.

For full details on the autogenerated API, please see `svd2rust` Peripheral API [here].

[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api

## Supported Devices
| Feature | Devices |
|:-----:|:-------:|
|`efm32g200`|EFM32G200F16, EFM32G200F32, EFM32G200F64|
|`efm32g210`|EFM32G210F128|
|`efm32g222`|EFM32G222F32, EFM32G222F64, EFM32G222F128|
|`efm32g230`|EFM32G230F32, EFM32G230F64, EFM32G230F128|
|`efm32g232`|EFM32G232F32, EFM32G232F64, EFM32G232F128|
|`efm32g280`|EFM32G280F32, EFM32G280F64, EFM32G280F128|
|`efm32g290`|EFM32G290F32, EFM32G290F64, EFM32G290F128|
|`efm32g800`|EFM32G800F128|
|`efm32g840`|EFM32G840F32, EFM32G840F64, EFM32G840F128|
|`efm32g842`|EFM32G842F32, EFM32G842F64, EFM32G842F128|
|`efm32g880`|EFM32G880F32, EFM32G880F64, EFM32G880F128|
|`efm32g890`|EFM32G890F32, EFM32G890F64, EFM32G890F128|
