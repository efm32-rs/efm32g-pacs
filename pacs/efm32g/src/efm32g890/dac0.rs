#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Channel 0 Control Register"]
    pub ch0ctrl: CH0CTRL,
    #[doc = "0x0c - Channel 1 Control Register"]
    pub ch1ctrl: CH1CTRL,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x14 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x18 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x1c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x20 - Channel 0 Data Register"]
    pub ch0data: CH0DATA,
    #[doc = "0x24 - Channel 1 Data Register"]
    pub ch1data: CH1DATA,
    #[doc = "0x28 - Combined Data Register"]
    pub combdata: COMBDATA,
    #[doc = "0x2c - Calibration Register"]
    pub cal: CAL,
    #[doc = "0x30 - Bias Programming Register"]
    pub biasprog: BIASPROG,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CH0CTRL (rw) register accessor: an alias for `Reg<CH0CTRL_SPEC>`"]
pub type CH0CTRL = crate::Reg<ch0ctrl::CH0CTRL_SPEC>;
#[doc = "Channel 0 Control Register"]
pub mod ch0ctrl;
#[doc = "CH1CTRL (rw) register accessor: an alias for `Reg<CH1CTRL_SPEC>`"]
pub type CH1CTRL = crate::Reg<ch1ctrl::CH1CTRL_SPEC>;
#[doc = "Channel 1 Control Register"]
pub mod ch1ctrl;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "IF (r) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "CH0DATA (rw) register accessor: an alias for `Reg<CH0DATA_SPEC>`"]
pub type CH0DATA = crate::Reg<ch0data::CH0DATA_SPEC>;
#[doc = "Channel 0 Data Register"]
pub mod ch0data;
#[doc = "CH1DATA (rw) register accessor: an alias for `Reg<CH1DATA_SPEC>`"]
pub type CH1DATA = crate::Reg<ch1data::CH1DATA_SPEC>;
#[doc = "Channel 1 Data Register"]
pub mod ch1data;
#[doc = "COMBDATA (w) register accessor: an alias for `Reg<COMBDATA_SPEC>`"]
pub type COMBDATA = crate::Reg<combdata::COMBDATA_SPEC>;
#[doc = "Combined Data Register"]
pub mod combdata;
#[doc = "CAL (rw) register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration Register"]
pub mod cal;
#[doc = "BIASPROG (rw) register accessor: an alias for `Reg<BIASPROG_SPEC>`"]
pub type BIASPROG = crate::Reg<biasprog::BIASPROG_SPEC>;
#[doc = "Bias Programming Register"]
pub mod biasprog;
