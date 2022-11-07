#[doc = "Register `FPDSCR` reader"]
pub struct R(crate::R<FPDSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPDSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPDSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPDSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPDSCR` writer"]
pub struct W(crate::W<FPDSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPDSCR_SPEC>;
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
impl From<crate::W<FPDSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPDSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMode` reader - Default value for FPSCR.RMode (Rounding Mode control field)."]
pub type RMODE_R = crate::FieldReader<u8, RMODE_A>;
#[doc = "Default value for FPSCR.RMode (Rounding Mode control field).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RMODE_A {
    #[doc = "0: Round to Nearest (RN) mode"]
    _00 = 0,
    #[doc = "1: Round towards Plus Infinity (RP) mode."]
    _01 = 1,
    #[doc = "2: Round towards Minus Infinity (RM) mode."]
    _10 = 2,
    #[doc = "3: Round towards Zero (RZ) mode."]
    _11 = 3,
}
impl From<RMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: RMODE_A) -> Self {
        variant as _
    }
}
impl RMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMODE_A {
        match self.bits {
            0 => RMODE_A::_00,
            1 => RMODE_A::_01,
            2 => RMODE_A::_10,
            3 => RMODE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RMODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RMODE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RMODE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RMODE_A::_11
    }
}
#[doc = "Field `RMode` writer - Default value for FPSCR.RMode (Rounding Mode control field)."]
pub type RMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FPDSCR_SPEC, u8, RMODE_A, 2, O>;
impl<'a, const O: u8> RMODE_W<'a, O> {
    #[doc = "Round to Nearest (RN) mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RMODE_A::_00)
    }
    #[doc = "Round towards Plus Infinity (RP) mode."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RMODE_A::_01)
    }
    #[doc = "Round towards Minus Infinity (RM) mode."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RMODE_A::_10)
    }
    #[doc = "Round towards Zero (RZ) mode."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RMODE_A::_11)
    }
}
#[doc = "Field `FZ` reader - Default value for FPSCR.FZ (Flush-to-zero mode control bit)."]
pub type FZ_R = crate::BitReader<FZ_A>;
#[doc = "Default value for FPSCR.FZ (Flush-to-zero mode control bit).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FZ_A {
    #[doc = "0: Flush-to-zero mode disabled. Behavior of the floating-point system is fully compliant with the IEEE 754 standard."]
    _0 = 0,
    #[doc = "1: Flush-to-zero mode enabled."]
    _1 = 1,
}
impl From<FZ_A> for bool {
    #[inline(always)]
    fn from(variant: FZ_A) -> Self {
        variant as u8 != 0
    }
}
impl FZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FZ_A {
        match self.bits {
            false => FZ_A::_0,
            true => FZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FZ_A::_1
    }
}
#[doc = "Field `FZ` writer - Default value for FPSCR.FZ (Flush-to-zero mode control bit)."]
pub type FZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPDSCR_SPEC, FZ_A, O>;
impl<'a, const O: u8> FZ_W<'a, O> {
    #[doc = "Flush-to-zero mode disabled. Behavior of the floating-point system is fully compliant with the IEEE 754 standard."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FZ_A::_0)
    }
    #[doc = "Flush-to-zero mode enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FZ_A::_1)
    }
}
#[doc = "Field `DN` reader - Default value for FPSCR.DN (Default NaN mode control bit)."]
pub type DN_R = crate::BitReader<DN_A>;
#[doc = "Default value for FPSCR.DN (Default NaN mode control bit).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DN_A {
    #[doc = "0: NaN operands propagate through to the output of a floating-point operation."]
    _0 = 0,
    #[doc = "1: Any operation involving one or more NaNs returns the Default NaN."]
    _1 = 1,
}
impl From<DN_A> for bool {
    #[inline(always)]
    fn from(variant: DN_A) -> Self {
        variant as u8 != 0
    }
}
impl DN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DN_A {
        match self.bits {
            false => DN_A::_0,
            true => DN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DN_A::_1
    }
}
#[doc = "Field `DN` writer - Default value for FPSCR.DN (Default NaN mode control bit)."]
pub type DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPDSCR_SPEC, DN_A, O>;
impl<'a, const O: u8> DN_W<'a, O> {
    #[doc = "NaN operands propagate through to the output of a floating-point operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DN_A::_0)
    }
    #[doc = "Any operation involving one or more NaNs returns the Default NaN."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DN_A::_1)
    }
}
#[doc = "Field `AHP` reader - Default value for FPSCR.AHP (Alternative half-precision control bit)."]
pub type AHP_R = crate::BitReader<AHP_A>;
#[doc = "Default value for FPSCR.AHP (Alternative half-precision control bit).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHP_A {
    #[doc = "0: IEEE half-precision format selected."]
    _0 = 0,
    #[doc = "1: Alternative half-precision format selected."]
    _1 = 1,
}
impl From<AHP_A> for bool {
    #[inline(always)]
    fn from(variant: AHP_A) -> Self {
        variant as u8 != 0
    }
}
impl AHP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHP_A {
        match self.bits {
            false => AHP_A::_0,
            true => AHP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AHP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AHP_A::_1
    }
}
#[doc = "Field `AHP` writer - Default value for FPSCR.AHP (Alternative half-precision control bit)."]
pub type AHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPDSCR_SPEC, AHP_A, O>;
impl<'a, const O: u8> AHP_W<'a, O> {
    #[doc = "IEEE half-precision format selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AHP_A::_0)
    }
    #[doc = "Alternative half-precision format selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AHP_A::_1)
    }
}
impl R {
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode (Rounding Mode control field)."]
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ (Flush-to-zero mode control bit)."]
    #[inline(always)]
    pub fn fz(&self) -> FZ_R {
        FZ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN (Default NaN mode control bit)."]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP (Alternative half-precision control bit)."]
    #[inline(always)]
    pub fn ahp(&self) -> AHP_R {
        AHP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode (Rounding Mode control field)."]
    #[inline(always)]
    #[must_use]
    pub fn rmode(&mut self) -> RMODE_W<22> {
        RMODE_W::new(self)
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ (Flush-to-zero mode control bit)."]
    #[inline(always)]
    #[must_use]
    pub fn fz(&mut self) -> FZ_W<24> {
        FZ_W::new(self)
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN (Default NaN mode control bit)."]
    #[inline(always)]
    #[must_use]
    pub fn dn(&mut self) -> DN_W<25> {
        DN_W::new(self)
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP (Alternative half-precision control bit)."]
    #[inline(always)]
    #[must_use]
    pub fn ahp(&mut self) -> AHP_W<26> {
        AHP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Floating-point Default Status Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpdscr](index.html) module"]
pub struct FPDSCR_SPEC;
impl crate::RegisterSpec for FPDSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpdscr::R](R) reader structure"]
impl crate::Readable for FPDSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpdscr::W](W) writer structure"]
impl crate::Writable for FPDSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPDSCR to value 0"]
impl crate::Resettable for FPDSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
