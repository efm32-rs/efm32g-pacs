#[doc = "Register `SEGD1H` reader"]
pub struct R(crate::R<SEGD1H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEGD1H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEGD1H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEGD1H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEGD1H` writer"]
pub struct W(crate::W<SEGD1H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEGD1H_SPEC>;
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
impl From<crate::W<SEGD1H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEGD1H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEGD1H` reader - COM1 Segment Data High"]
pub type SEGD1H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEGD1H` writer - COM1 Segment Data High"]
pub type SEGD1H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEGD1H_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    pub fn segd1h(&self) -> SEGD1H_R {
        SEGD1H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COM1 Segment Data High"]
    #[inline(always)]
    #[must_use]
    pub fn segd1h(&mut self) -> SEGD1H_W<0> {
        SEGD1H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Segment Data High Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [segd1h](index.html) module"]
pub struct SEGD1H_SPEC;
impl crate::RegisterSpec for SEGD1H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [segd1h::R](R) reader structure"]
impl crate::Readable for SEGD1H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [segd1h::W](W) writer structure"]
impl crate::Writable for SEGD1H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEGD1H to value 0"]
impl crate::Resettable for SEGD1H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
