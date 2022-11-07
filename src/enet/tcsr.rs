#[doc = "Register `TCSR%s` reader"]
pub struct R(crate::R<TCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCSR%s` writer"]
pub struct W(crate::W<TCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCSR_SPEC>;
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
impl From<crate::W<TCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDRE` reader - Timer DMA Request Enable"]
pub type TDRE_R = crate::BitReader<TDRE_A>;
#[doc = "Timer DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRE_A {
    #[doc = "0: DMA request is disabled"]
    _0 = 0,
    #[doc = "1: DMA request is enabled"]
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRE_A::_1
    }
}
#[doc = "Field `TDRE` writer - Timer DMA Request Enable"]
pub type TDRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCSR_SPEC, TDRE_A, O>;
impl<'a, const O: u8> TDRE_W<'a, O> {
    #[doc = "DMA request is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDRE_A::_0)
    }
    #[doc = "DMA request is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDRE_A::_1)
    }
}
#[doc = "Field `TMODE` reader - Timer Mode"]
pub type TMODE_R = crate::FieldReader<u8, TMODE_A>;
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMODE_A {
    #[doc = "0: Timer Channel is disabled."]
    _0000 = 0,
    #[doc = "1: Timer Channel is configured for Input Capture on rising edge"]
    _0001 = 1,
    #[doc = "2: Timer Channel is configured for Input Capture on falling edge"]
    _0010 = 2,
    #[doc = "3: Timer Channel is configured for Input Capture on both edges"]
    _0011 = 3,
    #[doc = "4: Timer Channel is configured for Output Compare - software only"]
    _0100 = 4,
    #[doc = "5: Timer Channel is configured for Output Compare - toggle output on compare"]
    _0101 = 5,
    #[doc = "6: Timer Channel is configured for Output Compare - clear output on compare"]
    _0110 = 6,
    #[doc = "7: Timer Channel is configured for Output Compare - set output on compare"]
    _0111 = 7,
    #[doc = "10: Timer Channel is configured for Output Compare - clear output on compare, set output on overflow"]
    _1010 = 10,
    #[doc = "14: Timer Channel is configured for Output Compare - pulse output low on compare for one 1588 clock cycle"]
    _1110 = 14,
    #[doc = "15: Timer Channel is configured for Output Compare - pulse output high on compare for one 1588 clock cycle"]
    _1111 = 15,
}
impl From<TMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TMODE_A) -> Self {
        variant as _
    }
}
impl TMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMODE_A> {
        match self.bits {
            0 => Some(TMODE_A::_0000),
            1 => Some(TMODE_A::_0001),
            2 => Some(TMODE_A::_0010),
            3 => Some(TMODE_A::_0011),
            4 => Some(TMODE_A::_0100),
            5 => Some(TMODE_A::_0101),
            6 => Some(TMODE_A::_0110),
            7 => Some(TMODE_A::_0111),
            10 => Some(TMODE_A::_1010),
            14 => Some(TMODE_A::_1110),
            15 => Some(TMODE_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TMODE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TMODE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == TMODE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == TMODE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == TMODE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == TMODE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == TMODE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == TMODE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == TMODE_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == TMODE_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == TMODE_A::_1111
    }
}
#[doc = "Field `TMODE` writer - Timer Mode"]
pub type TMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCSR_SPEC, u8, TMODE_A, 4, O>;
impl<'a, const O: u8> TMODE_W<'a, O> {
    #[doc = "Timer Channel is disabled."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TMODE_A::_0000)
    }
    #[doc = "Timer Channel is configured for Input Capture on rising edge"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TMODE_A::_0001)
    }
    #[doc = "Timer Channel is configured for Input Capture on falling edge"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TMODE_A::_0010)
    }
    #[doc = "Timer Channel is configured for Input Capture on both edges"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TMODE_A::_0011)
    }
    #[doc = "Timer Channel is configured for Output Compare - software only"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TMODE_A::_0100)
    }
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TMODE_A::_0101)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TMODE_A::_0110)
    }
    #[doc = "Timer Channel is configured for Output Compare - set output on compare"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TMODE_A::_0111)
    }
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TMODE_A::_1010)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for one 1588 clock cycle"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TMODE_A::_1110)
    }
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for one 1588 clock cycle"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TMODE_A::_1111)
    }
}
#[doc = "Field `TIE` reader - Timer Interrupt Enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Field `TIE` writer - Timer Interrupt Enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCSR_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
    }
}
#[doc = "Field `TF` reader - Timer Flag"]
pub type TF_R = crate::BitReader<TF_A>;
#[doc = "Timer Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TF_A {
    #[doc = "0: Input Capture or Output Compare has not occurred"]
    _0 = 0,
    #[doc = "1: Input Capture or Output Compare has occurred"]
    _1 = 1,
}
impl From<TF_A> for bool {
    #[inline(always)]
    fn from(variant: TF_A) -> Self {
        variant as u8 != 0
    }
}
impl TF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TF_A {
        match self.bits {
            false => TF_A::_0,
            true => TF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TF_A::_1
    }
}
#[doc = "Field `TF` writer - Timer Flag"]
pub type TF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCSR_SPEC, TF_A, O>;
impl<'a, const O: u8> TF_W<'a, O> {
    #[doc = "Input Capture or Output Compare has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TF_A::_0)
    }
    #[doc = "Input Capture or Output Compare has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdre(&mut self) -> TDRE_W<0> {
        TDRE_W::new(self)
    }
    #[doc = "Bits 2:5 - Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmode(&mut self) -> TMODE_W<2> {
        TMODE_W::new(self)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<6> {
        TIE_W::new(self)
    }
    #[doc = "Bit 7 - Timer Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tf(&mut self) -> TF_W<7> {
        TF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcsr](index.html) module"]
pub struct TCSR_SPEC;
impl crate::RegisterSpec for TCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcsr::R](R) reader structure"]
impl crate::Readable for TCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcsr::W](W) writer structure"]
impl crate::Writable for TCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCSR%s to value 0"]
impl crate::Resettable for TCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
