#[doc = "Register `HRS` reader"]
pub struct R(crate::R<HRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HRS0` reader - Hardware Request Status Channel 0"]
pub type HRS0_R = crate::BitReader<HRS0_A>;
#[doc = "Hardware Request Status Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS0_A {
    #[doc = "0: A hardware service request for channel 0 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 0 is present"]
    _1 = 1,
}
impl From<HRS0_A> for bool {
    #[inline(always)]
    fn from(variant: HRS0_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS0_A {
        match self.bits {
            false => HRS0_A::_0,
            true => HRS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS0_A::_1
    }
}
#[doc = "Field `HRS1` reader - Hardware Request Status Channel 1"]
pub type HRS1_R = crate::BitReader<HRS1_A>;
#[doc = "Hardware Request Status Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS1_A {
    #[doc = "0: A hardware service request for channel 1 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 1 is present"]
    _1 = 1,
}
impl From<HRS1_A> for bool {
    #[inline(always)]
    fn from(variant: HRS1_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS1_A {
        match self.bits {
            false => HRS1_A::_0,
            true => HRS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS1_A::_1
    }
}
#[doc = "Field `HRS2` reader - Hardware Request Status Channel 2"]
pub type HRS2_R = crate::BitReader<HRS2_A>;
#[doc = "Hardware Request Status Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS2_A {
    #[doc = "0: A hardware service request for channel 2 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 2 is present"]
    _1 = 1,
}
impl From<HRS2_A> for bool {
    #[inline(always)]
    fn from(variant: HRS2_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS2_A {
        match self.bits {
            false => HRS2_A::_0,
            true => HRS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS2_A::_1
    }
}
#[doc = "Field `HRS3` reader - Hardware Request Status Channel 3"]
pub type HRS3_R = crate::BitReader<HRS3_A>;
#[doc = "Hardware Request Status Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS3_A {
    #[doc = "0: A hardware service request for channel 3 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 3 is present"]
    _1 = 1,
}
impl From<HRS3_A> for bool {
    #[inline(always)]
    fn from(variant: HRS3_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS3_A {
        match self.bits {
            false => HRS3_A::_0,
            true => HRS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS3_A::_1
    }
}
#[doc = "Field `HRS4` reader - Hardware Request Status Channel 4"]
pub type HRS4_R = crate::BitReader<HRS4_A>;
#[doc = "Hardware Request Status Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS4_A {
    #[doc = "0: A hardware service request for channel 4 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 4 is present"]
    _1 = 1,
}
impl From<HRS4_A> for bool {
    #[inline(always)]
    fn from(variant: HRS4_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS4_A {
        match self.bits {
            false => HRS4_A::_0,
            true => HRS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS4_A::_1
    }
}
#[doc = "Field `HRS5` reader - Hardware Request Status Channel 5"]
pub type HRS5_R = crate::BitReader<HRS5_A>;
#[doc = "Hardware Request Status Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS5_A {
    #[doc = "0: A hardware service request for channel 5 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 5 is present"]
    _1 = 1,
}
impl From<HRS5_A> for bool {
    #[inline(always)]
    fn from(variant: HRS5_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS5_A {
        match self.bits {
            false => HRS5_A::_0,
            true => HRS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS5_A::_1
    }
}
#[doc = "Field `HRS6` reader - Hardware Request Status Channel 6"]
pub type HRS6_R = crate::BitReader<HRS6_A>;
#[doc = "Hardware Request Status Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS6_A {
    #[doc = "0: A hardware service request for channel 6 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 6 is present"]
    _1 = 1,
}
impl From<HRS6_A> for bool {
    #[inline(always)]
    fn from(variant: HRS6_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS6_A {
        match self.bits {
            false => HRS6_A::_0,
            true => HRS6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS6_A::_1
    }
}
#[doc = "Field `HRS7` reader - Hardware Request Status Channel 7"]
pub type HRS7_R = crate::BitReader<HRS7_A>;
#[doc = "Hardware Request Status Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS7_A {
    #[doc = "0: A hardware service request for channel 7 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 7 is present"]
    _1 = 1,
}
impl From<HRS7_A> for bool {
    #[inline(always)]
    fn from(variant: HRS7_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS7_A {
        match self.bits {
            false => HRS7_A::_0,
            true => HRS7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS7_A::_1
    }
}
#[doc = "Field `HRS8` reader - Hardware Request Status Channel 8"]
pub type HRS8_R = crate::BitReader<HRS8_A>;
#[doc = "Hardware Request Status Channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS8_A {
    #[doc = "0: A hardware service request for channel 8 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 8 is present"]
    _1 = 1,
}
impl From<HRS8_A> for bool {
    #[inline(always)]
    fn from(variant: HRS8_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS8_A {
        match self.bits {
            false => HRS8_A::_0,
            true => HRS8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS8_A::_1
    }
}
#[doc = "Field `HRS9` reader - Hardware Request Status Channel 9"]
pub type HRS9_R = crate::BitReader<HRS9_A>;
#[doc = "Hardware Request Status Channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS9_A {
    #[doc = "0: A hardware service request for channel 9 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 9 is present"]
    _1 = 1,
}
impl From<HRS9_A> for bool {
    #[inline(always)]
    fn from(variant: HRS9_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS9_A {
        match self.bits {
            false => HRS9_A::_0,
            true => HRS9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS9_A::_1
    }
}
#[doc = "Field `HRS10` reader - Hardware Request Status Channel 10"]
pub type HRS10_R = crate::BitReader<HRS10_A>;
#[doc = "Hardware Request Status Channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS10_A {
    #[doc = "0: A hardware service request for channel 10 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 10 is present"]
    _1 = 1,
}
impl From<HRS10_A> for bool {
    #[inline(always)]
    fn from(variant: HRS10_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS10_A {
        match self.bits {
            false => HRS10_A::_0,
            true => HRS10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS10_A::_1
    }
}
#[doc = "Field `HRS11` reader - Hardware Request Status Channel 11"]
pub type HRS11_R = crate::BitReader<HRS11_A>;
#[doc = "Hardware Request Status Channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS11_A {
    #[doc = "0: A hardware service request for channel 11 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 11 is present"]
    _1 = 1,
}
impl From<HRS11_A> for bool {
    #[inline(always)]
    fn from(variant: HRS11_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS11_A {
        match self.bits {
            false => HRS11_A::_0,
            true => HRS11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS11_A::_1
    }
}
#[doc = "Field `HRS12` reader - Hardware Request Status Channel 12"]
pub type HRS12_R = crate::BitReader<HRS12_A>;
#[doc = "Hardware Request Status Channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS12_A {
    #[doc = "0: A hardware service request for channel 12 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 12 is present"]
    _1 = 1,
}
impl From<HRS12_A> for bool {
    #[inline(always)]
    fn from(variant: HRS12_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS12_A {
        match self.bits {
            false => HRS12_A::_0,
            true => HRS12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS12_A::_1
    }
}
#[doc = "Field `HRS13` reader - Hardware Request Status Channel 13"]
pub type HRS13_R = crate::BitReader<HRS13_A>;
#[doc = "Hardware Request Status Channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS13_A {
    #[doc = "0: A hardware service request for channel 13 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 13 is present"]
    _1 = 1,
}
impl From<HRS13_A> for bool {
    #[inline(always)]
    fn from(variant: HRS13_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS13_A {
        match self.bits {
            false => HRS13_A::_0,
            true => HRS13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS13_A::_1
    }
}
#[doc = "Field `HRS14` reader - Hardware Request Status Channel 14"]
pub type HRS14_R = crate::BitReader<HRS14_A>;
#[doc = "Hardware Request Status Channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS14_A {
    #[doc = "0: A hardware service request for channel 14 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 14 is present"]
    _1 = 1,
}
impl From<HRS14_A> for bool {
    #[inline(always)]
    fn from(variant: HRS14_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS14_A {
        match self.bits {
            false => HRS14_A::_0,
            true => HRS14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS14_A::_1
    }
}
#[doc = "Field `HRS15` reader - Hardware Request Status Channel 15"]
pub type HRS15_R = crate::BitReader<HRS15_A>;
#[doc = "Hardware Request Status Channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRS15_A {
    #[doc = "0: A hardware service request for channel 15 is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for channel 15 is present"]
    _1 = 1,
}
impl From<HRS15_A> for bool {
    #[inline(always)]
    fn from(variant: HRS15_A) -> Self {
        variant as u8 != 0
    }
}
impl HRS15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS15_A {
        match self.bits {
            false => HRS15_A::_0,
            true => HRS15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS15_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline(always)]
    pub fn hrs0(&self) -> HRS0_R {
        HRS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline(always)]
    pub fn hrs1(&self) -> HRS1_R {
        HRS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline(always)]
    pub fn hrs2(&self) -> HRS2_R {
        HRS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline(always)]
    pub fn hrs3(&self) -> HRS3_R {
        HRS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline(always)]
    pub fn hrs4(&self) -> HRS4_R {
        HRS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline(always)]
    pub fn hrs5(&self) -> HRS5_R {
        HRS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline(always)]
    pub fn hrs6(&self) -> HRS6_R {
        HRS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline(always)]
    pub fn hrs7(&self) -> HRS7_R {
        HRS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline(always)]
    pub fn hrs8(&self) -> HRS8_R {
        HRS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline(always)]
    pub fn hrs9(&self) -> HRS9_R {
        HRS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline(always)]
    pub fn hrs10(&self) -> HRS10_R {
        HRS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline(always)]
    pub fn hrs11(&self) -> HRS11_R {
        HRS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline(always)]
    pub fn hrs12(&self) -> HRS12_R {
        HRS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline(always)]
    pub fn hrs13(&self) -> HRS13_R {
        HRS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline(always)]
    pub fn hrs14(&self) -> HRS14_R {
        HRS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline(always)]
    pub fn hrs15(&self) -> HRS15_R {
        HRS15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Hardware Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrs](index.html) module"]
pub struct HRS_SPEC;
impl crate::RegisterSpec for HRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrs::R](R) reader structure"]
impl crate::Readable for HRS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HRS to value 0"]
impl crate::Resettable for HRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
