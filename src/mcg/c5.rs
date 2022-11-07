#[doc = "Register `C5` reader"]
pub struct R(crate::R<C5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C5` writer"]
pub struct W(crate::W<C5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C5_SPEC>;
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
impl From<crate::W<C5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRDIV0` reader - PLL External Reference Divider"]
pub type PRDIV0_R = crate::FieldReader<u8, PRDIV0_A>;
#[doc = "PLL External Reference Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRDIV0_A {
    #[doc = "0: Divide Factor is 1"]
    _0 = 0,
    #[doc = "1: Divide Factor is 2"]
    _1 = 1,
    #[doc = "2: Divide Factor is 3"]
    _2 = 2,
    #[doc = "3: Divide Factor is 4"]
    _3 = 3,
    #[doc = "4: Divide Factor is 5"]
    _4 = 4,
    #[doc = "5: Divide Factor is 6"]
    _5 = 5,
    #[doc = "6: Divide Factor is 7"]
    _6 = 6,
    #[doc = "7: Divide Factor is 8"]
    _7 = 7,
    #[doc = "8: Divide Factor is 9"]
    _8 = 8,
    #[doc = "9: Divide Factor is 10"]
    _9 = 9,
    #[doc = "10: Divide Factor is 11"]
    _10 = 10,
    #[doc = "11: Divide Factor is 12"]
    _11 = 11,
    #[doc = "12: Divide Factor is 13"]
    _12 = 12,
    #[doc = "13: Divide Factor is 14"]
    _13 = 13,
    #[doc = "14: Divide Factor is 15"]
    _14 = 14,
    #[doc = "15: Divide Factor is 16"]
    _15 = 15,
    #[doc = "16: Divide Factor is 17"]
    _16 = 16,
    #[doc = "17: Divide Factor is 18"]
    _17 = 17,
    #[doc = "18: Divide Factor is 19"]
    _18 = 18,
    #[doc = "19: Divide Factor is 20"]
    _19 = 19,
    #[doc = "20: Divide Factor is 21"]
    _20 = 20,
    #[doc = "21: Divide Factor is 22"]
    _21 = 21,
    #[doc = "22: Divide Factor is 23"]
    _22 = 22,
    #[doc = "23: Divide Factor is 24"]
    _23 = 23,
    #[doc = "24: Divide Factor is 25"]
    _24 = 24,
    #[doc = "25: Divide Factor is 26"]
    _25 = 25,
    #[doc = "26: Divide Factor is 27"]
    _26 = 26,
    #[doc = "27: Divide Factor is 28"]
    _27 = 27,
    #[doc = "28: Divide Factor is 29"]
    _28 = 28,
    #[doc = "29: Divide Factor is 30"]
    _29 = 29,
    #[doc = "30: Divide Factor is 31"]
    _30 = 30,
    #[doc = "31: Divide Factor is 32"]
    _31 = 31,
}
impl From<PRDIV0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRDIV0_A) -> Self {
        variant as _
    }
}
impl PRDIV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRDIV0_A {
        match self.bits {
            0 => PRDIV0_A::_0,
            1 => PRDIV0_A::_1,
            2 => PRDIV0_A::_2,
            3 => PRDIV0_A::_3,
            4 => PRDIV0_A::_4,
            5 => PRDIV0_A::_5,
            6 => PRDIV0_A::_6,
            7 => PRDIV0_A::_7,
            8 => PRDIV0_A::_8,
            9 => PRDIV0_A::_9,
            10 => PRDIV0_A::_10,
            11 => PRDIV0_A::_11,
            12 => PRDIV0_A::_12,
            13 => PRDIV0_A::_13,
            14 => PRDIV0_A::_14,
            15 => PRDIV0_A::_15,
            16 => PRDIV0_A::_16,
            17 => PRDIV0_A::_17,
            18 => PRDIV0_A::_18,
            19 => PRDIV0_A::_19,
            20 => PRDIV0_A::_20,
            21 => PRDIV0_A::_21,
            22 => PRDIV0_A::_22,
            23 => PRDIV0_A::_23,
            24 => PRDIV0_A::_24,
            25 => PRDIV0_A::_25,
            26 => PRDIV0_A::_26,
            27 => PRDIV0_A::_27,
            28 => PRDIV0_A::_28,
            29 => PRDIV0_A::_29,
            30 => PRDIV0_A::_30,
            31 => PRDIV0_A::_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRDIV0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRDIV0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == PRDIV0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == PRDIV0_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == PRDIV0_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == PRDIV0_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == PRDIV0_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == PRDIV0_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PRDIV0_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == PRDIV0_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PRDIV0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PRDIV0_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == PRDIV0_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        *self == PRDIV0_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        *self == PRDIV0_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        *self == PRDIV0_A::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PRDIV0_A::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline(always)]
    pub fn is_17(&self) -> bool {
        *self == PRDIV0_A::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        *self == PRDIV0_A::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline(always)]
    pub fn is_19(&self) -> bool {
        *self == PRDIV0_A::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == PRDIV0_A::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline(always)]
    pub fn is_21(&self) -> bool {
        *self == PRDIV0_A::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline(always)]
    pub fn is_22(&self) -> bool {
        *self == PRDIV0_A::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline(always)]
    pub fn is_23(&self) -> bool {
        *self == PRDIV0_A::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == PRDIV0_A::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        *self == PRDIV0_A::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline(always)]
    pub fn is_26(&self) -> bool {
        *self == PRDIV0_A::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline(always)]
    pub fn is_27(&self) -> bool {
        *self == PRDIV0_A::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline(always)]
    pub fn is_28(&self) -> bool {
        *self == PRDIV0_A::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline(always)]
    pub fn is_29(&self) -> bool {
        *self == PRDIV0_A::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        *self == PRDIV0_A::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == PRDIV0_A::_31
    }
}
#[doc = "Field `PRDIV0` writer - PLL External Reference Divider"]
pub type PRDIV0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, C5_SPEC, u8, PRDIV0_A, 5, O>;
impl<'a, const O: u8> PRDIV0_W<'a, O> {
    #[doc = "Divide Factor is 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRDIV0_A::_0)
    }
    #[doc = "Divide Factor is 2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRDIV0_A::_1)
    }
    #[doc = "Divide Factor is 3"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PRDIV0_A::_2)
    }
    #[doc = "Divide Factor is 4"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PRDIV0_A::_3)
    }
    #[doc = "Divide Factor is 5"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(PRDIV0_A::_4)
    }
    #[doc = "Divide Factor is 6"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(PRDIV0_A::_5)
    }
    #[doc = "Divide Factor is 7"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(PRDIV0_A::_6)
    }
    #[doc = "Divide Factor is 8"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(PRDIV0_A::_7)
    }
    #[doc = "Divide Factor is 9"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(PRDIV0_A::_8)
    }
    #[doc = "Divide Factor is 10"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(PRDIV0_A::_9)
    }
    #[doc = "Divide Factor is 11"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PRDIV0_A::_10)
    }
    #[doc = "Divide Factor is 12"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PRDIV0_A::_11)
    }
    #[doc = "Divide Factor is 13"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(PRDIV0_A::_12)
    }
    #[doc = "Divide Factor is 14"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(PRDIV0_A::_13)
    }
    #[doc = "Divide Factor is 15"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(PRDIV0_A::_14)
    }
    #[doc = "Divide Factor is 16"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(PRDIV0_A::_15)
    }
    #[doc = "Divide Factor is 17"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(PRDIV0_A::_16)
    }
    #[doc = "Divide Factor is 18"]
    #[inline(always)]
    pub fn _17(self) -> &'a mut W {
        self.variant(PRDIV0_A::_17)
    }
    #[doc = "Divide Factor is 19"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(PRDIV0_A::_18)
    }
    #[doc = "Divide Factor is 20"]
    #[inline(always)]
    pub fn _19(self) -> &'a mut W {
        self.variant(PRDIV0_A::_19)
    }
    #[doc = "Divide Factor is 21"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(PRDIV0_A::_20)
    }
    #[doc = "Divide Factor is 22"]
    #[inline(always)]
    pub fn _21(self) -> &'a mut W {
        self.variant(PRDIV0_A::_21)
    }
    #[doc = "Divide Factor is 23"]
    #[inline(always)]
    pub fn _22(self) -> &'a mut W {
        self.variant(PRDIV0_A::_22)
    }
    #[doc = "Divide Factor is 24"]
    #[inline(always)]
    pub fn _23(self) -> &'a mut W {
        self.variant(PRDIV0_A::_23)
    }
    #[doc = "Divide Factor is 25"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(PRDIV0_A::_24)
    }
    #[doc = "Divide Factor is 26"]
    #[inline(always)]
    pub fn _25(self) -> &'a mut W {
        self.variant(PRDIV0_A::_25)
    }
    #[doc = "Divide Factor is 27"]
    #[inline(always)]
    pub fn _26(self) -> &'a mut W {
        self.variant(PRDIV0_A::_26)
    }
    #[doc = "Divide Factor is 28"]
    #[inline(always)]
    pub fn _27(self) -> &'a mut W {
        self.variant(PRDIV0_A::_27)
    }
    #[doc = "Divide Factor is 29"]
    #[inline(always)]
    pub fn _28(self) -> &'a mut W {
        self.variant(PRDIV0_A::_28)
    }
    #[doc = "Divide Factor is 30"]
    #[inline(always)]
    pub fn _29(self) -> &'a mut W {
        self.variant(PRDIV0_A::_29)
    }
    #[doc = "Divide Factor is 31"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(PRDIV0_A::_30)
    }
    #[doc = "Divide Factor is 32"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(PRDIV0_A::_31)
    }
}
#[doc = "Field `PLLSTEN0` reader - PLL Stop Enable"]
pub type PLLSTEN0_R = crate::BitReader<PLLSTEN0_A>;
#[doc = "PLL Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSTEN0_A {
    #[doc = "0: MCGPLLCLK is disabled in any of the Stop modes."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK is enabled if system is in Normal Stop mode."]
    _1 = 1,
}
impl From<PLLSTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSTEN0_A {
        match self.bits {
            false => PLLSTEN0_A::_0,
            true => PLLSTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLSTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLSTEN0_A::_1
    }
}
#[doc = "Field `PLLSTEN0` writer - PLL Stop Enable"]
pub type PLLSTEN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, C5_SPEC, PLLSTEN0_A, O>;
impl<'a, const O: u8> PLLSTEN0_W<'a, O> {
    #[doc = "MCGPLLCLK is disabled in any of the Stop modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLSTEN0_A::_0)
    }
    #[doc = "MCGPLLCLK is enabled if system is in Normal Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLSTEN0_A::_1)
    }
}
#[doc = "Field `PLLCLKEN0` reader - PLL Clock Enable"]
pub type PLLCLKEN0_R = crate::BitReader<PLLCLKEN0_A>;
#[doc = "PLL Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLCLKEN0_A {
    #[doc = "0: MCGPLLCLK is inactive."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK is active."]
    _1 = 1,
}
impl From<PLLCLKEN0_A> for bool {
    #[inline(always)]
    fn from(variant: PLLCLKEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLCLKEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLCLKEN0_A {
        match self.bits {
            false => PLLCLKEN0_A::_0,
            true => PLLCLKEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLCLKEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLCLKEN0_A::_1
    }
}
#[doc = "Field `PLLCLKEN0` writer - PLL Clock Enable"]
pub type PLLCLKEN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, C5_SPEC, PLLCLKEN0_A, O>;
impl<'a, const O: u8> PLLCLKEN0_W<'a, O> {
    #[doc = "MCGPLLCLK is inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLCLKEN0_A::_0)
    }
    #[doc = "MCGPLLCLK is active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLCLKEN0_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - PLL External Reference Divider"]
    #[inline(always)]
    pub fn prdiv0(&self) -> PRDIV0_R {
        PRDIV0_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    pub fn pllsten0(&self) -> PLLSTEN0_R {
        PLLSTEN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    pub fn pllclken0(&self) -> PLLCLKEN0_R {
        PLLCLKEN0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL External Reference Divider"]
    #[inline(always)]
    #[must_use]
    pub fn prdiv0(&mut self) -> PRDIV0_W<0> {
        PRDIV0_W::new(self)
    }
    #[doc = "Bit 5 - PLL Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllsten0(&mut self) -> PLLSTEN0_W<5> {
        PLLSTEN0_W::new(self)
    }
    #[doc = "Bit 6 - PLL Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllclken0(&mut self) -> PLLCLKEN0_W<6> {
        PLLCLKEN0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5](index.html) module"]
pub struct C5_SPEC;
impl crate::RegisterSpec for C5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c5::R](R) reader structure"]
impl crate::Readable for C5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c5::W](W) writer structure"]
impl crate::Writable for C5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C5 to value 0"]
impl crate::Resettable for C5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
