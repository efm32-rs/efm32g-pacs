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
#[doc = "Field `HFXOMODE` reader - HFXO Mode"]
pub type HFXOMODE_R = crate::FieldReader<u8, HFXOMODE_A>;
#[doc = "HFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HFXOMODE_A {
    #[doc = "0: 4-32 MHz crystal oscillator."]
    XTAL = 0,
    #[doc = "1: An AC coupled buffer is coupled in series with HFXTAL_N, suitable for external sine wave (4-32 MHz). The sine wave should have a minimum of 200 mV peak to peak."]
    BUFEXTCLK = 1,
    #[doc = "2: Digital external clock on HFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK = 2,
}
impl From<HFXOMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXOMODE_A) -> Self {
        variant as _
    }
}
impl HFXOMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HFXOMODE_A> {
        match self.bits {
            0 => Some(HFXOMODE_A::XTAL),
            1 => Some(HFXOMODE_A::BUFEXTCLK),
            2 => Some(HFXOMODE_A::DIGEXTCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == HFXOMODE_A::XTAL
    }
    #[doc = "Checks if the value of the field is `BUFEXTCLK`"]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == HFXOMODE_A::BUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == HFXOMODE_A::DIGEXTCLK
    }
}
#[doc = "Field `HFXOMODE` writer - HFXO Mode"]
pub type HFXOMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, HFXOMODE_A, 2, O>;
impl<'a, const O: u8> HFXOMODE_W<'a, O> {
    #[doc = "4-32 MHz crystal oscillator."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(HFXOMODE_A::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with HFXTAL_N, suitable for external sine wave (4-32 MHz). The sine wave should have a minimum of 200 mV peak to peak."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut W {
        self.variant(HFXOMODE_A::BUFEXTCLK)
    }
    #[doc = "Digital external clock on HFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(HFXOMODE_A::DIGEXTCLK)
    }
}
#[doc = "Field `HFXOBOOST` reader - HFXO Start-up Boost Current"]
pub type HFXOBOOST_R = crate::FieldReader<u8, HFXOBOOST_A>;
#[doc = "HFXO Start-up Boost Current\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HFXOBOOST_A {
    #[doc = "0: 50 %."]
    _50PCENT = 0,
    #[doc = "1: 70 %."]
    _70PCENT = 1,
    #[doc = "2: 80 %."]
    _80PCENT = 2,
    #[doc = "3: 100 % (default)."]
    _100PCENT = 3,
}
impl From<HFXOBOOST_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXOBOOST_A) -> Self {
        variant as _
    }
}
impl HFXOBOOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXOBOOST_A {
        match self.bits {
            0 => HFXOBOOST_A::_50PCENT,
            1 => HFXOBOOST_A::_70PCENT,
            2 => HFXOBOOST_A::_80PCENT,
            3 => HFXOBOOST_A::_100PCENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_50PCENT`"]
    #[inline(always)]
    pub fn is_50pcent(&self) -> bool {
        *self == HFXOBOOST_A::_50PCENT
    }
    #[doc = "Checks if the value of the field is `_70PCENT`"]
    #[inline(always)]
    pub fn is_70pcent(&self) -> bool {
        *self == HFXOBOOST_A::_70PCENT
    }
    #[doc = "Checks if the value of the field is `_80PCENT`"]
    #[inline(always)]
    pub fn is_80pcent(&self) -> bool {
        *self == HFXOBOOST_A::_80PCENT
    }
    #[doc = "Checks if the value of the field is `_100PCENT`"]
    #[inline(always)]
    pub fn is_100pcent(&self) -> bool {
        *self == HFXOBOOST_A::_100PCENT
    }
}
#[doc = "Field `HFXOBOOST` writer - HFXO Start-up Boost Current"]
pub type HFXOBOOST_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, HFXOBOOST_A, 2, O>;
impl<'a, const O: u8> HFXOBOOST_W<'a, O> {
    #[doc = "50 %."]
    #[inline(always)]
    pub fn _50pcent(self) -> &'a mut W {
        self.variant(HFXOBOOST_A::_50PCENT)
    }
    #[doc = "70 %."]
    #[inline(always)]
    pub fn _70pcent(self) -> &'a mut W {
        self.variant(HFXOBOOST_A::_70PCENT)
    }
    #[doc = "80 %."]
    #[inline(always)]
    pub fn _80pcent(self) -> &'a mut W {
        self.variant(HFXOBOOST_A::_80PCENT)
    }
    #[doc = "100 % (default)."]
    #[inline(always)]
    pub fn _100pcent(self) -> &'a mut W {
        self.variant(HFXOBOOST_A::_100PCENT)
    }
}
#[doc = "Field `HFXOBUFCUR` reader - HFXO Boost Buffer Current"]
pub type HFXOBUFCUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HFXOBUFCUR` writer - HFXO Boost Buffer Current"]
pub type HFXOBUFCUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `HFXOGLITCHDETEN` reader - HFXO Glitch Detector Enable"]
pub type HFXOGLITCHDETEN_R = crate::BitReader<bool>;
#[doc = "Field `HFXOGLITCHDETEN` writer - HFXO Glitch Detector Enable"]
pub type HFXOGLITCHDETEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `HFXOTIMEOUT` reader - HFXO Timeout"]
pub type HFXOTIMEOUT_R = crate::FieldReader<u8, HFXOTIMEOUT_A>;
#[doc = "HFXO Timeout\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HFXOTIMEOUT_A {
    #[doc = "0: Timeout period of 8 cycles."]
    _8CYCLES = 0,
    #[doc = "1: Timeout period of 256 cycles."]
    _256CYCLES = 1,
    #[doc = "2: Timeout period of 1024 cycles."]
    _1KCYCLES = 2,
    #[doc = "3: Timeout period of 16384 cycles."]
    _16KCYCLES = 3,
}
impl From<HFXOTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXOTIMEOUT_A) -> Self {
        variant as _
    }
}
impl HFXOTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXOTIMEOUT_A {
        match self.bits {
            0 => HFXOTIMEOUT_A::_8CYCLES,
            1 => HFXOTIMEOUT_A::_256CYCLES,
            2 => HFXOTIMEOUT_A::_1KCYCLES,
            3 => HFXOTIMEOUT_A::_16KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == HFXOTIMEOUT_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_256CYCLES`"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == HFXOTIMEOUT_A::_256CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == HFXOTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == HFXOTIMEOUT_A::_16KCYCLES
    }
}
#[doc = "Field `HFXOTIMEOUT` writer - HFXO Timeout"]
pub type HFXOTIMEOUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, HFXOTIMEOUT_A, 2, O>;
impl<'a, const O: u8> HFXOTIMEOUT_W<'a, O> {
    #[doc = "Timeout period of 8 cycles."]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUT_A::_8CYCLES)
    }
    #[doc = "Timeout period of 256 cycles."]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUT_A::_256CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles."]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles."]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(HFXOTIMEOUT_A::_16KCYCLES)
    }
}
#[doc = "Field `LFXOMODE` reader - LFXO Mode"]
pub type LFXOMODE_R = crate::FieldReader<u8, LFXOMODE_A>;
#[doc = "LFXO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFXOMODE_A {
    #[doc = "0: 32.768 kHz crystal oscillator."]
    XTAL = 0,
    #[doc = "1: An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32.768 kHz)."]
    BUFEXTCLK = 1,
    #[doc = "2: Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    DIGEXTCLK = 2,
}
impl From<LFXOMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFXOMODE_A) -> Self {
        variant as _
    }
}
impl LFXOMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LFXOMODE_A> {
        match self.bits {
            0 => Some(LFXOMODE_A::XTAL),
            1 => Some(LFXOMODE_A::BUFEXTCLK),
            2 => Some(LFXOMODE_A::DIGEXTCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == LFXOMODE_A::XTAL
    }
    #[doc = "Checks if the value of the field is `BUFEXTCLK`"]
    #[inline(always)]
    pub fn is_bufextclk(&self) -> bool {
        *self == LFXOMODE_A::BUFEXTCLK
    }
    #[doc = "Checks if the value of the field is `DIGEXTCLK`"]
    #[inline(always)]
    pub fn is_digextclk(&self) -> bool {
        *self == LFXOMODE_A::DIGEXTCLK
    }
}
#[doc = "Field `LFXOMODE` writer - LFXO Mode"]
pub type LFXOMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, LFXOMODE_A, 2, O>;
impl<'a, const O: u8> LFXOMODE_W<'a, O> {
    #[doc = "32.768 kHz crystal oscillator."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(LFXOMODE_A::XTAL)
    }
    #[doc = "An AC coupled buffer is coupled in series with LFXTAL_N pin, suitable for external sinus wave (32.768 kHz)."]
    #[inline(always)]
    pub fn bufextclk(self) -> &'a mut W {
        self.variant(LFXOMODE_A::BUFEXTCLK)
    }
    #[doc = "Digital external clock on LFXTAL_N pin. Oscillator is effectively bypassed."]
    #[inline(always)]
    pub fn digextclk(self) -> &'a mut W {
        self.variant(LFXOMODE_A::DIGEXTCLK)
    }
}
#[doc = "Field `LFXOBOOST` reader - LFXO Start-up Boost Current"]
pub type LFXOBOOST_R = crate::BitReader<bool>;
#[doc = "Field `LFXOBOOST` writer - LFXO Start-up Boost Current"]
pub type LFXOBOOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LFXOBUFCUR` reader - LFXO Boost Buffer Current"]
pub type LFXOBUFCUR_R = crate::BitReader<bool>;
#[doc = "Field `LFXOBUFCUR` writer - LFXO Boost Buffer Current"]
pub type LFXOBUFCUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LFXOTIMEOUT` reader - LFXO Timeout"]
pub type LFXOTIMEOUT_R = crate::FieldReader<u8, LFXOTIMEOUT_A>;
#[doc = "LFXO Timeout\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFXOTIMEOUT_A {
    #[doc = "0: Timeout period of 8 cycles."]
    _8CYCLES = 0,
    #[doc = "1: Timeout period of 1024 cycles."]
    _1KCYCLES = 1,
    #[doc = "2: Timeout period of 16384 cycles."]
    _16KCYCLES = 2,
    #[doc = "3: Timeout period of 32768 cycles."]
    _32KCYCLES = 3,
}
impl From<LFXOTIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: LFXOTIMEOUT_A) -> Self {
        variant as _
    }
}
impl LFXOTIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXOTIMEOUT_A {
        match self.bits {
            0 => LFXOTIMEOUT_A::_8CYCLES,
            1 => LFXOTIMEOUT_A::_1KCYCLES,
            2 => LFXOTIMEOUT_A::_16KCYCLES,
            3 => LFXOTIMEOUT_A::_32KCYCLES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8CYCLES`"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == LFXOTIMEOUT_A::_8CYCLES
    }
    #[doc = "Checks if the value of the field is `_1KCYCLES`"]
    #[inline(always)]
    pub fn is_1kcycles(&self) -> bool {
        *self == LFXOTIMEOUT_A::_1KCYCLES
    }
    #[doc = "Checks if the value of the field is `_16KCYCLES`"]
    #[inline(always)]
    pub fn is_16kcycles(&self) -> bool {
        *self == LFXOTIMEOUT_A::_16KCYCLES
    }
    #[doc = "Checks if the value of the field is `_32KCYCLES`"]
    #[inline(always)]
    pub fn is_32kcycles(&self) -> bool {
        *self == LFXOTIMEOUT_A::_32KCYCLES
    }
}
#[doc = "Field `LFXOTIMEOUT` writer - LFXO Timeout"]
pub type LFXOTIMEOUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, LFXOTIMEOUT_A, 2, O>;
impl<'a, const O: u8> LFXOTIMEOUT_W<'a, O> {
    #[doc = "Timeout period of 8 cycles."]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUT_A::_8CYCLES)
    }
    #[doc = "Timeout period of 1024 cycles."]
    #[inline(always)]
    pub fn _1kcycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUT_A::_1KCYCLES)
    }
    #[doc = "Timeout period of 16384 cycles."]
    #[inline(always)]
    pub fn _16kcycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUT_A::_16KCYCLES)
    }
    #[doc = "Timeout period of 32768 cycles."]
    #[inline(always)]
    pub fn _32kcycles(self) -> &'a mut W {
        self.variant(LFXOTIMEOUT_A::_32KCYCLES)
    }
}
#[doc = "Field `CLKOUTSEL0` reader - Clock Output Select 0"]
pub type CLKOUTSEL0_R = crate::FieldReader<u8, CLKOUTSEL0_A>;
#[doc = "Clock Output Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL0_A {
    #[doc = "0: HFRCO (directly from oscillator)."]
    HFRCO = 0,
    #[doc = "1: HFXO (directly from oscillator)."]
    HFXO = 1,
    #[doc = "2: HFCLK/2."]
    HFCLK2 = 2,
    #[doc = "3: HFCLK/4."]
    HFCLK4 = 3,
    #[doc = "4: HFCLK/8."]
    HFCLK8 = 4,
    #[doc = "5: HFCLK/16."]
    HFCLK16 = 5,
    #[doc = "6: ULFRCO (directly from oscillator)."]
    ULFRCO = 6,
}
impl From<CLKOUTSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL0_A) -> Self {
        variant as _
    }
}
impl CLKOUTSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL0_A> {
        match self.bits {
            0 => Some(CLKOUTSEL0_A::HFRCO),
            1 => Some(CLKOUTSEL0_A::HFXO),
            2 => Some(CLKOUTSEL0_A::HFCLK2),
            3 => Some(CLKOUTSEL0_A::HFCLK4),
            4 => Some(CLKOUTSEL0_A::HFCLK8),
            5 => Some(CLKOUTSEL0_A::HFCLK16),
            6 => Some(CLKOUTSEL0_A::ULFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL0_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFCLK2`"]
    #[inline(always)]
    pub fn is_hfclk2(&self) -> bool {
        *self == CLKOUTSEL0_A::HFCLK2
    }
    #[doc = "Checks if the value of the field is `HFCLK4`"]
    #[inline(always)]
    pub fn is_hfclk4(&self) -> bool {
        *self == CLKOUTSEL0_A::HFCLK4
    }
    #[doc = "Checks if the value of the field is `HFCLK8`"]
    #[inline(always)]
    pub fn is_hfclk8(&self) -> bool {
        *self == CLKOUTSEL0_A::HFCLK8
    }
    #[doc = "Checks if the value of the field is `HFCLK16`"]
    #[inline(always)]
    pub fn is_hfclk16(&self) -> bool {
        *self == CLKOUTSEL0_A::HFCLK16
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::ULFRCO
    }
}
#[doc = "Field `CLKOUTSEL0` writer - Clock Output Select 0"]
pub type CLKOUTSEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, CLKOUTSEL0_A, 3, O>;
impl<'a, const O: u8> CLKOUTSEL0_W<'a, O> {
    #[doc = "HFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFRCO)
    }
    #[doc = "HFXO (directly from oscillator)."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFXO)
    }
    #[doc = "HFCLK/2."]
    #[inline(always)]
    pub fn hfclk2(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFCLK2)
    }
    #[doc = "HFCLK/4."]
    #[inline(always)]
    pub fn hfclk4(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFCLK4)
    }
    #[doc = "HFCLK/8."]
    #[inline(always)]
    pub fn hfclk8(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFCLK8)
    }
    #[doc = "HFCLK/16."]
    #[inline(always)]
    pub fn hfclk16(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFCLK16)
    }
    #[doc = "ULFRCO (directly from oscillator)."]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::ULFRCO)
    }
}
#[doc = "Field `CLKOUTSEL1` reader - Clock Output Select 1"]
pub type CLKOUTSEL1_R = crate::BitReader<bool>;
#[doc = "Field `CLKOUTSEL1` writer - Clock Output Select 1"]
pub type CLKOUTSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    pub fn hfxomode(&self) -> HFXOMODE_R {
        HFXOMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - HFXO Start-up Boost Current"]
    #[inline(always)]
    pub fn hfxoboost(&self) -> HFXOBOOST_R {
        HFXOBOOST_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - HFXO Boost Buffer Current"]
    #[inline(always)]
    pub fn hfxobufcur(&self) -> HFXOBUFCUR_R {
        HFXOBUFCUR_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - HFXO Glitch Detector Enable"]
    #[inline(always)]
    pub fn hfxoglitchdeten(&self) -> HFXOGLITCHDETEN_R {
        HFXOGLITCHDETEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:10 - HFXO Timeout"]
    #[inline(always)]
    pub fn hfxotimeout(&self) -> HFXOTIMEOUT_R {
        HFXOTIMEOUT_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - LFXO Mode"]
    #[inline(always)]
    pub fn lfxomode(&self) -> LFXOMODE_R {
        LFXOMODE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - LFXO Start-up Boost Current"]
    #[inline(always)]
    pub fn lfxoboost(&self) -> LFXOBOOST_R {
        LFXOBOOST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - LFXO Boost Buffer Current"]
    #[inline(always)]
    pub fn lfxobufcur(&self) -> LFXOBUFCUR_R {
        LFXOBUFCUR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - LFXO Timeout"]
    #[inline(always)]
    pub fn lfxotimeout(&self) -> LFXOTIMEOUT_R {
        LFXOTIMEOUT_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&self) -> CLKOUTSEL0_R {
        CLKOUTSEL0_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&self) -> CLKOUTSEL1_R {
        CLKOUTSEL1_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - HFXO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hfxomode(&mut self) -> HFXOMODE_W<0> {
        HFXOMODE_W::new(self)
    }
    #[doc = "Bits 2:3 - HFXO Start-up Boost Current"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoboost(&mut self) -> HFXOBOOST_W<2> {
        HFXOBOOST_W::new(self)
    }
    #[doc = "Bits 5:6 - HFXO Boost Buffer Current"]
    #[inline(always)]
    #[must_use]
    pub fn hfxobufcur(&mut self) -> HFXOBUFCUR_W<5> {
        HFXOBUFCUR_W::new(self)
    }
    #[doc = "Bit 7 - HFXO Glitch Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoglitchdeten(&mut self) -> HFXOGLITCHDETEN_W<7> {
        HFXOGLITCHDETEN_W::new(self)
    }
    #[doc = "Bits 9:10 - HFXO Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn hfxotimeout(&mut self) -> HFXOTIMEOUT_W<9> {
        HFXOTIMEOUT_W::new(self)
    }
    #[doc = "Bits 11:12 - LFXO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lfxomode(&mut self) -> LFXOMODE_W<11> {
        LFXOMODE_W::new(self)
    }
    #[doc = "Bit 13 - LFXO Start-up Boost Current"]
    #[inline(always)]
    #[must_use]
    pub fn lfxoboost(&mut self) -> LFXOBOOST_W<13> {
        LFXOBOOST_W::new(self)
    }
    #[doc = "Bit 17 - LFXO Boost Buffer Current"]
    #[inline(always)]
    #[must_use]
    pub fn lfxobufcur(&mut self) -> LFXOBUFCUR_W<17> {
        LFXOBUFCUR_W::new(self)
    }
    #[doc = "Bits 18:19 - LFXO Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn lfxotimeout(&mut self) -> LFXOTIMEOUT_W<18> {
        LFXOTIMEOUT_W::new(self)
    }
    #[doc = "Bits 20:22 - Clock Output Select 0"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel0(&mut self) -> CLKOUTSEL0_W<20> {
        CLKOUTSEL0_W::new(self)
    }
    #[doc = "Bit 23 - Clock Output Select 1"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel1(&mut self) -> CLKOUTSEL1_W<23> {
        CLKOUTSEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0x000c_262c"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x000c_262c;
}
