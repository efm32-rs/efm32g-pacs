#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Address Timing Register"]
    pub addrtiming: crate::Reg<addrtiming::ADDRTIMING_SPEC>,
    #[doc = "0x08 - Read Timing Register"]
    pub rdtiming: crate::Reg<rdtiming::RDTIMING_SPEC>,
    #[doc = "0x0c - Write Timing Register"]
    pub wrtiming: crate::Reg<wrtiming::WRTIMING_SPEC>,
    #[doc = "0x10 - Polarity Register"]
    pub polarity: crate::Reg<polarity::POLARITY_SPEC>,
    #[doc = "0x14 - I/O Routing Register"]
    pub route: crate::Reg<route::ROUTE_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "ADDRTIMING register accessor: an alias for `Reg<ADDRTIMING_SPEC>`"]
pub type ADDRTIMING = crate::Reg<addrtiming::ADDRTIMING_SPEC>;
#[doc = "Address Timing Register"]
pub mod addrtiming;
#[doc = "RDTIMING register accessor: an alias for `Reg<RDTIMING_SPEC>`"]
pub type RDTIMING = crate::Reg<rdtiming::RDTIMING_SPEC>;
#[doc = "Read Timing Register"]
pub mod rdtiming;
#[doc = "WRTIMING register accessor: an alias for `Reg<WRTIMING_SPEC>`"]
pub type WRTIMING = crate::Reg<wrtiming::WRTIMING_SPEC>;
#[doc = "Write Timing Register"]
pub mod wrtiming;
#[doc = "POLARITY register accessor: an alias for `Reg<POLARITY_SPEC>`"]
pub type POLARITY = crate::Reg<polarity::POLARITY_SPEC>;
#[doc = "Polarity Register"]
pub mod polarity;
#[doc = "ROUTE register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
