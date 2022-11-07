#[doc = "Register `C4` reader"]
pub struct R(crate::R<C4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C4` writer"]
pub struct W(crate::W<C4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C4_SPEC>;
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
impl From<crate::W<C4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRFA` reader - Baud Rate Fine Adjust"]
pub type BRFA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRFA` writer - Baud Rate Fine Adjust"]
pub type BRFA_W<'a, const O: u8> = crate::FieldWriter<'a, u8, C4_SPEC, u8, u8, 5, O>;
#[doc = "Field `M10` reader - 10-bit Mode select"]
pub type M10_R = crate::BitReader<M10_A>;
#[doc = "10-bit Mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M10_A {
    #[doc = "0: The parity bit is the ninth bit in the serial transmission."]
    _0 = 0,
    #[doc = "1: The parity bit is the tenth bit in the serial transmission."]
    _1 = 1,
}
impl From<M10_A> for bool {
    #[inline(always)]
    fn from(variant: M10_A) -> Self {
        variant as u8 != 0
    }
}
impl M10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M10_A {
        match self.bits {
            false => M10_A::_0,
            true => M10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M10_A::_1
    }
}
#[doc = "Field `M10` writer - 10-bit Mode select"]
pub type M10_W<'a, const O: u8> = crate::BitWriter<'a, u8, C4_SPEC, M10_A, O>;
impl<'a, const O: u8> M10_W<'a, O> {
    #[doc = "The parity bit is the ninth bit in the serial transmission."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M10_A::_0)
    }
    #[doc = "The parity bit is the tenth bit in the serial transmission."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M10_A::_1)
    }
}
#[doc = "Field `MAEN2` reader - Match Address Mode Enable 2"]
pub type MAEN2_R = crate::BitReader<MAEN2_A>;
#[doc = "Match Address Mode Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAEN2_A {
    #[doc = "0: All data received is transferred to the data buffer if MAEN1 is cleared."]
    _0 = 0,
    #[doc = "1: All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If a match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\]
is set/enabled."]
    _1 = 1,
}
impl From<MAEN2_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl MAEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN2_A {
        match self.bits {
            false => MAEN2_A::_0,
            true => MAEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAEN2_A::_1
    }
}
#[doc = "Field `MAEN2` writer - Match Address Mode Enable 2"]
pub type MAEN2_W<'a, const O: u8> = crate::BitWriter<'a, u8, C4_SPEC, MAEN2_A, O>;
impl<'a, const O: u8> MAEN2_W<'a, O> {
    #[doc = "All data received is transferred to the data buffer if MAEN1 is cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN2_A::_0)
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If a match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\]
is set/enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN2_A::_1)
    }
}
#[doc = "Field `MAEN1` reader - Match Address Mode Enable 1"]
pub type MAEN1_R = crate::BitReader<MAEN1_A>;
#[doc = "Match Address Mode Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAEN1_A {
    #[doc = "0: All data received is transferred to the data buffer if MAEN2 is cleared."]
    _0 = 0,
    #[doc = "1: All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\]
is set/enabled."]
    _1 = 1,
}
impl From<MAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl MAEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN1_A {
        match self.bits {
            false => MAEN1_A::_0,
            true => MAEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAEN1_A::_1
    }
}
#[doc = "Field `MAEN1` writer - Match Address Mode Enable 1"]
pub type MAEN1_W<'a, const O: u8> = crate::BitWriter<'a, u8, C4_SPEC, MAEN1_A, O>;
impl<'a, const O: u8> MAEN1_W<'a, O> {
    #[doc = "All data received is transferred to the data buffer if MAEN2 is cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN1_A::_0)
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\]
is set/enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN1_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Baud Rate Fine Adjust"]
    #[inline(always)]
    pub fn brfa(&self) -> BRFA_R {
        BRFA_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&self) -> M10_R {
        M10_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&self) -> MAEN2_R {
        MAEN2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&self) -> MAEN1_R {
        MAEN1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Baud Rate Fine Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn brfa(&mut self) -> BRFA_W<0> {
        BRFA_W::new(self)
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline(always)]
    #[must_use]
    pub fn m10(&mut self) -> M10_W<5> {
        M10_W::new(self)
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn maen2(&mut self) -> MAEN2_W<6> {
        MAEN2_W::new(self)
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn maen1(&mut self) -> MAEN1_W<7> {
        MAEN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4](index.html) module"]
pub struct C4_SPEC;
impl crate::RegisterSpec for C4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c4::R](R) reader structure"]
impl crate::Readable for C4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c4::W](W) writer structure"]
impl crate::Writable for C4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C4 to value 0"]
impl crate::Resettable for C4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
