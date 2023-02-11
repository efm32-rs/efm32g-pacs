#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 8 bit data, 8 bit address, ALE not used."]
    D8A8 = 0,
    #[doc = "1: 16 bit data, 16 bit address, ALE is used for address latching."]
    D16A16ALE = 1,
    #[doc = "2: 8 bit data, 24 bit address, ALE is used for address latching."]
    D8A24ALE = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::D8A8),
            1 => Some(MODE_A::D16A16ALE),
            2 => Some(MODE_A::D8A24ALE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE_A::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE_A::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE_A::D8A24ALE
    }
}
#[doc = "Field `MODE` writer - Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "8 bit data, 8 bit address, ALE not used."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE_A::D8A8)
    }
    #[doc = "16 bit data, 16 bit address, ALE is used for address latching."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE_A::D16A16ALE)
    }
    #[doc = "8 bit data, 24 bit address, ALE is used for address latching."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE_A::D8A24ALE)
    }
}
#[doc = "Field `BANK0EN` reader - Bank 0 Enable"]
pub type BANK0EN_R = crate::BitReader<bool>;
#[doc = "Field `BANK0EN` writer - Bank 0 Enable"]
pub type BANK0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BANK1EN` reader - Bank 1 Enable"]
pub type BANK1EN_R = crate::BitReader<bool>;
#[doc = "Field `BANK1EN` writer - Bank 1 Enable"]
pub type BANK1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BANK2EN` reader - Bank 2 Enable"]
pub type BANK2EN_R = crate::BitReader<bool>;
#[doc = "Field `BANK2EN` writer - Bank 2 Enable"]
pub type BANK2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BANK3EN` reader - Bank 3 Enable"]
pub type BANK3EN_R = crate::BitReader<bool>;
#[doc = "Field `BANK3EN` writer - Bank 3 Enable"]
pub type BANK3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ARDYEN` reader - ARDY Enable"]
pub type ARDYEN_R = crate::BitReader<bool>;
#[doc = "Field `ARDYEN` writer - ARDY Enable"]
pub type ARDYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ARDYTODIS` reader - ARDY Timeout Disable"]
pub type ARDYTODIS_R = crate::BitReader<bool>;
#[doc = "Field `ARDYTODIS` writer - ARDY Timeout Disable"]
pub type ARDYTODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline(always)]
    pub fn bank0en(&self) -> BANK0EN_R {
        BANK0EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline(always)]
    pub fn bank1en(&self) -> BANK1EN_R {
        BANK1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline(always)]
    pub fn bank2en(&self) -> BANK2EN_R {
        BANK2EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline(always)]
    pub fn bank3en(&self) -> BANK3EN_R {
        BANK3EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline(always)]
    pub fn ardyen(&self) -> ARDYEN_R {
        ARDYEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline(always)]
    pub fn ardytodis(&self) -> ARDYTODIS_R {
        ARDYTODIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank0en(&mut self) -> BANK0EN_W<8> {
        BANK0EN_W::new(self)
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank1en(&mut self) -> BANK1EN_W<9> {
        BANK1EN_W::new(self)
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank2en(&mut self) -> BANK2EN_W<10> {
        BANK2EN_W::new(self)
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bank3en(&mut self) -> BANK3EN_W<11> {
        BANK3EN_W::new(self)
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ardyen(&mut self) -> ARDYEN_W<16> {
        ARDYEN_W::new(self)
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ardytodis(&mut self) -> ARDYTODIS_W<17> {
        ARDYTODIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
