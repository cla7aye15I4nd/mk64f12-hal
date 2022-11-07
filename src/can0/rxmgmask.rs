#[doc = "Register `RXMGMASK` reader"]
pub struct R(crate::R<RXMGMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMGMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMGMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMGMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXMGMASK` writer"]
pub struct W(crate::W<RXMGMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXMGMASK_SPEC>;
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
impl From<crate::W<RXMGMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXMGMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MG0` reader - Rx Mailboxes Global Mask Bits"]
pub type MG0_R = crate::BitReader<MG0_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG0_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG0_A> for bool {
    #[inline(always)]
    fn from(variant: MG0_A) -> Self {
        variant as u8 != 0
    }
}
impl MG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG0_A {
        match self.bits {
            false => MG0_A::_0,
            true => MG0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG0_A::_1
    }
}
#[doc = "Field `MG0` writer - Rx Mailboxes Global Mask Bits"]
pub type MG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG0_A, O>;
impl<'a, const O: u8> MG0_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG0_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG0_A::_1)
    }
}
#[doc = "Field `MG1` reader - Rx Mailboxes Global Mask Bits"]
pub type MG1_R = crate::BitReader<MG1_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG1_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG1_A> for bool {
    #[inline(always)]
    fn from(variant: MG1_A) -> Self {
        variant as u8 != 0
    }
}
impl MG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG1_A {
        match self.bits {
            false => MG1_A::_0,
            true => MG1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG1_A::_1
    }
}
#[doc = "Field `MG1` writer - Rx Mailboxes Global Mask Bits"]
pub type MG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG1_A, O>;
impl<'a, const O: u8> MG1_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG1_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG1_A::_1)
    }
}
#[doc = "Field `MG2` reader - Rx Mailboxes Global Mask Bits"]
pub type MG2_R = crate::BitReader<MG2_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG2_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG2_A> for bool {
    #[inline(always)]
    fn from(variant: MG2_A) -> Self {
        variant as u8 != 0
    }
}
impl MG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG2_A {
        match self.bits {
            false => MG2_A::_0,
            true => MG2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG2_A::_1
    }
}
#[doc = "Field `MG2` writer - Rx Mailboxes Global Mask Bits"]
pub type MG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG2_A, O>;
impl<'a, const O: u8> MG2_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG2_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG2_A::_1)
    }
}
#[doc = "Field `MG3` reader - Rx Mailboxes Global Mask Bits"]
pub type MG3_R = crate::BitReader<MG3_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG3_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG3_A> for bool {
    #[inline(always)]
    fn from(variant: MG3_A) -> Self {
        variant as u8 != 0
    }
}
impl MG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG3_A {
        match self.bits {
            false => MG3_A::_0,
            true => MG3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG3_A::_1
    }
}
#[doc = "Field `MG3` writer - Rx Mailboxes Global Mask Bits"]
pub type MG3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG3_A, O>;
impl<'a, const O: u8> MG3_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG3_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG3_A::_1)
    }
}
#[doc = "Field `MG4` reader - Rx Mailboxes Global Mask Bits"]
pub type MG4_R = crate::BitReader<MG4_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG4_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG4_A> for bool {
    #[inline(always)]
    fn from(variant: MG4_A) -> Self {
        variant as u8 != 0
    }
}
impl MG4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG4_A {
        match self.bits {
            false => MG4_A::_0,
            true => MG4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG4_A::_1
    }
}
#[doc = "Field `MG4` writer - Rx Mailboxes Global Mask Bits"]
pub type MG4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG4_A, O>;
impl<'a, const O: u8> MG4_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG4_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG4_A::_1)
    }
}
#[doc = "Field `MG5` reader - Rx Mailboxes Global Mask Bits"]
pub type MG5_R = crate::BitReader<MG5_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG5_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG5_A> for bool {
    #[inline(always)]
    fn from(variant: MG5_A) -> Self {
        variant as u8 != 0
    }
}
impl MG5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG5_A {
        match self.bits {
            false => MG5_A::_0,
            true => MG5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG5_A::_1
    }
}
#[doc = "Field `MG5` writer - Rx Mailboxes Global Mask Bits"]
pub type MG5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG5_A, O>;
impl<'a, const O: u8> MG5_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG5_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG5_A::_1)
    }
}
#[doc = "Field `MG6` reader - Rx Mailboxes Global Mask Bits"]
pub type MG6_R = crate::BitReader<MG6_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG6_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG6_A> for bool {
    #[inline(always)]
    fn from(variant: MG6_A) -> Self {
        variant as u8 != 0
    }
}
impl MG6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG6_A {
        match self.bits {
            false => MG6_A::_0,
            true => MG6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG6_A::_1
    }
}
#[doc = "Field `MG6` writer - Rx Mailboxes Global Mask Bits"]
pub type MG6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG6_A, O>;
impl<'a, const O: u8> MG6_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG6_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG6_A::_1)
    }
}
#[doc = "Field `MG7` reader - Rx Mailboxes Global Mask Bits"]
pub type MG7_R = crate::BitReader<MG7_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG7_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG7_A> for bool {
    #[inline(always)]
    fn from(variant: MG7_A) -> Self {
        variant as u8 != 0
    }
}
impl MG7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG7_A {
        match self.bits {
            false => MG7_A::_0,
            true => MG7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG7_A::_1
    }
}
#[doc = "Field `MG7` writer - Rx Mailboxes Global Mask Bits"]
pub type MG7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG7_A, O>;
impl<'a, const O: u8> MG7_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG7_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG7_A::_1)
    }
}
#[doc = "Field `MG8` reader - Rx Mailboxes Global Mask Bits"]
pub type MG8_R = crate::BitReader<MG8_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG8_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG8_A> for bool {
    #[inline(always)]
    fn from(variant: MG8_A) -> Self {
        variant as u8 != 0
    }
}
impl MG8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG8_A {
        match self.bits {
            false => MG8_A::_0,
            true => MG8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG8_A::_1
    }
}
#[doc = "Field `MG8` writer - Rx Mailboxes Global Mask Bits"]
pub type MG8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG8_A, O>;
impl<'a, const O: u8> MG8_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG8_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG8_A::_1)
    }
}
#[doc = "Field `MG9` reader - Rx Mailboxes Global Mask Bits"]
pub type MG9_R = crate::BitReader<MG9_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG9_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG9_A> for bool {
    #[inline(always)]
    fn from(variant: MG9_A) -> Self {
        variant as u8 != 0
    }
}
impl MG9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG9_A {
        match self.bits {
            false => MG9_A::_0,
            true => MG9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG9_A::_1
    }
}
#[doc = "Field `MG9` writer - Rx Mailboxes Global Mask Bits"]
pub type MG9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG9_A, O>;
impl<'a, const O: u8> MG9_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG9_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG9_A::_1)
    }
}
#[doc = "Field `MG10` reader - Rx Mailboxes Global Mask Bits"]
pub type MG10_R = crate::BitReader<MG10_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG10_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG10_A> for bool {
    #[inline(always)]
    fn from(variant: MG10_A) -> Self {
        variant as u8 != 0
    }
}
impl MG10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG10_A {
        match self.bits {
            false => MG10_A::_0,
            true => MG10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG10_A::_1
    }
}
#[doc = "Field `MG10` writer - Rx Mailboxes Global Mask Bits"]
pub type MG10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG10_A, O>;
impl<'a, const O: u8> MG10_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG10_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG10_A::_1)
    }
}
#[doc = "Field `MG11` reader - Rx Mailboxes Global Mask Bits"]
pub type MG11_R = crate::BitReader<MG11_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG11_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG11_A> for bool {
    #[inline(always)]
    fn from(variant: MG11_A) -> Self {
        variant as u8 != 0
    }
}
impl MG11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG11_A {
        match self.bits {
            false => MG11_A::_0,
            true => MG11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG11_A::_1
    }
}
#[doc = "Field `MG11` writer - Rx Mailboxes Global Mask Bits"]
pub type MG11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG11_A, O>;
impl<'a, const O: u8> MG11_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG11_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG11_A::_1)
    }
}
#[doc = "Field `MG12` reader - Rx Mailboxes Global Mask Bits"]
pub type MG12_R = crate::BitReader<MG12_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG12_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG12_A> for bool {
    #[inline(always)]
    fn from(variant: MG12_A) -> Self {
        variant as u8 != 0
    }
}
impl MG12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG12_A {
        match self.bits {
            false => MG12_A::_0,
            true => MG12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG12_A::_1
    }
}
#[doc = "Field `MG12` writer - Rx Mailboxes Global Mask Bits"]
pub type MG12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG12_A, O>;
impl<'a, const O: u8> MG12_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG12_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG12_A::_1)
    }
}
#[doc = "Field `MG13` reader - Rx Mailboxes Global Mask Bits"]
pub type MG13_R = crate::BitReader<MG13_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG13_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG13_A> for bool {
    #[inline(always)]
    fn from(variant: MG13_A) -> Self {
        variant as u8 != 0
    }
}
impl MG13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG13_A {
        match self.bits {
            false => MG13_A::_0,
            true => MG13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG13_A::_1
    }
}
#[doc = "Field `MG13` writer - Rx Mailboxes Global Mask Bits"]
pub type MG13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG13_A, O>;
impl<'a, const O: u8> MG13_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG13_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG13_A::_1)
    }
}
#[doc = "Field `MG14` reader - Rx Mailboxes Global Mask Bits"]
pub type MG14_R = crate::BitReader<MG14_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG14_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG14_A> for bool {
    #[inline(always)]
    fn from(variant: MG14_A) -> Self {
        variant as u8 != 0
    }
}
impl MG14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG14_A {
        match self.bits {
            false => MG14_A::_0,
            true => MG14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG14_A::_1
    }
}
#[doc = "Field `MG14` writer - Rx Mailboxes Global Mask Bits"]
pub type MG14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG14_A, O>;
impl<'a, const O: u8> MG14_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG14_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG14_A::_1)
    }
}
#[doc = "Field `MG15` reader - Rx Mailboxes Global Mask Bits"]
pub type MG15_R = crate::BitReader<MG15_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG15_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG15_A> for bool {
    #[inline(always)]
    fn from(variant: MG15_A) -> Self {
        variant as u8 != 0
    }
}
impl MG15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG15_A {
        match self.bits {
            false => MG15_A::_0,
            true => MG15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG15_A::_1
    }
}
#[doc = "Field `MG15` writer - Rx Mailboxes Global Mask Bits"]
pub type MG15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG15_A, O>;
impl<'a, const O: u8> MG15_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG15_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG15_A::_1)
    }
}
#[doc = "Field `MG16` reader - Rx Mailboxes Global Mask Bits"]
pub type MG16_R = crate::BitReader<MG16_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG16_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG16_A> for bool {
    #[inline(always)]
    fn from(variant: MG16_A) -> Self {
        variant as u8 != 0
    }
}
impl MG16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG16_A {
        match self.bits {
            false => MG16_A::_0,
            true => MG16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG16_A::_1
    }
}
#[doc = "Field `MG16` writer - Rx Mailboxes Global Mask Bits"]
pub type MG16_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG16_A, O>;
impl<'a, const O: u8> MG16_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG16_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG16_A::_1)
    }
}
#[doc = "Field `MG17` reader - Rx Mailboxes Global Mask Bits"]
pub type MG17_R = crate::BitReader<MG17_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG17_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG17_A> for bool {
    #[inline(always)]
    fn from(variant: MG17_A) -> Self {
        variant as u8 != 0
    }
}
impl MG17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG17_A {
        match self.bits {
            false => MG17_A::_0,
            true => MG17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG17_A::_1
    }
}
#[doc = "Field `MG17` writer - Rx Mailboxes Global Mask Bits"]
pub type MG17_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG17_A, O>;
impl<'a, const O: u8> MG17_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG17_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG17_A::_1)
    }
}
#[doc = "Field `MG18` reader - Rx Mailboxes Global Mask Bits"]
pub type MG18_R = crate::BitReader<MG18_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG18_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG18_A> for bool {
    #[inline(always)]
    fn from(variant: MG18_A) -> Self {
        variant as u8 != 0
    }
}
impl MG18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG18_A {
        match self.bits {
            false => MG18_A::_0,
            true => MG18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG18_A::_1
    }
}
#[doc = "Field `MG18` writer - Rx Mailboxes Global Mask Bits"]
pub type MG18_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG18_A, O>;
impl<'a, const O: u8> MG18_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG18_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG18_A::_1)
    }
}
#[doc = "Field `MG19` reader - Rx Mailboxes Global Mask Bits"]
pub type MG19_R = crate::BitReader<MG19_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG19_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG19_A> for bool {
    #[inline(always)]
    fn from(variant: MG19_A) -> Self {
        variant as u8 != 0
    }
}
impl MG19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG19_A {
        match self.bits {
            false => MG19_A::_0,
            true => MG19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG19_A::_1
    }
}
#[doc = "Field `MG19` writer - Rx Mailboxes Global Mask Bits"]
pub type MG19_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG19_A, O>;
impl<'a, const O: u8> MG19_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG19_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG19_A::_1)
    }
}
#[doc = "Field `MG20` reader - Rx Mailboxes Global Mask Bits"]
pub type MG20_R = crate::BitReader<MG20_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG20_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG20_A> for bool {
    #[inline(always)]
    fn from(variant: MG20_A) -> Self {
        variant as u8 != 0
    }
}
impl MG20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG20_A {
        match self.bits {
            false => MG20_A::_0,
            true => MG20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG20_A::_1
    }
}
#[doc = "Field `MG20` writer - Rx Mailboxes Global Mask Bits"]
pub type MG20_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG20_A, O>;
impl<'a, const O: u8> MG20_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG20_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG20_A::_1)
    }
}
#[doc = "Field `MG21` reader - Rx Mailboxes Global Mask Bits"]
pub type MG21_R = crate::BitReader<MG21_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG21_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG21_A> for bool {
    #[inline(always)]
    fn from(variant: MG21_A) -> Self {
        variant as u8 != 0
    }
}
impl MG21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG21_A {
        match self.bits {
            false => MG21_A::_0,
            true => MG21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG21_A::_1
    }
}
#[doc = "Field `MG21` writer - Rx Mailboxes Global Mask Bits"]
pub type MG21_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG21_A, O>;
impl<'a, const O: u8> MG21_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG21_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG21_A::_1)
    }
}
#[doc = "Field `MG22` reader - Rx Mailboxes Global Mask Bits"]
pub type MG22_R = crate::BitReader<MG22_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG22_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG22_A> for bool {
    #[inline(always)]
    fn from(variant: MG22_A) -> Self {
        variant as u8 != 0
    }
}
impl MG22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG22_A {
        match self.bits {
            false => MG22_A::_0,
            true => MG22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG22_A::_1
    }
}
#[doc = "Field `MG22` writer - Rx Mailboxes Global Mask Bits"]
pub type MG22_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG22_A, O>;
impl<'a, const O: u8> MG22_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG22_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG22_A::_1)
    }
}
#[doc = "Field `MG23` reader - Rx Mailboxes Global Mask Bits"]
pub type MG23_R = crate::BitReader<MG23_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG23_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG23_A> for bool {
    #[inline(always)]
    fn from(variant: MG23_A) -> Self {
        variant as u8 != 0
    }
}
impl MG23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG23_A {
        match self.bits {
            false => MG23_A::_0,
            true => MG23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG23_A::_1
    }
}
#[doc = "Field `MG23` writer - Rx Mailboxes Global Mask Bits"]
pub type MG23_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG23_A, O>;
impl<'a, const O: u8> MG23_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG23_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG23_A::_1)
    }
}
#[doc = "Field `MG24` reader - Rx Mailboxes Global Mask Bits"]
pub type MG24_R = crate::BitReader<MG24_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG24_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG24_A> for bool {
    #[inline(always)]
    fn from(variant: MG24_A) -> Self {
        variant as u8 != 0
    }
}
impl MG24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG24_A {
        match self.bits {
            false => MG24_A::_0,
            true => MG24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG24_A::_1
    }
}
#[doc = "Field `MG24` writer - Rx Mailboxes Global Mask Bits"]
pub type MG24_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG24_A, O>;
impl<'a, const O: u8> MG24_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG24_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG24_A::_1)
    }
}
#[doc = "Field `MG25` reader - Rx Mailboxes Global Mask Bits"]
pub type MG25_R = crate::BitReader<MG25_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG25_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG25_A> for bool {
    #[inline(always)]
    fn from(variant: MG25_A) -> Self {
        variant as u8 != 0
    }
}
impl MG25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG25_A {
        match self.bits {
            false => MG25_A::_0,
            true => MG25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG25_A::_1
    }
}
#[doc = "Field `MG25` writer - Rx Mailboxes Global Mask Bits"]
pub type MG25_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG25_A, O>;
impl<'a, const O: u8> MG25_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG25_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG25_A::_1)
    }
}
#[doc = "Field `MG26` reader - Rx Mailboxes Global Mask Bits"]
pub type MG26_R = crate::BitReader<MG26_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG26_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG26_A> for bool {
    #[inline(always)]
    fn from(variant: MG26_A) -> Self {
        variant as u8 != 0
    }
}
impl MG26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG26_A {
        match self.bits {
            false => MG26_A::_0,
            true => MG26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG26_A::_1
    }
}
#[doc = "Field `MG26` writer - Rx Mailboxes Global Mask Bits"]
pub type MG26_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG26_A, O>;
impl<'a, const O: u8> MG26_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG26_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG26_A::_1)
    }
}
#[doc = "Field `MG27` reader - Rx Mailboxes Global Mask Bits"]
pub type MG27_R = crate::BitReader<MG27_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG27_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG27_A> for bool {
    #[inline(always)]
    fn from(variant: MG27_A) -> Self {
        variant as u8 != 0
    }
}
impl MG27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG27_A {
        match self.bits {
            false => MG27_A::_0,
            true => MG27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG27_A::_1
    }
}
#[doc = "Field `MG27` writer - Rx Mailboxes Global Mask Bits"]
pub type MG27_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG27_A, O>;
impl<'a, const O: u8> MG27_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG27_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG27_A::_1)
    }
}
#[doc = "Field `MG28` reader - Rx Mailboxes Global Mask Bits"]
pub type MG28_R = crate::BitReader<MG28_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG28_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG28_A> for bool {
    #[inline(always)]
    fn from(variant: MG28_A) -> Self {
        variant as u8 != 0
    }
}
impl MG28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG28_A {
        match self.bits {
            false => MG28_A::_0,
            true => MG28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG28_A::_1
    }
}
#[doc = "Field `MG28` writer - Rx Mailboxes Global Mask Bits"]
pub type MG28_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG28_A, O>;
impl<'a, const O: u8> MG28_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG28_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG28_A::_1)
    }
}
#[doc = "Field `MG29` reader - Rx Mailboxes Global Mask Bits"]
pub type MG29_R = crate::BitReader<MG29_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG29_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG29_A> for bool {
    #[inline(always)]
    fn from(variant: MG29_A) -> Self {
        variant as u8 != 0
    }
}
impl MG29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG29_A {
        match self.bits {
            false => MG29_A::_0,
            true => MG29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG29_A::_1
    }
}
#[doc = "Field `MG29` writer - Rx Mailboxes Global Mask Bits"]
pub type MG29_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG29_A, O>;
impl<'a, const O: u8> MG29_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG29_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG29_A::_1)
    }
}
#[doc = "Field `MG30` reader - Rx Mailboxes Global Mask Bits"]
pub type MG30_R = crate::BitReader<MG30_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG30_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG30_A> for bool {
    #[inline(always)]
    fn from(variant: MG30_A) -> Self {
        variant as u8 != 0
    }
}
impl MG30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG30_A {
        match self.bits {
            false => MG30_A::_0,
            true => MG30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG30_A::_1
    }
}
#[doc = "Field `MG30` writer - Rx Mailboxes Global Mask Bits"]
pub type MG30_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG30_A, O>;
impl<'a, const O: u8> MG30_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG30_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG30_A::_1)
    }
}
#[doc = "Field `MG31` reader - Rx Mailboxes Global Mask Bits"]
pub type MG31_R = crate::BitReader<MG31_A>;
#[doc = "Rx Mailboxes Global Mask Bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MG31_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MG31_A> for bool {
    #[inline(always)]
    fn from(variant: MG31_A) -> Self {
        variant as u8 != 0
    }
}
impl MG31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MG31_A {
        match self.bits {
            false => MG31_A::_0,
            true => MG31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MG31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MG31_A::_1
    }
}
#[doc = "Field `MG31` writer - Rx Mailboxes Global Mask Bits"]
pub type MG31_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXMGMASK_SPEC, MG31_A, O>;
impl<'a, const O: u8> MG31_W<'a, O> {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MG31_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MG31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg0(&self) -> MG0_R {
        MG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg1(&self) -> MG1_R {
        MG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg2(&self) -> MG2_R {
        MG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg3(&self) -> MG3_R {
        MG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg4(&self) -> MG4_R {
        MG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg5(&self) -> MG5_R {
        MG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg6(&self) -> MG6_R {
        MG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg7(&self) -> MG7_R {
        MG7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg8(&self) -> MG8_R {
        MG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg9(&self) -> MG9_R {
        MG9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg10(&self) -> MG10_R {
        MG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg11(&self) -> MG11_R {
        MG11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg12(&self) -> MG12_R {
        MG12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg13(&self) -> MG13_R {
        MG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg14(&self) -> MG14_R {
        MG14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg15(&self) -> MG15_R {
        MG15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg16(&self) -> MG16_R {
        MG16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg17(&self) -> MG17_R {
        MG17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg18(&self) -> MG18_R {
        MG18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg19(&self) -> MG19_R {
        MG19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg20(&self) -> MG20_R {
        MG20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg21(&self) -> MG21_R {
        MG21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg22(&self) -> MG22_R {
        MG22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg23(&self) -> MG23_R {
        MG23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg24(&self) -> MG24_R {
        MG24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg25(&self) -> MG25_R {
        MG25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg26(&self) -> MG26_R {
        MG26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg27(&self) -> MG27_R {
        MG27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg28(&self) -> MG28_R {
        MG28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg29(&self) -> MG29_R {
        MG29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg30(&self) -> MG30_R {
        MG30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    pub fn mg31(&self) -> MG31_R {
        MG31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg0(&mut self) -> MG0_W<0> {
        MG0_W::new(self)
    }
    #[doc = "Bit 1 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg1(&mut self) -> MG1_W<1> {
        MG1_W::new(self)
    }
    #[doc = "Bit 2 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg2(&mut self) -> MG2_W<2> {
        MG2_W::new(self)
    }
    #[doc = "Bit 3 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg3(&mut self) -> MG3_W<3> {
        MG3_W::new(self)
    }
    #[doc = "Bit 4 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg4(&mut self) -> MG4_W<4> {
        MG4_W::new(self)
    }
    #[doc = "Bit 5 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg5(&mut self) -> MG5_W<5> {
        MG5_W::new(self)
    }
    #[doc = "Bit 6 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg6(&mut self) -> MG6_W<6> {
        MG6_W::new(self)
    }
    #[doc = "Bit 7 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg7(&mut self) -> MG7_W<7> {
        MG7_W::new(self)
    }
    #[doc = "Bit 8 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg8(&mut self) -> MG8_W<8> {
        MG8_W::new(self)
    }
    #[doc = "Bit 9 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg9(&mut self) -> MG9_W<9> {
        MG9_W::new(self)
    }
    #[doc = "Bit 10 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg10(&mut self) -> MG10_W<10> {
        MG10_W::new(self)
    }
    #[doc = "Bit 11 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg11(&mut self) -> MG11_W<11> {
        MG11_W::new(self)
    }
    #[doc = "Bit 12 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg12(&mut self) -> MG12_W<12> {
        MG12_W::new(self)
    }
    #[doc = "Bit 13 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg13(&mut self) -> MG13_W<13> {
        MG13_W::new(self)
    }
    #[doc = "Bit 14 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg14(&mut self) -> MG14_W<14> {
        MG14_W::new(self)
    }
    #[doc = "Bit 15 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg15(&mut self) -> MG15_W<15> {
        MG15_W::new(self)
    }
    #[doc = "Bit 16 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg16(&mut self) -> MG16_W<16> {
        MG16_W::new(self)
    }
    #[doc = "Bit 17 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg17(&mut self) -> MG17_W<17> {
        MG17_W::new(self)
    }
    #[doc = "Bit 18 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg18(&mut self) -> MG18_W<18> {
        MG18_W::new(self)
    }
    #[doc = "Bit 19 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg19(&mut self) -> MG19_W<19> {
        MG19_W::new(self)
    }
    #[doc = "Bit 20 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg20(&mut self) -> MG20_W<20> {
        MG20_W::new(self)
    }
    #[doc = "Bit 21 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg21(&mut self) -> MG21_W<21> {
        MG21_W::new(self)
    }
    #[doc = "Bit 22 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg22(&mut self) -> MG22_W<22> {
        MG22_W::new(self)
    }
    #[doc = "Bit 23 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg23(&mut self) -> MG23_W<23> {
        MG23_W::new(self)
    }
    #[doc = "Bit 24 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg24(&mut self) -> MG24_W<24> {
        MG24_W::new(self)
    }
    #[doc = "Bit 25 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg25(&mut self) -> MG25_W<25> {
        MG25_W::new(self)
    }
    #[doc = "Bit 26 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg26(&mut self) -> MG26_W<26> {
        MG26_W::new(self)
    }
    #[doc = "Bit 27 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg27(&mut self) -> MG27_W<27> {
        MG27_W::new(self)
    }
    #[doc = "Bit 28 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg28(&mut self) -> MG28_W<28> {
        MG28_W::new(self)
    }
    #[doc = "Bit 29 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg29(&mut self) -> MG29_W<29> {
        MG29_W::new(self)
    }
    #[doc = "Bit 30 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg30(&mut self) -> MG30_W<30> {
        MG30_W::new(self)
    }
    #[doc = "Bit 31 - Rx Mailboxes Global Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn mg31(&mut self) -> MG31_W<31> {
        MG31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Mailboxes Global Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmgmask](index.html) module"]
pub struct RXMGMASK_SPEC;
impl crate::RegisterSpec for RXMGMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmgmask::R](R) reader structure"]
impl crate::Readable for RXMGMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxmgmask::W](W) writer structure"]
impl crate::Writable for RXMGMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXMGMASK to value 0xffff_ffff"]
impl crate::Resettable for RXMGMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
