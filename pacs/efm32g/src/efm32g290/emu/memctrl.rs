#[doc = "Register `MEMCTRL` reader"]
pub struct R(crate::R<MEMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMCTRL` writer"]
pub struct W(crate::W<MEMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMCTRL_SPEC>;
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
impl From<crate::W<MEMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RAM block power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POWERDOWN_A {
    #[doc = "4: Power down RAM block 3 (address range 0x20018000-0x2001FFFF)."]
    BLK3 = 4,
    #[doc = "6: Power down RAM blocks 2-3 (address range 0x20010000-0x2001FFFF)."]
    BLK23 = 6,
    #[doc = "7: Power down RAM blocks 1-3 (address range 0x20008000-0x2001FFFF)."]
    BLK123 = 7,
}
impl From<POWERDOWN_A> for u8 {
    #[inline(always)]
    fn from(variant: POWERDOWN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POWERDOWN` reader - RAM block power-down"]
pub type POWERDOWN_R = crate::FieldReader<u8, POWERDOWN_A>;
impl POWERDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POWERDOWN_A> {
        match self.bits {
            4 => Some(POWERDOWN_A::BLK3),
            6 => Some(POWERDOWN_A::BLK23),
            7 => Some(POWERDOWN_A::BLK123),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLK3`"]
    #[inline(always)]
    pub fn is_blk3(&self) -> bool {
        *self == POWERDOWN_A::BLK3
    }
    #[doc = "Checks if the value of the field is `BLK23`"]
    #[inline(always)]
    pub fn is_blk23(&self) -> bool {
        *self == POWERDOWN_A::BLK23
    }
    #[doc = "Checks if the value of the field is `BLK123`"]
    #[inline(always)]
    pub fn is_blk123(&self) -> bool {
        *self == POWERDOWN_A::BLK123
    }
}
#[doc = "Field `POWERDOWN` writer - RAM block power-down"]
pub type POWERDOWN_W<'a> = crate::FieldWriter<'a, u32, MEMCTRL_SPEC, u8, POWERDOWN_A, 3, 0>;
impl<'a> POWERDOWN_W<'a> {
    #[doc = "Power down RAM block 3 (address range 0x20018000-0x2001FFFF)."]
    #[inline(always)]
    pub fn blk3(self) -> &'a mut W {
        self.variant(POWERDOWN_A::BLK3)
    }
    #[doc = "Power down RAM blocks 2-3 (address range 0x20010000-0x2001FFFF)."]
    #[inline(always)]
    pub fn blk23(self) -> &'a mut W {
        self.variant(POWERDOWN_A::BLK23)
    }
    #[doc = "Power down RAM blocks 1-3 (address range 0x20008000-0x2001FFFF)."]
    #[inline(always)]
    pub fn blk123(self) -> &'a mut W {
        self.variant(POWERDOWN_A::BLK123)
    }
}
impl R {
    #[doc = "Bits 0:2 - RAM block power-down"]
    #[inline(always)]
    pub fn powerdown(&self) -> POWERDOWN_R {
        POWERDOWN_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RAM block power-down"]
    #[inline(always)]
    pub fn powerdown(&mut self) -> POWERDOWN_W {
        POWERDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memctrl](index.html) module"]
pub struct MEMCTRL_SPEC;
impl crate::RegisterSpec for MEMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memctrl::R](R) reader structure"]
impl crate::Readable for MEMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memctrl::W](W) writer structure"]
impl crate::Writable for MEMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMCTRL to value 0"]
impl crate::Resettable for MEMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
