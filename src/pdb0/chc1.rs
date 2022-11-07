#[doc = "Register `CH%sC1` reader"]
pub struct R(crate::R<CHC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sC1` writer"]
pub struct W(crate::W<CHC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHC1_SPEC>;
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
impl From<crate::W<CHC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN0` reader - PDB Channel Pre-Trigger Enable"]
pub type EN0_R = crate::BitReader<EN0_A>;
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN0_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1 = 1,
}
impl From<EN0_A> for bool {
    #[inline(always)]
    fn from(variant: EN0_A) -> Self {
        variant as u8 != 0
    }
}
impl EN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN0_A {
        match self.bits {
            false => EN0_A::_0,
            true => EN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN0_A::_1
    }
}
#[doc = "Field `EN0` writer - PDB Channel Pre-Trigger Enable"]
pub type EN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, EN0_A, O>;
impl<'a, const O: u8> EN0_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN0_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN0_A::_1)
    }
}
#[doc = "Field `EN1` reader - PDB Channel Pre-Trigger Enable"]
pub type EN1_R = crate::BitReader<EN1_A>;
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1 = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
impl EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::_0,
            true => EN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN1_A::_1
    }
}
#[doc = "Field `EN1` writer - PDB Channel Pre-Trigger Enable"]
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, EN1_A, O>;
impl<'a, const O: u8> EN1_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN1_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN1_A::_1)
    }
}
#[doc = "Field `EN2` reader - PDB Channel Pre-Trigger Enable"]
pub type EN2_R = crate::BitReader<EN2_A>;
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN2_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1 = 1,
}
impl From<EN2_A> for bool {
    #[inline(always)]
    fn from(variant: EN2_A) -> Self {
        variant as u8 != 0
    }
}
impl EN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN2_A {
        match self.bits {
            false => EN2_A::_0,
            true => EN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN2_A::_1
    }
}
#[doc = "Field `EN2` writer - PDB Channel Pre-Trigger Enable"]
pub type EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, EN2_A, O>;
impl<'a, const O: u8> EN2_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN2_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN2_A::_1)
    }
}
#[doc = "Field `EN3` reader - PDB Channel Pre-Trigger Enable"]
pub type EN3_R = crate::BitReader<EN3_A>;
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN3_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1 = 1,
}
impl From<EN3_A> for bool {
    #[inline(always)]
    fn from(variant: EN3_A) -> Self {
        variant as u8 != 0
    }
}
impl EN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN3_A {
        match self.bits {
            false => EN3_A::_0,
            true => EN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN3_A::_1
    }
}
#[doc = "Field `EN3` writer - PDB Channel Pre-Trigger Enable"]
pub type EN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, EN3_A, O>;
impl<'a, const O: u8> EN3_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN3_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN3_A::_1)
    }
}
#[doc = "Field `EN4` reader - PDB Channel Pre-Trigger Enable"]
pub type EN4_R = crate::BitReader<EN4_A>;
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN4_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1 = 1,
}
impl From<EN4_A> for bool {
    #[inline(always)]
    fn from(variant: EN4_A) -> Self {
        variant as u8 != 0
    }
}
impl EN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN4_A {
        match self.bits {
            false => EN4_A::_0,
            true => EN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN4_A::_1
    }
}
#[doc = "Field `EN4` writer - PDB Channel Pre-Trigger Enable"]
pub type EN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, EN4_A, O>;
impl<'a, const O: u8> EN4_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN4_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN4_A::_1)
    }
}
#[doc = "Field `EN5` reader - PDB Channel Pre-Trigger Enable"]
pub type EN5_R = crate::BitReader<EN5_A>;
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN5_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1 = 1,
}
impl From<EN5_A> for bool {
    #[inline(always)]
    fn from(variant: EN5_A) -> Self {
        variant as u8 != 0
    }
}
impl EN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN5_A {
        match self.bits {
            false => EN5_A::_0,
            true => EN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN5_A::_1
    }
}
#[doc = "Field `EN5` writer - PDB Channel Pre-Trigger Enable"]
pub type EN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, EN5_A, O>;
impl<'a, const O: u8> EN5_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN5_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN5_A::_1)
    }
}
#[doc = "Field `EN6` reader - PDB Channel Pre-Trigger Enable"]
pub type EN6_R = crate::BitReader<EN6_A>;
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN6_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1 = 1,
}
impl From<EN6_A> for bool {
    #[inline(always)]
    fn from(variant: EN6_A) -> Self {
        variant as u8 != 0
    }
}
impl EN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN6_A {
        match self.bits {
            false => EN6_A::_0,
            true => EN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN6_A::_1
    }
}
#[doc = "Field `EN6` writer - PDB Channel Pre-Trigger Enable"]
pub type EN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, EN6_A, O>;
impl<'a, const O: u8> EN6_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN6_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN6_A::_1)
    }
}
#[doc = "Field `EN7` reader - PDB Channel Pre-Trigger Enable"]
pub type EN7_R = crate::BitReader<EN7_A>;
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN7_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1 = 1,
}
impl From<EN7_A> for bool {
    #[inline(always)]
    fn from(variant: EN7_A) -> Self {
        variant as u8 != 0
    }
}
impl EN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN7_A {
        match self.bits {
            false => EN7_A::_0,
            true => EN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN7_A::_1
    }
}
#[doc = "Field `EN7` writer - PDB Channel Pre-Trigger Enable"]
pub type EN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, EN7_A, O>;
impl<'a, const O: u8> EN7_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN7_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN7_A::_1)
    }
}
#[doc = "Field `TOS0` reader - PDB Channel Pre-Trigger Output Select"]
pub type TOS0_R = crate::BitReader<TOS0_A>;
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOS0_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1 = 1,
}
impl From<TOS0_A> for bool {
    #[inline(always)]
    fn from(variant: TOS0_A) -> Self {
        variant as u8 != 0
    }
}
impl TOS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS0_A {
        match self.bits {
            false => TOS0_A::_0,
            true => TOS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS0_A::_1
    }
}
#[doc = "Field `TOS0` writer - PDB Channel Pre-Trigger Output Select"]
pub type TOS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, TOS0_A, O>;
impl<'a, const O: u8> TOS0_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS0_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS0_A::_1)
    }
}
#[doc = "Field `TOS1` reader - PDB Channel Pre-Trigger Output Select"]
pub type TOS1_R = crate::BitReader<TOS1_A>;
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOS1_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1 = 1,
}
impl From<TOS1_A> for bool {
    #[inline(always)]
    fn from(variant: TOS1_A) -> Self {
        variant as u8 != 0
    }
}
impl TOS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS1_A {
        match self.bits {
            false => TOS1_A::_0,
            true => TOS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS1_A::_1
    }
}
#[doc = "Field `TOS1` writer - PDB Channel Pre-Trigger Output Select"]
pub type TOS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, TOS1_A, O>;
impl<'a, const O: u8> TOS1_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS1_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS1_A::_1)
    }
}
#[doc = "Field `TOS2` reader - PDB Channel Pre-Trigger Output Select"]
pub type TOS2_R = crate::BitReader<TOS2_A>;
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOS2_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1 = 1,
}
impl From<TOS2_A> for bool {
    #[inline(always)]
    fn from(variant: TOS2_A) -> Self {
        variant as u8 != 0
    }
}
impl TOS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS2_A {
        match self.bits {
            false => TOS2_A::_0,
            true => TOS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS2_A::_1
    }
}
#[doc = "Field `TOS2` writer - PDB Channel Pre-Trigger Output Select"]
pub type TOS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, TOS2_A, O>;
impl<'a, const O: u8> TOS2_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS2_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS2_A::_1)
    }
}
#[doc = "Field `TOS3` reader - PDB Channel Pre-Trigger Output Select"]
pub type TOS3_R = crate::BitReader<TOS3_A>;
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOS3_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1 = 1,
}
impl From<TOS3_A> for bool {
    #[inline(always)]
    fn from(variant: TOS3_A) -> Self {
        variant as u8 != 0
    }
}
impl TOS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS3_A {
        match self.bits {
            false => TOS3_A::_0,
            true => TOS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS3_A::_1
    }
}
#[doc = "Field `TOS3` writer - PDB Channel Pre-Trigger Output Select"]
pub type TOS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, TOS3_A, O>;
impl<'a, const O: u8> TOS3_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS3_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS3_A::_1)
    }
}
#[doc = "Field `TOS4` reader - PDB Channel Pre-Trigger Output Select"]
pub type TOS4_R = crate::BitReader<TOS4_A>;
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOS4_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1 = 1,
}
impl From<TOS4_A> for bool {
    #[inline(always)]
    fn from(variant: TOS4_A) -> Self {
        variant as u8 != 0
    }
}
impl TOS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS4_A {
        match self.bits {
            false => TOS4_A::_0,
            true => TOS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS4_A::_1
    }
}
#[doc = "Field `TOS4` writer - PDB Channel Pre-Trigger Output Select"]
pub type TOS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, TOS4_A, O>;
impl<'a, const O: u8> TOS4_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS4_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS4_A::_1)
    }
}
#[doc = "Field `TOS5` reader - PDB Channel Pre-Trigger Output Select"]
pub type TOS5_R = crate::BitReader<TOS5_A>;
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOS5_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1 = 1,
}
impl From<TOS5_A> for bool {
    #[inline(always)]
    fn from(variant: TOS5_A) -> Self {
        variant as u8 != 0
    }
}
impl TOS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS5_A {
        match self.bits {
            false => TOS5_A::_0,
            true => TOS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS5_A::_1
    }
}
#[doc = "Field `TOS5` writer - PDB Channel Pre-Trigger Output Select"]
pub type TOS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, TOS5_A, O>;
impl<'a, const O: u8> TOS5_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS5_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS5_A::_1)
    }
}
#[doc = "Field `TOS6` reader - PDB Channel Pre-Trigger Output Select"]
pub type TOS6_R = crate::BitReader<TOS6_A>;
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOS6_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1 = 1,
}
impl From<TOS6_A> for bool {
    #[inline(always)]
    fn from(variant: TOS6_A) -> Self {
        variant as u8 != 0
    }
}
impl TOS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS6_A {
        match self.bits {
            false => TOS6_A::_0,
            true => TOS6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS6_A::_1
    }
}
#[doc = "Field `TOS6` writer - PDB Channel Pre-Trigger Output Select"]
pub type TOS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, TOS6_A, O>;
impl<'a, const O: u8> TOS6_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS6_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS6_A::_1)
    }
}
#[doc = "Field `TOS7` reader - PDB Channel Pre-Trigger Output Select"]
pub type TOS7_R = crate::BitReader<TOS7_A>;
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOS7_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1 = 1,
}
impl From<TOS7_A> for bool {
    #[inline(always)]
    fn from(variant: TOS7_A) -> Self {
        variant as u8 != 0
    }
}
impl TOS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS7_A {
        match self.bits {
            false => TOS7_A::_0,
            true => TOS7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS7_A::_1
    }
}
#[doc = "Field `TOS7` writer - PDB Channel Pre-Trigger Output Select"]
pub type TOS7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, TOS7_A, O>;
impl<'a, const O: u8> TOS7_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS7_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS7_A::_1)
    }
}
#[doc = "Field `BB0` reader - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB0_R = crate::BitReader<BB0_A>;
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB0_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1 = 1,
}
impl From<BB0_A> for bool {
    #[inline(always)]
    fn from(variant: BB0_A) -> Self {
        variant as u8 != 0
    }
}
impl BB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB0_A {
        match self.bits {
            false => BB0_A::_0,
            true => BB0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB0_A::_1
    }
}
#[doc = "Field `BB0` writer - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, BB0_A, O>;
impl<'a, const O: u8> BB0_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB0_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB0_A::_1)
    }
}
#[doc = "Field `BB1` reader - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB1_R = crate::BitReader<BB1_A>;
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB1_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1 = 1,
}
impl From<BB1_A> for bool {
    #[inline(always)]
    fn from(variant: BB1_A) -> Self {
        variant as u8 != 0
    }
}
impl BB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB1_A {
        match self.bits {
            false => BB1_A::_0,
            true => BB1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB1_A::_1
    }
}
#[doc = "Field `BB1` writer - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, BB1_A, O>;
impl<'a, const O: u8> BB1_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB1_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB1_A::_1)
    }
}
#[doc = "Field `BB2` reader - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB2_R = crate::BitReader<BB2_A>;
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB2_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1 = 1,
}
impl From<BB2_A> for bool {
    #[inline(always)]
    fn from(variant: BB2_A) -> Self {
        variant as u8 != 0
    }
}
impl BB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB2_A {
        match self.bits {
            false => BB2_A::_0,
            true => BB2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB2_A::_1
    }
}
#[doc = "Field `BB2` writer - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, BB2_A, O>;
impl<'a, const O: u8> BB2_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB2_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB2_A::_1)
    }
}
#[doc = "Field `BB3` reader - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB3_R = crate::BitReader<BB3_A>;
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB3_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1 = 1,
}
impl From<BB3_A> for bool {
    #[inline(always)]
    fn from(variant: BB3_A) -> Self {
        variant as u8 != 0
    }
}
impl BB3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB3_A {
        match self.bits {
            false => BB3_A::_0,
            true => BB3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB3_A::_1
    }
}
#[doc = "Field `BB3` writer - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, BB3_A, O>;
impl<'a, const O: u8> BB3_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB3_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB3_A::_1)
    }
}
#[doc = "Field `BB4` reader - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB4_R = crate::BitReader<BB4_A>;
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB4_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1 = 1,
}
impl From<BB4_A> for bool {
    #[inline(always)]
    fn from(variant: BB4_A) -> Self {
        variant as u8 != 0
    }
}
impl BB4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB4_A {
        match self.bits {
            false => BB4_A::_0,
            true => BB4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB4_A::_1
    }
}
#[doc = "Field `BB4` writer - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, BB4_A, O>;
impl<'a, const O: u8> BB4_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB4_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB4_A::_1)
    }
}
#[doc = "Field `BB5` reader - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB5_R = crate::BitReader<BB5_A>;
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB5_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1 = 1,
}
impl From<BB5_A> for bool {
    #[inline(always)]
    fn from(variant: BB5_A) -> Self {
        variant as u8 != 0
    }
}
impl BB5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB5_A {
        match self.bits {
            false => BB5_A::_0,
            true => BB5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB5_A::_1
    }
}
#[doc = "Field `BB5` writer - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, BB5_A, O>;
impl<'a, const O: u8> BB5_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB5_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB5_A::_1)
    }
}
#[doc = "Field `BB6` reader - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB6_R = crate::BitReader<BB6_A>;
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB6_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1 = 1,
}
impl From<BB6_A> for bool {
    #[inline(always)]
    fn from(variant: BB6_A) -> Self {
        variant as u8 != 0
    }
}
impl BB6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB6_A {
        match self.bits {
            false => BB6_A::_0,
            true => BB6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB6_A::_1
    }
}
#[doc = "Field `BB6` writer - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, BB6_A, O>;
impl<'a, const O: u8> BB6_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB6_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB6_A::_1)
    }
}
#[doc = "Field `BB7` reader - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB7_R = crate::BitReader<BB7_A>;
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB7_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1 = 1,
}
impl From<BB7_A> for bool {
    #[inline(always)]
    fn from(variant: BB7_A) -> Self {
        variant as u8 != 0
    }
}
impl BB7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB7_A {
        match self.bits {
            false => BB7_A::_0,
            true => BB7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB7_A::_1
    }
}
#[doc = "Field `BB7` writer - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHC1_SPEC, BB7_A, O>;
impl<'a, const O: u8> BB7_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB7_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en0(&self) -> EN0_R {
        EN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en3(&self) -> EN3_R {
        EN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en4(&self) -> EN4_R {
        EN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en5(&self) -> EN5_R {
        EN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en6(&self) -> EN6_R {
        EN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en7(&self) -> EN7_R {
        EN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos0(&self) -> TOS0_R {
        TOS0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos1(&self) -> TOS1_R {
        TOS1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos2(&self) -> TOS2_R {
        TOS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos3(&self) -> TOS3_R {
        TOS3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos4(&self) -> TOS4_R {
        TOS4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos5(&self) -> TOS5_R {
        TOS5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos6(&self) -> TOS6_R {
        TOS6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos7(&self) -> TOS7_R {
        TOS7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb0(&self) -> BB0_R {
        BB0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb1(&self) -> BB1_R {
        BB1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb2(&self) -> BB2_R {
        BB2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb3(&self) -> BB3_R {
        BB3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb4(&self) -> BB4_R {
        BB4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb5(&self) -> BB5_R {
        BB5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb6(&self) -> BB6_R {
        BB6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb7(&self) -> BB7_R {
        BB7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en0(&mut self) -> EN0_W<0> {
        EN0_W::new(self)
    }
    #[doc = "Bit 1 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<1> {
        EN1_W::new(self)
    }
    #[doc = "Bit 2 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<2> {
        EN2_W::new(self)
    }
    #[doc = "Bit 3 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en3(&mut self) -> EN3_W<3> {
        EN3_W::new(self)
    }
    #[doc = "Bit 4 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en4(&mut self) -> EN4_W<4> {
        EN4_W::new(self)
    }
    #[doc = "Bit 5 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en5(&mut self) -> EN5_W<5> {
        EN5_W::new(self)
    }
    #[doc = "Bit 6 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en6(&mut self) -> EN6_W<6> {
        EN6_W::new(self)
    }
    #[doc = "Bit 7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en7(&mut self) -> EN7_W<7> {
        EN7_W::new(self)
    }
    #[doc = "Bit 8 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos0(&mut self) -> TOS0_W<8> {
        TOS0_W::new(self)
    }
    #[doc = "Bit 9 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos1(&mut self) -> TOS1_W<9> {
        TOS1_W::new(self)
    }
    #[doc = "Bit 10 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos2(&mut self) -> TOS2_W<10> {
        TOS2_W::new(self)
    }
    #[doc = "Bit 11 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos3(&mut self) -> TOS3_W<11> {
        TOS3_W::new(self)
    }
    #[doc = "Bit 12 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos4(&mut self) -> TOS4_W<12> {
        TOS4_W::new(self)
    }
    #[doc = "Bit 13 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos5(&mut self) -> TOS5_W<13> {
        TOS5_W::new(self)
    }
    #[doc = "Bit 14 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos6(&mut self) -> TOS6_W<14> {
        TOS6_W::new(self)
    }
    #[doc = "Bit 15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos7(&mut self) -> TOS7_W<15> {
        TOS7_W::new(self)
    }
    #[doc = "Bit 16 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bb0(&mut self) -> BB0_W<16> {
        BB0_W::new(self)
    }
    #[doc = "Bit 17 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bb1(&mut self) -> BB1_W<17> {
        BB1_W::new(self)
    }
    #[doc = "Bit 18 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bb2(&mut self) -> BB2_W<18> {
        BB2_W::new(self)
    }
    #[doc = "Bit 19 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bb3(&mut self) -> BB3_W<19> {
        BB3_W::new(self)
    }
    #[doc = "Bit 20 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bb4(&mut self) -> BB4_W<20> {
        BB4_W::new(self)
    }
    #[doc = "Bit 21 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bb5(&mut self) -> BB5_W<21> {
        BB5_W::new(self)
    }
    #[doc = "Bit 22 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bb6(&mut self) -> BB6_W<22> {
        BB6_W::new(self)
    }
    #[doc = "Bit 23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bb7(&mut self) -> BB7_W<23> {
        BB7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chc1](index.html) module"]
pub struct CHC1_SPEC;
impl crate::RegisterSpec for CHC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chc1::R](R) reader structure"]
impl crate::Readable for CHC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chc1::W](W) writer structure"]
impl crate::Writable for CHC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%sC1 to value 0"]
impl crate::Resettable for CHC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
