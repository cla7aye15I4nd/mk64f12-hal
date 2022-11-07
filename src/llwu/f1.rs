#[doc = "Register `F1` reader"]
pub struct R(crate::R<F1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F1` writer"]
pub struct W(crate::W<F1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F1_SPEC>;
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
impl From<crate::W<F1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUF0` reader - Wakeup Flag For LLWU_P0"]
pub type WUF0_R = crate::BitReader<WUF0_A>;
#[doc = "Wakeup Flag For LLWU_P0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF0_A {
    #[doc = "0: LLWU_P0 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P0 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF0_A> for bool {
    #[inline(always)]
    fn from(variant: WUF0_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF0_A {
        match self.bits {
            false => WUF0_A::_0,
            true => WUF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF0_A::_1
    }
}
#[doc = "Field `WUF0` writer - Wakeup Flag For LLWU_P0"]
pub type WUF0_W<'a, const O: u8> = crate::BitWriter<'a, u8, F1_SPEC, WUF0_A, O>;
impl<'a, const O: u8> WUF0_W<'a, O> {
    #[doc = "LLWU_P0 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF0_A::_0)
    }
    #[doc = "LLWU_P0 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF0_A::_1)
    }
}
#[doc = "Field `WUF1` reader - Wakeup Flag For LLWU_P1"]
pub type WUF1_R = crate::BitReader<WUF1_A>;
#[doc = "Wakeup Flag For LLWU_P1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF1_A {
    #[doc = "0: LLWU_P1 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P1 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF1_A> for bool {
    #[inline(always)]
    fn from(variant: WUF1_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF1_A {
        match self.bits {
            false => WUF1_A::_0,
            true => WUF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF1_A::_1
    }
}
#[doc = "Field `WUF1` writer - Wakeup Flag For LLWU_P1"]
pub type WUF1_W<'a, const O: u8> = crate::BitWriter<'a, u8, F1_SPEC, WUF1_A, O>;
impl<'a, const O: u8> WUF1_W<'a, O> {
    #[doc = "LLWU_P1 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF1_A::_0)
    }
    #[doc = "LLWU_P1 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF1_A::_1)
    }
}
#[doc = "Field `WUF2` reader - Wakeup Flag For LLWU_P2"]
pub type WUF2_R = crate::BitReader<WUF2_A>;
#[doc = "Wakeup Flag For LLWU_P2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF2_A {
    #[doc = "0: LLWU_P2 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P2 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF2_A> for bool {
    #[inline(always)]
    fn from(variant: WUF2_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF2_A {
        match self.bits {
            false => WUF2_A::_0,
            true => WUF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF2_A::_1
    }
}
#[doc = "Field `WUF2` writer - Wakeup Flag For LLWU_P2"]
pub type WUF2_W<'a, const O: u8> = crate::BitWriter<'a, u8, F1_SPEC, WUF2_A, O>;
impl<'a, const O: u8> WUF2_W<'a, O> {
    #[doc = "LLWU_P2 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF2_A::_0)
    }
    #[doc = "LLWU_P2 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF2_A::_1)
    }
}
#[doc = "Field `WUF3` reader - Wakeup Flag For LLWU_P3"]
pub type WUF3_R = crate::BitReader<WUF3_A>;
#[doc = "Wakeup Flag For LLWU_P3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF3_A {
    #[doc = "0: LLWU_P3 input was not a wake-up source"]
    _0 = 0,
    #[doc = "1: LLWU_P3 input was a wake-up source"]
    _1 = 1,
}
impl From<WUF3_A> for bool {
    #[inline(always)]
    fn from(variant: WUF3_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF3_A {
        match self.bits {
            false => WUF3_A::_0,
            true => WUF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF3_A::_1
    }
}
#[doc = "Field `WUF3` writer - Wakeup Flag For LLWU_P3"]
pub type WUF3_W<'a, const O: u8> = crate::BitWriter<'a, u8, F1_SPEC, WUF3_A, O>;
impl<'a, const O: u8> WUF3_W<'a, O> {
    #[doc = "LLWU_P3 input was not a wake-up source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF3_A::_0)
    }
    #[doc = "LLWU_P3 input was a wake-up source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF3_A::_1)
    }
}
#[doc = "Field `WUF4` reader - Wakeup Flag For LLWU_P4"]
pub type WUF4_R = crate::BitReader<WUF4_A>;
#[doc = "Wakeup Flag For LLWU_P4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF4_A {
    #[doc = "0: LLWU_P4 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P4 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF4_A> for bool {
    #[inline(always)]
    fn from(variant: WUF4_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF4_A {
        match self.bits {
            false => WUF4_A::_0,
            true => WUF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF4_A::_1
    }
}
#[doc = "Field `WUF4` writer - Wakeup Flag For LLWU_P4"]
pub type WUF4_W<'a, const O: u8> = crate::BitWriter<'a, u8, F1_SPEC, WUF4_A, O>;
impl<'a, const O: u8> WUF4_W<'a, O> {
    #[doc = "LLWU_P4 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF4_A::_0)
    }
    #[doc = "LLWU_P4 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF4_A::_1)
    }
}
#[doc = "Field `WUF5` reader - Wakeup Flag For LLWU_P5"]
pub type WUF5_R = crate::BitReader<WUF5_A>;
#[doc = "Wakeup Flag For LLWU_P5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF5_A {
    #[doc = "0: LLWU_P5 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P5 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF5_A> for bool {
    #[inline(always)]
    fn from(variant: WUF5_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF5_A {
        match self.bits {
            false => WUF5_A::_0,
            true => WUF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF5_A::_1
    }
}
#[doc = "Field `WUF5` writer - Wakeup Flag For LLWU_P5"]
pub type WUF5_W<'a, const O: u8> = crate::BitWriter<'a, u8, F1_SPEC, WUF5_A, O>;
impl<'a, const O: u8> WUF5_W<'a, O> {
    #[doc = "LLWU_P5 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF5_A::_0)
    }
    #[doc = "LLWU_P5 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF5_A::_1)
    }
}
#[doc = "Field `WUF6` reader - Wakeup Flag For LLWU_P6"]
pub type WUF6_R = crate::BitReader<WUF6_A>;
#[doc = "Wakeup Flag For LLWU_P6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF6_A {
    #[doc = "0: LLWU_P6 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P6 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF6_A> for bool {
    #[inline(always)]
    fn from(variant: WUF6_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF6_A {
        match self.bits {
            false => WUF6_A::_0,
            true => WUF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF6_A::_1
    }
}
#[doc = "Field `WUF6` writer - Wakeup Flag For LLWU_P6"]
pub type WUF6_W<'a, const O: u8> = crate::BitWriter<'a, u8, F1_SPEC, WUF6_A, O>;
impl<'a, const O: u8> WUF6_W<'a, O> {
    #[doc = "LLWU_P6 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF6_A::_0)
    }
    #[doc = "LLWU_P6 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF6_A::_1)
    }
}
#[doc = "Field `WUF7` reader - Wakeup Flag For LLWU_P7"]
pub type WUF7_R = crate::BitReader<WUF7_A>;
#[doc = "Wakeup Flag For LLWU_P7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF7_A {
    #[doc = "0: LLWU_P7 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P7 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF7_A> for bool {
    #[inline(always)]
    fn from(variant: WUF7_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF7_A {
        match self.bits {
            false => WUF7_A::_0,
            true => WUF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF7_A::_1
    }
}
#[doc = "Field `WUF7` writer - Wakeup Flag For LLWU_P7"]
pub type WUF7_W<'a, const O: u8> = crate::BitWriter<'a, u8, F1_SPEC, WUF7_A, O>;
impl<'a, const O: u8> WUF7_W<'a, O> {
    #[doc = "LLWU_P7 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF7_A::_0)
    }
    #[doc = "LLWU_P7 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P0"]
    #[inline(always)]
    pub fn wuf0(&self) -> WUF0_R {
        WUF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P1"]
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P2"]
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P3"]
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P4"]
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P5"]
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P6"]
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P7"]
    #[inline(always)]
    pub fn wuf7(&self) -> WUF7_R {
        WUF7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P0"]
    #[inline(always)]
    #[must_use]
    pub fn wuf0(&mut self) -> WUF0_W<0> {
        WUF0_W::new(self)
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P1"]
    #[inline(always)]
    #[must_use]
    pub fn wuf1(&mut self) -> WUF1_W<1> {
        WUF1_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P2"]
    #[inline(always)]
    #[must_use]
    pub fn wuf2(&mut self) -> WUF2_W<2> {
        WUF2_W::new(self)
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P3"]
    #[inline(always)]
    #[must_use]
    pub fn wuf3(&mut self) -> WUF3_W<3> {
        WUF3_W::new(self)
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P4"]
    #[inline(always)]
    #[must_use]
    pub fn wuf4(&mut self) -> WUF4_W<4> {
        WUF4_W::new(self)
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P5"]
    #[inline(always)]
    #[must_use]
    pub fn wuf5(&mut self) -> WUF5_W<5> {
        WUF5_W::new(self)
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P6"]
    #[inline(always)]
    #[must_use]
    pub fn wuf6(&mut self) -> WUF6_W<6> {
        WUF6_W::new(self)
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P7"]
    #[inline(always)]
    #[must_use]
    pub fn wuf7(&mut self) -> WUF7_W<7> {
        WUF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Flag 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f1](index.html) module"]
pub struct F1_SPEC;
impl crate::RegisterSpec for F1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [f1::R](R) reader structure"]
impl crate::Readable for F1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f1::W](W) writer structure"]
impl crate::Writable for F1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets F1 to value 0"]
impl crate::Resettable for F1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
