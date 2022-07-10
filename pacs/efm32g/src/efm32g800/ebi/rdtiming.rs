#[doc = "Register `RDTIMING` reader"]
pub struct R(crate::R<RDTIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDTIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDTIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDTIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDTIMING` writer"]
pub struct W(crate::W<RDTIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDTIMING_SPEC>;
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
impl From<crate::W<RDTIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDTIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDSETUP` reader - Read Setup Time"]
pub type RDSETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDSETUP` writer - Read Setup Time"]
pub type RDSETUP_W<'a> = crate::FieldWriter<'a, u32, RDTIMING_SPEC, u8, u8, 2, 0>;
#[doc = "Field `RDSTRB` reader - Read Strobe Time"]
pub type RDSTRB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDSTRB` writer - Read Strobe Time"]
pub type RDSTRB_W<'a> = crate::FieldWriter<'a, u32, RDTIMING_SPEC, u8, u8, 4, 8>;
#[doc = "Field `RDHOLD` reader - Read Hold Time"]
pub type RDHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDHOLD` writer - Read Hold Time"]
pub type RDHOLD_W<'a> = crate::FieldWriter<'a, u32, RDTIMING_SPEC, u8, u8, 2, 16>;
impl R {
    #[doc = "Bits 0:1 - Read Setup Time"]
    #[inline(always)]
    pub fn rdsetup(&self) -> RDSETUP_R {
        RDSETUP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - Read Strobe Time"]
    #[inline(always)]
    pub fn rdstrb(&self) -> RDSTRB_R {
        RDSTRB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Read Hold Time"]
    #[inline(always)]
    pub fn rdhold(&self) -> RDHOLD_R {
        RDHOLD_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read Setup Time"]
    #[inline(always)]
    pub fn rdsetup(&mut self) -> RDSETUP_W {
        RDSETUP_W::new(self)
    }
    #[doc = "Bits 8:11 - Read Strobe Time"]
    #[inline(always)]
    pub fn rdstrb(&mut self) -> RDSTRB_W {
        RDSTRB_W::new(self)
    }
    #[doc = "Bits 16:17 - Read Hold Time"]
    #[inline(always)]
    pub fn rdhold(&mut self) -> RDHOLD_W {
        RDHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdtiming](index.html) module"]
pub struct RDTIMING_SPEC;
impl crate::RegisterSpec for RDTIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdtiming::R](R) reader structure"]
impl crate::Readable for RDTIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdtiming::W](W) writer structure"]
impl crate::Writable for RDTIMING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RDTIMING to value 0"]
impl crate::Resettable for RDTIMING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
