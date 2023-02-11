#[doc = "Register `ROUTE` reader"]
pub struct R(crate::R<ROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTE` writer"]
pub struct W(crate::W<ROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTE_SPEC>;
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
impl From<crate::W<ROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EBIPEN` reader - EBI Pin Enable"]
pub type EBIPEN_R = crate::BitReader<bool>;
#[doc = "Field `EBIPEN` writer - EBI Pin Enable"]
pub type EBIPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `CS0PEN` reader - EBI_CS0 Pin Enable"]
pub type CS0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CS0PEN` writer - EBI_CS0 Pin Enable"]
pub type CS0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `CS1PEN` reader - EBI_CS1 Pin Enable"]
pub type CS1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CS1PEN` writer - EBI_CS1 Pin Enable"]
pub type CS1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `CS2PEN` reader - EBI_CS2 Pin Enable"]
pub type CS2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CS2PEN` writer - EBI_CS2 Pin Enable"]
pub type CS2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `CS3PEN` reader - EBI_CS3 Pin Enable"]
pub type CS3PEN_R = crate::BitReader<bool>;
#[doc = "Field `CS3PEN` writer - EBI_CS3 Pin Enable"]
pub type CS3PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `ALEPEN` reader - EBI_ALE Pin Enable"]
pub type ALEPEN_R = crate::BitReader<bool>;
#[doc = "Field `ALEPEN` writer - EBI_ALE Pin Enable"]
pub type ALEPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `ARDYPEN` reader - EBI_ARDY Pin Enable"]
pub type ARDYPEN_R = crate::BitReader<bool>;
#[doc = "Field `ARDYPEN` writer - EBI_ARDY Pin Enable"]
pub type ARDYPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline(always)]
    pub fn ebipen(&self) -> EBIPEN_R {
        EBIPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&self) -> CS0PEN_R {
        CS0PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&self) -> CS1PEN_R {
        CS1PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline(always)]
    pub fn cs2pen(&self) -> CS2PEN_R {
        CS2PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline(always)]
    pub fn cs3pen(&self) -> CS3PEN_R {
        CS3PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline(always)]
    pub fn alepen(&self) -> ALEPEN_R {
        ALEPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline(always)]
    pub fn ardypen(&self) -> ARDYPEN_R {
        ARDYPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EBI Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ebipen(&mut self) -> EBIPEN_W<0> {
        EBIPEN_W::new(self)
    }
    #[doc = "Bit 1 - EBI_CS0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs0pen(&mut self) -> CS0PEN_W<1> {
        CS0PEN_W::new(self)
    }
    #[doc = "Bit 2 - EBI_CS1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs1pen(&mut self) -> CS1PEN_W<2> {
        CS1PEN_W::new(self)
    }
    #[doc = "Bit 3 - EBI_CS2 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs2pen(&mut self) -> CS2PEN_W<3> {
        CS2PEN_W::new(self)
    }
    #[doc = "Bit 4 - EBI_CS3 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs3pen(&mut self) -> CS3PEN_W<4> {
        CS3PEN_W::new(self)
    }
    #[doc = "Bit 5 - EBI_ALE Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alepen(&mut self) -> ALEPEN_W<5> {
        ALEPEN_W::new(self)
    }
    #[doc = "Bit 6 - EBI_ARDY Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ardypen(&mut self) -> ARDYPEN_W<6> {
        ARDYPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [route](index.html) module"]
pub struct ROUTE_SPEC;
impl crate::RegisterSpec for ROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [route::R](R) reader structure"]
impl crate::Readable for ROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [route::W](W) writer structure"]
impl crate::Writable for ROUTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTE to value 0"]
impl crate::Resettable for ROUTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
