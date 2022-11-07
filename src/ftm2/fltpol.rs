#[doc = "Register `FLTPOL` reader"]
pub struct R(crate::R<FLTPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTPOL` writer"]
pub struct W(crate::W<FLTPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTPOL_SPEC>;
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
impl From<crate::W<FLTPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTPOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT0POL` reader - Fault Input 0 Polarity"]
pub type FLT0POL_R = crate::BitReader<FLT0POL_A>;
#[doc = "Fault Input 0 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT0POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    _1 = 1,
}
impl From<FLT0POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT0POL_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT0POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT0POL_A {
        match self.bits {
            false => FLT0POL_A::_0,
            true => FLT0POL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT0POL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLT0POL_A::_1
    }
}
#[doc = "Field `FLT0POL` writer - Fault Input 0 Polarity"]
pub type FLT0POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTPOL_SPEC, FLT0POL_A, O>;
impl<'a, const O: u8> FLT0POL_W<'a, O> {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT0POL_A::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT0POL_A::_1)
    }
}
#[doc = "Field `FLT1POL` reader - Fault Input 1 Polarity"]
pub type FLT1POL_R = crate::BitReader<FLT1POL_A>;
#[doc = "Fault Input 1 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    _1 = 1,
}
impl From<FLT1POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1POL_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT1POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1POL_A {
        match self.bits {
            false => FLT1POL_A::_0,
            true => FLT1POL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT1POL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLT1POL_A::_1
    }
}
#[doc = "Field `FLT1POL` writer - Fault Input 1 Polarity"]
pub type FLT1POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTPOL_SPEC, FLT1POL_A, O>;
impl<'a, const O: u8> FLT1POL_W<'a, O> {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT1POL_A::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT1POL_A::_1)
    }
}
#[doc = "Field `FLT2POL` reader - Fault Input 2 Polarity"]
pub type FLT2POL_R = crate::BitReader<FLT2POL_A>;
#[doc = "Fault Input 2 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT2POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    _1 = 1,
}
impl From<FLT2POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT2POL_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT2POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT2POL_A {
        match self.bits {
            false => FLT2POL_A::_0,
            true => FLT2POL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT2POL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLT2POL_A::_1
    }
}
#[doc = "Field `FLT2POL` writer - Fault Input 2 Polarity"]
pub type FLT2POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTPOL_SPEC, FLT2POL_A, O>;
impl<'a, const O: u8> FLT2POL_W<'a, O> {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT2POL_A::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT2POL_A::_1)
    }
}
#[doc = "Field `FLT3POL` reader - Fault Input 3 Polarity"]
pub type FLT3POL_R = crate::BitReader<FLT3POL_A>;
#[doc = "Fault Input 3 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT3POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    _1 = 1,
}
impl From<FLT3POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT3POL_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT3POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT3POL_A {
        match self.bits {
            false => FLT3POL_A::_0,
            true => FLT3POL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT3POL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLT3POL_A::_1
    }
}
#[doc = "Field `FLT3POL` writer - Fault Input 3 Polarity"]
pub type FLT3POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTPOL_SPEC, FLT3POL_A, O>;
impl<'a, const O: u8> FLT3POL_W<'a, O> {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT3POL_A::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT3POL_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Fault Input 0 Polarity"]
    #[inline(always)]
    pub fn flt0pol(&self) -> FLT0POL_R {
        FLT0POL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Input 1 Polarity"]
    #[inline(always)]
    pub fn flt1pol(&self) -> FLT1POL_R {
        FLT1POL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Input 2 Polarity"]
    #[inline(always)]
    pub fn flt2pol(&self) -> FLT2POL_R {
        FLT2POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Input 3 Polarity"]
    #[inline(always)]
    pub fn flt3pol(&self) -> FLT3POL_R {
        FLT3POL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Input 0 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt0pol(&mut self) -> FLT0POL_W<0> {
        FLT0POL_W::new(self)
    }
    #[doc = "Bit 1 - Fault Input 1 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt1pol(&mut self) -> FLT1POL_W<1> {
        FLT1POL_W::new(self)
    }
    #[doc = "Bit 2 - Fault Input 2 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt2pol(&mut self) -> FLT2POL_W<2> {
        FLT2POL_W::new(self)
    }
    #[doc = "Bit 3 - Fault Input 3 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn flt3pol(&mut self) -> FLT3POL_W<3> {
        FLT3POL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FTM Fault Input Polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltpol](index.html) module"]
pub struct FLTPOL_SPEC;
impl crate::RegisterSpec for FLTPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltpol::R](R) reader structure"]
impl crate::Readable for FLTPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltpol::W](W) writer structure"]
impl crate::Writable for FLTPOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTPOL to value 0"]
impl crate::Resettable for FLTPOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
