#[doc = "Register `DFER` reader"]
pub struct R(crate::R<DFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFER` writer"]
pub struct W(crate::W<DFER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFER_SPEC>;
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
impl From<crate::W<DFER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFE0` reader - Digital Filter Enable"]
pub type DFE0_R = crate::BitReader<DFE0_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE0_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE0_A> for bool {
    #[inline(always)]
    fn from(variant: DFE0_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE0_A {
        match self.bits {
            false => DFE0_A::_0,
            true => DFE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE0_A::_1
    }
}
#[doc = "Field `DFE0` writer - Digital Filter Enable"]
pub type DFE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE0_A, O>;
impl<'a, const O: u8> DFE0_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE0_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE0_A::_1)
    }
}
#[doc = "Field `DFE1` reader - Digital Filter Enable"]
pub type DFE1_R = crate::BitReader<DFE1_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE1_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE1_A> for bool {
    #[inline(always)]
    fn from(variant: DFE1_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE1_A {
        match self.bits {
            false => DFE1_A::_0,
            true => DFE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE1_A::_1
    }
}
#[doc = "Field `DFE1` writer - Digital Filter Enable"]
pub type DFE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE1_A, O>;
impl<'a, const O: u8> DFE1_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE1_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE1_A::_1)
    }
}
#[doc = "Field `DFE2` reader - Digital Filter Enable"]
pub type DFE2_R = crate::BitReader<DFE2_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE2_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE2_A> for bool {
    #[inline(always)]
    fn from(variant: DFE2_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE2_A {
        match self.bits {
            false => DFE2_A::_0,
            true => DFE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE2_A::_1
    }
}
#[doc = "Field `DFE2` writer - Digital Filter Enable"]
pub type DFE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE2_A, O>;
impl<'a, const O: u8> DFE2_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE2_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE2_A::_1)
    }
}
#[doc = "Field `DFE3` reader - Digital Filter Enable"]
pub type DFE3_R = crate::BitReader<DFE3_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE3_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE3_A> for bool {
    #[inline(always)]
    fn from(variant: DFE3_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE3_A {
        match self.bits {
            false => DFE3_A::_0,
            true => DFE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE3_A::_1
    }
}
#[doc = "Field `DFE3` writer - Digital Filter Enable"]
pub type DFE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE3_A, O>;
impl<'a, const O: u8> DFE3_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE3_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE3_A::_1)
    }
}
#[doc = "Field `DFE4` reader - Digital Filter Enable"]
pub type DFE4_R = crate::BitReader<DFE4_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE4_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE4_A> for bool {
    #[inline(always)]
    fn from(variant: DFE4_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE4_A {
        match self.bits {
            false => DFE4_A::_0,
            true => DFE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE4_A::_1
    }
}
#[doc = "Field `DFE4` writer - Digital Filter Enable"]
pub type DFE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE4_A, O>;
impl<'a, const O: u8> DFE4_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE4_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE4_A::_1)
    }
}
#[doc = "Field `DFE5` reader - Digital Filter Enable"]
pub type DFE5_R = crate::BitReader<DFE5_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE5_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE5_A> for bool {
    #[inline(always)]
    fn from(variant: DFE5_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE5_A {
        match self.bits {
            false => DFE5_A::_0,
            true => DFE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE5_A::_1
    }
}
#[doc = "Field `DFE5` writer - Digital Filter Enable"]
pub type DFE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE5_A, O>;
impl<'a, const O: u8> DFE5_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE5_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE5_A::_1)
    }
}
#[doc = "Field `DFE6` reader - Digital Filter Enable"]
pub type DFE6_R = crate::BitReader<DFE6_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE6_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE6_A> for bool {
    #[inline(always)]
    fn from(variant: DFE6_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE6_A {
        match self.bits {
            false => DFE6_A::_0,
            true => DFE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE6_A::_1
    }
}
#[doc = "Field `DFE6` writer - Digital Filter Enable"]
pub type DFE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE6_A, O>;
impl<'a, const O: u8> DFE6_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE6_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE6_A::_1)
    }
}
#[doc = "Field `DFE7` reader - Digital Filter Enable"]
pub type DFE7_R = crate::BitReader<DFE7_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE7_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE7_A> for bool {
    #[inline(always)]
    fn from(variant: DFE7_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE7_A {
        match self.bits {
            false => DFE7_A::_0,
            true => DFE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE7_A::_1
    }
}
#[doc = "Field `DFE7` writer - Digital Filter Enable"]
pub type DFE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE7_A, O>;
impl<'a, const O: u8> DFE7_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE7_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE7_A::_1)
    }
}
#[doc = "Field `DFE8` reader - Digital Filter Enable"]
pub type DFE8_R = crate::BitReader<DFE8_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE8_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE8_A> for bool {
    #[inline(always)]
    fn from(variant: DFE8_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE8_A {
        match self.bits {
            false => DFE8_A::_0,
            true => DFE8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE8_A::_1
    }
}
#[doc = "Field `DFE8` writer - Digital Filter Enable"]
pub type DFE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE8_A, O>;
impl<'a, const O: u8> DFE8_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE8_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE8_A::_1)
    }
}
#[doc = "Field `DFE9` reader - Digital Filter Enable"]
pub type DFE9_R = crate::BitReader<DFE9_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE9_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE9_A> for bool {
    #[inline(always)]
    fn from(variant: DFE9_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE9_A {
        match self.bits {
            false => DFE9_A::_0,
            true => DFE9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE9_A::_1
    }
}
#[doc = "Field `DFE9` writer - Digital Filter Enable"]
pub type DFE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE9_A, O>;
impl<'a, const O: u8> DFE9_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE9_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE9_A::_1)
    }
}
#[doc = "Field `DFE10` reader - Digital Filter Enable"]
pub type DFE10_R = crate::BitReader<DFE10_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE10_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE10_A> for bool {
    #[inline(always)]
    fn from(variant: DFE10_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE10_A {
        match self.bits {
            false => DFE10_A::_0,
            true => DFE10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE10_A::_1
    }
}
#[doc = "Field `DFE10` writer - Digital Filter Enable"]
pub type DFE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE10_A, O>;
impl<'a, const O: u8> DFE10_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE10_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE10_A::_1)
    }
}
#[doc = "Field `DFE11` reader - Digital Filter Enable"]
pub type DFE11_R = crate::BitReader<DFE11_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE11_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE11_A> for bool {
    #[inline(always)]
    fn from(variant: DFE11_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE11_A {
        match self.bits {
            false => DFE11_A::_0,
            true => DFE11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE11_A::_1
    }
}
#[doc = "Field `DFE11` writer - Digital Filter Enable"]
pub type DFE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE11_A, O>;
impl<'a, const O: u8> DFE11_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE11_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE11_A::_1)
    }
}
#[doc = "Field `DFE12` reader - Digital Filter Enable"]
pub type DFE12_R = crate::BitReader<DFE12_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE12_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE12_A> for bool {
    #[inline(always)]
    fn from(variant: DFE12_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE12_A {
        match self.bits {
            false => DFE12_A::_0,
            true => DFE12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE12_A::_1
    }
}
#[doc = "Field `DFE12` writer - Digital Filter Enable"]
pub type DFE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE12_A, O>;
impl<'a, const O: u8> DFE12_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE12_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE12_A::_1)
    }
}
#[doc = "Field `DFE13` reader - Digital Filter Enable"]
pub type DFE13_R = crate::BitReader<DFE13_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE13_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE13_A> for bool {
    #[inline(always)]
    fn from(variant: DFE13_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE13_A {
        match self.bits {
            false => DFE13_A::_0,
            true => DFE13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE13_A::_1
    }
}
#[doc = "Field `DFE13` writer - Digital Filter Enable"]
pub type DFE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE13_A, O>;
impl<'a, const O: u8> DFE13_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE13_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE13_A::_1)
    }
}
#[doc = "Field `DFE14` reader - Digital Filter Enable"]
pub type DFE14_R = crate::BitReader<DFE14_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE14_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE14_A> for bool {
    #[inline(always)]
    fn from(variant: DFE14_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE14_A {
        match self.bits {
            false => DFE14_A::_0,
            true => DFE14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE14_A::_1
    }
}
#[doc = "Field `DFE14` writer - Digital Filter Enable"]
pub type DFE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE14_A, O>;
impl<'a, const O: u8> DFE14_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE14_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE14_A::_1)
    }
}
#[doc = "Field `DFE15` reader - Digital Filter Enable"]
pub type DFE15_R = crate::BitReader<DFE15_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE15_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE15_A> for bool {
    #[inline(always)]
    fn from(variant: DFE15_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE15_A {
        match self.bits {
            false => DFE15_A::_0,
            true => DFE15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE15_A::_1
    }
}
#[doc = "Field `DFE15` writer - Digital Filter Enable"]
pub type DFE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE15_A, O>;
impl<'a, const O: u8> DFE15_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE15_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE15_A::_1)
    }
}
#[doc = "Field `DFE16` reader - Digital Filter Enable"]
pub type DFE16_R = crate::BitReader<DFE16_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE16_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE16_A> for bool {
    #[inline(always)]
    fn from(variant: DFE16_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE16_A {
        match self.bits {
            false => DFE16_A::_0,
            true => DFE16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE16_A::_1
    }
}
#[doc = "Field `DFE16` writer - Digital Filter Enable"]
pub type DFE16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE16_A, O>;
impl<'a, const O: u8> DFE16_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE16_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE16_A::_1)
    }
}
#[doc = "Field `DFE17` reader - Digital Filter Enable"]
pub type DFE17_R = crate::BitReader<DFE17_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE17_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE17_A> for bool {
    #[inline(always)]
    fn from(variant: DFE17_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE17_A {
        match self.bits {
            false => DFE17_A::_0,
            true => DFE17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE17_A::_1
    }
}
#[doc = "Field `DFE17` writer - Digital Filter Enable"]
pub type DFE17_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE17_A, O>;
impl<'a, const O: u8> DFE17_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE17_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE17_A::_1)
    }
}
#[doc = "Field `DFE18` reader - Digital Filter Enable"]
pub type DFE18_R = crate::BitReader<DFE18_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE18_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE18_A> for bool {
    #[inline(always)]
    fn from(variant: DFE18_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE18_A {
        match self.bits {
            false => DFE18_A::_0,
            true => DFE18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE18_A::_1
    }
}
#[doc = "Field `DFE18` writer - Digital Filter Enable"]
pub type DFE18_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE18_A, O>;
impl<'a, const O: u8> DFE18_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE18_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE18_A::_1)
    }
}
#[doc = "Field `DFE19` reader - Digital Filter Enable"]
pub type DFE19_R = crate::BitReader<DFE19_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE19_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE19_A> for bool {
    #[inline(always)]
    fn from(variant: DFE19_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE19_A {
        match self.bits {
            false => DFE19_A::_0,
            true => DFE19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE19_A::_1
    }
}
#[doc = "Field `DFE19` writer - Digital Filter Enable"]
pub type DFE19_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE19_A, O>;
impl<'a, const O: u8> DFE19_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE19_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE19_A::_1)
    }
}
#[doc = "Field `DFE20` reader - Digital Filter Enable"]
pub type DFE20_R = crate::BitReader<DFE20_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE20_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE20_A> for bool {
    #[inline(always)]
    fn from(variant: DFE20_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE20_A {
        match self.bits {
            false => DFE20_A::_0,
            true => DFE20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE20_A::_1
    }
}
#[doc = "Field `DFE20` writer - Digital Filter Enable"]
pub type DFE20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE20_A, O>;
impl<'a, const O: u8> DFE20_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE20_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE20_A::_1)
    }
}
#[doc = "Field `DFE21` reader - Digital Filter Enable"]
pub type DFE21_R = crate::BitReader<DFE21_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE21_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE21_A> for bool {
    #[inline(always)]
    fn from(variant: DFE21_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE21_A {
        match self.bits {
            false => DFE21_A::_0,
            true => DFE21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE21_A::_1
    }
}
#[doc = "Field `DFE21` writer - Digital Filter Enable"]
pub type DFE21_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE21_A, O>;
impl<'a, const O: u8> DFE21_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE21_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE21_A::_1)
    }
}
#[doc = "Field `DFE22` reader - Digital Filter Enable"]
pub type DFE22_R = crate::BitReader<DFE22_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE22_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE22_A> for bool {
    #[inline(always)]
    fn from(variant: DFE22_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE22_A {
        match self.bits {
            false => DFE22_A::_0,
            true => DFE22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE22_A::_1
    }
}
#[doc = "Field `DFE22` writer - Digital Filter Enable"]
pub type DFE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE22_A, O>;
impl<'a, const O: u8> DFE22_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE22_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE22_A::_1)
    }
}
#[doc = "Field `DFE23` reader - Digital Filter Enable"]
pub type DFE23_R = crate::BitReader<DFE23_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE23_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE23_A> for bool {
    #[inline(always)]
    fn from(variant: DFE23_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE23_A {
        match self.bits {
            false => DFE23_A::_0,
            true => DFE23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE23_A::_1
    }
}
#[doc = "Field `DFE23` writer - Digital Filter Enable"]
pub type DFE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE23_A, O>;
impl<'a, const O: u8> DFE23_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE23_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE23_A::_1)
    }
}
#[doc = "Field `DFE24` reader - Digital Filter Enable"]
pub type DFE24_R = crate::BitReader<DFE24_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE24_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE24_A> for bool {
    #[inline(always)]
    fn from(variant: DFE24_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE24_A {
        match self.bits {
            false => DFE24_A::_0,
            true => DFE24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE24_A::_1
    }
}
#[doc = "Field `DFE24` writer - Digital Filter Enable"]
pub type DFE24_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE24_A, O>;
impl<'a, const O: u8> DFE24_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE24_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE24_A::_1)
    }
}
#[doc = "Field `DFE25` reader - Digital Filter Enable"]
pub type DFE25_R = crate::BitReader<DFE25_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE25_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE25_A> for bool {
    #[inline(always)]
    fn from(variant: DFE25_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE25_A {
        match self.bits {
            false => DFE25_A::_0,
            true => DFE25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE25_A::_1
    }
}
#[doc = "Field `DFE25` writer - Digital Filter Enable"]
pub type DFE25_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE25_A, O>;
impl<'a, const O: u8> DFE25_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE25_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE25_A::_1)
    }
}
#[doc = "Field `DFE26` reader - Digital Filter Enable"]
pub type DFE26_R = crate::BitReader<DFE26_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE26_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE26_A> for bool {
    #[inline(always)]
    fn from(variant: DFE26_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE26_A {
        match self.bits {
            false => DFE26_A::_0,
            true => DFE26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE26_A::_1
    }
}
#[doc = "Field `DFE26` writer - Digital Filter Enable"]
pub type DFE26_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE26_A, O>;
impl<'a, const O: u8> DFE26_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE26_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE26_A::_1)
    }
}
#[doc = "Field `DFE27` reader - Digital Filter Enable"]
pub type DFE27_R = crate::BitReader<DFE27_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE27_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE27_A> for bool {
    #[inline(always)]
    fn from(variant: DFE27_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE27_A {
        match self.bits {
            false => DFE27_A::_0,
            true => DFE27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE27_A::_1
    }
}
#[doc = "Field `DFE27` writer - Digital Filter Enable"]
pub type DFE27_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE27_A, O>;
impl<'a, const O: u8> DFE27_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE27_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE27_A::_1)
    }
}
#[doc = "Field `DFE28` reader - Digital Filter Enable"]
pub type DFE28_R = crate::BitReader<DFE28_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE28_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE28_A> for bool {
    #[inline(always)]
    fn from(variant: DFE28_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE28_A {
        match self.bits {
            false => DFE28_A::_0,
            true => DFE28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE28_A::_1
    }
}
#[doc = "Field `DFE28` writer - Digital Filter Enable"]
pub type DFE28_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE28_A, O>;
impl<'a, const O: u8> DFE28_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE28_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE28_A::_1)
    }
}
#[doc = "Field `DFE29` reader - Digital Filter Enable"]
pub type DFE29_R = crate::BitReader<DFE29_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE29_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE29_A> for bool {
    #[inline(always)]
    fn from(variant: DFE29_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE29_A {
        match self.bits {
            false => DFE29_A::_0,
            true => DFE29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE29_A::_1
    }
}
#[doc = "Field `DFE29` writer - Digital Filter Enable"]
pub type DFE29_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE29_A, O>;
impl<'a, const O: u8> DFE29_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE29_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE29_A::_1)
    }
}
#[doc = "Field `DFE30` reader - Digital Filter Enable"]
pub type DFE30_R = crate::BitReader<DFE30_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE30_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE30_A> for bool {
    #[inline(always)]
    fn from(variant: DFE30_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE30_A {
        match self.bits {
            false => DFE30_A::_0,
            true => DFE30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE30_A::_1
    }
}
#[doc = "Field `DFE30` writer - Digital Filter Enable"]
pub type DFE30_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE30_A, O>;
impl<'a, const O: u8> DFE30_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE30_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE30_A::_1)
    }
}
#[doc = "Field `DFE31` reader - Digital Filter Enable"]
pub type DFE31_R = crate::BitReader<DFE31_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFE31_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE31_A> for bool {
    #[inline(always)]
    fn from(variant: DFE31_A) -> Self {
        variant as u8 != 0
    }
}
impl DFE31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE31_A {
        match self.bits {
            false => DFE31_A::_0,
            true => DFE31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE31_A::_1
    }
}
#[doc = "Field `DFE31` writer - Digital Filter Enable"]
pub type DFE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFER_SPEC, DFE31_A, O>;
impl<'a, const O: u8> DFE31_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE31_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe0(&self) -> DFE0_R {
        DFE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe1(&self) -> DFE1_R {
        DFE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe2(&self) -> DFE2_R {
        DFE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe3(&self) -> DFE3_R {
        DFE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe4(&self) -> DFE4_R {
        DFE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe5(&self) -> DFE5_R {
        DFE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe6(&self) -> DFE6_R {
        DFE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe7(&self) -> DFE7_R {
        DFE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe8(&self) -> DFE8_R {
        DFE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe9(&self) -> DFE9_R {
        DFE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe10(&self) -> DFE10_R {
        DFE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe11(&self) -> DFE11_R {
        DFE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe12(&self) -> DFE12_R {
        DFE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe13(&self) -> DFE13_R {
        DFE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe14(&self) -> DFE14_R {
        DFE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe15(&self) -> DFE15_R {
        DFE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe16(&self) -> DFE16_R {
        DFE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe17(&self) -> DFE17_R {
        DFE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe18(&self) -> DFE18_R {
        DFE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe19(&self) -> DFE19_R {
        DFE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe20(&self) -> DFE20_R {
        DFE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe21(&self) -> DFE21_R {
        DFE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe22(&self) -> DFE22_R {
        DFE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe23(&self) -> DFE23_R {
        DFE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe24(&self) -> DFE24_R {
        DFE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe25(&self) -> DFE25_R {
        DFE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe26(&self) -> DFE26_R {
        DFE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe27(&self) -> DFE27_R {
        DFE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe28(&self) -> DFE28_R {
        DFE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe29(&self) -> DFE29_R {
        DFE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe30(&self) -> DFE30_R {
        DFE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe31(&self) -> DFE31_R {
        DFE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe0(&mut self) -> DFE0_W<0> {
        DFE0_W::new(self)
    }
    #[doc = "Bit 1 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe1(&mut self) -> DFE1_W<1> {
        DFE1_W::new(self)
    }
    #[doc = "Bit 2 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe2(&mut self) -> DFE2_W<2> {
        DFE2_W::new(self)
    }
    #[doc = "Bit 3 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe3(&mut self) -> DFE3_W<3> {
        DFE3_W::new(self)
    }
    #[doc = "Bit 4 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe4(&mut self) -> DFE4_W<4> {
        DFE4_W::new(self)
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe5(&mut self) -> DFE5_W<5> {
        DFE5_W::new(self)
    }
    #[doc = "Bit 6 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe6(&mut self) -> DFE6_W<6> {
        DFE6_W::new(self)
    }
    #[doc = "Bit 7 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe7(&mut self) -> DFE7_W<7> {
        DFE7_W::new(self)
    }
    #[doc = "Bit 8 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe8(&mut self) -> DFE8_W<8> {
        DFE8_W::new(self)
    }
    #[doc = "Bit 9 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe9(&mut self) -> DFE9_W<9> {
        DFE9_W::new(self)
    }
    #[doc = "Bit 10 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe10(&mut self) -> DFE10_W<10> {
        DFE10_W::new(self)
    }
    #[doc = "Bit 11 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe11(&mut self) -> DFE11_W<11> {
        DFE11_W::new(self)
    }
    #[doc = "Bit 12 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe12(&mut self) -> DFE12_W<12> {
        DFE12_W::new(self)
    }
    #[doc = "Bit 13 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe13(&mut self) -> DFE13_W<13> {
        DFE13_W::new(self)
    }
    #[doc = "Bit 14 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe14(&mut self) -> DFE14_W<14> {
        DFE14_W::new(self)
    }
    #[doc = "Bit 15 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe15(&mut self) -> DFE15_W<15> {
        DFE15_W::new(self)
    }
    #[doc = "Bit 16 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe16(&mut self) -> DFE16_W<16> {
        DFE16_W::new(self)
    }
    #[doc = "Bit 17 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe17(&mut self) -> DFE17_W<17> {
        DFE17_W::new(self)
    }
    #[doc = "Bit 18 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe18(&mut self) -> DFE18_W<18> {
        DFE18_W::new(self)
    }
    #[doc = "Bit 19 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe19(&mut self) -> DFE19_W<19> {
        DFE19_W::new(self)
    }
    #[doc = "Bit 20 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe20(&mut self) -> DFE20_W<20> {
        DFE20_W::new(self)
    }
    #[doc = "Bit 21 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe21(&mut self) -> DFE21_W<21> {
        DFE21_W::new(self)
    }
    #[doc = "Bit 22 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe22(&mut self) -> DFE22_W<22> {
        DFE22_W::new(self)
    }
    #[doc = "Bit 23 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe23(&mut self) -> DFE23_W<23> {
        DFE23_W::new(self)
    }
    #[doc = "Bit 24 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe24(&mut self) -> DFE24_W<24> {
        DFE24_W::new(self)
    }
    #[doc = "Bit 25 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe25(&mut self) -> DFE25_W<25> {
        DFE25_W::new(self)
    }
    #[doc = "Bit 26 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe26(&mut self) -> DFE26_W<26> {
        DFE26_W::new(self)
    }
    #[doc = "Bit 27 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe27(&mut self) -> DFE27_W<27> {
        DFE27_W::new(self)
    }
    #[doc = "Bit 28 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe28(&mut self) -> DFE28_W<28> {
        DFE28_W::new(self)
    }
    #[doc = "Bit 29 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe29(&mut self) -> DFE29_W<29> {
        DFE29_W::new(self)
    }
    #[doc = "Bit 30 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe30(&mut self) -> DFE30_W<30> {
        DFE30_W::new(self)
    }
    #[doc = "Bit 31 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe31(&mut self) -> DFE31_W<31> {
        DFE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Filter Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfer](index.html) module"]
pub struct DFER_SPEC;
impl crate::RegisterSpec for DFER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfer::R](R) reader structure"]
impl crate::Readable for DFER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfer::W](W) writer structure"]
impl crate::Writable for DFER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFER to value 0"]
impl crate::Resettable for DFER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
