#[doc = "Register `QDCTRL` reader"]
pub struct R(crate::R<QDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QDCTRL` writer"]
pub struct W(crate::W<QDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDCTRL_SPEC>;
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
impl From<crate::W<QDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUADEN` reader - Quadrature Decoder Mode Enable"]
pub type QUADEN_R = crate::BitReader<QUADEN_A>;
#[doc = "Quadrature Decoder Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QUADEN_A {
    #[doc = "0: Quadrature Decoder mode is disabled."]
    _0 = 0,
    #[doc = "1: Quadrature Decoder mode is enabled."]
    _1 = 1,
}
impl From<QUADEN_A> for bool {
    #[inline(always)]
    fn from(variant: QUADEN_A) -> Self {
        variant as u8 != 0
    }
}
impl QUADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADEN_A {
        match self.bits {
            false => QUADEN_A::_0,
            true => QUADEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QUADEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QUADEN_A::_1
    }
}
#[doc = "Field `QUADEN` writer - Quadrature Decoder Mode Enable"]
pub type QUADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDCTRL_SPEC, QUADEN_A, O>;
impl<'a, const O: u8> QUADEN_W<'a, O> {
    #[doc = "Quadrature Decoder mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUADEN_A::_0)
    }
    #[doc = "Quadrature Decoder mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUADEN_A::_1)
    }
}
#[doc = "Field `TOFDIR` reader - Timer Overflow Direction In Quadrature Decoder Mode"]
pub type TOFDIR_R = crate::BitReader<TOFDIR_A>;
#[doc = "Timer Overflow Direction In Quadrature Decoder Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOFDIR_A {
    #[doc = "0: TOF bit was set on the bottom of counting. There was an FTM counter decrement and FTM counter changes from its minimum value (CNTIN register) to its maximum value (MOD register)."]
    _0 = 0,
    #[doc = "1: TOF bit was set on the top of counting. There was an FTM counter increment and FTM counter changes from its maximum value (MOD register) to its minimum value (CNTIN register)."]
    _1 = 1,
}
impl From<TOFDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TOFDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl TOFDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOFDIR_A {
        match self.bits {
            false => TOFDIR_A::_0,
            true => TOFDIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOFDIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOFDIR_A::_1
    }
}
#[doc = "Field `QUADIR` reader - FTM Counter Direction In Quadrature Decoder Mode"]
pub type QUADIR_R = crate::BitReader<QUADIR_A>;
#[doc = "FTM Counter Direction In Quadrature Decoder Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QUADIR_A {
    #[doc = "0: Counting direction is decreasing (FTM counter decrement)."]
    _0 = 0,
    #[doc = "1: Counting direction is increasing (FTM counter increment)."]
    _1 = 1,
}
impl From<QUADIR_A> for bool {
    #[inline(always)]
    fn from(variant: QUADIR_A) -> Self {
        variant as u8 != 0
    }
}
impl QUADIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADIR_A {
        match self.bits {
            false => QUADIR_A::_0,
            true => QUADIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QUADIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QUADIR_A::_1
    }
}
#[doc = "Field `QUADMODE` reader - Quadrature Decoder Mode"]
pub type QUADMODE_R = crate::BitReader<QUADMODE_A>;
#[doc = "Quadrature Decoder Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QUADMODE_A {
    #[doc = "0: Phase A and phase B encoding mode."]
    _0 = 0,
    #[doc = "1: Count and direction encoding mode."]
    _1 = 1,
}
impl From<QUADMODE_A> for bool {
    #[inline(always)]
    fn from(variant: QUADMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl QUADMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADMODE_A {
        match self.bits {
            false => QUADMODE_A::_0,
            true => QUADMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QUADMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QUADMODE_A::_1
    }
}
#[doc = "Field `QUADMODE` writer - Quadrature Decoder Mode"]
pub type QUADMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDCTRL_SPEC, QUADMODE_A, O>;
impl<'a, const O: u8> QUADMODE_W<'a, O> {
    #[doc = "Phase A and phase B encoding mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUADMODE_A::_0)
    }
    #[doc = "Count and direction encoding mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUADMODE_A::_1)
    }
}
#[doc = "Field `PHBPOL` reader - Phase B Input Polarity"]
pub type PHBPOL_R = crate::BitReader<PHBPOL_A>;
#[doc = "Phase B Input Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHBPOL_A {
    #[doc = "0: Normal polarity. Phase B input signal is not inverted before identifying the rising and falling edges of this signal."]
    _0 = 0,
    #[doc = "1: Inverted polarity. Phase B input signal is inverted before identifying the rising and falling edges of this signal."]
    _1 = 1,
}
impl From<PHBPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PHBPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl PHBPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHBPOL_A {
        match self.bits {
            false => PHBPOL_A::_0,
            true => PHBPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PHBPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PHBPOL_A::_1
    }
}
#[doc = "Field `PHBPOL` writer - Phase B Input Polarity"]
pub type PHBPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDCTRL_SPEC, PHBPOL_A, O>;
impl<'a, const O: u8> PHBPOL_W<'a, O> {
    #[doc = "Normal polarity. Phase B input signal is not inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHBPOL_A::_0)
    }
    #[doc = "Inverted polarity. Phase B input signal is inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHBPOL_A::_1)
    }
}
#[doc = "Field `PHAPOL` reader - Phase A Input Polarity"]
pub type PHAPOL_R = crate::BitReader<PHAPOL_A>;
#[doc = "Phase A Input Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHAPOL_A {
    #[doc = "0: Normal polarity. Phase A input signal is not inverted before identifying the rising and falling edges of this signal."]
    _0 = 0,
    #[doc = "1: Inverted polarity. Phase A input signal is inverted before identifying the rising and falling edges of this signal."]
    _1 = 1,
}
impl From<PHAPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PHAPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl PHAPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHAPOL_A {
        match self.bits {
            false => PHAPOL_A::_0,
            true => PHAPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PHAPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PHAPOL_A::_1
    }
}
#[doc = "Field `PHAPOL` writer - Phase A Input Polarity"]
pub type PHAPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDCTRL_SPEC, PHAPOL_A, O>;
impl<'a, const O: u8> PHAPOL_W<'a, O> {
    #[doc = "Normal polarity. Phase A input signal is not inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHAPOL_A::_0)
    }
    #[doc = "Inverted polarity. Phase A input signal is inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHAPOL_A::_1)
    }
}
#[doc = "Field `PHBFLTREN` reader - Phase B Input Filter Enable"]
pub type PHBFLTREN_R = crate::BitReader<PHBFLTREN_A>;
#[doc = "Phase B Input Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHBFLTREN_A {
    #[doc = "0: Phase B input filter is disabled."]
    _0 = 0,
    #[doc = "1: Phase B input filter is enabled."]
    _1 = 1,
}
impl From<PHBFLTREN_A> for bool {
    #[inline(always)]
    fn from(variant: PHBFLTREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PHBFLTREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHBFLTREN_A {
        match self.bits {
            false => PHBFLTREN_A::_0,
            true => PHBFLTREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PHBFLTREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PHBFLTREN_A::_1
    }
}
#[doc = "Field `PHBFLTREN` writer - Phase B Input Filter Enable"]
pub type PHBFLTREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDCTRL_SPEC, PHBFLTREN_A, O>;
impl<'a, const O: u8> PHBFLTREN_W<'a, O> {
    #[doc = "Phase B input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHBFLTREN_A::_0)
    }
    #[doc = "Phase B input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHBFLTREN_A::_1)
    }
}
#[doc = "Field `PHAFLTREN` reader - Phase A Input Filter Enable"]
pub type PHAFLTREN_R = crate::BitReader<PHAFLTREN_A>;
#[doc = "Phase A Input Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHAFLTREN_A {
    #[doc = "0: Phase A input filter is disabled."]
    _0 = 0,
    #[doc = "1: Phase A input filter is enabled."]
    _1 = 1,
}
impl From<PHAFLTREN_A> for bool {
    #[inline(always)]
    fn from(variant: PHAFLTREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PHAFLTREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHAFLTREN_A {
        match self.bits {
            false => PHAFLTREN_A::_0,
            true => PHAFLTREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PHAFLTREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PHAFLTREN_A::_1
    }
}
#[doc = "Field `PHAFLTREN` writer - Phase A Input Filter Enable"]
pub type PHAFLTREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDCTRL_SPEC, PHAFLTREN_A, O>;
impl<'a, const O: u8> PHAFLTREN_W<'a, O> {
    #[doc = "Phase A input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHAFLTREN_A::_0)
    }
    #[doc = "Phase A input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHAFLTREN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Quadrature Decoder Mode Enable"]
    #[inline(always)]
    pub fn quaden(&self) -> QUADEN_R {
        QUADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Overflow Direction In Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn tofdir(&self) -> TOFDIR_R {
        TOFDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FTM Counter Direction In Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn quadir(&self) -> QUADIR_R {
        QUADIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn quadmode(&self) -> QUADMODE_R {
        QUADMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Phase B Input Polarity"]
    #[inline(always)]
    pub fn phbpol(&self) -> PHBPOL_R {
        PHBPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Phase A Input Polarity"]
    #[inline(always)]
    pub fn phapol(&self) -> PHAPOL_R {
        PHAPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Phase B Input Filter Enable"]
    #[inline(always)]
    pub fn phbfltren(&self) -> PHBFLTREN_R {
        PHBFLTREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Phase A Input Filter Enable"]
    #[inline(always)]
    pub fn phafltren(&self) -> PHAFLTREN_R {
        PHAFLTREN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Quadrature Decoder Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn quaden(&mut self) -> QUADEN_W<0> {
        QUADEN_W::new(self)
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline(always)]
    #[must_use]
    pub fn quadmode(&mut self) -> QUADMODE_W<3> {
        QUADMODE_W::new(self)
    }
    #[doc = "Bit 4 - Phase B Input Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn phbpol(&mut self) -> PHBPOL_W<4> {
        PHBPOL_W::new(self)
    }
    #[doc = "Bit 5 - Phase A Input Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn phapol(&mut self) -> PHAPOL_W<5> {
        PHAPOL_W::new(self)
    }
    #[doc = "Bit 6 - Phase B Input Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn phbfltren(&mut self) -> PHBFLTREN_W<6> {
        PHBFLTREN_W::new(self)
    }
    #[doc = "Bit 7 - Phase A Input Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn phafltren(&mut self) -> PHAFLTREN_W<7> {
        PHAFLTREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Quadrature Decoder Control And Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdctrl](index.html) module"]
pub struct QDCTRL_SPEC;
impl crate::RegisterSpec for QDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdctrl::R](R) reader structure"]
impl crate::Readable for QDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdctrl::W](W) writer structure"]
impl crate::Writable for QDCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QDCTRL to value 0"]
impl crate::Resettable for QDCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
