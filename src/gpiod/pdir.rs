#[doc = "Register `PDIR` reader"]
pub struct R(crate::R<PDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PDI0` reader - Port Data Input"]
pub type PDI0_R = crate::BitReader<PDI0_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI0_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI0_A> for bool {
    #[inline(always)]
    fn from(variant: PDI0_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI0_A {
        match self.bits {
            false => PDI0_A::_0,
            true => PDI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI0_A::_1
    }
}
#[doc = "Field `PDI1` reader - Port Data Input"]
pub type PDI1_R = crate::BitReader<PDI1_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI1_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI1_A> for bool {
    #[inline(always)]
    fn from(variant: PDI1_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI1_A {
        match self.bits {
            false => PDI1_A::_0,
            true => PDI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI1_A::_1
    }
}
#[doc = "Field `PDI2` reader - Port Data Input"]
pub type PDI2_R = crate::BitReader<PDI2_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI2_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI2_A> for bool {
    #[inline(always)]
    fn from(variant: PDI2_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI2_A {
        match self.bits {
            false => PDI2_A::_0,
            true => PDI2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI2_A::_1
    }
}
#[doc = "Field `PDI3` reader - Port Data Input"]
pub type PDI3_R = crate::BitReader<PDI3_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI3_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI3_A> for bool {
    #[inline(always)]
    fn from(variant: PDI3_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI3_A {
        match self.bits {
            false => PDI3_A::_0,
            true => PDI3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI3_A::_1
    }
}
#[doc = "Field `PDI4` reader - Port Data Input"]
pub type PDI4_R = crate::BitReader<PDI4_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI4_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI4_A> for bool {
    #[inline(always)]
    fn from(variant: PDI4_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI4_A {
        match self.bits {
            false => PDI4_A::_0,
            true => PDI4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI4_A::_1
    }
}
#[doc = "Field `PDI5` reader - Port Data Input"]
pub type PDI5_R = crate::BitReader<PDI5_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI5_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI5_A> for bool {
    #[inline(always)]
    fn from(variant: PDI5_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI5_A {
        match self.bits {
            false => PDI5_A::_0,
            true => PDI5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI5_A::_1
    }
}
#[doc = "Field `PDI6` reader - Port Data Input"]
pub type PDI6_R = crate::BitReader<PDI6_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI6_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI6_A> for bool {
    #[inline(always)]
    fn from(variant: PDI6_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI6_A {
        match self.bits {
            false => PDI6_A::_0,
            true => PDI6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI6_A::_1
    }
}
#[doc = "Field `PDI7` reader - Port Data Input"]
pub type PDI7_R = crate::BitReader<PDI7_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI7_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI7_A> for bool {
    #[inline(always)]
    fn from(variant: PDI7_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI7_A {
        match self.bits {
            false => PDI7_A::_0,
            true => PDI7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI7_A::_1
    }
}
#[doc = "Field `PDI8` reader - Port Data Input"]
pub type PDI8_R = crate::BitReader<PDI8_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI8_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI8_A> for bool {
    #[inline(always)]
    fn from(variant: PDI8_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI8_A {
        match self.bits {
            false => PDI8_A::_0,
            true => PDI8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI8_A::_1
    }
}
#[doc = "Field `PDI9` reader - Port Data Input"]
pub type PDI9_R = crate::BitReader<PDI9_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI9_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI9_A> for bool {
    #[inline(always)]
    fn from(variant: PDI9_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI9_A {
        match self.bits {
            false => PDI9_A::_0,
            true => PDI9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI9_A::_1
    }
}
#[doc = "Field `PDI10` reader - Port Data Input"]
pub type PDI10_R = crate::BitReader<PDI10_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI10_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI10_A> for bool {
    #[inline(always)]
    fn from(variant: PDI10_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI10_A {
        match self.bits {
            false => PDI10_A::_0,
            true => PDI10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI10_A::_1
    }
}
#[doc = "Field `PDI11` reader - Port Data Input"]
pub type PDI11_R = crate::BitReader<PDI11_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI11_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI11_A> for bool {
    #[inline(always)]
    fn from(variant: PDI11_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI11_A {
        match self.bits {
            false => PDI11_A::_0,
            true => PDI11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI11_A::_1
    }
}
#[doc = "Field `PDI12` reader - Port Data Input"]
pub type PDI12_R = crate::BitReader<PDI12_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI12_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI12_A> for bool {
    #[inline(always)]
    fn from(variant: PDI12_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI12_A {
        match self.bits {
            false => PDI12_A::_0,
            true => PDI12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI12_A::_1
    }
}
#[doc = "Field `PDI13` reader - Port Data Input"]
pub type PDI13_R = crate::BitReader<PDI13_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI13_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI13_A> for bool {
    #[inline(always)]
    fn from(variant: PDI13_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI13_A {
        match self.bits {
            false => PDI13_A::_0,
            true => PDI13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI13_A::_1
    }
}
#[doc = "Field `PDI14` reader - Port Data Input"]
pub type PDI14_R = crate::BitReader<PDI14_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI14_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI14_A> for bool {
    #[inline(always)]
    fn from(variant: PDI14_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI14_A {
        match self.bits {
            false => PDI14_A::_0,
            true => PDI14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI14_A::_1
    }
}
#[doc = "Field `PDI15` reader - Port Data Input"]
pub type PDI15_R = crate::BitReader<PDI15_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI15_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI15_A> for bool {
    #[inline(always)]
    fn from(variant: PDI15_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI15_A {
        match self.bits {
            false => PDI15_A::_0,
            true => PDI15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI15_A::_1
    }
}
#[doc = "Field `PDI16` reader - Port Data Input"]
pub type PDI16_R = crate::BitReader<PDI16_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI16_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI16_A> for bool {
    #[inline(always)]
    fn from(variant: PDI16_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI16_A {
        match self.bits {
            false => PDI16_A::_0,
            true => PDI16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI16_A::_1
    }
}
#[doc = "Field `PDI17` reader - Port Data Input"]
pub type PDI17_R = crate::BitReader<PDI17_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI17_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI17_A> for bool {
    #[inline(always)]
    fn from(variant: PDI17_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI17_A {
        match self.bits {
            false => PDI17_A::_0,
            true => PDI17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI17_A::_1
    }
}
#[doc = "Field `PDI18` reader - Port Data Input"]
pub type PDI18_R = crate::BitReader<PDI18_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI18_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI18_A> for bool {
    #[inline(always)]
    fn from(variant: PDI18_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI18_A {
        match self.bits {
            false => PDI18_A::_0,
            true => PDI18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI18_A::_1
    }
}
#[doc = "Field `PDI19` reader - Port Data Input"]
pub type PDI19_R = crate::BitReader<PDI19_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI19_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI19_A> for bool {
    #[inline(always)]
    fn from(variant: PDI19_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI19_A {
        match self.bits {
            false => PDI19_A::_0,
            true => PDI19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI19_A::_1
    }
}
#[doc = "Field `PDI20` reader - Port Data Input"]
pub type PDI20_R = crate::BitReader<PDI20_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI20_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI20_A> for bool {
    #[inline(always)]
    fn from(variant: PDI20_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI20_A {
        match self.bits {
            false => PDI20_A::_0,
            true => PDI20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI20_A::_1
    }
}
#[doc = "Field `PDI21` reader - Port Data Input"]
pub type PDI21_R = crate::BitReader<PDI21_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI21_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI21_A> for bool {
    #[inline(always)]
    fn from(variant: PDI21_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI21_A {
        match self.bits {
            false => PDI21_A::_0,
            true => PDI21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI21_A::_1
    }
}
#[doc = "Field `PDI22` reader - Port Data Input"]
pub type PDI22_R = crate::BitReader<PDI22_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI22_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI22_A> for bool {
    #[inline(always)]
    fn from(variant: PDI22_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI22_A {
        match self.bits {
            false => PDI22_A::_0,
            true => PDI22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI22_A::_1
    }
}
#[doc = "Field `PDI23` reader - Port Data Input"]
pub type PDI23_R = crate::BitReader<PDI23_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI23_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI23_A> for bool {
    #[inline(always)]
    fn from(variant: PDI23_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI23_A {
        match self.bits {
            false => PDI23_A::_0,
            true => PDI23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI23_A::_1
    }
}
#[doc = "Field `PDI24` reader - Port Data Input"]
pub type PDI24_R = crate::BitReader<PDI24_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI24_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI24_A> for bool {
    #[inline(always)]
    fn from(variant: PDI24_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI24_A {
        match self.bits {
            false => PDI24_A::_0,
            true => PDI24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI24_A::_1
    }
}
#[doc = "Field `PDI25` reader - Port Data Input"]
pub type PDI25_R = crate::BitReader<PDI25_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI25_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI25_A> for bool {
    #[inline(always)]
    fn from(variant: PDI25_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI25_A {
        match self.bits {
            false => PDI25_A::_0,
            true => PDI25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI25_A::_1
    }
}
#[doc = "Field `PDI26` reader - Port Data Input"]
pub type PDI26_R = crate::BitReader<PDI26_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI26_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI26_A> for bool {
    #[inline(always)]
    fn from(variant: PDI26_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI26_A {
        match self.bits {
            false => PDI26_A::_0,
            true => PDI26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI26_A::_1
    }
}
#[doc = "Field `PDI27` reader - Port Data Input"]
pub type PDI27_R = crate::BitReader<PDI27_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI27_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI27_A> for bool {
    #[inline(always)]
    fn from(variant: PDI27_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI27_A {
        match self.bits {
            false => PDI27_A::_0,
            true => PDI27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI27_A::_1
    }
}
#[doc = "Field `PDI28` reader - Port Data Input"]
pub type PDI28_R = crate::BitReader<PDI28_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI28_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI28_A> for bool {
    #[inline(always)]
    fn from(variant: PDI28_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI28_A {
        match self.bits {
            false => PDI28_A::_0,
            true => PDI28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI28_A::_1
    }
}
#[doc = "Field `PDI29` reader - Port Data Input"]
pub type PDI29_R = crate::BitReader<PDI29_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI29_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI29_A> for bool {
    #[inline(always)]
    fn from(variant: PDI29_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI29_A {
        match self.bits {
            false => PDI29_A::_0,
            true => PDI29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI29_A::_1
    }
}
#[doc = "Field `PDI30` reader - Port Data Input"]
pub type PDI30_R = crate::BitReader<PDI30_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI30_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI30_A> for bool {
    #[inline(always)]
    fn from(variant: PDI30_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI30_A {
        match self.bits {
            false => PDI30_A::_0,
            true => PDI30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI30_A::_1
    }
}
#[doc = "Field `PDI31` reader - Port Data Input"]
pub type PDI31_R = crate::BitReader<PDI31_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDI31_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI31_A> for bool {
    #[inline(always)]
    fn from(variant: PDI31_A) -> Self {
        variant as u8 != 0
    }
}
impl PDI31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDI31_A {
        match self.bits {
            false => PDI31_A::_0,
            true => PDI31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI31_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Port Data Input"]
    #[inline(always)]
    pub fn pdi0(&self) -> PDI0_R {
        PDI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Data Input"]
    #[inline(always)]
    pub fn pdi1(&self) -> PDI1_R {
        PDI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Data Input"]
    #[inline(always)]
    pub fn pdi2(&self) -> PDI2_R {
        PDI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Data Input"]
    #[inline(always)]
    pub fn pdi3(&self) -> PDI3_R {
        PDI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Data Input"]
    #[inline(always)]
    pub fn pdi4(&self) -> PDI4_R {
        PDI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Data Input"]
    #[inline(always)]
    pub fn pdi5(&self) -> PDI5_R {
        PDI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Data Input"]
    #[inline(always)]
    pub fn pdi6(&self) -> PDI6_R {
        PDI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Data Input"]
    #[inline(always)]
    pub fn pdi7(&self) -> PDI7_R {
        PDI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Data Input"]
    #[inline(always)]
    pub fn pdi8(&self) -> PDI8_R {
        PDI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Data Input"]
    #[inline(always)]
    pub fn pdi9(&self) -> PDI9_R {
        PDI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Data Input"]
    #[inline(always)]
    pub fn pdi10(&self) -> PDI10_R {
        PDI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Data Input"]
    #[inline(always)]
    pub fn pdi11(&self) -> PDI11_R {
        PDI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Data Input"]
    #[inline(always)]
    pub fn pdi12(&self) -> PDI12_R {
        PDI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Data Input"]
    #[inline(always)]
    pub fn pdi13(&self) -> PDI13_R {
        PDI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Data Input"]
    #[inline(always)]
    pub fn pdi14(&self) -> PDI14_R {
        PDI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Data Input"]
    #[inline(always)]
    pub fn pdi15(&self) -> PDI15_R {
        PDI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Data Input"]
    #[inline(always)]
    pub fn pdi16(&self) -> PDI16_R {
        PDI16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Data Input"]
    #[inline(always)]
    pub fn pdi17(&self) -> PDI17_R {
        PDI17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Data Input"]
    #[inline(always)]
    pub fn pdi18(&self) -> PDI18_R {
        PDI18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Data Input"]
    #[inline(always)]
    pub fn pdi19(&self) -> PDI19_R {
        PDI19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Data Input"]
    #[inline(always)]
    pub fn pdi20(&self) -> PDI20_R {
        PDI20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Data Input"]
    #[inline(always)]
    pub fn pdi21(&self) -> PDI21_R {
        PDI21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Data Input"]
    #[inline(always)]
    pub fn pdi22(&self) -> PDI22_R {
        PDI22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Data Input"]
    #[inline(always)]
    pub fn pdi23(&self) -> PDI23_R {
        PDI23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Data Input"]
    #[inline(always)]
    pub fn pdi24(&self) -> PDI24_R {
        PDI24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Data Input"]
    #[inline(always)]
    pub fn pdi25(&self) -> PDI25_R {
        PDI25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Data Input"]
    #[inline(always)]
    pub fn pdi26(&self) -> PDI26_R {
        PDI26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Data Input"]
    #[inline(always)]
    pub fn pdi27(&self) -> PDI27_R {
        PDI27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Data Input"]
    #[inline(always)]
    pub fn pdi28(&self) -> PDI28_R {
        PDI28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Data Input"]
    #[inline(always)]
    pub fn pdi29(&self) -> PDI29_R {
        PDI29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Data Input"]
    #[inline(always)]
    pub fn pdi30(&self) -> PDI30_R {
        PDI30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Data Input"]
    #[inline(always)]
    pub fn pdi31(&self) -> PDI31_R {
        PDI31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Port Data Input Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdir](index.html) module"]
pub struct PDIR_SPEC;
impl crate::RegisterSpec for PDIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdir::R](R) reader structure"]
impl crate::Readable for PDIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDIR to value 0"]
impl crate::Resettable for PDIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
