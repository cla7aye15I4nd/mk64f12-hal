#[doc = "Register `ISFR` reader"]
pub struct R(crate::R<ISFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISFR` writer"]
pub struct W(crate::W<ISFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISFR_SPEC>;
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
impl From<crate::W<ISFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISF0` reader - Interrupt Status Flag"]
pub type ISF0_R = crate::BitReader<ISF0_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF0_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF0_A> for bool {
    #[inline(always)]
    fn from(variant: ISF0_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF0_A {
        match self.bits {
            false => ISF0_A::_0,
            true => ISF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF0_A::_1
    }
}
#[doc = "Field `ISF0` writer - Interrupt Status Flag"]
pub type ISF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF0_A, O>;
impl<'a, const O: u8> ISF0_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF0_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF0_A::_1)
    }
}
#[doc = "Field `ISF1` reader - Interrupt Status Flag"]
pub type ISF1_R = crate::BitReader<ISF1_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF1_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF1_A> for bool {
    #[inline(always)]
    fn from(variant: ISF1_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF1_A {
        match self.bits {
            false => ISF1_A::_0,
            true => ISF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF1_A::_1
    }
}
#[doc = "Field `ISF1` writer - Interrupt Status Flag"]
pub type ISF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF1_A, O>;
impl<'a, const O: u8> ISF1_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF1_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF1_A::_1)
    }
}
#[doc = "Field `ISF2` reader - Interrupt Status Flag"]
pub type ISF2_R = crate::BitReader<ISF2_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF2_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF2_A> for bool {
    #[inline(always)]
    fn from(variant: ISF2_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF2_A {
        match self.bits {
            false => ISF2_A::_0,
            true => ISF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF2_A::_1
    }
}
#[doc = "Field `ISF2` writer - Interrupt Status Flag"]
pub type ISF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF2_A, O>;
impl<'a, const O: u8> ISF2_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF2_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF2_A::_1)
    }
}
#[doc = "Field `ISF3` reader - Interrupt Status Flag"]
pub type ISF3_R = crate::BitReader<ISF3_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF3_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF3_A> for bool {
    #[inline(always)]
    fn from(variant: ISF3_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF3_A {
        match self.bits {
            false => ISF3_A::_0,
            true => ISF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF3_A::_1
    }
}
#[doc = "Field `ISF3` writer - Interrupt Status Flag"]
pub type ISF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF3_A, O>;
impl<'a, const O: u8> ISF3_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF3_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF3_A::_1)
    }
}
#[doc = "Field `ISF4` reader - Interrupt Status Flag"]
pub type ISF4_R = crate::BitReader<ISF4_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF4_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF4_A> for bool {
    #[inline(always)]
    fn from(variant: ISF4_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF4_A {
        match self.bits {
            false => ISF4_A::_0,
            true => ISF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF4_A::_1
    }
}
#[doc = "Field `ISF4` writer - Interrupt Status Flag"]
pub type ISF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF4_A, O>;
impl<'a, const O: u8> ISF4_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF4_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF4_A::_1)
    }
}
#[doc = "Field `ISF5` reader - Interrupt Status Flag"]
pub type ISF5_R = crate::BitReader<ISF5_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF5_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF5_A> for bool {
    #[inline(always)]
    fn from(variant: ISF5_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF5_A {
        match self.bits {
            false => ISF5_A::_0,
            true => ISF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF5_A::_1
    }
}
#[doc = "Field `ISF5` writer - Interrupt Status Flag"]
pub type ISF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF5_A, O>;
impl<'a, const O: u8> ISF5_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF5_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF5_A::_1)
    }
}
#[doc = "Field `ISF6` reader - Interrupt Status Flag"]
pub type ISF6_R = crate::BitReader<ISF6_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF6_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF6_A> for bool {
    #[inline(always)]
    fn from(variant: ISF6_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF6_A {
        match self.bits {
            false => ISF6_A::_0,
            true => ISF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF6_A::_1
    }
}
#[doc = "Field `ISF6` writer - Interrupt Status Flag"]
pub type ISF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF6_A, O>;
impl<'a, const O: u8> ISF6_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF6_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF6_A::_1)
    }
}
#[doc = "Field `ISF7` reader - Interrupt Status Flag"]
pub type ISF7_R = crate::BitReader<ISF7_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF7_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF7_A> for bool {
    #[inline(always)]
    fn from(variant: ISF7_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF7_A {
        match self.bits {
            false => ISF7_A::_0,
            true => ISF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF7_A::_1
    }
}
#[doc = "Field `ISF7` writer - Interrupt Status Flag"]
pub type ISF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF7_A, O>;
impl<'a, const O: u8> ISF7_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF7_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF7_A::_1)
    }
}
#[doc = "Field `ISF8` reader - Interrupt Status Flag"]
pub type ISF8_R = crate::BitReader<ISF8_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF8_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF8_A> for bool {
    #[inline(always)]
    fn from(variant: ISF8_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF8_A {
        match self.bits {
            false => ISF8_A::_0,
            true => ISF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF8_A::_1
    }
}
#[doc = "Field `ISF8` writer - Interrupt Status Flag"]
pub type ISF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF8_A, O>;
impl<'a, const O: u8> ISF8_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF8_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF8_A::_1)
    }
}
#[doc = "Field `ISF9` reader - Interrupt Status Flag"]
pub type ISF9_R = crate::BitReader<ISF9_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF9_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF9_A> for bool {
    #[inline(always)]
    fn from(variant: ISF9_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF9_A {
        match self.bits {
            false => ISF9_A::_0,
            true => ISF9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF9_A::_1
    }
}
#[doc = "Field `ISF9` writer - Interrupt Status Flag"]
pub type ISF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF9_A, O>;
impl<'a, const O: u8> ISF9_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF9_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF9_A::_1)
    }
}
#[doc = "Field `ISF10` reader - Interrupt Status Flag"]
pub type ISF10_R = crate::BitReader<ISF10_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF10_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF10_A> for bool {
    #[inline(always)]
    fn from(variant: ISF10_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF10_A {
        match self.bits {
            false => ISF10_A::_0,
            true => ISF10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF10_A::_1
    }
}
#[doc = "Field `ISF10` writer - Interrupt Status Flag"]
pub type ISF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF10_A, O>;
impl<'a, const O: u8> ISF10_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF10_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF10_A::_1)
    }
}
#[doc = "Field `ISF11` reader - Interrupt Status Flag"]
pub type ISF11_R = crate::BitReader<ISF11_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF11_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF11_A> for bool {
    #[inline(always)]
    fn from(variant: ISF11_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF11_A {
        match self.bits {
            false => ISF11_A::_0,
            true => ISF11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF11_A::_1
    }
}
#[doc = "Field `ISF11` writer - Interrupt Status Flag"]
pub type ISF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF11_A, O>;
impl<'a, const O: u8> ISF11_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF11_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF11_A::_1)
    }
}
#[doc = "Field `ISF12` reader - Interrupt Status Flag"]
pub type ISF12_R = crate::BitReader<ISF12_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF12_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF12_A> for bool {
    #[inline(always)]
    fn from(variant: ISF12_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF12_A {
        match self.bits {
            false => ISF12_A::_0,
            true => ISF12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF12_A::_1
    }
}
#[doc = "Field `ISF12` writer - Interrupt Status Flag"]
pub type ISF12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF12_A, O>;
impl<'a, const O: u8> ISF12_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF12_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF12_A::_1)
    }
}
#[doc = "Field `ISF13` reader - Interrupt Status Flag"]
pub type ISF13_R = crate::BitReader<ISF13_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF13_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF13_A> for bool {
    #[inline(always)]
    fn from(variant: ISF13_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF13_A {
        match self.bits {
            false => ISF13_A::_0,
            true => ISF13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF13_A::_1
    }
}
#[doc = "Field `ISF13` writer - Interrupt Status Flag"]
pub type ISF13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF13_A, O>;
impl<'a, const O: u8> ISF13_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF13_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF13_A::_1)
    }
}
#[doc = "Field `ISF14` reader - Interrupt Status Flag"]
pub type ISF14_R = crate::BitReader<ISF14_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF14_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF14_A> for bool {
    #[inline(always)]
    fn from(variant: ISF14_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF14_A {
        match self.bits {
            false => ISF14_A::_0,
            true => ISF14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF14_A::_1
    }
}
#[doc = "Field `ISF14` writer - Interrupt Status Flag"]
pub type ISF14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF14_A, O>;
impl<'a, const O: u8> ISF14_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF14_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF14_A::_1)
    }
}
#[doc = "Field `ISF15` reader - Interrupt Status Flag"]
pub type ISF15_R = crate::BitReader<ISF15_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF15_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF15_A> for bool {
    #[inline(always)]
    fn from(variant: ISF15_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF15_A {
        match self.bits {
            false => ISF15_A::_0,
            true => ISF15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF15_A::_1
    }
}
#[doc = "Field `ISF15` writer - Interrupt Status Flag"]
pub type ISF15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF15_A, O>;
impl<'a, const O: u8> ISF15_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF15_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF15_A::_1)
    }
}
#[doc = "Field `ISF16` reader - Interrupt Status Flag"]
pub type ISF16_R = crate::BitReader<ISF16_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF16_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF16_A> for bool {
    #[inline(always)]
    fn from(variant: ISF16_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF16_A {
        match self.bits {
            false => ISF16_A::_0,
            true => ISF16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF16_A::_1
    }
}
#[doc = "Field `ISF16` writer - Interrupt Status Flag"]
pub type ISF16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF16_A, O>;
impl<'a, const O: u8> ISF16_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF16_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF16_A::_1)
    }
}
#[doc = "Field `ISF17` reader - Interrupt Status Flag"]
pub type ISF17_R = crate::BitReader<ISF17_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF17_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF17_A> for bool {
    #[inline(always)]
    fn from(variant: ISF17_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF17_A {
        match self.bits {
            false => ISF17_A::_0,
            true => ISF17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF17_A::_1
    }
}
#[doc = "Field `ISF17` writer - Interrupt Status Flag"]
pub type ISF17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF17_A, O>;
impl<'a, const O: u8> ISF17_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF17_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF17_A::_1)
    }
}
#[doc = "Field `ISF18` reader - Interrupt Status Flag"]
pub type ISF18_R = crate::BitReader<ISF18_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF18_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF18_A> for bool {
    #[inline(always)]
    fn from(variant: ISF18_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF18_A {
        match self.bits {
            false => ISF18_A::_0,
            true => ISF18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF18_A::_1
    }
}
#[doc = "Field `ISF18` writer - Interrupt Status Flag"]
pub type ISF18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF18_A, O>;
impl<'a, const O: u8> ISF18_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF18_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF18_A::_1)
    }
}
#[doc = "Field `ISF19` reader - Interrupt Status Flag"]
pub type ISF19_R = crate::BitReader<ISF19_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF19_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF19_A> for bool {
    #[inline(always)]
    fn from(variant: ISF19_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF19_A {
        match self.bits {
            false => ISF19_A::_0,
            true => ISF19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF19_A::_1
    }
}
#[doc = "Field `ISF19` writer - Interrupt Status Flag"]
pub type ISF19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF19_A, O>;
impl<'a, const O: u8> ISF19_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF19_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF19_A::_1)
    }
}
#[doc = "Field `ISF20` reader - Interrupt Status Flag"]
pub type ISF20_R = crate::BitReader<ISF20_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF20_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF20_A> for bool {
    #[inline(always)]
    fn from(variant: ISF20_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF20_A {
        match self.bits {
            false => ISF20_A::_0,
            true => ISF20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF20_A::_1
    }
}
#[doc = "Field `ISF20` writer - Interrupt Status Flag"]
pub type ISF20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF20_A, O>;
impl<'a, const O: u8> ISF20_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF20_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF20_A::_1)
    }
}
#[doc = "Field `ISF21` reader - Interrupt Status Flag"]
pub type ISF21_R = crate::BitReader<ISF21_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF21_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF21_A> for bool {
    #[inline(always)]
    fn from(variant: ISF21_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF21_A {
        match self.bits {
            false => ISF21_A::_0,
            true => ISF21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF21_A::_1
    }
}
#[doc = "Field `ISF21` writer - Interrupt Status Flag"]
pub type ISF21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF21_A, O>;
impl<'a, const O: u8> ISF21_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF21_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF21_A::_1)
    }
}
#[doc = "Field `ISF22` reader - Interrupt Status Flag"]
pub type ISF22_R = crate::BitReader<ISF22_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF22_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF22_A> for bool {
    #[inline(always)]
    fn from(variant: ISF22_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF22_A {
        match self.bits {
            false => ISF22_A::_0,
            true => ISF22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF22_A::_1
    }
}
#[doc = "Field `ISF22` writer - Interrupt Status Flag"]
pub type ISF22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF22_A, O>;
impl<'a, const O: u8> ISF22_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF22_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF22_A::_1)
    }
}
#[doc = "Field `ISF23` reader - Interrupt Status Flag"]
pub type ISF23_R = crate::BitReader<ISF23_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF23_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF23_A> for bool {
    #[inline(always)]
    fn from(variant: ISF23_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF23_A {
        match self.bits {
            false => ISF23_A::_0,
            true => ISF23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF23_A::_1
    }
}
#[doc = "Field `ISF23` writer - Interrupt Status Flag"]
pub type ISF23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF23_A, O>;
impl<'a, const O: u8> ISF23_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF23_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF23_A::_1)
    }
}
#[doc = "Field `ISF24` reader - Interrupt Status Flag"]
pub type ISF24_R = crate::BitReader<ISF24_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF24_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF24_A> for bool {
    #[inline(always)]
    fn from(variant: ISF24_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF24_A {
        match self.bits {
            false => ISF24_A::_0,
            true => ISF24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF24_A::_1
    }
}
#[doc = "Field `ISF24` writer - Interrupt Status Flag"]
pub type ISF24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF24_A, O>;
impl<'a, const O: u8> ISF24_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF24_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF24_A::_1)
    }
}
#[doc = "Field `ISF25` reader - Interrupt Status Flag"]
pub type ISF25_R = crate::BitReader<ISF25_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF25_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF25_A> for bool {
    #[inline(always)]
    fn from(variant: ISF25_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF25_A {
        match self.bits {
            false => ISF25_A::_0,
            true => ISF25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF25_A::_1
    }
}
#[doc = "Field `ISF25` writer - Interrupt Status Flag"]
pub type ISF25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF25_A, O>;
impl<'a, const O: u8> ISF25_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF25_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF25_A::_1)
    }
}
#[doc = "Field `ISF26` reader - Interrupt Status Flag"]
pub type ISF26_R = crate::BitReader<ISF26_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF26_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF26_A> for bool {
    #[inline(always)]
    fn from(variant: ISF26_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF26_A {
        match self.bits {
            false => ISF26_A::_0,
            true => ISF26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF26_A::_1
    }
}
#[doc = "Field `ISF26` writer - Interrupt Status Flag"]
pub type ISF26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF26_A, O>;
impl<'a, const O: u8> ISF26_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF26_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF26_A::_1)
    }
}
#[doc = "Field `ISF27` reader - Interrupt Status Flag"]
pub type ISF27_R = crate::BitReader<ISF27_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF27_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF27_A> for bool {
    #[inline(always)]
    fn from(variant: ISF27_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF27_A {
        match self.bits {
            false => ISF27_A::_0,
            true => ISF27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF27_A::_1
    }
}
#[doc = "Field `ISF27` writer - Interrupt Status Flag"]
pub type ISF27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF27_A, O>;
impl<'a, const O: u8> ISF27_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF27_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF27_A::_1)
    }
}
#[doc = "Field `ISF28` reader - Interrupt Status Flag"]
pub type ISF28_R = crate::BitReader<ISF28_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF28_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF28_A> for bool {
    #[inline(always)]
    fn from(variant: ISF28_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF28_A {
        match self.bits {
            false => ISF28_A::_0,
            true => ISF28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF28_A::_1
    }
}
#[doc = "Field `ISF28` writer - Interrupt Status Flag"]
pub type ISF28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF28_A, O>;
impl<'a, const O: u8> ISF28_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF28_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF28_A::_1)
    }
}
#[doc = "Field `ISF29` reader - Interrupt Status Flag"]
pub type ISF29_R = crate::BitReader<ISF29_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF29_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF29_A> for bool {
    #[inline(always)]
    fn from(variant: ISF29_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF29_A {
        match self.bits {
            false => ISF29_A::_0,
            true => ISF29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF29_A::_1
    }
}
#[doc = "Field `ISF29` writer - Interrupt Status Flag"]
pub type ISF29_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF29_A, O>;
impl<'a, const O: u8> ISF29_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF29_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF29_A::_1)
    }
}
#[doc = "Field `ISF30` reader - Interrupt Status Flag"]
pub type ISF30_R = crate::BitReader<ISF30_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF30_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF30_A> for bool {
    #[inline(always)]
    fn from(variant: ISF30_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF30_A {
        match self.bits {
            false => ISF30_A::_0,
            true => ISF30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF30_A::_1
    }
}
#[doc = "Field `ISF30` writer - Interrupt Status Flag"]
pub type ISF30_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF30_A, O>;
impl<'a, const O: u8> ISF30_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF30_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF30_A::_1)
    }
}
#[doc = "Field `ISF31` reader - Interrupt Status Flag"]
pub type ISF31_R = crate::BitReader<ISF31_A>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISF31_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF31_A> for bool {
    #[inline(always)]
    fn from(variant: ISF31_A) -> Self {
        variant as u8 != 0
    }
}
impl ISF31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF31_A {
        match self.bits {
            false => ISF31_A::_0,
            true => ISF31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF31_A::_1
    }
}
#[doc = "Field `ISF31` writer - Interrupt Status Flag"]
pub type ISF31_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISFR_SPEC, ISF31_A, O>;
impl<'a, const O: u8> ISF31_W<'a, O> {
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF31_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf0(&self) -> ISF0_R {
        ISF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf1(&self) -> ISF1_R {
        ISF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf2(&self) -> ISF2_R {
        ISF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf3(&self) -> ISF3_R {
        ISF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf4(&self) -> ISF4_R {
        ISF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf5(&self) -> ISF5_R {
        ISF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf6(&self) -> ISF6_R {
        ISF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf7(&self) -> ISF7_R {
        ISF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf8(&self) -> ISF8_R {
        ISF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf9(&self) -> ISF9_R {
        ISF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf10(&self) -> ISF10_R {
        ISF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf11(&self) -> ISF11_R {
        ISF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf12(&self) -> ISF12_R {
        ISF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf13(&self) -> ISF13_R {
        ISF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf14(&self) -> ISF14_R {
        ISF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf15(&self) -> ISF15_R {
        ISF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf16(&self) -> ISF16_R {
        ISF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf17(&self) -> ISF17_R {
        ISF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf18(&self) -> ISF18_R {
        ISF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf19(&self) -> ISF19_R {
        ISF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf20(&self) -> ISF20_R {
        ISF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf21(&self) -> ISF21_R {
        ISF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf22(&self) -> ISF22_R {
        ISF22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf23(&self) -> ISF23_R {
        ISF23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf24(&self) -> ISF24_R {
        ISF24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf25(&self) -> ISF25_R {
        ISF25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf26(&self) -> ISF26_R {
        ISF26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf27(&self) -> ISF27_R {
        ISF27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf28(&self) -> ISF28_R {
        ISF28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf29(&self) -> ISF29_R {
        ISF29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf30(&self) -> ISF30_R {
        ISF30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf31(&self) -> ISF31_R {
        ISF31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf0(&mut self) -> ISF0_W<0> {
        ISF0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf1(&mut self) -> ISF1_W<1> {
        ISF1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf2(&mut self) -> ISF2_W<2> {
        ISF2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf3(&mut self) -> ISF3_W<3> {
        ISF3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf4(&mut self) -> ISF4_W<4> {
        ISF4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf5(&mut self) -> ISF5_W<5> {
        ISF5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf6(&mut self) -> ISF6_W<6> {
        ISF6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf7(&mut self) -> ISF7_W<7> {
        ISF7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf8(&mut self) -> ISF8_W<8> {
        ISF8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf9(&mut self) -> ISF9_W<9> {
        ISF9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf10(&mut self) -> ISF10_W<10> {
        ISF10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf11(&mut self) -> ISF11_W<11> {
        ISF11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf12(&mut self) -> ISF12_W<12> {
        ISF12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf13(&mut self) -> ISF13_W<13> {
        ISF13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf14(&mut self) -> ISF14_W<14> {
        ISF14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf15(&mut self) -> ISF15_W<15> {
        ISF15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf16(&mut self) -> ISF16_W<16> {
        ISF16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf17(&mut self) -> ISF17_W<17> {
        ISF17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf18(&mut self) -> ISF18_W<18> {
        ISF18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf19(&mut self) -> ISF19_W<19> {
        ISF19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf20(&mut self) -> ISF20_W<20> {
        ISF20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf21(&mut self) -> ISF21_W<21> {
        ISF21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf22(&mut self) -> ISF22_W<22> {
        ISF22_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf23(&mut self) -> ISF23_W<23> {
        ISF23_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf24(&mut self) -> ISF24_W<24> {
        ISF24_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf25(&mut self) -> ISF25_W<25> {
        ISF25_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf26(&mut self) -> ISF26_W<26> {
        ISF26_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf27(&mut self) -> ISF27_W<27> {
        ISF27_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf28(&mut self) -> ISF28_W<28> {
        ISF28_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf29(&mut self) -> ISF29_W<29> {
        ISF29_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf30(&mut self) -> ISF30_W<30> {
        ISF30_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf31(&mut self) -> ISF31_W<31> {
        ISF31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isfr](index.html) module"]
pub struct ISFR_SPEC;
impl crate::RegisterSpec for ISFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isfr::R](R) reader structure"]
impl crate::Readable for ISFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isfr::W](W) writer structure"]
impl crate::Writable for ISFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISFR to value 0"]
impl crate::Resettable for ISFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
