#[doc = "Register `CH%sS` reader"]
pub struct R(crate::R<CHS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sS` writer"]
pub struct W(crate::W<CHS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHS_SPEC>;
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
impl From<crate::W<CHS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERR0` reader - PDB Channel Sequence Error Flags"]
pub type ERR0_R = crate::BitReader<ERR0_A>;
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR0_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0 = 0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    _1 = 1,
}
impl From<ERR0_A> for bool {
    #[inline(always)]
    fn from(variant: ERR0_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR0_A {
        match self.bits {
            false => ERR0_A::_0,
            true => ERR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR0_A::_1
    }
}
#[doc = "Field `ERR0` writer - PDB Channel Sequence Error Flags"]
pub type ERR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHS_SPEC, ERR0_A, O>;
impl<'a, const O: u8> ERR0_W<'a, O> {
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR0_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR0_A::_1)
    }
}
#[doc = "Field `ERR1` reader - PDB Channel Sequence Error Flags"]
pub type ERR1_R = crate::BitReader<ERR1_A>;
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR1_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0 = 0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    _1 = 1,
}
impl From<ERR1_A> for bool {
    #[inline(always)]
    fn from(variant: ERR1_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR1_A {
        match self.bits {
            false => ERR1_A::_0,
            true => ERR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR1_A::_1
    }
}
#[doc = "Field `ERR1` writer - PDB Channel Sequence Error Flags"]
pub type ERR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHS_SPEC, ERR1_A, O>;
impl<'a, const O: u8> ERR1_W<'a, O> {
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR1_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR1_A::_1)
    }
}
#[doc = "Field `ERR2` reader - PDB Channel Sequence Error Flags"]
pub type ERR2_R = crate::BitReader<ERR2_A>;
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR2_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0 = 0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    _1 = 1,
}
impl From<ERR2_A> for bool {
    #[inline(always)]
    fn from(variant: ERR2_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR2_A {
        match self.bits {
            false => ERR2_A::_0,
            true => ERR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR2_A::_1
    }
}
#[doc = "Field `ERR2` writer - PDB Channel Sequence Error Flags"]
pub type ERR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHS_SPEC, ERR2_A, O>;
impl<'a, const O: u8> ERR2_W<'a, O> {
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR2_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR2_A::_1)
    }
}
#[doc = "Field `ERR3` reader - PDB Channel Sequence Error Flags"]
pub type ERR3_R = crate::BitReader<ERR3_A>;
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR3_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0 = 0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    _1 = 1,
}
impl From<ERR3_A> for bool {
    #[inline(always)]
    fn from(variant: ERR3_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR3_A {
        match self.bits {
            false => ERR3_A::_0,
            true => ERR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR3_A::_1
    }
}
#[doc = "Field `ERR3` writer - PDB Channel Sequence Error Flags"]
pub type ERR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHS_SPEC, ERR3_A, O>;
impl<'a, const O: u8> ERR3_W<'a, O> {
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR3_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR3_A::_1)
    }
}
#[doc = "Field `ERR4` reader - PDB Channel Sequence Error Flags"]
pub type ERR4_R = crate::BitReader<ERR4_A>;
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR4_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0 = 0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    _1 = 1,
}
impl From<ERR4_A> for bool {
    #[inline(always)]
    fn from(variant: ERR4_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR4_A {
        match self.bits {
            false => ERR4_A::_0,
            true => ERR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR4_A::_1
    }
}
#[doc = "Field `ERR4` writer - PDB Channel Sequence Error Flags"]
pub type ERR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHS_SPEC, ERR4_A, O>;
impl<'a, const O: u8> ERR4_W<'a, O> {
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR4_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR4_A::_1)
    }
}
#[doc = "Field `ERR5` reader - PDB Channel Sequence Error Flags"]
pub type ERR5_R = crate::BitReader<ERR5_A>;
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR5_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0 = 0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    _1 = 1,
}
impl From<ERR5_A> for bool {
    #[inline(always)]
    fn from(variant: ERR5_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR5_A {
        match self.bits {
            false => ERR5_A::_0,
            true => ERR5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR5_A::_1
    }
}
#[doc = "Field `ERR5` writer - PDB Channel Sequence Error Flags"]
pub type ERR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHS_SPEC, ERR5_A, O>;
impl<'a, const O: u8> ERR5_W<'a, O> {
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR5_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR5_A::_1)
    }
}
#[doc = "Field `ERR6` reader - PDB Channel Sequence Error Flags"]
pub type ERR6_R = crate::BitReader<ERR6_A>;
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR6_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0 = 0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    _1 = 1,
}
impl From<ERR6_A> for bool {
    #[inline(always)]
    fn from(variant: ERR6_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR6_A {
        match self.bits {
            false => ERR6_A::_0,
            true => ERR6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR6_A::_1
    }
}
#[doc = "Field `ERR6` writer - PDB Channel Sequence Error Flags"]
pub type ERR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHS_SPEC, ERR6_A, O>;
impl<'a, const O: u8> ERR6_W<'a, O> {
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR6_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR6_A::_1)
    }
}
#[doc = "Field `ERR7` reader - PDB Channel Sequence Error Flags"]
pub type ERR7_R = crate::BitReader<ERR7_A>;
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR7_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0 = 0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    _1 = 1,
}
impl From<ERR7_A> for bool {
    #[inline(always)]
    fn from(variant: ERR7_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR7_A {
        match self.bits {
            false => ERR7_A::_0,
            true => ERR7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR7_A::_1
    }
}
#[doc = "Field `ERR7` writer - PDB Channel Sequence Error Flags"]
pub type ERR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHS_SPEC, ERR7_A, O>;
impl<'a, const O: u8> ERR7_W<'a, O> {
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR7_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR7_A::_1)
    }
}
#[doc = "Field `CF` reader - PDB Channel Flags"]
pub type CF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CF` writer - PDB Channel Flags"]
pub type CF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err0(&self) -> ERR0_R {
        ERR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err1(&self) -> ERR1_R {
        ERR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err2(&self) -> ERR2_R {
        ERR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err3(&self) -> ERR3_R {
        ERR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err4(&self) -> ERR4_R {
        ERR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err5(&self) -> ERR5_R {
        ERR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err6(&self) -> ERR6_R {
        ERR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err7(&self) -> ERR7_R {
        ERR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:23 - PDB Channel Flags"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    #[must_use]
    pub fn err0(&mut self) -> ERR0_W<0> {
        ERR0_W::new(self)
    }
    #[doc = "Bit 1 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    #[must_use]
    pub fn err1(&mut self) -> ERR1_W<1> {
        ERR1_W::new(self)
    }
    #[doc = "Bit 2 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    #[must_use]
    pub fn err2(&mut self) -> ERR2_W<2> {
        ERR2_W::new(self)
    }
    #[doc = "Bit 3 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    #[must_use]
    pub fn err3(&mut self) -> ERR3_W<3> {
        ERR3_W::new(self)
    }
    #[doc = "Bit 4 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    #[must_use]
    pub fn err4(&mut self) -> ERR4_W<4> {
        ERR4_W::new(self)
    }
    #[doc = "Bit 5 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    #[must_use]
    pub fn err5(&mut self) -> ERR5_W<5> {
        ERR5_W::new(self)
    }
    #[doc = "Bit 6 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    #[must_use]
    pub fn err6(&mut self) -> ERR6_W<6> {
        ERR6_W::new(self)
    }
    #[doc = "Bit 7 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    #[must_use]
    pub fn err7(&mut self) -> ERR7_W<7> {
        ERR7_W::new(self)
    }
    #[doc = "Bits 16:23 - PDB Channel Flags"]
    #[inline(always)]
    #[must_use]
    pub fn cf(&mut self) -> CF_W<16> {
        CF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chs](index.html) module"]
pub struct CHS_SPEC;
impl crate::RegisterSpec for CHS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chs::R](R) reader structure"]
impl crate::Readable for CHS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chs::W](W) writer structure"]
impl crate::Writable for CHS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%sS to value 0"]
impl crate::Resettable for CHS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
