//! Peripheral access API for EFM32G microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.24.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.24.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32g-pacs)
//!
//! This crate supports all EFM32G devices; for the complete list please see:
//! [efm32g](https://github.com/efm32-rs/efm32g-pacs/pacs/efm32g)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32g200")]
pub mod efm32g200;

#[cfg(feature = "efm32g210")]
pub mod efm32g210;

#[cfg(feature = "efm32g222")]
pub mod efm32g222;

#[cfg(feature = "efm32g230")]
pub mod efm32g230;

#[cfg(feature = "efm32g232")]
pub mod efm32g232;

#[cfg(feature = "efm32g280")]
pub mod efm32g280;

#[cfg(feature = "efm32g290")]
pub mod efm32g290;

#[cfg(feature = "efm32g800")]
pub mod efm32g800;

#[cfg(feature = "efm32g840")]
pub mod efm32g840;

#[cfg(feature = "efm32g842")]
pub mod efm32g842;

#[cfg(feature = "efm32g880")]
pub mod efm32g880;

#[cfg(feature = "efm32g890")]
pub mod efm32g890;
