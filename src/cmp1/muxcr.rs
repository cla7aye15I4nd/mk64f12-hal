#[doc = "Register `MUXCR` reader"]
pub struct R(crate::R<MUXCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUXCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUXCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUXCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUXCR` writer"]
pub struct W(crate::W<MUXCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUXCR_SPEC>;
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
impl From<crate::W<MUXCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUXCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSEL` reader - Minus Input Mux Control"]
pub type MSEL_R = crate::FieldReader<u8, MSEL_A>;
#[doc = "Minus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: IN0"]
    _000 = 0,
    #[doc = "1: IN1"]
    _001 = 1,
    #[doc = "2: IN2"]
    _010 = 2,
    #[doc = "3: IN3"]
    _011 = 3,
    #[doc = "4: IN4"]
    _100 = 4,
    #[doc = "5: IN5"]
    _101 = 5,
    #[doc = "6: IN6"]
    _110 = 6,
    #[doc = "7: IN7"]
    _111 = 7,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
impl MSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::_000,
            1 => MSEL_A::_001,
            2 => MSEL_A::_010,
            3 => MSEL_A::_011,
            4 => MSEL_A::_100,
            5 => MSEL_A::_101,
            6 => MSEL_A::_110,
            7 => MSEL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == MSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == MSEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == MSEL_A::_111
    }
}
#[doc = "Field `MSEL` writer - Minus Input Mux Control"]
pub type MSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MUXCR_SPEC, u8, MSEL_A, 3, O>;
impl<'a, const O: u8> MSEL_W<'a, O> {
    #[doc = "IN0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MSEL_A::_000)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(MSEL_A::_001)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MSEL_A::_010)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MSEL_A::_011)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MSEL_A::_100)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(MSEL_A::_101)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(MSEL_A::_110)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(MSEL_A::_111)
    }
}
#[doc = "Field `PSEL` reader - Plus Input Mux Control"]
pub type PSEL_R = crate::FieldReader<u8, PSEL_A>;
#[doc = "Plus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: IN0"]
    _000 = 0,
    #[doc = "1: IN1"]
    _001 = 1,
    #[doc = "2: IN2"]
    _010 = 2,
    #[doc = "3: IN3"]
    _011 = 3,
    #[doc = "4: IN4"]
    _100 = 4,
    #[doc = "5: IN5"]
    _101 = 5,
    #[doc = "6: IN6"]
    _110 = 6,
    #[doc = "7: IN7"]
    _111 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
impl PSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::_000,
            1 => PSEL_A::_001,
            2 => PSEL_A::_010,
            3 => PSEL_A::_011,
            4 => PSEL_A::_100,
            5 => PSEL_A::_101,
            6 => PSEL_A::_110,
            7 => PSEL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PSEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PSEL_A::_111
    }
}
#[doc = "Field `PSEL` writer - Plus Input Mux Control"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MUXCR_SPEC, u8, PSEL_A, 3, O>;
impl<'a, const O: u8> PSEL_W<'a, O> {
    #[doc = "IN0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PSEL_A::_000)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PSEL_A::_001)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PSEL_A::_010)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PSEL_A::_011)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PSEL_A::_100)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PSEL_A::_101)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PSEL_A::_110)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PSEL_A::_111)
    }
}
#[doc = "Field `PSTM` reader - Pass Through Mode Enable"]
pub type PSTM_R = crate::BitReader<PSTM_A>;
#[doc = "Pass Through Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSTM_A {
    #[doc = "0: Pass Through Mode is disabled."]
    _0 = 0,
    #[doc = "1: Pass Through Mode is enabled."]
    _1 = 1,
}
impl From<PSTM_A> for bool {
    #[inline(always)]
    fn from(variant: PSTM_A) -> Self {
        variant as u8 != 0
    }
}
impl PSTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSTM_A {
        match self.bits {
            false => PSTM_A::_0,
            true => PSTM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSTM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSTM_A::_1
    }
}
#[doc = "Field `PSTM` writer - Pass Through Mode Enable"]
pub type PSTM_W<'a, const O: u8> = crate::BitWriter<'a, u8, MUXCR_SPEC, PSTM_A, O>;
impl<'a, const O: u8> PSTM_W<'a, O> {
    #[doc = "Pass Through Mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSTM_A::_0)
    }
    #[doc = "Pass Through Mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSTM_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new((self.bits >> 3) & 7)
    }
    #[doc = "Bit 7 - Pass Through Mode Enable"]
    #[inline(always)]
    pub fn pstm(&self) -> PSTM_R {
        PSTM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    #[must_use]
    pub fn msel(&mut self) -> MSEL_W<0> {
        MSEL_W::new(self)
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<3> {
        PSEL_W::new(self)
    }
    #[doc = "Bit 7 - Pass Through Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pstm(&mut self) -> PSTM_W<7> {
        PSTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [muxcr](index.html) module"]
pub struct MUXCR_SPEC;
impl crate::RegisterSpec for MUXCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [muxcr::R](R) reader structure"]
impl crate::Readable for MUXCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [muxcr::W](W) writer structure"]
impl crate::Writable for MUXCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXCR to value 0"]
impl crate::Resettable for MUXCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
