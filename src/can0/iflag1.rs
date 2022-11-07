#[doc = "Register `IFLAG1` reader"]
pub struct R(crate::R<IFLAG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFLAG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFLAG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFLAG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFLAG1` writer"]
pub struct W(crate::W<IFLAG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFLAG1_SPEC>;
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
impl From<crate::W<IFLAG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFLAG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF0I` reader - Buffer MB0 Interrupt Or \"reserved\""]
pub type BUF0I_R = crate::BitReader<BUF0I_A>;
#[doc = "Buffer MB0 Interrupt Or \"reserved\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF0I_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1 = 1,
}
impl From<BUF0I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF0I_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF0I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF0I_A {
        match self.bits {
            false => BUF0I_A::_0,
            true => BUF0I_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF0I_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF0I_A::_1
    }
}
#[doc = "Field `BUF0I` writer - Buffer MB0 Interrupt Or \"reserved\""]
pub type BUF0I_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF0I_A, O>;
impl<'a, const O: u8> BUF0I_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF0I_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF0I_A::_1)
    }
}
#[doc = "Field `BUF4TO1I0` reader - Buffer MB i Interrupt Or \"reserved\""]
pub type BUF4TO1I0_R = crate::BitReader<BUF4TO1I0_A>;
#[doc = "Buffer MB i Interrupt Or \"reserved\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF4TO1I0_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1 = 1,
}
impl From<BUF4TO1I0_A> for bool {
    #[inline(always)]
    fn from(variant: BUF4TO1I0_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF4TO1I0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF4TO1I0_A {
        match self.bits {
            false => BUF4TO1I0_A::_0,
            true => BUF4TO1I0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I0_A::_1
    }
}
#[doc = "Field `BUF4TO1I0` writer - Buffer MB i Interrupt Or \"reserved\""]
pub type BUF4TO1I0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF4TO1I0_A, O>;
impl<'a, const O: u8> BUF4TO1I0_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I0_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I0_A::_1)
    }
}
#[doc = "Field `BUF4TO1I1` reader - Buffer MB i Interrupt Or \"reserved\""]
pub type BUF4TO1I1_R = crate::BitReader<BUF4TO1I1_A>;
#[doc = "Buffer MB i Interrupt Or \"reserved\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF4TO1I1_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1 = 1,
}
impl From<BUF4TO1I1_A> for bool {
    #[inline(always)]
    fn from(variant: BUF4TO1I1_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF4TO1I1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF4TO1I1_A {
        match self.bits {
            false => BUF4TO1I1_A::_0,
            true => BUF4TO1I1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I1_A::_1
    }
}
#[doc = "Field `BUF4TO1I1` writer - Buffer MB i Interrupt Or \"reserved\""]
pub type BUF4TO1I1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF4TO1I1_A, O>;
impl<'a, const O: u8> BUF4TO1I1_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I1_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I1_A::_1)
    }
}
#[doc = "Field `BUF4TO1I2` reader - Buffer MB i Interrupt Or \"reserved\""]
pub type BUF4TO1I2_R = crate::BitReader<BUF4TO1I2_A>;
#[doc = "Buffer MB i Interrupt Or \"reserved\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF4TO1I2_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1 = 1,
}
impl From<BUF4TO1I2_A> for bool {
    #[inline(always)]
    fn from(variant: BUF4TO1I2_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF4TO1I2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF4TO1I2_A {
        match self.bits {
            false => BUF4TO1I2_A::_0,
            true => BUF4TO1I2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I2_A::_1
    }
}
#[doc = "Field `BUF4TO1I2` writer - Buffer MB i Interrupt Or \"reserved\""]
pub type BUF4TO1I2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF4TO1I2_A, O>;
impl<'a, const O: u8> BUF4TO1I2_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I2_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I2_A::_1)
    }
}
#[doc = "Field `BUF4TO1I3` reader - Buffer MB i Interrupt Or \"reserved\""]
pub type BUF4TO1I3_R = crate::BitReader<BUF4TO1I3_A>;
#[doc = "Buffer MB i Interrupt Or \"reserved\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF4TO1I3_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1 = 1,
}
impl From<BUF4TO1I3_A> for bool {
    #[inline(always)]
    fn from(variant: BUF4TO1I3_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF4TO1I3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF4TO1I3_A {
        match self.bits {
            false => BUF4TO1I3_A::_0,
            true => BUF4TO1I3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I3_A::_1
    }
}
#[doc = "Field `BUF4TO1I3` writer - Buffer MB i Interrupt Or \"reserved\""]
pub type BUF4TO1I3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF4TO1I3_A, O>;
impl<'a, const O: u8> BUF4TO1I3_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I3_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I3_A::_1)
    }
}
#[doc = "Field `BUF5I` reader - Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\""]
pub type BUF5I_R = crate::BitReader<BUF5I_A>;
#[doc = "Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF5I_A {
    #[doc = "0: No occurrence of MB5 completing transmission/reception when MCR\\[RFEN\\]=0, or of frame(s) available in the FIFO, when MCR\\[RFEN\\]=1"]
    _0 = 0,
    #[doc = "1: MB5 completed transmission/reception when MCR\\[RFEN\\]=0, or frame(s) available in the Rx FIFO when MCR\\[RFEN\\]=1"]
    _1 = 1,
}
impl From<BUF5I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF5I_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF5I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF5I_A {
        match self.bits {
            false => BUF5I_A::_0,
            true => BUF5I_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF5I_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF5I_A::_1
    }
}
#[doc = "Field `BUF5I` writer - Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\""]
pub type BUF5I_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF5I_A, O>;
impl<'a, const O: u8> BUF5I_W<'a, O> {
    #[doc = "No occurrence of MB5 completing transmission/reception when MCR\\[RFEN\\]=0, or of frame(s) available in the FIFO, when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF5I_A::_0)
    }
    #[doc = "MB5 completed transmission/reception when MCR\\[RFEN\\]=0, or frame(s) available in the Rx FIFO when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF5I_A::_1)
    }
}
#[doc = "Field `BUF6I` reader - Buffer MB6 Interrupt Or \"Rx FIFO Warning\""]
pub type BUF6I_R = crate::BitReader<BUF6I_A>;
#[doc = "Buffer MB6 Interrupt Or \"Rx FIFO Warning\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF6I_A {
    #[doc = "0: No occurrence of MB6 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    _0 = 0,
    #[doc = "1: MB6 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    _1 = 1,
}
impl From<BUF6I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF6I_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF6I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF6I_A {
        match self.bits {
            false => BUF6I_A::_0,
            true => BUF6I_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF6I_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF6I_A::_1
    }
}
#[doc = "Field `BUF6I` writer - Buffer MB6 Interrupt Or \"Rx FIFO Warning\""]
pub type BUF6I_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF6I_A, O>;
impl<'a, const O: u8> BUF6I_W<'a, O> {
    #[doc = "No occurrence of MB6 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF6I_A::_0)
    }
    #[doc = "MB6 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF6I_A::_1)
    }
}
#[doc = "Field `BUF7I` reader - Buffer MB7 Interrupt Or \"Rx FIFO Overflow\""]
pub type BUF7I_R = crate::BitReader<BUF7I_A>;
#[doc = "Buffer MB7 Interrupt Or \"Rx FIFO Overflow\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF7I_A {
    #[doc = "0: No occurrence of MB7 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    _0 = 0,
    #[doc = "1: MB7 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    _1 = 1,
}
impl From<BUF7I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF7I_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF7I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF7I_A {
        match self.bits {
            false => BUF7I_A::_0,
            true => BUF7I_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF7I_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF7I_A::_1
    }
}
#[doc = "Field `BUF7I` writer - Buffer MB7 Interrupt Or \"Rx FIFO Overflow\""]
pub type BUF7I_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF7I_A, O>;
impl<'a, const O: u8> BUF7I_W<'a, O> {
    #[doc = "No occurrence of MB7 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF7I_A::_0)
    }
    #[doc = "MB7 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF7I_A::_1)
    }
}
#[doc = "Field `BUF31TO8I0` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I0_R = crate::BitReader<BUF31TO8I0_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I0_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I0_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I0_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I0_A {
        match self.bits {
            false => BUF31TO8I0_A::_0,
            true => BUF31TO8I0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I0_A::_1
    }
}
#[doc = "Field `BUF31TO8I0` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I0_A, O>;
impl<'a, const O: u8> BUF31TO8I0_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I0_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I0_A::_1)
    }
}
#[doc = "Field `BUF31TO8I1` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I1_R = crate::BitReader<BUF31TO8I1_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I1_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I1_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I1_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I1_A {
        match self.bits {
            false => BUF31TO8I1_A::_0,
            true => BUF31TO8I1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I1_A::_1
    }
}
#[doc = "Field `BUF31TO8I1` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I1_A, O>;
impl<'a, const O: u8> BUF31TO8I1_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I1_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I1_A::_1)
    }
}
#[doc = "Field `BUF31TO8I2` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I2_R = crate::BitReader<BUF31TO8I2_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I2_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I2_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I2_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I2_A {
        match self.bits {
            false => BUF31TO8I2_A::_0,
            true => BUF31TO8I2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I2_A::_1
    }
}
#[doc = "Field `BUF31TO8I2` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I2_A, O>;
impl<'a, const O: u8> BUF31TO8I2_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I2_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I2_A::_1)
    }
}
#[doc = "Field `BUF31TO8I3` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I3_R = crate::BitReader<BUF31TO8I3_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I3_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I3_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I3_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I3_A {
        match self.bits {
            false => BUF31TO8I3_A::_0,
            true => BUF31TO8I3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I3_A::_1
    }
}
#[doc = "Field `BUF31TO8I3` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I3_A, O>;
impl<'a, const O: u8> BUF31TO8I3_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I3_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I3_A::_1)
    }
}
#[doc = "Field `BUF31TO8I4` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I4_R = crate::BitReader<BUF31TO8I4_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I4_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I4_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I4_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I4_A {
        match self.bits {
            false => BUF31TO8I4_A::_0,
            true => BUF31TO8I4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I4_A::_1
    }
}
#[doc = "Field `BUF31TO8I4` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I4_A, O>;
impl<'a, const O: u8> BUF31TO8I4_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I4_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I4_A::_1)
    }
}
#[doc = "Field `BUF31TO8I5` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I5_R = crate::BitReader<BUF31TO8I5_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I5_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I5_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I5_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I5_A {
        match self.bits {
            false => BUF31TO8I5_A::_0,
            true => BUF31TO8I5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I5_A::_1
    }
}
#[doc = "Field `BUF31TO8I5` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I5_A, O>;
impl<'a, const O: u8> BUF31TO8I5_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I5_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I5_A::_1)
    }
}
#[doc = "Field `BUF31TO8I6` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I6_R = crate::BitReader<BUF31TO8I6_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I6_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I6_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I6_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I6_A {
        match self.bits {
            false => BUF31TO8I6_A::_0,
            true => BUF31TO8I6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I6_A::_1
    }
}
#[doc = "Field `BUF31TO8I6` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I6_A, O>;
impl<'a, const O: u8> BUF31TO8I6_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I6_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I6_A::_1)
    }
}
#[doc = "Field `BUF31TO8I7` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I7_R = crate::BitReader<BUF31TO8I7_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I7_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I7_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I7_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I7_A {
        match self.bits {
            false => BUF31TO8I7_A::_0,
            true => BUF31TO8I7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I7_A::_1
    }
}
#[doc = "Field `BUF31TO8I7` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I7_A, O>;
impl<'a, const O: u8> BUF31TO8I7_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I7_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I7_A::_1)
    }
}
#[doc = "Field `BUF31TO8I8` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I8_R = crate::BitReader<BUF31TO8I8_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I8_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I8_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I8_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I8_A {
        match self.bits {
            false => BUF31TO8I8_A::_0,
            true => BUF31TO8I8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I8_A::_1
    }
}
#[doc = "Field `BUF31TO8I8` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I8_A, O>;
impl<'a, const O: u8> BUF31TO8I8_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I8_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I8_A::_1)
    }
}
#[doc = "Field `BUF31TO8I9` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I9_R = crate::BitReader<BUF31TO8I9_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I9_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I9_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I9_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I9_A {
        match self.bits {
            false => BUF31TO8I9_A::_0,
            true => BUF31TO8I9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I9_A::_1
    }
}
#[doc = "Field `BUF31TO8I9` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I9_A, O>;
impl<'a, const O: u8> BUF31TO8I9_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I9_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I9_A::_1)
    }
}
#[doc = "Field `BUF31TO8I10` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I10_R = crate::BitReader<BUF31TO8I10_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I10_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I10_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I10_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I10_A {
        match self.bits {
            false => BUF31TO8I10_A::_0,
            true => BUF31TO8I10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I10_A::_1
    }
}
#[doc = "Field `BUF31TO8I10` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I10_A, O>;
impl<'a, const O: u8> BUF31TO8I10_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I10_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I10_A::_1)
    }
}
#[doc = "Field `BUF31TO8I11` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I11_R = crate::BitReader<BUF31TO8I11_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I11_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I11_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I11_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I11_A {
        match self.bits {
            false => BUF31TO8I11_A::_0,
            true => BUF31TO8I11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I11_A::_1
    }
}
#[doc = "Field `BUF31TO8I11` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I11_A, O>;
impl<'a, const O: u8> BUF31TO8I11_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I11_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I11_A::_1)
    }
}
#[doc = "Field `BUF31TO8I12` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I12_R = crate::BitReader<BUF31TO8I12_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I12_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I12_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I12_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I12_A {
        match self.bits {
            false => BUF31TO8I12_A::_0,
            true => BUF31TO8I12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I12_A::_1
    }
}
#[doc = "Field `BUF31TO8I12` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I12_A, O>;
impl<'a, const O: u8> BUF31TO8I12_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I12_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I12_A::_1)
    }
}
#[doc = "Field `BUF31TO8I13` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I13_R = crate::BitReader<BUF31TO8I13_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I13_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I13_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I13_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I13_A {
        match self.bits {
            false => BUF31TO8I13_A::_0,
            true => BUF31TO8I13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I13_A::_1
    }
}
#[doc = "Field `BUF31TO8I13` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I13_A, O>;
impl<'a, const O: u8> BUF31TO8I13_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I13_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I13_A::_1)
    }
}
#[doc = "Field `BUF31TO8I14` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I14_R = crate::BitReader<BUF31TO8I14_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I14_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I14_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I14_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I14_A {
        match self.bits {
            false => BUF31TO8I14_A::_0,
            true => BUF31TO8I14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I14_A::_1
    }
}
#[doc = "Field `BUF31TO8I14` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I14_A, O>;
impl<'a, const O: u8> BUF31TO8I14_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I14_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I14_A::_1)
    }
}
#[doc = "Field `BUF31TO8I15` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I15_R = crate::BitReader<BUF31TO8I15_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I15_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I15_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I15_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I15_A {
        match self.bits {
            false => BUF31TO8I15_A::_0,
            true => BUF31TO8I15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I15_A::_1
    }
}
#[doc = "Field `BUF31TO8I15` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I15_A, O>;
impl<'a, const O: u8> BUF31TO8I15_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I15_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I15_A::_1)
    }
}
#[doc = "Field `BUF31TO8I16` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I16_R = crate::BitReader<BUF31TO8I16_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I16_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I16_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I16_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I16_A {
        match self.bits {
            false => BUF31TO8I16_A::_0,
            true => BUF31TO8I16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I16_A::_1
    }
}
#[doc = "Field `BUF31TO8I16` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I16_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I16_A, O>;
impl<'a, const O: u8> BUF31TO8I16_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I16_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I16_A::_1)
    }
}
#[doc = "Field `BUF31TO8I17` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I17_R = crate::BitReader<BUF31TO8I17_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I17_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I17_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I17_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I17_A {
        match self.bits {
            false => BUF31TO8I17_A::_0,
            true => BUF31TO8I17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I17_A::_1
    }
}
#[doc = "Field `BUF31TO8I17` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I17_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I17_A, O>;
impl<'a, const O: u8> BUF31TO8I17_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I17_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I17_A::_1)
    }
}
#[doc = "Field `BUF31TO8I18` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I18_R = crate::BitReader<BUF31TO8I18_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I18_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I18_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I18_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I18_A {
        match self.bits {
            false => BUF31TO8I18_A::_0,
            true => BUF31TO8I18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I18_A::_1
    }
}
#[doc = "Field `BUF31TO8I18` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I18_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I18_A, O>;
impl<'a, const O: u8> BUF31TO8I18_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I18_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I18_A::_1)
    }
}
#[doc = "Field `BUF31TO8I19` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I19_R = crate::BitReader<BUF31TO8I19_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I19_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I19_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I19_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I19_A {
        match self.bits {
            false => BUF31TO8I19_A::_0,
            true => BUF31TO8I19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I19_A::_1
    }
}
#[doc = "Field `BUF31TO8I19` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I19_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I19_A, O>;
impl<'a, const O: u8> BUF31TO8I19_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I19_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I19_A::_1)
    }
}
#[doc = "Field `BUF31TO8I20` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I20_R = crate::BitReader<BUF31TO8I20_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I20_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I20_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I20_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I20_A {
        match self.bits {
            false => BUF31TO8I20_A::_0,
            true => BUF31TO8I20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I20_A::_1
    }
}
#[doc = "Field `BUF31TO8I20` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I20_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I20_A, O>;
impl<'a, const O: u8> BUF31TO8I20_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I20_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I20_A::_1)
    }
}
#[doc = "Field `BUF31TO8I21` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I21_R = crate::BitReader<BUF31TO8I21_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I21_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I21_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I21_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I21_A {
        match self.bits {
            false => BUF31TO8I21_A::_0,
            true => BUF31TO8I21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I21_A::_1
    }
}
#[doc = "Field `BUF31TO8I21` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I21_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I21_A, O>;
impl<'a, const O: u8> BUF31TO8I21_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I21_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I21_A::_1)
    }
}
#[doc = "Field `BUF31TO8I22` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I22_R = crate::BitReader<BUF31TO8I22_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I22_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I22_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I22_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I22_A {
        match self.bits {
            false => BUF31TO8I22_A::_0,
            true => BUF31TO8I22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I22_A::_1
    }
}
#[doc = "Field `BUF31TO8I22` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I22_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I22_A, O>;
impl<'a, const O: u8> BUF31TO8I22_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I22_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I22_A::_1)
    }
}
#[doc = "Field `BUF31TO8I23` reader - Buffer MBi Interrupt"]
pub type BUF31TO8I23_R = crate::BitReader<BUF31TO8I23_A>;
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF31TO8I23_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUF31TO8I23_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I23_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF31TO8I23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I23_A {
        match self.bits {
            false => BUF31TO8I23_A::_0,
            true => BUF31TO8I23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I23_A::_1
    }
}
#[doc = "Field `BUF31TO8I23` writer - Buffer MBi Interrupt"]
pub type BUF31TO8I23_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF31TO8I23_A, O>;
impl<'a, const O: u8> BUF31TO8I23_W<'a, O> {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I23_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I23_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf0i(&self) -> BUF0I_R {
        BUF0I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i0(&self) -> BUF4TO1I0_R {
        BUF4TO1I0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i1(&self) -> BUF4TO1I1_R {
        BUF4TO1I1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i2(&self) -> BUF4TO1I2_R {
        BUF4TO1I2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i3(&self) -> BUF4TO1I3_R {
        BUF4TO1I3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\""]
    #[inline(always)]
    pub fn buf5i(&self) -> BUF5I_R {
        BUF5I_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or \"Rx FIFO Warning\""]
    #[inline(always)]
    pub fn buf6i(&self) -> BUF6I_R {
        BUF6I_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or \"Rx FIFO Overflow\""]
    #[inline(always)]
    pub fn buf7i(&self) -> BUF7I_R {
        BUF7I_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i0(&self) -> BUF31TO8I0_R {
        BUF31TO8I0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i1(&self) -> BUF31TO8I1_R {
        BUF31TO8I1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i2(&self) -> BUF31TO8I2_R {
        BUF31TO8I2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i3(&self) -> BUF31TO8I3_R {
        BUF31TO8I3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i4(&self) -> BUF31TO8I4_R {
        BUF31TO8I4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i5(&self) -> BUF31TO8I5_R {
        BUF31TO8I5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i6(&self) -> BUF31TO8I6_R {
        BUF31TO8I6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i7(&self) -> BUF31TO8I7_R {
        BUF31TO8I7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i8(&self) -> BUF31TO8I8_R {
        BUF31TO8I8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i9(&self) -> BUF31TO8I9_R {
        BUF31TO8I9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i10(&self) -> BUF31TO8I10_R {
        BUF31TO8I10_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i11(&self) -> BUF31TO8I11_R {
        BUF31TO8I11_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i12(&self) -> BUF31TO8I12_R {
        BUF31TO8I12_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i13(&self) -> BUF31TO8I13_R {
        BUF31TO8I13_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i14(&self) -> BUF31TO8I14_R {
        BUF31TO8I14_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i15(&self) -> BUF31TO8I15_R {
        BUF31TO8I15_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i16(&self) -> BUF31TO8I16_R {
        BUF31TO8I16_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i17(&self) -> BUF31TO8I17_R {
        BUF31TO8I17_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i18(&self) -> BUF31TO8I18_R {
        BUF31TO8I18_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i19(&self) -> BUF31TO8I19_R {
        BUF31TO8I19_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i20(&self) -> BUF31TO8I20_R {
        BUF31TO8I20_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i21(&self) -> BUF31TO8I21_R {
        BUF31TO8I21_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i22(&self) -> BUF31TO8I22_R {
        BUF31TO8I22_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i23(&self) -> BUF31TO8I23_R {
        BUF31TO8I23_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or \"reserved\""]
    #[inline(always)]
    #[must_use]
    pub fn buf0i(&mut self) -> BUF0I_W<0> {
        BUF0I_W::new(self)
    }
    #[doc = "Bit 1 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    #[must_use]
    pub fn buf4to1i0(&mut self) -> BUF4TO1I0_W<1> {
        BUF4TO1I0_W::new(self)
    }
    #[doc = "Bit 2 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    #[must_use]
    pub fn buf4to1i1(&mut self) -> BUF4TO1I1_W<2> {
        BUF4TO1I1_W::new(self)
    }
    #[doc = "Bit 3 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    #[must_use]
    pub fn buf4to1i2(&mut self) -> BUF4TO1I2_W<3> {
        BUF4TO1I2_W::new(self)
    }
    #[doc = "Bit 4 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    #[must_use]
    pub fn buf4to1i3(&mut self) -> BUF4TO1I3_W<4> {
        BUF4TO1I3_W::new(self)
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\""]
    #[inline(always)]
    #[must_use]
    pub fn buf5i(&mut self) -> BUF5I_W<5> {
        BUF5I_W::new(self)
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or \"Rx FIFO Warning\""]
    #[inline(always)]
    #[must_use]
    pub fn buf6i(&mut self) -> BUF6I_W<6> {
        BUF6I_W::new(self)
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or \"Rx FIFO Overflow\""]
    #[inline(always)]
    #[must_use]
    pub fn buf7i(&mut self) -> BUF7I_W<7> {
        BUF7I_W::new(self)
    }
    #[doc = "Bit 8 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i0(&mut self) -> BUF31TO8I0_W<8> {
        BUF31TO8I0_W::new(self)
    }
    #[doc = "Bit 9 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i1(&mut self) -> BUF31TO8I1_W<9> {
        BUF31TO8I1_W::new(self)
    }
    #[doc = "Bit 10 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i2(&mut self) -> BUF31TO8I2_W<10> {
        BUF31TO8I2_W::new(self)
    }
    #[doc = "Bit 11 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i3(&mut self) -> BUF31TO8I3_W<11> {
        BUF31TO8I3_W::new(self)
    }
    #[doc = "Bit 12 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i4(&mut self) -> BUF31TO8I4_W<12> {
        BUF31TO8I4_W::new(self)
    }
    #[doc = "Bit 13 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i5(&mut self) -> BUF31TO8I5_W<13> {
        BUF31TO8I5_W::new(self)
    }
    #[doc = "Bit 14 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i6(&mut self) -> BUF31TO8I6_W<14> {
        BUF31TO8I6_W::new(self)
    }
    #[doc = "Bit 15 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i7(&mut self) -> BUF31TO8I7_W<15> {
        BUF31TO8I7_W::new(self)
    }
    #[doc = "Bit 16 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i8(&mut self) -> BUF31TO8I8_W<16> {
        BUF31TO8I8_W::new(self)
    }
    #[doc = "Bit 17 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i9(&mut self) -> BUF31TO8I9_W<17> {
        BUF31TO8I9_W::new(self)
    }
    #[doc = "Bit 18 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i10(&mut self) -> BUF31TO8I10_W<18> {
        BUF31TO8I10_W::new(self)
    }
    #[doc = "Bit 19 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i11(&mut self) -> BUF31TO8I11_W<19> {
        BUF31TO8I11_W::new(self)
    }
    #[doc = "Bit 20 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i12(&mut self) -> BUF31TO8I12_W<20> {
        BUF31TO8I12_W::new(self)
    }
    #[doc = "Bit 21 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i13(&mut self) -> BUF31TO8I13_W<21> {
        BUF31TO8I13_W::new(self)
    }
    #[doc = "Bit 22 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i14(&mut self) -> BUF31TO8I14_W<22> {
        BUF31TO8I14_W::new(self)
    }
    #[doc = "Bit 23 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i15(&mut self) -> BUF31TO8I15_W<23> {
        BUF31TO8I15_W::new(self)
    }
    #[doc = "Bit 24 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i16(&mut self) -> BUF31TO8I16_W<24> {
        BUF31TO8I16_W::new(self)
    }
    #[doc = "Bit 25 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i17(&mut self) -> BUF31TO8I17_W<25> {
        BUF31TO8I17_W::new(self)
    }
    #[doc = "Bit 26 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i18(&mut self) -> BUF31TO8I18_W<26> {
        BUF31TO8I18_W::new(self)
    }
    #[doc = "Bit 27 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i19(&mut self) -> BUF31TO8I19_W<27> {
        BUF31TO8I19_W::new(self)
    }
    #[doc = "Bit 28 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i20(&mut self) -> BUF31TO8I20_W<28> {
        BUF31TO8I20_W::new(self)
    }
    #[doc = "Bit 29 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i21(&mut self) -> BUF31TO8I21_W<29> {
        BUF31TO8I21_W::new(self)
    }
    #[doc = "Bit 30 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i22(&mut self) -> BUF31TO8I22_W<30> {
        BUF31TO8I22_W::new(self)
    }
    #[doc = "Bit 31 - Buffer MBi Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i23(&mut self) -> BUF31TO8I23_W<31> {
        BUF31TO8I23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iflag1](index.html) module"]
pub struct IFLAG1_SPEC;
impl crate::RegisterSpec for IFLAG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iflag1::R](R) reader structure"]
impl crate::Readable for IFLAG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iflag1::W](W) writer structure"]
impl crate::Writable for IFLAG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFLAG1 to value 0"]
impl crate::Resettable for IFLAG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
