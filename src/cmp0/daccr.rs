#[doc = "Register `DACCR` reader"]
pub struct R(crate::R<DACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACCR` writer"]
pub struct W(crate::W<DACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACCR_SPEC>;
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
impl From<crate::W<DACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOSEL` reader - DAC Output Voltage Select"]
pub type VOSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOSEL` writer - DAC Output Voltage Select"]
pub type VOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DACCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `VRSEL` reader - Supply Voltage Reference Source Select"]
pub type VRSEL_R = crate::BitReader<VRSEL_A>;
#[doc = "Supply Voltage Reference Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VRSEL_A {
    #[doc = "0: V is selected as resistor ladder network supply reference V. in1 in"]
    _0 = 0,
    #[doc = "1: V is selected as resistor ladder network supply reference V. in2 in"]
    _1 = 1,
}
impl From<VRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VRSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRSEL_A {
        match self.bits {
            false => VRSEL_A::_0,
            true => VRSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VRSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VRSEL_A::_1
    }
}
#[doc = "Field `VRSEL` writer - Supply Voltage Reference Source Select"]
pub type VRSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, DACCR_SPEC, VRSEL_A, O>;
impl<'a, const O: u8> VRSEL_W<'a, O> {
    #[doc = "V is selected as resistor ladder network supply reference V. in1 in"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VRSEL_A::_0)
    }
    #[doc = "V is selected as resistor ladder network supply reference V. in2 in"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VRSEL_A::_1)
    }
}
#[doc = "Field `DACEN` reader - DAC Enable"]
pub type DACEN_R = crate::BitReader<DACEN_A>;
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACEN_A {
    #[doc = "0: DAC is disabled."]
    _0 = 0,
    #[doc = "1: DAC is enabled."]
    _1 = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::_0,
            true => DACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACEN_A::_1
    }
}
#[doc = "Field `DACEN` writer - DAC Enable"]
pub type DACEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, DACCR_SPEC, DACEN_A, O>;
impl<'a, const O: u8> DACEN_W<'a, O> {
    #[doc = "DAC is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACEN_A::_0)
    }
    #[doc = "DAC is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&self) -> VOSEL_R {
        VOSEL_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&self) -> VRSEL_R {
        VRSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DAC Output Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn vosel(&mut self) -> VOSEL_W<0> {
        VOSEL_W::new(self)
    }
    #[doc = "Bit 6 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn vrsel(&mut self) -> VRSEL_W<6> {
        VRSEL_W::new(self)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<7> {
        DACEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daccr](index.html) module"]
pub struct DACCR_SPEC;
impl crate::RegisterSpec for DACCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [daccr::R](R) reader structure"]
impl crate::Readable for DACCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daccr::W](W) writer structure"]
impl crate::Writable for DACCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACCR to value 0"]
impl crate::Resettable for DACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
