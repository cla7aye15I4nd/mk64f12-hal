#[doc = "Register `SCGC1` reader"]
pub struct R(crate::R<SCGC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGC1` writer"]
pub struct W(crate::W<SCGC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGC1_SPEC>;
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
impl From<crate::W<SCGC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C2` reader - I2C2 Clock Gate Control"]
pub type I2C2_R = crate::BitReader<I2C2_A>;
#[doc = "I2C2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<I2C2_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_A {
        match self.bits {
            false => I2C2_A::_0,
            true => I2C2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C2_A::_1
    }
}
#[doc = "Field `I2C2` writer - I2C2 Clock Gate Control"]
pub type I2C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC1_SPEC, I2C2_A, O>;
impl<'a, const O: u8> I2C2_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C2_A::_1)
    }
}
#[doc = "Field `UART4` reader - UART4 Clock Gate Control"]
pub type UART4_R = crate::BitReader<UART4_A>;
#[doc = "UART4 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART4_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<UART4_A> for bool {
    #[inline(always)]
    fn from(variant: UART4_A) -> Self {
        variant as u8 != 0
    }
}
impl UART4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4_A {
        match self.bits {
            false => UART4_A::_0,
            true => UART4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART4_A::_1
    }
}
#[doc = "Field `UART4` writer - UART4 Clock Gate Control"]
pub type UART4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC1_SPEC, UART4_A, O>;
impl<'a, const O: u8> UART4_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART4_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART4_A::_1)
    }
}
#[doc = "Field `UART5` reader - UART5 Clock Gate Control"]
pub type UART5_R = crate::BitReader<UART5_A>;
#[doc = "UART5 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART5_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<UART5_A> for bool {
    #[inline(always)]
    fn from(variant: UART5_A) -> Self {
        variant as u8 != 0
    }
}
impl UART5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART5_A {
        match self.bits {
            false => UART5_A::_0,
            true => UART5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART5_A::_1
    }
}
#[doc = "Field `UART5` writer - UART5 Clock Gate Control"]
pub type UART5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC1_SPEC, UART5_A, O>;
impl<'a, const O: u8> UART5_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART5_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART5_A::_1)
    }
}
impl R {
    #[doc = "Bit 6 - I2C2 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - UART4 Clock Gate Control"]
    #[inline(always)]
    pub fn uart4(&self) -> UART4_R {
        UART4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART5 Clock Gate Control"]
    #[inline(always)]
    pub fn uart5(&self) -> UART5_R {
        UART5_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - I2C2 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<6> {
        I2C2_W::new(self)
    }
    #[doc = "Bit 10 - UART4 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn uart4(&mut self) -> UART4_W<10> {
        UART4_W::new(self)
    }
    #[doc = "Bit 11 - UART5 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn uart5(&mut self) -> UART5_W<11> {
        UART5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Gating Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc1](index.html) module"]
pub struct SCGC1_SPEC;
impl crate::RegisterSpec for SCGC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgc1::R](R) reader structure"]
impl crate::Readable for SCGC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgc1::W](W) writer structure"]
impl crate::Writable for SCGC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGC1 to value 0"]
impl crate::Resettable for SCGC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
