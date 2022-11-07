#[doc = "Register `RPFC` reader"]
pub struct R(crate::R<RPFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPFC` writer"]
pub struct W(crate::W<RPFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPFC_SPEC>;
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
impl From<crate::W<RPFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTFLTSRW` reader - Reset Pin Filter Select in Run and Wait Modes"]
pub type RSTFLTSRW_R = crate::FieldReader<u8, RSTFLTSRW_A>;
#[doc = "Reset Pin Filter Select in Run and Wait Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSTFLTSRW_A {
    #[doc = "0: All filtering disabled"]
    _00 = 0,
    #[doc = "1: Bus clock filter enabled for normal operation"]
    _01 = 1,
    #[doc = "2: LPO clock filter enabled for normal operation"]
    _10 = 2,
}
impl From<RSTFLTSRW_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTFLTSRW_A) -> Self {
        variant as _
    }
}
impl RSTFLTSRW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RSTFLTSRW_A> {
        match self.bits {
            0 => Some(RSTFLTSRW_A::_00),
            1 => Some(RSTFLTSRW_A::_01),
            2 => Some(RSTFLTSRW_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RSTFLTSRW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RSTFLTSRW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RSTFLTSRW_A::_10
    }
}
#[doc = "Field `RSTFLTSRW` writer - Reset Pin Filter Select in Run and Wait Modes"]
pub type RSTFLTSRW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, RPFC_SPEC, u8, RSTFLTSRW_A, 2, O>;
impl<'a, const O: u8> RSTFLTSRW_W<'a, O> {
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_00)
    }
    #[doc = "Bus clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_01)
    }
    #[doc = "LPO clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_10)
    }
}
#[doc = "Field `RSTFLTSS` reader - Reset Pin Filter Select in Stop Mode"]
pub type RSTFLTSS_R = crate::BitReader<RSTFLTSS_A>;
#[doc = "Reset Pin Filter Select in Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTFLTSS_A {
    #[doc = "0: All filtering disabled"]
    _0 = 0,
    #[doc = "1: LPO clock filter enabled"]
    _1 = 1,
}
impl From<RSTFLTSS_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFLTSS_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTFLTSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFLTSS_A {
        match self.bits {
            false => RSTFLTSS_A::_0,
            true => RSTFLTSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSTFLTSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSTFLTSS_A::_1
    }
}
#[doc = "Field `RSTFLTSS` writer - Reset Pin Filter Select in Stop Mode"]
pub type RSTFLTSS_W<'a, const O: u8> = crate::BitWriter<'a, u8, RPFC_SPEC, RSTFLTSS_A, O>;
impl<'a, const O: u8> RSTFLTSS_W<'a, O> {
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTFLTSS_A::_0)
    }
    #[doc = "LPO clock filter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTFLTSS_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Reset Pin Filter Select in Run and Wait Modes"]
    #[inline(always)]
    pub fn rstfltsrw(&self) -> RSTFLTSRW_R {
        RSTFLTSRW_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Reset Pin Filter Select in Stop Mode"]
    #[inline(always)]
    pub fn rstfltss(&self) -> RSTFLTSS_R {
        RSTFLTSS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reset Pin Filter Select in Run and Wait Modes"]
    #[inline(always)]
    #[must_use]
    pub fn rstfltsrw(&mut self) -> RSTFLTSRW_W<0> {
        RSTFLTSRW_W::new(self)
    }
    #[doc = "Bit 2 - Reset Pin Filter Select in Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rstfltss(&mut self) -> RSTFLTSS_W<2> {
        RSTFLTSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Pin Filter Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpfc](index.html) module"]
pub struct RPFC_SPEC;
impl crate::RegisterSpec for RPFC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rpfc::R](R) reader structure"]
impl crate::Readable for RPFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpfc::W](W) writer structure"]
impl crate::Writable for RPFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RPFC to value 0"]
impl crate::Resettable for RPFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
