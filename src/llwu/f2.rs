#[doc = "Register `F2` reader"]
pub struct R(crate::R<F2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F2` writer"]
pub struct W(crate::W<F2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F2_SPEC>;
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
impl From<crate::W<F2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUF8` reader - Wakeup Flag For LLWU_P8"]
pub type WUF8_R = crate::BitReader<WUF8_A>;
#[doc = "Wakeup Flag For LLWU_P8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF8_A {
    #[doc = "0: LLWU_P8 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P8 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF8_A> for bool {
    #[inline(always)]
    fn from(variant: WUF8_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF8_A {
        match self.bits {
            false => WUF8_A::_0,
            true => WUF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF8_A::_1
    }
}
#[doc = "Field `WUF8` writer - Wakeup Flag For LLWU_P8"]
pub type WUF8_W<'a, const O: u8> = crate::BitWriter<'a, u8, F2_SPEC, WUF8_A, O>;
impl<'a, const O: u8> WUF8_W<'a, O> {
    #[doc = "LLWU_P8 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF8_A::_0)
    }
    #[doc = "LLWU_P8 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF8_A::_1)
    }
}
#[doc = "Field `WUF9` reader - Wakeup Flag For LLWU_P9"]
pub type WUF9_R = crate::BitReader<WUF9_A>;
#[doc = "Wakeup Flag For LLWU_P9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF9_A {
    #[doc = "0: LLWU_P9 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P9 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF9_A> for bool {
    #[inline(always)]
    fn from(variant: WUF9_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF9_A {
        match self.bits {
            false => WUF9_A::_0,
            true => WUF9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF9_A::_1
    }
}
#[doc = "Field `WUF9` writer - Wakeup Flag For LLWU_P9"]
pub type WUF9_W<'a, const O: u8> = crate::BitWriter<'a, u8, F2_SPEC, WUF9_A, O>;
impl<'a, const O: u8> WUF9_W<'a, O> {
    #[doc = "LLWU_P9 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF9_A::_0)
    }
    #[doc = "LLWU_P9 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF9_A::_1)
    }
}
#[doc = "Field `WUF10` reader - Wakeup Flag For LLWU_P10"]
pub type WUF10_R = crate::BitReader<WUF10_A>;
#[doc = "Wakeup Flag For LLWU_P10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF10_A {
    #[doc = "0: LLWU_P10 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P10 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF10_A> for bool {
    #[inline(always)]
    fn from(variant: WUF10_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF10_A {
        match self.bits {
            false => WUF10_A::_0,
            true => WUF10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF10_A::_1
    }
}
#[doc = "Field `WUF10` writer - Wakeup Flag For LLWU_P10"]
pub type WUF10_W<'a, const O: u8> = crate::BitWriter<'a, u8, F2_SPEC, WUF10_A, O>;
impl<'a, const O: u8> WUF10_W<'a, O> {
    #[doc = "LLWU_P10 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF10_A::_0)
    }
    #[doc = "LLWU_P10 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF10_A::_1)
    }
}
#[doc = "Field `WUF11` reader - Wakeup Flag For LLWU_P11"]
pub type WUF11_R = crate::BitReader<WUF11_A>;
#[doc = "Wakeup Flag For LLWU_P11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF11_A {
    #[doc = "0: LLWU_P11 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P11 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF11_A> for bool {
    #[inline(always)]
    fn from(variant: WUF11_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF11_A {
        match self.bits {
            false => WUF11_A::_0,
            true => WUF11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF11_A::_1
    }
}
#[doc = "Field `WUF11` writer - Wakeup Flag For LLWU_P11"]
pub type WUF11_W<'a, const O: u8> = crate::BitWriter<'a, u8, F2_SPEC, WUF11_A, O>;
impl<'a, const O: u8> WUF11_W<'a, O> {
    #[doc = "LLWU_P11 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF11_A::_0)
    }
    #[doc = "LLWU_P11 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF11_A::_1)
    }
}
#[doc = "Field `WUF12` reader - Wakeup Flag For LLWU_P12"]
pub type WUF12_R = crate::BitReader<WUF12_A>;
#[doc = "Wakeup Flag For LLWU_P12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF12_A {
    #[doc = "0: LLWU_P12 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P12 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF12_A> for bool {
    #[inline(always)]
    fn from(variant: WUF12_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF12_A {
        match self.bits {
            false => WUF12_A::_0,
            true => WUF12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF12_A::_1
    }
}
#[doc = "Field `WUF12` writer - Wakeup Flag For LLWU_P12"]
pub type WUF12_W<'a, const O: u8> = crate::BitWriter<'a, u8, F2_SPEC, WUF12_A, O>;
impl<'a, const O: u8> WUF12_W<'a, O> {
    #[doc = "LLWU_P12 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF12_A::_0)
    }
    #[doc = "LLWU_P12 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF12_A::_1)
    }
}
#[doc = "Field `WUF13` reader - Wakeup Flag For LLWU_P13"]
pub type WUF13_R = crate::BitReader<WUF13_A>;
#[doc = "Wakeup Flag For LLWU_P13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF13_A {
    #[doc = "0: LLWU_P13 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P13 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF13_A> for bool {
    #[inline(always)]
    fn from(variant: WUF13_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF13_A {
        match self.bits {
            false => WUF13_A::_0,
            true => WUF13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF13_A::_1
    }
}
#[doc = "Field `WUF13` writer - Wakeup Flag For LLWU_P13"]
pub type WUF13_W<'a, const O: u8> = crate::BitWriter<'a, u8, F2_SPEC, WUF13_A, O>;
impl<'a, const O: u8> WUF13_W<'a, O> {
    #[doc = "LLWU_P13 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF13_A::_0)
    }
    #[doc = "LLWU_P13 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF13_A::_1)
    }
}
#[doc = "Field `WUF14` reader - Wakeup Flag For LLWU_P14"]
pub type WUF14_R = crate::BitReader<WUF14_A>;
#[doc = "Wakeup Flag For LLWU_P14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF14_A {
    #[doc = "0: LLWU_P14 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P14 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF14_A> for bool {
    #[inline(always)]
    fn from(variant: WUF14_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF14_A {
        match self.bits {
            false => WUF14_A::_0,
            true => WUF14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF14_A::_1
    }
}
#[doc = "Field `WUF14` writer - Wakeup Flag For LLWU_P14"]
pub type WUF14_W<'a, const O: u8> = crate::BitWriter<'a, u8, F2_SPEC, WUF14_A, O>;
impl<'a, const O: u8> WUF14_W<'a, O> {
    #[doc = "LLWU_P14 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF14_A::_0)
    }
    #[doc = "LLWU_P14 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF14_A::_1)
    }
}
#[doc = "Field `WUF15` reader - Wakeup Flag For LLWU_P15"]
pub type WUF15_R = crate::BitReader<WUF15_A>;
#[doc = "Wakeup Flag For LLWU_P15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUF15_A {
    #[doc = "0: LLWU_P15 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: LLWU_P15 input was a wakeup source"]
    _1 = 1,
}
impl From<WUF15_A> for bool {
    #[inline(always)]
    fn from(variant: WUF15_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF15_A {
        match self.bits {
            false => WUF15_A::_0,
            true => WUF15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF15_A::_1
    }
}
#[doc = "Field `WUF15` writer - Wakeup Flag For LLWU_P15"]
pub type WUF15_W<'a, const O: u8> = crate::BitWriter<'a, u8, F2_SPEC, WUF15_A, O>;
impl<'a, const O: u8> WUF15_W<'a, O> {
    #[doc = "LLWU_P15 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF15_A::_0)
    }
    #[doc = "LLWU_P15 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P8"]
    #[inline(always)]
    pub fn wuf8(&self) -> WUF8_R {
        WUF8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P9"]
    #[inline(always)]
    pub fn wuf9(&self) -> WUF9_R {
        WUF9_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P10"]
    #[inline(always)]
    pub fn wuf10(&self) -> WUF10_R {
        WUF10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P11"]
    #[inline(always)]
    pub fn wuf11(&self) -> WUF11_R {
        WUF11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P12"]
    #[inline(always)]
    pub fn wuf12(&self) -> WUF12_R {
        WUF12_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P13"]
    #[inline(always)]
    pub fn wuf13(&self) -> WUF13_R {
        WUF13_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P14"]
    #[inline(always)]
    pub fn wuf14(&self) -> WUF14_R {
        WUF14_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P15"]
    #[inline(always)]
    pub fn wuf15(&self) -> WUF15_R {
        WUF15_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P8"]
    #[inline(always)]
    #[must_use]
    pub fn wuf8(&mut self) -> WUF8_W<0> {
        WUF8_W::new(self)
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P9"]
    #[inline(always)]
    #[must_use]
    pub fn wuf9(&mut self) -> WUF9_W<1> {
        WUF9_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P10"]
    #[inline(always)]
    #[must_use]
    pub fn wuf10(&mut self) -> WUF10_W<2> {
        WUF10_W::new(self)
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P11"]
    #[inline(always)]
    #[must_use]
    pub fn wuf11(&mut self) -> WUF11_W<3> {
        WUF11_W::new(self)
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P12"]
    #[inline(always)]
    #[must_use]
    pub fn wuf12(&mut self) -> WUF12_W<4> {
        WUF12_W::new(self)
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P13"]
    #[inline(always)]
    #[must_use]
    pub fn wuf13(&mut self) -> WUF13_W<5> {
        WUF13_W::new(self)
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P14"]
    #[inline(always)]
    #[must_use]
    pub fn wuf14(&mut self) -> WUF14_W<6> {
        WUF14_W::new(self)
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P15"]
    #[inline(always)]
    #[must_use]
    pub fn wuf15(&mut self) -> WUF15_W<7> {
        WUF15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Flag 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f2](index.html) module"]
pub struct F2_SPEC;
impl crate::RegisterSpec for F2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [f2::R](R) reader structure"]
impl crate::Readable for F2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f2::W](W) writer structure"]
impl crate::Writable for F2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets F2 to value 0"]
impl crate::Resettable for F2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
