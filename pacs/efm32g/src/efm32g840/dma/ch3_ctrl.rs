#[doc = "Register `CH3_CTRL` reader"]
pub struct R(crate::R<CH3_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3_CTRL` writer"]
pub struct W(crate::W<CH3_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3_CTRL_SPEC>;
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
impl From<crate::W<CH3_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH3_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<u8, SOURCESEL_A>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE = 0,
    #[doc = "8: Analog to Digital Converter 0"]
    ADC0 = 8,
    #[doc = "10: Digital to Analog Converter 0"]
    DAC0 = 10,
    #[doc = "12: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 12,
    #[doc = "13: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 13,
    #[doc = "14: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2 = 14,
    #[doc = "16: Low Energy UART 0"]
    LEUART0 = 16,
    #[doc = "17: Low Energy UART 1"]
    LEUART1 = 17,
    #[doc = "20: I2C 0"]
    I2C0 = 20,
    #[doc = "24: Timer 0"]
    TIMER0 = 24,
    #[doc = "25: Timer 1"]
    TIMER1 = 25,
    #[doc = "26: Timer 2"]
    TIMER2 = 26,
    #[doc = "48: `110000`"]
    MSC = 48,
    #[doc = "49: Advanced Encryption Standard Accelerator"]
    AES = 49,
}
impl From<SOURCESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCESEL_A) -> Self {
        variant as _
    }
}
impl SOURCESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SOURCESEL_A> {
        match self.bits {
            0 => Some(SOURCESEL_A::NONE),
            8 => Some(SOURCESEL_A::ADC0),
            10 => Some(SOURCESEL_A::DAC0),
            12 => Some(SOURCESEL_A::USART0),
            13 => Some(SOURCESEL_A::USART1),
            14 => Some(SOURCESEL_A::USART2),
            16 => Some(SOURCESEL_A::LEUART0),
            17 => Some(SOURCESEL_A::LEUART1),
            20 => Some(SOURCESEL_A::I2C0),
            24 => Some(SOURCESEL_A::TIMER0),
            25 => Some(SOURCESEL_A::TIMER1),
            26 => Some(SOURCESEL_A::TIMER2),
            48 => Some(SOURCESEL_A::MSC),
            49 => Some(SOURCESEL_A::AES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SOURCESEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL_A::ADC0
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == SOURCESEL_A::DAC0
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESEL_A::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESEL_A::USART1
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == SOURCESEL_A::USART2
    }
    #[doc = "Checks if the value of the field is `LEUART0`"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == SOURCESEL_A::LEUART0
    }
    #[doc = "Checks if the value of the field is `LEUART1`"]
    #[inline(always)]
    pub fn is_leuart1(&self) -> bool {
        *self == SOURCESEL_A::LEUART1
    }
    #[doc = "Checks if the value of the field is `I2C0`"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == SOURCESEL_A::I2C0
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESEL_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESEL_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        *self == SOURCESEL_A::TIMER2
    }
    #[doc = "Checks if the value of the field is `MSC`"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == SOURCESEL_A::MSC
    }
    #[doc = "Checks if the value of the field is `AES`"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == SOURCESEL_A::AES
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH3_CTRL_SPEC, u8, SOURCESEL_A, 6, O>;
impl<'a, const O: u8> SOURCESEL_W<'a, O> {
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SOURCESEL_A::NONE)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ADC0)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::DAC0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART1)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline(always)]
    pub fn usart2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART2)
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn leuart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LEUART0)
    }
    #[doc = "Low Energy UART 1"]
    #[inline(always)]
    pub fn leuart1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LEUART1)
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::I2C0)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER1)
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER2)
    }
    #[doc = "`110000`"]
    #[inline(always)]
    pub fn msc(self) -> &'a mut W {
        self.variant(SOURCESEL_A::MSC)
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn aes(self) -> &'a mut W {
        self.variant(SOURCESEL_A::AES)
    }
}
impl R {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<0> {
        SIGSEL_W::new(self)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<16> {
        SOURCESEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_ctrl](index.html) module"]
pub struct CH3_CTRL_SPEC;
impl crate::RegisterSpec for CH3_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3_ctrl::R](R) reader structure"]
impl crate::Readable for CH3_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3_ctrl::W](W) writer structure"]
impl crate::Writable for CH3_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3_CTRL to value 0"]
impl crate::Resettable for CH3_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
