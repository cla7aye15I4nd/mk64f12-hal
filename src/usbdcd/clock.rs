#[doc = "Register `CLOCK` reader"]
pub struct R(crate::R<CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK` writer"]
pub struct W(crate::W<CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_SPEC>;
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
impl From<crate::W<CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLOCK_UNIT` reader - Unit of Measurement Encoding for Clock Speed"]
pub type CLOCK_UNIT_R = crate::BitReader<CLOCK_UNIT_A>;
#[doc = "Unit of Measurement Encoding for Clock Speed\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLOCK_UNIT_A {
    #[doc = "0: kHz Speed (between 1 kHz and 1023 kHz)"]
    _0 = 0,
    #[doc = "1: MHz Speed (between 1 MHz and 1023 MHz)"]
    _1 = 1,
}
impl From<CLOCK_UNIT_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_UNIT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLOCK_UNIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_UNIT_A {
        match self.bits {
            false => CLOCK_UNIT_A::_0,
            true => CLOCK_UNIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLOCK_UNIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLOCK_UNIT_A::_1
    }
}
#[doc = "Field `CLOCK_UNIT` writer - Unit of Measurement Encoding for Clock Speed"]
pub type CLOCK_UNIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_SPEC, CLOCK_UNIT_A, O>;
impl<'a, const O: u8> CLOCK_UNIT_W<'a, O> {
    #[doc = "kHz Speed (between 1 kHz and 1023 kHz)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLOCK_UNIT_A::_0)
    }
    #[doc = "MHz Speed (between 1 MHz and 1023 MHz)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLOCK_UNIT_A::_1)
    }
}
#[doc = "Field `CLOCK_SPEED` reader - Numerical Value of Clock Speed in Binary"]
pub type CLOCK_SPEED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLOCK_SPEED` writer - Numerical Value of Clock Speed in Binary"]
pub type CLOCK_SPEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCK_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - Unit of Measurement Encoding for Clock Speed"]
    #[inline(always)]
    pub fn clock_unit(&self) -> CLOCK_UNIT_R {
        CLOCK_UNIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:11 - Numerical Value of Clock Speed in Binary"]
    #[inline(always)]
    pub fn clock_speed(&self) -> CLOCK_SPEED_R {
        CLOCK_SPEED_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Unit of Measurement Encoding for Clock Speed"]
    #[inline(always)]
    #[must_use]
    pub fn clock_unit(&mut self) -> CLOCK_UNIT_W<0> {
        CLOCK_UNIT_W::new(self)
    }
    #[doc = "Bits 2:11 - Numerical Value of Clock Speed in Binary"]
    #[inline(always)]
    #[must_use]
    pub fn clock_speed(&mut self) -> CLOCK_SPEED_W<2> {
        CLOCK_SPEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock](index.html) module"]
pub struct CLOCK_SPEC;
impl crate::RegisterSpec for CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock::R](R) reader structure"]
impl crate::Readable for CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock::W](W) writer structure"]
impl crate::Writable for CLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK to value 0xc1"]
impl crate::Resettable for CLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0xc1;
}
