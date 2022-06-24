# EFM32G (Gecko) support for Rust

[![PACs](https://github.com/efm32-rs/efm32g-pacs/actions/workflows/pacs.yml/badge.svg)](https://github.com/efm32-rs/efm32g-pacs/actions/workflows/pacs.yml)

This repository contains Peripheral Access Crates (PACs) for Silabs' EFM32 series of Cortex-M microcontrollers.

All these crates are automatically generated using [svd2rust](https://github.com/rust-embedded/svd2rust).

Refer to the [CHANGELOG](CHANGELOG.md) to see what changed in the last releases.

## Crates

Every EFM32G chip has its own PAC, listed below:

| Crate           | Docs                                                                                 | crates.io                                                                                                 | Target               |
|-----------------|--------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------|----------------------|
| `efm32g200-pac` | [![docs.rs](https://docs.rs/efm32g200-pac/badge.svg)](https://docs.rs/efm32g200-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |
| `efm32g210-pac` | [![docs.rs](https://docs.rs/efm32g210-pac/badge.svg)](https://docs.rs/efm32g210-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |
| `efm32g222-pac` | [![docs.rs](https://docs.rs/efm32g222-pac/badge.svg)](https://docs.rs/efm32g222-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |
| `efm32g230-pac` | [![docs.rs](https://docs.rs/efm32g230-pac/badge.svg)](https://docs.rs/efm32g230-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |
| `efm32g232-pac` | [![docs.rs](https://docs.rs/efm32g232-pac/badge.svg)](https://docs.rs/efm32g232-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |
| `efm32g280-pac` | [![docs.rs](https://docs.rs/efm32g280-pac/badge.svg)](https://docs.rs/efm32g280-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |
| `efm32g290-pac` | [![docs.rs](https://docs.rs/efm32g290-pac/badge.svg)](https://docs.rs/efm32g290-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |
| `efm32g800-pac` | [![docs.rs](https://docs.rs/efm32g800-pac/badge.svg)](https://docs.rs/efm32g800-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |
| `efm32g840-pac` | [![docs.rs](https://docs.rs/efm32g840-pac/badge.svg)](https://docs.rs/efm32g840-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |
| `efm32g842-pac` | [![docs.rs](https://docs.rs/efm32g842-pac/badge.svg)](https://docs.rs/efm32g842-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |
| `efm32g880-pac` | [![docs.rs](https://docs.rs/efm32g880-pac/badge.svg)](https://docs.rs/efm32g880-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |
| `efm32g890-pac` | [![docs.rs](https://docs.rs/efm32g890-pac/badge.svg)](https://docs.rs/efm32g890-pac) | [![crates.io](https://img.shields.io/crates/d/efm32g200-pac.svg)](https://crates.io/crates/efm32g200-pac) | `thumbv7m-none-eabi` |

## Device Reference Manuals from Silabs

**WIP**

## License

The included SVD files are sourced from https://www.silabs.com/documents/public/cmsis-packs and
are licensed under the Zlib (see [LICENSE-3RD-PARTY](LICENSE-3RD-PARTY-Zlib)).

The remainder of the code is under:

- 3-Clause BSD license ([LICENSE-3BSD](LICENSE-3BSD) or https://opensource.org/licenses/BSD-3-Clause)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the BSD-3-Clause license without any additional terms or conditions.