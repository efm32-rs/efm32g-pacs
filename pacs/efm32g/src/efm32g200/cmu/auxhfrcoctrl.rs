#[doc = "Register `AUXHFRCOCTRL` reader"]
pub struct R(crate::R<AUXHFRCOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXHFRCOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXHFRCOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXHFRCOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXHFRCOCTRL` writer"]
pub struct W(crate::W<AUXHFRCOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXHFRCOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AUXHFRCOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXHFRCOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - AUXHFRCO Tuning Value"]
pub type TUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING` writer - AUXHFRCO Tuning Value"]
pub type TUNING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUXHFRCOCTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AUXHFRCO Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<0> {
        TUNING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUXHFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxhfrcoctrl](index.html) module"]
pub struct AUXHFRCOCTRL_SPEC;
impl crate::RegisterSpec for AUXHFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxhfrcoctrl::R](R) reader structure"]
impl crate::Readable for AUXHFRCOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxhfrcoctrl::W](W) writer structure"]
impl crate::Writable for AUXHFRCOCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUXHFRCOCTRL to value 0x80"]
impl crate::Resettable for AUXHFRCOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
