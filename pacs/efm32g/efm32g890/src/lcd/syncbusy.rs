#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTRL` reader - CTRL Register Busy"]
pub type CTRL_R = crate::BitReader<bool>;
#[doc = "Field `BACTRL` reader - BACTRL Register Busy"]
pub type BACTRL_R = crate::BitReader<bool>;
#[doc = "Field `AREGA` reader - AREGA Register Busy"]
pub type AREGA_R = crate::BitReader<bool>;
#[doc = "Field `AREGB` reader - AREGB Register Busy"]
pub type AREGB_R = crate::BitReader<bool>;
#[doc = "Field `SEGD0L` reader - SEGD0L Register Busy"]
pub type SEGD0L_R = crate::BitReader<bool>;
#[doc = "Field `SEGD1L` reader - SEGD1L Register Busy"]
pub type SEGD1L_R = crate::BitReader<bool>;
#[doc = "Field `SEGD2L` reader - SEGD2L Register Busy"]
pub type SEGD2L_R = crate::BitReader<bool>;
#[doc = "Field `SEGD3L` reader - SEGD3L Register Busy"]
pub type SEGD3L_R = crate::BitReader<bool>;
#[doc = "Field `SEGD0H` reader - SEGD0H Register Busy"]
pub type SEGD0H_R = crate::BitReader<bool>;
#[doc = "Field `SEGD1H` reader - SEGD1H Register Busy"]
pub type SEGD1H_R = crate::BitReader<bool>;
#[doc = "Field `SEGD2H` reader - SEGD2H Register Busy"]
pub type SEGD2H_R = crate::BitReader<bool>;
#[doc = "Field `SEGD3H` reader - SEGD3H Register Busy"]
pub type SEGD3H_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BACTRL Register Busy"]
    #[inline(always)]
    pub fn bactrl(&self) -> BACTRL_R {
        BACTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AREGA Register Busy"]
    #[inline(always)]
    pub fn arega(&self) -> AREGA_R {
        AREGA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AREGB Register Busy"]
    #[inline(always)]
    pub fn aregb(&self) -> AREGB_R {
        AREGB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SEGD0L Register Busy"]
    #[inline(always)]
    pub fn segd0l(&self) -> SEGD0L_R {
        SEGD0L_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SEGD1L Register Busy"]
    #[inline(always)]
    pub fn segd1l(&self) -> SEGD1L_R {
        SEGD1L_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SEGD2L Register Busy"]
    #[inline(always)]
    pub fn segd2l(&self) -> SEGD2L_R {
        SEGD2L_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SEGD3L Register Busy"]
    #[inline(always)]
    pub fn segd3l(&self) -> SEGD3L_R {
        SEGD3L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SEGD0H Register Busy"]
    #[inline(always)]
    pub fn segd0h(&self) -> SEGD0H_R {
        SEGD0H_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SEGD1H Register Busy"]
    #[inline(always)]
    pub fn segd1h(&self) -> SEGD1H_R {
        SEGD1H_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SEGD2H Register Busy"]
    #[inline(always)]
    pub fn segd2h(&self) -> SEGD2H_R {
        SEGD2H_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SEGD3H Register Busy"]
    #[inline(always)]
    pub fn segd3h(&self) -> SEGD3H_R {
        SEGD3H_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}