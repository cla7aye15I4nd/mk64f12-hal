#[doc = "Register `MPRA` reader"]
pub struct R(crate::R<MPRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPRA` writer"]
pub struct W(crate::W<MPRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPRA_SPEC>;
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
impl From<crate::W<MPRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPL5` reader - Master 5 Privilege Level"]
pub type MPL5_R = crate::BitReader<MPL5_A>;
#[doc = "Master 5 Privilege Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPL5_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL5_A> for bool {
    #[inline(always)]
    fn from(variant: MPL5_A) -> Self {
        variant as u8 != 0
    }
}
impl MPL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL5_A {
        match self.bits {
            false => MPL5_A::_0,
            true => MPL5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL5_A::_1
    }
}
#[doc = "Field `MPL5` writer - Master 5 Privilege Level"]
pub type MPL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MPL5_A, O>;
impl<'a, const O: u8> MPL5_W<'a, O> {
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL5_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL5_A::_1)
    }
}
#[doc = "Field `MTW5` reader - Master 5 Trusted For Writes"]
pub type MTW5_R = crate::BitReader<MTW5_A>;
#[doc = "Master 5 Trusted For Writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTW5_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW5_A> for bool {
    #[inline(always)]
    fn from(variant: MTW5_A) -> Self {
        variant as u8 != 0
    }
}
impl MTW5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW5_A {
        match self.bits {
            false => MTW5_A::_0,
            true => MTW5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW5_A::_1
    }
}
#[doc = "Field `MTW5` writer - Master 5 Trusted For Writes"]
pub type MTW5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTW5_A, O>;
impl<'a, const O: u8> MTW5_W<'a, O> {
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW5_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW5_A::_1)
    }
}
#[doc = "Field `MTR5` reader - Master 5 Trusted For Read"]
pub type MTR5_R = crate::BitReader<MTR5_A>;
#[doc = "Master 5 Trusted For Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTR5_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR5_A> for bool {
    #[inline(always)]
    fn from(variant: MTR5_A) -> Self {
        variant as u8 != 0
    }
}
impl MTR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR5_A {
        match self.bits {
            false => MTR5_A::_0,
            true => MTR5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR5_A::_1
    }
}
#[doc = "Field `MTR5` writer - Master 5 Trusted For Read"]
pub type MTR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTR5_A, O>;
impl<'a, const O: u8> MTR5_W<'a, O> {
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR5_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR5_A::_1)
    }
}
#[doc = "Field `MPL4` reader - Master 4 Privilege Level"]
pub type MPL4_R = crate::BitReader<MPL4_A>;
#[doc = "Master 4 Privilege Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPL4_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL4_A> for bool {
    #[inline(always)]
    fn from(variant: MPL4_A) -> Self {
        variant as u8 != 0
    }
}
impl MPL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL4_A {
        match self.bits {
            false => MPL4_A::_0,
            true => MPL4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL4_A::_1
    }
}
#[doc = "Field `MPL4` writer - Master 4 Privilege Level"]
pub type MPL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MPL4_A, O>;
impl<'a, const O: u8> MPL4_W<'a, O> {
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL4_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL4_A::_1)
    }
}
#[doc = "Field `MTW4` reader - Master 4 Trusted For Writes"]
pub type MTW4_R = crate::BitReader<MTW4_A>;
#[doc = "Master 4 Trusted For Writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTW4_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW4_A> for bool {
    #[inline(always)]
    fn from(variant: MTW4_A) -> Self {
        variant as u8 != 0
    }
}
impl MTW4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW4_A {
        match self.bits {
            false => MTW4_A::_0,
            true => MTW4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW4_A::_1
    }
}
#[doc = "Field `MTW4` writer - Master 4 Trusted For Writes"]
pub type MTW4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTW4_A, O>;
impl<'a, const O: u8> MTW4_W<'a, O> {
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW4_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW4_A::_1)
    }
}
#[doc = "Field `MTR4` reader - Master 4 Trusted For Read"]
pub type MTR4_R = crate::BitReader<MTR4_A>;
#[doc = "Master 4 Trusted For Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTR4_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR4_A> for bool {
    #[inline(always)]
    fn from(variant: MTR4_A) -> Self {
        variant as u8 != 0
    }
}
impl MTR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR4_A {
        match self.bits {
            false => MTR4_A::_0,
            true => MTR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR4_A::_1
    }
}
#[doc = "Field `MTR4` writer - Master 4 Trusted For Read"]
pub type MTR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTR4_A, O>;
impl<'a, const O: u8> MTR4_W<'a, O> {
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR4_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR4_A::_1)
    }
}
#[doc = "Field `MPL3` reader - Master 3 Privilege Level"]
pub type MPL3_R = crate::BitReader<MPL3_A>;
#[doc = "Master 3 Privilege Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPL3_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL3_A> for bool {
    #[inline(always)]
    fn from(variant: MPL3_A) -> Self {
        variant as u8 != 0
    }
}
impl MPL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL3_A {
        match self.bits {
            false => MPL3_A::_0,
            true => MPL3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL3_A::_1
    }
}
#[doc = "Field `MPL3` writer - Master 3 Privilege Level"]
pub type MPL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MPL3_A, O>;
impl<'a, const O: u8> MPL3_W<'a, O> {
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL3_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL3_A::_1)
    }
}
#[doc = "Field `MTW3` reader - Master 3 Trusted For Writes"]
pub type MTW3_R = crate::BitReader<MTW3_A>;
#[doc = "Master 3 Trusted For Writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTW3_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW3_A> for bool {
    #[inline(always)]
    fn from(variant: MTW3_A) -> Self {
        variant as u8 != 0
    }
}
impl MTW3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW3_A {
        match self.bits {
            false => MTW3_A::_0,
            true => MTW3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW3_A::_1
    }
}
#[doc = "Field `MTW3` writer - Master 3 Trusted For Writes"]
pub type MTW3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTW3_A, O>;
impl<'a, const O: u8> MTW3_W<'a, O> {
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW3_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW3_A::_1)
    }
}
#[doc = "Field `MTR3` reader - Master 3 Trusted For Read"]
pub type MTR3_R = crate::BitReader<MTR3_A>;
#[doc = "Master 3 Trusted For Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTR3_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR3_A> for bool {
    #[inline(always)]
    fn from(variant: MTR3_A) -> Self {
        variant as u8 != 0
    }
}
impl MTR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR3_A {
        match self.bits {
            false => MTR3_A::_0,
            true => MTR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR3_A::_1
    }
}
#[doc = "Field `MTR3` writer - Master 3 Trusted For Read"]
pub type MTR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTR3_A, O>;
impl<'a, const O: u8> MTR3_W<'a, O> {
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR3_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR3_A::_1)
    }
}
#[doc = "Field `MPL2` reader - Master 2 Privilege Level"]
pub type MPL2_R = crate::BitReader<MPL2_A>;
#[doc = "Master 2 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPL2_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL2_A> for bool {
    #[inline(always)]
    fn from(variant: MPL2_A) -> Self {
        variant as u8 != 0
    }
}
impl MPL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL2_A {
        match self.bits {
            false => MPL2_A::_0,
            true => MPL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL2_A::_1
    }
}
#[doc = "Field `MPL2` writer - Master 2 Privilege Level"]
pub type MPL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MPL2_A, O>;
impl<'a, const O: u8> MPL2_W<'a, O> {
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL2_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL2_A::_1)
    }
}
#[doc = "Field `MTW2` reader - Master 2 Trusted For Writes"]
pub type MTW2_R = crate::BitReader<MTW2_A>;
#[doc = "Master 2 Trusted For Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTW2_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW2_A> for bool {
    #[inline(always)]
    fn from(variant: MTW2_A) -> Self {
        variant as u8 != 0
    }
}
impl MTW2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW2_A {
        match self.bits {
            false => MTW2_A::_0,
            true => MTW2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW2_A::_1
    }
}
#[doc = "Field `MTW2` writer - Master 2 Trusted For Writes"]
pub type MTW2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTW2_A, O>;
impl<'a, const O: u8> MTW2_W<'a, O> {
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW2_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW2_A::_1)
    }
}
#[doc = "Field `MTR2` reader - Master 2 Trusted For Read"]
pub type MTR2_R = crate::BitReader<MTR2_A>;
#[doc = "Master 2 Trusted For Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTR2_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR2_A> for bool {
    #[inline(always)]
    fn from(variant: MTR2_A) -> Self {
        variant as u8 != 0
    }
}
impl MTR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR2_A {
        match self.bits {
            false => MTR2_A::_0,
            true => MTR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR2_A::_1
    }
}
#[doc = "Field `MTR2` writer - Master 2 Trusted For Read"]
pub type MTR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTR2_A, O>;
impl<'a, const O: u8> MTR2_W<'a, O> {
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR2_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR2_A::_1)
    }
}
#[doc = "Field `MPL1` reader - Master 1 Privilege Level"]
pub type MPL1_R = crate::BitReader<MPL1_A>;
#[doc = "Master 1 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPL1_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL1_A> for bool {
    #[inline(always)]
    fn from(variant: MPL1_A) -> Self {
        variant as u8 != 0
    }
}
impl MPL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL1_A {
        match self.bits {
            false => MPL1_A::_0,
            true => MPL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL1_A::_1
    }
}
#[doc = "Field `MPL1` writer - Master 1 Privilege Level"]
pub type MPL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MPL1_A, O>;
impl<'a, const O: u8> MPL1_W<'a, O> {
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL1_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL1_A::_1)
    }
}
#[doc = "Field `MTW1` reader - Master 1 Trusted for Writes"]
pub type MTW1_R = crate::BitReader<MTW1_A>;
#[doc = "Master 1 Trusted for Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTW1_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW1_A> for bool {
    #[inline(always)]
    fn from(variant: MTW1_A) -> Self {
        variant as u8 != 0
    }
}
impl MTW1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW1_A {
        match self.bits {
            false => MTW1_A::_0,
            true => MTW1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW1_A::_1
    }
}
#[doc = "Field `MTW1` writer - Master 1 Trusted for Writes"]
pub type MTW1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTW1_A, O>;
impl<'a, const O: u8> MTW1_W<'a, O> {
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW1_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW1_A::_1)
    }
}
#[doc = "Field `MTR1` reader - Master 1 Trusted for Read"]
pub type MTR1_R = crate::BitReader<MTR1_A>;
#[doc = "Master 1 Trusted for Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTR1_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR1_A> for bool {
    #[inline(always)]
    fn from(variant: MTR1_A) -> Self {
        variant as u8 != 0
    }
}
impl MTR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR1_A {
        match self.bits {
            false => MTR1_A::_0,
            true => MTR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR1_A::_1
    }
}
#[doc = "Field `MTR1` writer - Master 1 Trusted for Read"]
pub type MTR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTR1_A, O>;
impl<'a, const O: u8> MTR1_W<'a, O> {
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR1_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR1_A::_1)
    }
}
#[doc = "Field `MPL0` reader - Master 0 Privilege Level"]
pub type MPL0_R = crate::BitReader<MPL0_A>;
#[doc = "Master 0 Privilege Level\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPL0_A {
    #[doc = "0: Accesses from this master are forced to user-mode."]
    _0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode."]
    _1 = 1,
}
impl From<MPL0_A> for bool {
    #[inline(always)]
    fn from(variant: MPL0_A) -> Self {
        variant as u8 != 0
    }
}
impl MPL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPL0_A {
        match self.bits {
            false => MPL0_A::_0,
            true => MPL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPL0_A::_1
    }
}
#[doc = "Field `MPL0` writer - Master 0 Privilege Level"]
pub type MPL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MPL0_A, O>;
impl<'a, const O: u8> MPL0_W<'a, O> {
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL0_A::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL0_A::_1)
    }
}
#[doc = "Field `MTW0` reader - Master 0 Trusted For Writes"]
pub type MTW0_R = crate::BitReader<MTW0_A>;
#[doc = "Master 0 Trusted For Writes\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTW0_A {
    #[doc = "0: This master is not trusted for write accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for write accesses."]
    _1 = 1,
}
impl From<MTW0_A> for bool {
    #[inline(always)]
    fn from(variant: MTW0_A) -> Self {
        variant as u8 != 0
    }
}
impl MTW0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTW0_A {
        match self.bits {
            false => MTW0_A::_0,
            true => MTW0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTW0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTW0_A::_1
    }
}
#[doc = "Field `MTW0` writer - Master 0 Trusted For Writes"]
pub type MTW0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTW0_A, O>;
impl<'a, const O: u8> MTW0_W<'a, O> {
    #[doc = "This master is not trusted for write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW0_A::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW0_A::_1)
    }
}
#[doc = "Field `MTR0` reader - Master 0 Trusted For Read"]
pub type MTR0_R = crate::BitReader<MTR0_A>;
#[doc = "Master 0 Trusted For Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTR0_A {
    #[doc = "0: This master is not trusted for read accesses."]
    _0 = 0,
    #[doc = "1: This master is trusted for read accesses."]
    _1 = 1,
}
impl From<MTR0_A> for bool {
    #[inline(always)]
    fn from(variant: MTR0_A) -> Self {
        variant as u8 != 0
    }
}
impl MTR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTR0_A {
        match self.bits {
            false => MTR0_A::_0,
            true => MTR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTR0_A::_1
    }
}
#[doc = "Field `MTR0` writer - Master 0 Trusted For Read"]
pub type MTR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPRA_SPEC, MTR0_A, O>;
impl<'a, const O: u8> MTR0_W<'a, O> {
    #[doc = "This master is not trusted for read accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR0_A::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR0_A::_1)
    }
}
impl R {
    #[doc = "Bit 8 - Master 5 Privilege Level"]
    #[inline(always)]
    pub fn mpl5(&self) -> MPL5_R {
        MPL5_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master 5 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw5(&self) -> MTW5_R {
        MTW5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master 5 Trusted For Read"]
    #[inline(always)]
    pub fn mtr5(&self) -> MTR5_R {
        MTR5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Master 4 Privilege Level"]
    #[inline(always)]
    pub fn mpl4(&self) -> MPL4_R {
        MPL4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Master 4 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw4(&self) -> MTW4_R {
        MTW4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Master 4 Trusted For Read"]
    #[inline(always)]
    pub fn mtr4(&self) -> MTR4_R {
        MTR4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Master 3 Privilege Level"]
    #[inline(always)]
    pub fn mpl3(&self) -> MPL3_R {
        MPL3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Master 3 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw3(&self) -> MTW3_R {
        MTW3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Master 3 Trusted For Read"]
    #[inline(always)]
    pub fn mtr3(&self) -> MTR3_R {
        MTR3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline(always)]
    pub fn mpl2(&self) -> MPL2_R {
        MPL2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw2(&self) -> MTW2_R {
        MTW2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline(always)]
    pub fn mtr2(&self) -> MTR2_R {
        MTR2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline(always)]
    pub fn mpl1(&self) -> MPL1_R {
        MPL1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline(always)]
    pub fn mtw1(&self) -> MTW1_R {
        MTW1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline(always)]
    pub fn mtr1(&self) -> MTR1_R {
        MTR1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline(always)]
    pub fn mpl0(&self) -> MPL0_R {
        MPL0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline(always)]
    pub fn mtw0(&self) -> MTW0_R {
        MTW0_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline(always)]
    pub fn mtr0(&self) -> MTR0_R {
        MTR0_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Master 5 Privilege Level"]
    #[inline(always)]
    #[must_use]
    pub fn mpl5(&mut self) -> MPL5_W<8> {
        MPL5_W::new(self)
    }
    #[doc = "Bit 9 - Master 5 Trusted For Writes"]
    #[inline(always)]
    #[must_use]
    pub fn mtw5(&mut self) -> MTW5_W<9> {
        MTW5_W::new(self)
    }
    #[doc = "Bit 10 - Master 5 Trusted For Read"]
    #[inline(always)]
    #[must_use]
    pub fn mtr5(&mut self) -> MTR5_W<10> {
        MTR5_W::new(self)
    }
    #[doc = "Bit 12 - Master 4 Privilege Level"]
    #[inline(always)]
    #[must_use]
    pub fn mpl4(&mut self) -> MPL4_W<12> {
        MPL4_W::new(self)
    }
    #[doc = "Bit 13 - Master 4 Trusted For Writes"]
    #[inline(always)]
    #[must_use]
    pub fn mtw4(&mut self) -> MTW4_W<13> {
        MTW4_W::new(self)
    }
    #[doc = "Bit 14 - Master 4 Trusted For Read"]
    #[inline(always)]
    #[must_use]
    pub fn mtr4(&mut self) -> MTR4_W<14> {
        MTR4_W::new(self)
    }
    #[doc = "Bit 16 - Master 3 Privilege Level"]
    #[inline(always)]
    #[must_use]
    pub fn mpl3(&mut self) -> MPL3_W<16> {
        MPL3_W::new(self)
    }
    #[doc = "Bit 17 - Master 3 Trusted For Writes"]
    #[inline(always)]
    #[must_use]
    pub fn mtw3(&mut self) -> MTW3_W<17> {
        MTW3_W::new(self)
    }
    #[doc = "Bit 18 - Master 3 Trusted For Read"]
    #[inline(always)]
    #[must_use]
    pub fn mtr3(&mut self) -> MTR3_W<18> {
        MTR3_W::new(self)
    }
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline(always)]
    #[must_use]
    pub fn mpl2(&mut self) -> MPL2_W<20> {
        MPL2_W::new(self)
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline(always)]
    #[must_use]
    pub fn mtw2(&mut self) -> MTW2_W<21> {
        MTW2_W::new(self)
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline(always)]
    #[must_use]
    pub fn mtr2(&mut self) -> MTR2_W<22> {
        MTR2_W::new(self)
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline(always)]
    #[must_use]
    pub fn mpl1(&mut self) -> MPL1_W<24> {
        MPL1_W::new(self)
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline(always)]
    #[must_use]
    pub fn mtw1(&mut self) -> MTW1_W<25> {
        MTW1_W::new(self)
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline(always)]
    #[must_use]
    pub fn mtr1(&mut self) -> MTR1_W<26> {
        MTR1_W::new(self)
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline(always)]
    #[must_use]
    pub fn mpl0(&mut self) -> MPL0_W<28> {
        MPL0_W::new(self)
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline(always)]
    #[must_use]
    pub fn mtw0(&mut self) -> MTW0_W<29> {
        MTW0_W::new(self)
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline(always)]
    #[must_use]
    pub fn mtr0(&mut self) -> MTR0_W<30> {
        MTR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Privilege Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpra](index.html) module"]
pub struct MPRA_SPEC;
impl crate::RegisterSpec for MPRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpra::R](R) reader structure"]
impl crate::Readable for MPRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpra::W](W) writer structure"]
impl crate::Writable for MPRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPRA to value 0x7770_0000"]
impl crate::Resettable for MPRA_SPEC {
    const RESET_VALUE: Self::Ux = 0x7770_0000;
}
