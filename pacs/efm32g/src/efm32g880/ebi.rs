#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Address Timing Register"]
    pub addrtiming: ADDRTIMING,
    #[doc = "0x08 - Read Timing Register"]
    pub rdtiming: RDTIMING,
    #[doc = "0x0c - Write Timing Register"]
    pub wrtiming: WRTIMING,
    #[doc = "0x10 - Polarity Register"]
    pub polarity: POLARITY,
    #[doc = "0x14 - I/O Routing Register"]
    pub route: ROUTE,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "ADDRTIMING (rw) register accessor: an alias for `Reg<ADDRTIMING_SPEC>`"]
pub type ADDRTIMING = crate::Reg<addrtiming::ADDRTIMING_SPEC>;
#[doc = "Address Timing Register"]
pub mod addrtiming;
#[doc = "RDTIMING (rw) register accessor: an alias for `Reg<RDTIMING_SPEC>`"]
pub type RDTIMING = crate::Reg<rdtiming::RDTIMING_SPEC>;
#[doc = "Read Timing Register"]
pub mod rdtiming;
#[doc = "WRTIMING (rw) register accessor: an alias for `Reg<WRTIMING_SPEC>`"]
pub type WRTIMING = crate::Reg<wrtiming::WRTIMING_SPEC>;
#[doc = "Write Timing Register"]
pub mod wrtiming;
#[doc = "POLARITY (rw) register accessor: an alias for `Reg<POLARITY_SPEC>`"]
pub type POLARITY = crate::Reg<polarity::POLARITY_SPEC>;
#[doc = "Polarity Register"]
pub mod polarity;
#[doc = "ROUTE (rw) register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
