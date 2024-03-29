#[doc = "Register `HFPERCLKEN0` reader"]
pub struct R(crate::R<HFPERCLKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFPERCLKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFPERCLKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFPERCLKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFPERCLKEN0` writer"]
pub struct W(crate::W<HFPERCLKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFPERCLKEN0_SPEC>;
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
impl From<crate::W<HFPERCLKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFPERCLKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART0` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type USART0_R = crate::BitReader<bool>;
#[doc = "Field `USART0` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type USART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `USART1` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type USART1_R = crate::BitReader<bool>;
#[doc = "Field `USART1` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
pub type USART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `TIMER0` reader - Timer 0 Clock Enable"]
pub type TIMER0_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0` writer - Timer 0 Clock Enable"]
pub type TIMER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `TIMER1` reader - Timer 1 Clock Enable"]
pub type TIMER1_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1` writer - Timer 1 Clock Enable"]
pub type TIMER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `TIMER2` reader - Timer 2 Clock Enable"]
pub type TIMER2_R = crate::BitReader<bool>;
#[doc = "Field `TIMER2` writer - Timer 2 Clock Enable"]
pub type TIMER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `ACMP0` reader - Analog Comparator 0 Clock Enable"]
pub type ACMP0_R = crate::BitReader<bool>;
#[doc = "Field `ACMP0` writer - Analog Comparator 0 Clock Enable"]
pub type ACMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `ACMP1` reader - Analog Comparator 1 Clock Enable"]
pub type ACMP1_R = crate::BitReader<bool>;
#[doc = "Field `ACMP1` writer - Analog Comparator 1 Clock Enable"]
pub type ACMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `PRS` reader - Peripheral Reflex System Clock Enable"]
pub type PRS_R = crate::BitReader<bool>;
#[doc = "Field `PRS` writer - Peripheral Reflex System Clock Enable"]
pub type PRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `DAC0` reader - Digital to Analog Converter 0 Clock Enable"]
pub type DAC0_R = crate::BitReader<bool>;
#[doc = "Field `DAC0` writer - Digital to Analog Converter 0 Clock Enable"]
pub type DAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `GPIO` reader - General purpose Input/Output Clock Enable"]
pub type GPIO_R = crate::BitReader<bool>;
#[doc = "Field `GPIO` writer - General purpose Input/Output Clock Enable"]
pub type GPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `VCMP` reader - Voltage Comparator Clock Enable"]
pub type VCMP_R = crate::BitReader<bool>;
#[doc = "Field `VCMP` writer - Voltage Comparator Clock Enable"]
pub type VCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `ADC0` reader - Analog to Digital Converter 0 Clock Enable"]
pub type ADC0_R = crate::BitReader<bool>;
#[doc = "Field `ADC0` writer - Analog to Digital Converter 0 Clock Enable"]
pub type ADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
#[doc = "Field `I2C0` reader - I2C 0 Clock Enable"]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - I2C 0 Clock Enable"]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HFPERCLKEN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn usart0(&self) -> USART0_R {
        USART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    pub fn acmp1(&self) -> ACMP1_R {
        ACMP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn dac0(&self) -> DAC0_R {
        DAC0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Voltage Comparator Clock Enable"]
    #[inline(always)]
    pub fn vcmp(&self) -> VCMP_R {
        VCMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C 0 Clock Enable"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Universal Synchronous/Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0(&mut self) -> USART0_W<0> {
        USART0_W::new(self)
    }
    #[doc = "Bit 1 - Universal Synchronous/Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1(&mut self) -> USART1_W<1> {
        USART1_W::new(self)
    }
    #[doc = "Bit 4 - Timer 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer0(&mut self) -> TIMER0_W<4> {
        TIMER0_W::new(self)
    }
    #[doc = "Bit 5 - Timer 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer1(&mut self) -> TIMER1_W<5> {
        TIMER1_W::new(self)
    }
    #[doc = "Bit 6 - Timer 2 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer2(&mut self) -> TIMER2_W<6> {
        TIMER2_W::new(self)
    }
    #[doc = "Bit 7 - Analog Comparator 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0(&mut self) -> ACMP0_W<7> {
        ACMP0_W::new(self)
    }
    #[doc = "Bit 8 - Analog Comparator 1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1(&mut self) -> ACMP1_W<8> {
        ACMP1_W::new(self)
    }
    #[doc = "Bit 10 - Peripheral Reflex System Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<10> {
        PRS_W::new(self)
    }
    #[doc = "Bit 11 - Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac0(&mut self) -> DAC0_W<11> {
        DAC0_W::new(self)
    }
    #[doc = "Bit 12 - General purpose Input/Output Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<12> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 13 - Voltage Comparator Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vcmp(&mut self) -> VCMP_W<13> {
        VCMP_W::new(self)
    }
    #[doc = "Bit 14 - Analog to Digital Converter 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<14> {
        ADC0_W::new(self)
    }
    #[doc = "Bit 15 - I2C 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<15> {
        I2C0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Frequency Peripheral Clock Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfperclken0](index.html) module"]
pub struct HFPERCLKEN0_SPEC;
impl crate::RegisterSpec for HFPERCLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfperclken0::R](R) reader structure"]
impl crate::Readable for HFPERCLKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfperclken0::W](W) writer structure"]
impl crate::Writable for HFPERCLKEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFPERCLKEN0 to value 0"]
impl crate::Resettable for HFPERCLKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
