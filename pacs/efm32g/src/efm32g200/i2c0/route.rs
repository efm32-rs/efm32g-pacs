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
#[doc = "Field `SDAPEN` reader - SDA Pin Enable"]
pub type SDAPEN_R = crate::BitReader<bool>;
#[doc = "Field `SDAPEN` writer - SDA Pin Enable"]
pub type SDAPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `SCLPEN` reader - SCL Pin Enable"]
pub type SCLPEN_R = crate::BitReader<bool>;
#[doc = "Field `SCLPEN` writer - SCL Pin Enable"]
pub type SCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, O>;
#[doc = "Field `LOCATION` reader - I/O Location"]
pub type LOCATION_R = crate::FieldReader<u8, LOCATION_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCATION_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
}
impl From<LOCATION_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCATION_A) -> Self {
        variant as _
    }
}
impl LOCATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCATION_A {
        match self.bits {
            0 => LOCATION_A::LOC0,
            1 => LOCATION_A::LOC1,
            2 => LOCATION_A::LOC2,
            3 => LOCATION_A::LOC3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == LOCATION_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == LOCATION_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == LOCATION_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == LOCATION_A::LOC3
    }
}
#[doc = "Field `LOCATION` writer - I/O Location"]
pub type LOCATION_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ROUTE_SPEC, u8, LOCATION_A, 2, O>;
impl<'a, const O: u8> LOCATION_W<'a, O> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(LOCATION_A::LOC3)
    }
}
impl R {
    #[doc = "Bit 0 - SDA Pin Enable"]
    #[inline(always)]
    pub fn sdapen(&self) -> SDAPEN_R {
        SDAPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL Pin Enable"]
    #[inline(always)]
    pub fn sclpen(&self) -> SCLPEN_R {
        SCLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I/O Location"]
    #[inline(always)]
    pub fn location(&self) -> LOCATION_R {
        LOCATION_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SDA Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdapen(&mut self) -> SDAPEN_W<0> {
        SDAPEN_W::new(self)
    }
    #[doc = "Bit 1 - SCL Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sclpen(&mut self) -> SCLPEN_W<1> {
        SCLPEN_W::new(self)
    }
    #[doc = "Bits 8:9 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn location(&mut self) -> LOCATION_W<8> {
        LOCATION_W::new(self)
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
