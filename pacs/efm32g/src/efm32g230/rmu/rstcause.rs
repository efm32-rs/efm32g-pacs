#[doc = "Register `RSTCAUSE` reader"]
pub struct R(crate::R<RSTCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PORST` reader - Power On Reset"]
pub type PORST_R = crate::BitReader<bool>;
#[doc = "Field `BODUNREGRST` reader - Brown Out Detector Unregulated Domain Reset"]
pub type BODUNREGRST_R = crate::BitReader<bool>;
#[doc = "Field `BODREGRST` reader - Brown Out Detector Regulated Domain Reset"]
pub type BODREGRST_R = crate::BitReader<bool>;
#[doc = "Field `EXTRST` reader - External Pin Reset"]
pub type EXTRST_R = crate::BitReader<bool>;
#[doc = "Field `WDOGRST` reader - Watchdog Reset"]
pub type WDOGRST_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUPRST` reader - LOCKUP Reset"]
pub type LOCKUPRST_R = crate::BitReader<bool>;
#[doc = "Field `SYSREQRST` reader - System Request Reset"]
pub type SYSREQRST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn porst(&self) -> PORST_R {
        PORST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brown Out Detector Unregulated Domain Reset"]
    #[inline(always)]
    pub fn bodunregrst(&self) -> BODUNREGRST_R {
        BODUNREGRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Brown Out Detector Regulated Domain Reset"]
    #[inline(always)]
    pub fn bodregrst(&self) -> BODREGRST_R {
        BODREGRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Pin Reset"]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdogrst(&self) -> WDOGRST_R {
        WDOGRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LOCKUP Reset"]
    #[inline(always)]
    pub fn lockuprst(&self) -> LOCKUPRST_R {
        LOCKUPRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System Request Reset"]
    #[inline(always)]
    pub fn sysreqrst(&self) -> SYSREQRST_R {
        SYSREQRST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Reset Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcause](index.html) module"]
pub struct RSTCAUSE_SPEC;
impl crate::RegisterSpec for RSTCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcause::R](R) reader structure"]
impl crate::Readable for RSTCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCAUSE to value 0"]
impl crate::Resettable for RSTCAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
