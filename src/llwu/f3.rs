#[doc = "Register `F3` reader"]
pub struct R(crate::R<F3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MWUF0` reader - Wakeup flag For module 0"]
pub type MWUF0_R = crate::BitReader<MWUF0_A>;
#[doc = "Wakeup flag For module 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MWUF0_A {
    #[doc = "0: Module 0 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 0 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF0_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF0_A) -> Self {
        variant as u8 != 0
    }
}
impl MWUF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF0_A {
        match self.bits {
            false => MWUF0_A::_0,
            true => MWUF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF0_A::_1
    }
}
#[doc = "Field `MWUF1` reader - Wakeup flag For module 1"]
pub type MWUF1_R = crate::BitReader<MWUF1_A>;
#[doc = "Wakeup flag For module 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MWUF1_A {
    #[doc = "0: Module 1 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 1 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF1_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF1_A) -> Self {
        variant as u8 != 0
    }
}
impl MWUF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF1_A {
        match self.bits {
            false => MWUF1_A::_0,
            true => MWUF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF1_A::_1
    }
}
#[doc = "Field `MWUF2` reader - Wakeup flag For module 2"]
pub type MWUF2_R = crate::BitReader<MWUF2_A>;
#[doc = "Wakeup flag For module 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MWUF2_A {
    #[doc = "0: Module 2 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 2 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF2_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF2_A) -> Self {
        variant as u8 != 0
    }
}
impl MWUF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF2_A {
        match self.bits {
            false => MWUF2_A::_0,
            true => MWUF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF2_A::_1
    }
}
#[doc = "Field `MWUF3` reader - Wakeup flag For module 3"]
pub type MWUF3_R = crate::BitReader<MWUF3_A>;
#[doc = "Wakeup flag For module 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MWUF3_A {
    #[doc = "0: Module 3 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 3 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF3_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF3_A) -> Self {
        variant as u8 != 0
    }
}
impl MWUF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF3_A {
        match self.bits {
            false => MWUF3_A::_0,
            true => MWUF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF3_A::_1
    }
}
#[doc = "Field `MWUF4` reader - Wakeup flag For module 4"]
pub type MWUF4_R = crate::BitReader<MWUF4_A>;
#[doc = "Wakeup flag For module 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MWUF4_A {
    #[doc = "0: Module 4 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 4 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF4_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF4_A) -> Self {
        variant as u8 != 0
    }
}
impl MWUF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF4_A {
        match self.bits {
            false => MWUF4_A::_0,
            true => MWUF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF4_A::_1
    }
}
#[doc = "Field `MWUF5` reader - Wakeup flag For module 5"]
pub type MWUF5_R = crate::BitReader<MWUF5_A>;
#[doc = "Wakeup flag For module 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MWUF5_A {
    #[doc = "0: Module 5 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 5 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF5_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF5_A) -> Self {
        variant as u8 != 0
    }
}
impl MWUF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF5_A {
        match self.bits {
            false => MWUF5_A::_0,
            true => MWUF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF5_A::_1
    }
}
#[doc = "Field `MWUF6` reader - Wakeup flag For module 6"]
pub type MWUF6_R = crate::BitReader<MWUF6_A>;
#[doc = "Wakeup flag For module 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MWUF6_A {
    #[doc = "0: Module 6 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 6 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF6_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF6_A) -> Self {
        variant as u8 != 0
    }
}
impl MWUF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF6_A {
        match self.bits {
            false => MWUF6_A::_0,
            true => MWUF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF6_A::_1
    }
}
#[doc = "Field `MWUF7` reader - Wakeup flag For module 7"]
pub type MWUF7_R = crate::BitReader<MWUF7_A>;
#[doc = "Wakeup flag For module 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MWUF7_A {
    #[doc = "0: Module 7 input was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Module 7 input was a wakeup source"]
    _1 = 1,
}
impl From<MWUF7_A> for bool {
    #[inline(always)]
    fn from(variant: MWUF7_A) -> Self {
        variant as u8 != 0
    }
}
impl MWUF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWUF7_A {
        match self.bits {
            false => MWUF7_A::_0,
            true => MWUF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWUF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWUF7_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup flag For module 0"]
    #[inline(always)]
    pub fn mwuf0(&self) -> MWUF0_R {
        MWUF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup flag For module 1"]
    #[inline(always)]
    pub fn mwuf1(&self) -> MWUF1_R {
        MWUF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup flag For module 2"]
    #[inline(always)]
    pub fn mwuf2(&self) -> MWUF2_R {
        MWUF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup flag For module 3"]
    #[inline(always)]
    pub fn mwuf3(&self) -> MWUF3_R {
        MWUF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup flag For module 4"]
    #[inline(always)]
    pub fn mwuf4(&self) -> MWUF4_R {
        MWUF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup flag For module 5"]
    #[inline(always)]
    pub fn mwuf5(&self) -> MWUF5_R {
        MWUF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup flag For module 6"]
    #[inline(always)]
    pub fn mwuf6(&self) -> MWUF6_R {
        MWUF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup flag For module 7"]
    #[inline(always)]
    pub fn mwuf7(&self) -> MWUF7_R {
        MWUF7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "LLWU Flag 3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f3](index.html) module"]
pub struct F3_SPEC;
impl crate::RegisterSpec for F3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [f3::R](R) reader structure"]
impl crate::Readable for F3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets F3 to value 0"]
impl crate::Resettable for F3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
