#[doc = "Register `PDDR` reader"]
pub struct R(crate::R<PDDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDR` writer"]
pub struct W(crate::W<PDDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDR_SPEC>;
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
impl From<crate::W<PDDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDD0` reader - Port Data Direction"]
pub type PDD0_R = crate::BitReader<PDD0_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD0_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD0_A> for bool {
    #[inline(always)]
    fn from(variant: PDD0_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD0_A {
        match self.bits {
            false => PDD0_A::_0,
            true => PDD0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD0_A::_1
    }
}
#[doc = "Field `PDD0` writer - Port Data Direction"]
pub type PDD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD0_A, O>;
impl<'a, const O: u8> PDD0_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD0_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD0_A::_1)
    }
}
#[doc = "Field `PDD1` reader - Port Data Direction"]
pub type PDD1_R = crate::BitReader<PDD1_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD1_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD1_A> for bool {
    #[inline(always)]
    fn from(variant: PDD1_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD1_A {
        match self.bits {
            false => PDD1_A::_0,
            true => PDD1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD1_A::_1
    }
}
#[doc = "Field `PDD1` writer - Port Data Direction"]
pub type PDD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD1_A, O>;
impl<'a, const O: u8> PDD1_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD1_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD1_A::_1)
    }
}
#[doc = "Field `PDD2` reader - Port Data Direction"]
pub type PDD2_R = crate::BitReader<PDD2_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD2_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD2_A> for bool {
    #[inline(always)]
    fn from(variant: PDD2_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD2_A {
        match self.bits {
            false => PDD2_A::_0,
            true => PDD2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD2_A::_1
    }
}
#[doc = "Field `PDD2` writer - Port Data Direction"]
pub type PDD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD2_A, O>;
impl<'a, const O: u8> PDD2_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD2_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD2_A::_1)
    }
}
#[doc = "Field `PDD3` reader - Port Data Direction"]
pub type PDD3_R = crate::BitReader<PDD3_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD3_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD3_A> for bool {
    #[inline(always)]
    fn from(variant: PDD3_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD3_A {
        match self.bits {
            false => PDD3_A::_0,
            true => PDD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD3_A::_1
    }
}
#[doc = "Field `PDD3` writer - Port Data Direction"]
pub type PDD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD3_A, O>;
impl<'a, const O: u8> PDD3_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD3_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD3_A::_1)
    }
}
#[doc = "Field `PDD4` reader - Port Data Direction"]
pub type PDD4_R = crate::BitReader<PDD4_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD4_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD4_A> for bool {
    #[inline(always)]
    fn from(variant: PDD4_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD4_A {
        match self.bits {
            false => PDD4_A::_0,
            true => PDD4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD4_A::_1
    }
}
#[doc = "Field `PDD4` writer - Port Data Direction"]
pub type PDD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD4_A, O>;
impl<'a, const O: u8> PDD4_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD4_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD4_A::_1)
    }
}
#[doc = "Field `PDD5` reader - Port Data Direction"]
pub type PDD5_R = crate::BitReader<PDD5_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD5_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD5_A> for bool {
    #[inline(always)]
    fn from(variant: PDD5_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD5_A {
        match self.bits {
            false => PDD5_A::_0,
            true => PDD5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD5_A::_1
    }
}
#[doc = "Field `PDD5` writer - Port Data Direction"]
pub type PDD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD5_A, O>;
impl<'a, const O: u8> PDD5_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD5_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD5_A::_1)
    }
}
#[doc = "Field `PDD6` reader - Port Data Direction"]
pub type PDD6_R = crate::BitReader<PDD6_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD6_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD6_A> for bool {
    #[inline(always)]
    fn from(variant: PDD6_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD6_A {
        match self.bits {
            false => PDD6_A::_0,
            true => PDD6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD6_A::_1
    }
}
#[doc = "Field `PDD6` writer - Port Data Direction"]
pub type PDD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD6_A, O>;
impl<'a, const O: u8> PDD6_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD6_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD6_A::_1)
    }
}
#[doc = "Field `PDD7` reader - Port Data Direction"]
pub type PDD7_R = crate::BitReader<PDD7_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD7_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD7_A> for bool {
    #[inline(always)]
    fn from(variant: PDD7_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD7_A {
        match self.bits {
            false => PDD7_A::_0,
            true => PDD7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD7_A::_1
    }
}
#[doc = "Field `PDD7` writer - Port Data Direction"]
pub type PDD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD7_A, O>;
impl<'a, const O: u8> PDD7_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD7_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD7_A::_1)
    }
}
#[doc = "Field `PDD8` reader - Port Data Direction"]
pub type PDD8_R = crate::BitReader<PDD8_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD8_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD8_A> for bool {
    #[inline(always)]
    fn from(variant: PDD8_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD8_A {
        match self.bits {
            false => PDD8_A::_0,
            true => PDD8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD8_A::_1
    }
}
#[doc = "Field `PDD8` writer - Port Data Direction"]
pub type PDD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD8_A, O>;
impl<'a, const O: u8> PDD8_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD8_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD8_A::_1)
    }
}
#[doc = "Field `PDD9` reader - Port Data Direction"]
pub type PDD9_R = crate::BitReader<PDD9_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD9_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD9_A> for bool {
    #[inline(always)]
    fn from(variant: PDD9_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD9_A {
        match self.bits {
            false => PDD9_A::_0,
            true => PDD9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD9_A::_1
    }
}
#[doc = "Field `PDD9` writer - Port Data Direction"]
pub type PDD9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD9_A, O>;
impl<'a, const O: u8> PDD9_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD9_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD9_A::_1)
    }
}
#[doc = "Field `PDD10` reader - Port Data Direction"]
pub type PDD10_R = crate::BitReader<PDD10_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD10_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD10_A> for bool {
    #[inline(always)]
    fn from(variant: PDD10_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD10_A {
        match self.bits {
            false => PDD10_A::_0,
            true => PDD10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD10_A::_1
    }
}
#[doc = "Field `PDD10` writer - Port Data Direction"]
pub type PDD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD10_A, O>;
impl<'a, const O: u8> PDD10_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD10_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD10_A::_1)
    }
}
#[doc = "Field `PDD11` reader - Port Data Direction"]
pub type PDD11_R = crate::BitReader<PDD11_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD11_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD11_A> for bool {
    #[inline(always)]
    fn from(variant: PDD11_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD11_A {
        match self.bits {
            false => PDD11_A::_0,
            true => PDD11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD11_A::_1
    }
}
#[doc = "Field `PDD11` writer - Port Data Direction"]
pub type PDD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD11_A, O>;
impl<'a, const O: u8> PDD11_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD11_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD11_A::_1)
    }
}
#[doc = "Field `PDD12` reader - Port Data Direction"]
pub type PDD12_R = crate::BitReader<PDD12_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD12_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD12_A> for bool {
    #[inline(always)]
    fn from(variant: PDD12_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD12_A {
        match self.bits {
            false => PDD12_A::_0,
            true => PDD12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD12_A::_1
    }
}
#[doc = "Field `PDD12` writer - Port Data Direction"]
pub type PDD12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD12_A, O>;
impl<'a, const O: u8> PDD12_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD12_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD12_A::_1)
    }
}
#[doc = "Field `PDD13` reader - Port Data Direction"]
pub type PDD13_R = crate::BitReader<PDD13_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD13_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD13_A> for bool {
    #[inline(always)]
    fn from(variant: PDD13_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD13_A {
        match self.bits {
            false => PDD13_A::_0,
            true => PDD13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD13_A::_1
    }
}
#[doc = "Field `PDD13` writer - Port Data Direction"]
pub type PDD13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD13_A, O>;
impl<'a, const O: u8> PDD13_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD13_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD13_A::_1)
    }
}
#[doc = "Field `PDD14` reader - Port Data Direction"]
pub type PDD14_R = crate::BitReader<PDD14_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD14_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD14_A> for bool {
    #[inline(always)]
    fn from(variant: PDD14_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD14_A {
        match self.bits {
            false => PDD14_A::_0,
            true => PDD14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD14_A::_1
    }
}
#[doc = "Field `PDD14` writer - Port Data Direction"]
pub type PDD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD14_A, O>;
impl<'a, const O: u8> PDD14_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD14_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD14_A::_1)
    }
}
#[doc = "Field `PDD15` reader - Port Data Direction"]
pub type PDD15_R = crate::BitReader<PDD15_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD15_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD15_A> for bool {
    #[inline(always)]
    fn from(variant: PDD15_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD15_A {
        match self.bits {
            false => PDD15_A::_0,
            true => PDD15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD15_A::_1
    }
}
#[doc = "Field `PDD15` writer - Port Data Direction"]
pub type PDD15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD15_A, O>;
impl<'a, const O: u8> PDD15_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD15_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD15_A::_1)
    }
}
#[doc = "Field `PDD16` reader - Port Data Direction"]
pub type PDD16_R = crate::BitReader<PDD16_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD16_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD16_A> for bool {
    #[inline(always)]
    fn from(variant: PDD16_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD16_A {
        match self.bits {
            false => PDD16_A::_0,
            true => PDD16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD16_A::_1
    }
}
#[doc = "Field `PDD16` writer - Port Data Direction"]
pub type PDD16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD16_A, O>;
impl<'a, const O: u8> PDD16_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD16_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD16_A::_1)
    }
}
#[doc = "Field `PDD17` reader - Port Data Direction"]
pub type PDD17_R = crate::BitReader<PDD17_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD17_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD17_A> for bool {
    #[inline(always)]
    fn from(variant: PDD17_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD17_A {
        match self.bits {
            false => PDD17_A::_0,
            true => PDD17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD17_A::_1
    }
}
#[doc = "Field `PDD17` writer - Port Data Direction"]
pub type PDD17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD17_A, O>;
impl<'a, const O: u8> PDD17_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD17_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD17_A::_1)
    }
}
#[doc = "Field `PDD18` reader - Port Data Direction"]
pub type PDD18_R = crate::BitReader<PDD18_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD18_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD18_A> for bool {
    #[inline(always)]
    fn from(variant: PDD18_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD18_A {
        match self.bits {
            false => PDD18_A::_0,
            true => PDD18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD18_A::_1
    }
}
#[doc = "Field `PDD18` writer - Port Data Direction"]
pub type PDD18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD18_A, O>;
impl<'a, const O: u8> PDD18_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD18_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD18_A::_1)
    }
}
#[doc = "Field `PDD19` reader - Port Data Direction"]
pub type PDD19_R = crate::BitReader<PDD19_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD19_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD19_A> for bool {
    #[inline(always)]
    fn from(variant: PDD19_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD19_A {
        match self.bits {
            false => PDD19_A::_0,
            true => PDD19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD19_A::_1
    }
}
#[doc = "Field `PDD19` writer - Port Data Direction"]
pub type PDD19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD19_A, O>;
impl<'a, const O: u8> PDD19_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD19_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD19_A::_1)
    }
}
#[doc = "Field `PDD20` reader - Port Data Direction"]
pub type PDD20_R = crate::BitReader<PDD20_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD20_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD20_A> for bool {
    #[inline(always)]
    fn from(variant: PDD20_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD20_A {
        match self.bits {
            false => PDD20_A::_0,
            true => PDD20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD20_A::_1
    }
}
#[doc = "Field `PDD20` writer - Port Data Direction"]
pub type PDD20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD20_A, O>;
impl<'a, const O: u8> PDD20_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD20_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD20_A::_1)
    }
}
#[doc = "Field `PDD21` reader - Port Data Direction"]
pub type PDD21_R = crate::BitReader<PDD21_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD21_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD21_A> for bool {
    #[inline(always)]
    fn from(variant: PDD21_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD21_A {
        match self.bits {
            false => PDD21_A::_0,
            true => PDD21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD21_A::_1
    }
}
#[doc = "Field `PDD21` writer - Port Data Direction"]
pub type PDD21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD21_A, O>;
impl<'a, const O: u8> PDD21_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD21_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD21_A::_1)
    }
}
#[doc = "Field `PDD22` reader - Port Data Direction"]
pub type PDD22_R = crate::BitReader<PDD22_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD22_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD22_A> for bool {
    #[inline(always)]
    fn from(variant: PDD22_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD22_A {
        match self.bits {
            false => PDD22_A::_0,
            true => PDD22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD22_A::_1
    }
}
#[doc = "Field `PDD22` writer - Port Data Direction"]
pub type PDD22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD22_A, O>;
impl<'a, const O: u8> PDD22_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD22_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD22_A::_1)
    }
}
#[doc = "Field `PDD23` reader - Port Data Direction"]
pub type PDD23_R = crate::BitReader<PDD23_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD23_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD23_A> for bool {
    #[inline(always)]
    fn from(variant: PDD23_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD23_A {
        match self.bits {
            false => PDD23_A::_0,
            true => PDD23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD23_A::_1
    }
}
#[doc = "Field `PDD23` writer - Port Data Direction"]
pub type PDD23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD23_A, O>;
impl<'a, const O: u8> PDD23_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD23_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD23_A::_1)
    }
}
#[doc = "Field `PDD24` reader - Port Data Direction"]
pub type PDD24_R = crate::BitReader<PDD24_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD24_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD24_A> for bool {
    #[inline(always)]
    fn from(variant: PDD24_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD24_A {
        match self.bits {
            false => PDD24_A::_0,
            true => PDD24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD24_A::_1
    }
}
#[doc = "Field `PDD24` writer - Port Data Direction"]
pub type PDD24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD24_A, O>;
impl<'a, const O: u8> PDD24_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD24_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD24_A::_1)
    }
}
#[doc = "Field `PDD25` reader - Port Data Direction"]
pub type PDD25_R = crate::BitReader<PDD25_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD25_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD25_A> for bool {
    #[inline(always)]
    fn from(variant: PDD25_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD25_A {
        match self.bits {
            false => PDD25_A::_0,
            true => PDD25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD25_A::_1
    }
}
#[doc = "Field `PDD25` writer - Port Data Direction"]
pub type PDD25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD25_A, O>;
impl<'a, const O: u8> PDD25_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD25_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD25_A::_1)
    }
}
#[doc = "Field `PDD26` reader - Port Data Direction"]
pub type PDD26_R = crate::BitReader<PDD26_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD26_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD26_A> for bool {
    #[inline(always)]
    fn from(variant: PDD26_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD26_A {
        match self.bits {
            false => PDD26_A::_0,
            true => PDD26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD26_A::_1
    }
}
#[doc = "Field `PDD26` writer - Port Data Direction"]
pub type PDD26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD26_A, O>;
impl<'a, const O: u8> PDD26_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD26_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD26_A::_1)
    }
}
#[doc = "Field `PDD27` reader - Port Data Direction"]
pub type PDD27_R = crate::BitReader<PDD27_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD27_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD27_A> for bool {
    #[inline(always)]
    fn from(variant: PDD27_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD27_A {
        match self.bits {
            false => PDD27_A::_0,
            true => PDD27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD27_A::_1
    }
}
#[doc = "Field `PDD27` writer - Port Data Direction"]
pub type PDD27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD27_A, O>;
impl<'a, const O: u8> PDD27_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD27_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD27_A::_1)
    }
}
#[doc = "Field `PDD28` reader - Port Data Direction"]
pub type PDD28_R = crate::BitReader<PDD28_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD28_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD28_A> for bool {
    #[inline(always)]
    fn from(variant: PDD28_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD28_A {
        match self.bits {
            false => PDD28_A::_0,
            true => PDD28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD28_A::_1
    }
}
#[doc = "Field `PDD28` writer - Port Data Direction"]
pub type PDD28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD28_A, O>;
impl<'a, const O: u8> PDD28_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD28_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD28_A::_1)
    }
}
#[doc = "Field `PDD29` reader - Port Data Direction"]
pub type PDD29_R = crate::BitReader<PDD29_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD29_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD29_A> for bool {
    #[inline(always)]
    fn from(variant: PDD29_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD29_A {
        match self.bits {
            false => PDD29_A::_0,
            true => PDD29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD29_A::_1
    }
}
#[doc = "Field `PDD29` writer - Port Data Direction"]
pub type PDD29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD29_A, O>;
impl<'a, const O: u8> PDD29_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD29_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD29_A::_1)
    }
}
#[doc = "Field `PDD30` reader - Port Data Direction"]
pub type PDD30_R = crate::BitReader<PDD30_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD30_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD30_A> for bool {
    #[inline(always)]
    fn from(variant: PDD30_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD30_A {
        match self.bits {
            false => PDD30_A::_0,
            true => PDD30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD30_A::_1
    }
}
#[doc = "Field `PDD30` writer - Port Data Direction"]
pub type PDD30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD30_A, O>;
impl<'a, const O: u8> PDD30_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD30_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD30_A::_1)
    }
}
#[doc = "Field `PDD31` reader - Port Data Direction"]
pub type PDD31_R = crate::BitReader<PDD31_A>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDD31_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1 = 1,
}
impl From<PDD31_A> for bool {
    #[inline(always)]
    fn from(variant: PDD31_A) -> Self {
        variant as u8 != 0
    }
}
impl PDD31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD31_A {
        match self.bits {
            false => PDD31_A::_0,
            true => PDD31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD31_A::_1
    }
}
#[doc = "Field `PDD31` writer - Port Data Direction"]
pub type PDD31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDDR_SPEC, PDD31_A, O>;
impl<'a, const O: u8> PDD31_W<'a, O> {
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD31_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd0(&self) -> PDD0_R {
        PDD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd1(&self) -> PDD1_R {
        PDD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd2(&self) -> PDD2_R {
        PDD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd3(&self) -> PDD3_R {
        PDD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd4(&self) -> PDD4_R {
        PDD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd5(&self) -> PDD5_R {
        PDD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd6(&self) -> PDD6_R {
        PDD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd7(&self) -> PDD7_R {
        PDD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd8(&self) -> PDD8_R {
        PDD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd9(&self) -> PDD9_R {
        PDD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd10(&self) -> PDD10_R {
        PDD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd11(&self) -> PDD11_R {
        PDD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd12(&self) -> PDD12_R {
        PDD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd13(&self) -> PDD13_R {
        PDD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd14(&self) -> PDD14_R {
        PDD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd15(&self) -> PDD15_R {
        PDD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd16(&self) -> PDD16_R {
        PDD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd17(&self) -> PDD17_R {
        PDD17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd18(&self) -> PDD18_R {
        PDD18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd19(&self) -> PDD19_R {
        PDD19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd20(&self) -> PDD20_R {
        PDD20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd21(&self) -> PDD21_R {
        PDD21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd22(&self) -> PDD22_R {
        PDD22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd23(&self) -> PDD23_R {
        PDD23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd24(&self) -> PDD24_R {
        PDD24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd25(&self) -> PDD25_R {
        PDD25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd26(&self) -> PDD26_R {
        PDD26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd27(&self) -> PDD27_R {
        PDD27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd28(&self) -> PDD28_R {
        PDD28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd29(&self) -> PDD29_R {
        PDD29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd30(&self) -> PDD30_R {
        PDD30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd31(&self) -> PDD31_R {
        PDD31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd0(&mut self) -> PDD0_W<0> {
        PDD0_W::new(self)
    }
    #[doc = "Bit 1 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd1(&mut self) -> PDD1_W<1> {
        PDD1_W::new(self)
    }
    #[doc = "Bit 2 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd2(&mut self) -> PDD2_W<2> {
        PDD2_W::new(self)
    }
    #[doc = "Bit 3 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd3(&mut self) -> PDD3_W<3> {
        PDD3_W::new(self)
    }
    #[doc = "Bit 4 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd4(&mut self) -> PDD4_W<4> {
        PDD4_W::new(self)
    }
    #[doc = "Bit 5 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd5(&mut self) -> PDD5_W<5> {
        PDD5_W::new(self)
    }
    #[doc = "Bit 6 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd6(&mut self) -> PDD6_W<6> {
        PDD6_W::new(self)
    }
    #[doc = "Bit 7 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd7(&mut self) -> PDD7_W<7> {
        PDD7_W::new(self)
    }
    #[doc = "Bit 8 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd8(&mut self) -> PDD8_W<8> {
        PDD8_W::new(self)
    }
    #[doc = "Bit 9 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd9(&mut self) -> PDD9_W<9> {
        PDD9_W::new(self)
    }
    #[doc = "Bit 10 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd10(&mut self) -> PDD10_W<10> {
        PDD10_W::new(self)
    }
    #[doc = "Bit 11 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd11(&mut self) -> PDD11_W<11> {
        PDD11_W::new(self)
    }
    #[doc = "Bit 12 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd12(&mut self) -> PDD12_W<12> {
        PDD12_W::new(self)
    }
    #[doc = "Bit 13 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd13(&mut self) -> PDD13_W<13> {
        PDD13_W::new(self)
    }
    #[doc = "Bit 14 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd14(&mut self) -> PDD14_W<14> {
        PDD14_W::new(self)
    }
    #[doc = "Bit 15 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd15(&mut self) -> PDD15_W<15> {
        PDD15_W::new(self)
    }
    #[doc = "Bit 16 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd16(&mut self) -> PDD16_W<16> {
        PDD16_W::new(self)
    }
    #[doc = "Bit 17 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd17(&mut self) -> PDD17_W<17> {
        PDD17_W::new(self)
    }
    #[doc = "Bit 18 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd18(&mut self) -> PDD18_W<18> {
        PDD18_W::new(self)
    }
    #[doc = "Bit 19 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd19(&mut self) -> PDD19_W<19> {
        PDD19_W::new(self)
    }
    #[doc = "Bit 20 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd20(&mut self) -> PDD20_W<20> {
        PDD20_W::new(self)
    }
    #[doc = "Bit 21 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd21(&mut self) -> PDD21_W<21> {
        PDD21_W::new(self)
    }
    #[doc = "Bit 22 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd22(&mut self) -> PDD22_W<22> {
        PDD22_W::new(self)
    }
    #[doc = "Bit 23 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd23(&mut self) -> PDD23_W<23> {
        PDD23_W::new(self)
    }
    #[doc = "Bit 24 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd24(&mut self) -> PDD24_W<24> {
        PDD24_W::new(self)
    }
    #[doc = "Bit 25 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd25(&mut self) -> PDD25_W<25> {
        PDD25_W::new(self)
    }
    #[doc = "Bit 26 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd26(&mut self) -> PDD26_W<26> {
        PDD26_W::new(self)
    }
    #[doc = "Bit 27 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd27(&mut self) -> PDD27_W<27> {
        PDD27_W::new(self)
    }
    #[doc = "Bit 28 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd28(&mut self) -> PDD28_W<28> {
        PDD28_W::new(self)
    }
    #[doc = "Bit 29 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd29(&mut self) -> PDD29_W<29> {
        PDD29_W::new(self)
    }
    #[doc = "Bit 30 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd30(&mut self) -> PDD30_W<30> {
        PDD30_W::new(self)
    }
    #[doc = "Bit 31 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd31(&mut self) -> PDD31_W<31> {
        PDD31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Data Direction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddr](index.html) module"]
pub struct PDDR_SPEC;
impl crate::RegisterSpec for PDDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddr::R](R) reader structure"]
impl crate::Readable for PDDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddr::W](W) writer structure"]
impl crate::Writable for PDDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDR to value 0"]
impl crate::Resettable for PDDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
