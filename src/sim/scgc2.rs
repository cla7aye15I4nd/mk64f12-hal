#[doc = "Register `SCGC2` reader"]
pub struct R(crate::R<SCGC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGC2` writer"]
pub struct W(crate::W<SCGC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGC2_SPEC>;
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
impl From<crate::W<SCGC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENET` reader - ENET Clock Gate Control"]
pub type ENET_R = crate::BitReader<ENET_A>;
#[doc = "ENET Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENET_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<ENET_A> for bool {
    #[inline(always)]
    fn from(variant: ENET_A) -> Self {
        variant as u8 != 0
    }
}
impl ENET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET_A {
        match self.bits {
            false => ENET_A::_0,
            true => ENET_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENET_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENET_A::_1
    }
}
#[doc = "Field `ENET` writer - ENET Clock Gate Control"]
pub type ENET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC2_SPEC, ENET_A, O>;
impl<'a, const O: u8> ENET_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENET_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENET_A::_1)
    }
}
#[doc = "Field `DAC0` reader - DAC0 Clock Gate Control"]
pub type DAC0_R = crate::BitReader<DAC0_A>;
#[doc = "DAC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DAC0_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC0_A {
        match self.bits {
            false => DAC0_A::_0,
            true => DAC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAC0_A::_1
    }
}
#[doc = "Field `DAC0` writer - DAC0 Clock Gate Control"]
pub type DAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC2_SPEC, DAC0_A, O>;
impl<'a, const O: u8> DAC0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAC0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAC0_A::_1)
    }
}
#[doc = "Field `DAC1` reader - DAC1 Clock Gate Control"]
pub type DAC1_R = crate::BitReader<DAC1_A>;
#[doc = "DAC1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DAC1_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1_A {
        match self.bits {
            false => DAC1_A::_0,
            true => DAC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAC1_A::_1
    }
}
#[doc = "Field `DAC1` writer - DAC1 Clock Gate Control"]
pub type DAC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC2_SPEC, DAC1_A, O>;
impl<'a, const O: u8> DAC1_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAC1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAC1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ENET Clock Gate Control"]
    #[inline(always)]
    pub fn enet(&self) -> ENET_R {
        ENET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 12 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&self) -> DAC0_R {
        DAC0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC1 Clock Gate Control"]
    #[inline(always)]
    pub fn dac1(&self) -> DAC1_R {
        DAC1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENET Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn enet(&mut self) -> ENET_W<0> {
        ENET_W::new(self)
    }
    #[doc = "Bit 12 - DAC0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dac0(&mut self) -> DAC0_W<12> {
        DAC0_W::new(self)
    }
    #[doc = "Bit 13 - DAC1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dac1(&mut self) -> DAC1_W<13> {
        DAC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Gating Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc2](index.html) module"]
pub struct SCGC2_SPEC;
impl crate::RegisterSpec for SCGC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgc2::R](R) reader structure"]
impl crate::Readable for SCGC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgc2::W](W) writer structure"]
impl crate::Writable for SCGC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGC2 to value 0"]
impl crate::Resettable for SCGC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
