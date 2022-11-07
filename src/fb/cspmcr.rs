#[doc = "Register `CSPMCR` reader"]
pub struct R(crate::R<CSPMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSPMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSPMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSPMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSPMCR` writer"]
pub struct W(crate::W<CSPMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSPMCR_SPEC>;
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
impl From<crate::W<CSPMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSPMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GROUP5` reader - FlexBus Signal Group 5 Multiplex control"]
pub type GROUP5_R = crate::FieldReader<u8, GROUP5_A>;
#[doc = "FlexBus Signal Group 5 Multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GROUP5_A {
    #[doc = "0: FB_TA"]
    _0000 = 0,
    #[doc = "1: FB_CS3 . You must also write 1b to CSCR\\[AA\\]."]
    _0001 = 1,
    #[doc = "2: FB_BE_7_0 . You must also write 1b to CSCR\\[AA\\]."]
    _0010 = 2,
}
impl From<GROUP5_A> for u8 {
    #[inline(always)]
    fn from(variant: GROUP5_A) -> Self {
        variant as _
    }
}
impl GROUP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GROUP5_A> {
        match self.bits {
            0 => Some(GROUP5_A::_0000),
            1 => Some(GROUP5_A::_0001),
            2 => Some(GROUP5_A::_0010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == GROUP5_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == GROUP5_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == GROUP5_A::_0010
    }
}
#[doc = "Field `GROUP5` writer - FlexBus Signal Group 5 Multiplex control"]
pub type GROUP5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSPMCR_SPEC, u8, GROUP5_A, 4, O>;
impl<'a, const O: u8> GROUP5_W<'a, O> {
    #[doc = "FB_TA"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP5_A::_0000)
    }
    #[doc = "FB_CS3 . You must also write 1b to CSCR\\[AA\\]."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP5_A::_0001)
    }
    #[doc = "FB_BE_7_0 . You must also write 1b to CSCR\\[AA\\]."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP5_A::_0010)
    }
}
#[doc = "Field `GROUP4` reader - FlexBus Signal Group 4 Multiplex control"]
pub type GROUP4_R = crate::FieldReader<u8, GROUP4_A>;
#[doc = "FlexBus Signal Group 4 Multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GROUP4_A {
    #[doc = "0: FB_TBST"]
    _0000 = 0,
    #[doc = "1: FB_CS2"]
    _0001 = 1,
    #[doc = "2: FB_BE_15_8"]
    _0010 = 2,
}
impl From<GROUP4_A> for u8 {
    #[inline(always)]
    fn from(variant: GROUP4_A) -> Self {
        variant as _
    }
}
impl GROUP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GROUP4_A> {
        match self.bits {
            0 => Some(GROUP4_A::_0000),
            1 => Some(GROUP4_A::_0001),
            2 => Some(GROUP4_A::_0010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == GROUP4_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == GROUP4_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == GROUP4_A::_0010
    }
}
#[doc = "Field `GROUP4` writer - FlexBus Signal Group 4 Multiplex control"]
pub type GROUP4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSPMCR_SPEC, u8, GROUP4_A, 4, O>;
impl<'a, const O: u8> GROUP4_W<'a, O> {
    #[doc = "FB_TBST"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP4_A::_0000)
    }
    #[doc = "FB_CS2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP4_A::_0001)
    }
    #[doc = "FB_BE_15_8"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP4_A::_0010)
    }
}
#[doc = "Field `GROUP3` reader - FlexBus Signal Group 3 Multiplex control"]
pub type GROUP3_R = crate::FieldReader<u8, GROUP3_A>;
#[doc = "FlexBus Signal Group 3 Multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GROUP3_A {
    #[doc = "0: FB_CS5"]
    _0000 = 0,
    #[doc = "1: FB_TSIZ1"]
    _0001 = 1,
    #[doc = "2: FB_BE_23_16"]
    _0010 = 2,
}
impl From<GROUP3_A> for u8 {
    #[inline(always)]
    fn from(variant: GROUP3_A) -> Self {
        variant as _
    }
}
impl GROUP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GROUP3_A> {
        match self.bits {
            0 => Some(GROUP3_A::_0000),
            1 => Some(GROUP3_A::_0001),
            2 => Some(GROUP3_A::_0010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == GROUP3_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == GROUP3_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == GROUP3_A::_0010
    }
}
#[doc = "Field `GROUP3` writer - FlexBus Signal Group 3 Multiplex control"]
pub type GROUP3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSPMCR_SPEC, u8, GROUP3_A, 4, O>;
impl<'a, const O: u8> GROUP3_W<'a, O> {
    #[doc = "FB_CS5"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP3_A::_0000)
    }
    #[doc = "FB_TSIZ1"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP3_A::_0001)
    }
    #[doc = "FB_BE_23_16"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP3_A::_0010)
    }
}
#[doc = "Field `GROUP2` reader - FlexBus Signal Group 2 Multiplex control"]
pub type GROUP2_R = crate::FieldReader<u8, GROUP2_A>;
#[doc = "FlexBus Signal Group 2 Multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GROUP2_A {
    #[doc = "0: FB_CS4"]
    _0000 = 0,
    #[doc = "1: FB_TSIZ0"]
    _0001 = 1,
    #[doc = "2: FB_BE_31_24"]
    _0010 = 2,
}
impl From<GROUP2_A> for u8 {
    #[inline(always)]
    fn from(variant: GROUP2_A) -> Self {
        variant as _
    }
}
impl GROUP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GROUP2_A> {
        match self.bits {
            0 => Some(GROUP2_A::_0000),
            1 => Some(GROUP2_A::_0001),
            2 => Some(GROUP2_A::_0010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == GROUP2_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == GROUP2_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == GROUP2_A::_0010
    }
}
#[doc = "Field `GROUP2` writer - FlexBus Signal Group 2 Multiplex control"]
pub type GROUP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSPMCR_SPEC, u8, GROUP2_A, 4, O>;
impl<'a, const O: u8> GROUP2_W<'a, O> {
    #[doc = "FB_CS4"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP2_A::_0000)
    }
    #[doc = "FB_TSIZ0"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP2_A::_0001)
    }
    #[doc = "FB_BE_31_24"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP2_A::_0010)
    }
}
#[doc = "Field `GROUP1` reader - FlexBus Signal Group 1 Multiplex control"]
pub type GROUP1_R = crate::FieldReader<u8, GROUP1_A>;
#[doc = "FlexBus Signal Group 1 Multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GROUP1_A {
    #[doc = "0: FB_ALE"]
    _0000 = 0,
    #[doc = "1: FB_CS1"]
    _0001 = 1,
    #[doc = "2: FB_TS"]
    _0010 = 2,
}
impl From<GROUP1_A> for u8 {
    #[inline(always)]
    fn from(variant: GROUP1_A) -> Self {
        variant as _
    }
}
impl GROUP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GROUP1_A> {
        match self.bits {
            0 => Some(GROUP1_A::_0000),
            1 => Some(GROUP1_A::_0001),
            2 => Some(GROUP1_A::_0010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == GROUP1_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == GROUP1_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == GROUP1_A::_0010
    }
}
#[doc = "Field `GROUP1` writer - FlexBus Signal Group 1 Multiplex control"]
pub type GROUP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSPMCR_SPEC, u8, GROUP1_A, 4, O>;
impl<'a, const O: u8> GROUP1_W<'a, O> {
    #[doc = "FB_ALE"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP1_A::_0000)
    }
    #[doc = "FB_CS1"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP1_A::_0001)
    }
    #[doc = "FB_TS"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP1_A::_0010)
    }
}
impl R {
    #[doc = "Bits 12:15 - FlexBus Signal Group 5 Multiplex control"]
    #[inline(always)]
    pub fn group5(&self) -> GROUP5_R {
        GROUP5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - FlexBus Signal Group 4 Multiplex control"]
    #[inline(always)]
    pub fn group4(&self) -> GROUP4_R {
        GROUP4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - FlexBus Signal Group 3 Multiplex control"]
    #[inline(always)]
    pub fn group3(&self) -> GROUP3_R {
        GROUP3_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - FlexBus Signal Group 2 Multiplex control"]
    #[inline(always)]
    pub fn group2(&self) -> GROUP2_R {
        GROUP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - FlexBus Signal Group 1 Multiplex control"]
    #[inline(always)]
    pub fn group1(&self) -> GROUP1_R {
        GROUP1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - FlexBus Signal Group 5 Multiplex control"]
    #[inline(always)]
    #[must_use]
    pub fn group5(&mut self) -> GROUP5_W<12> {
        GROUP5_W::new(self)
    }
    #[doc = "Bits 16:19 - FlexBus Signal Group 4 Multiplex control"]
    #[inline(always)]
    #[must_use]
    pub fn group4(&mut self) -> GROUP4_W<16> {
        GROUP4_W::new(self)
    }
    #[doc = "Bits 20:23 - FlexBus Signal Group 3 Multiplex control"]
    #[inline(always)]
    #[must_use]
    pub fn group3(&mut self) -> GROUP3_W<20> {
        GROUP3_W::new(self)
    }
    #[doc = "Bits 24:27 - FlexBus Signal Group 2 Multiplex control"]
    #[inline(always)]
    #[must_use]
    pub fn group2(&mut self) -> GROUP2_W<24> {
        GROUP2_W::new(self)
    }
    #[doc = "Bits 28:31 - FlexBus Signal Group 1 Multiplex control"]
    #[inline(always)]
    #[must_use]
    pub fn group1(&mut self) -> GROUP1_W<28> {
        GROUP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Select port Multiplexing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspmcr](index.html) module"]
pub struct CSPMCR_SPEC;
impl crate::RegisterSpec for CSPMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cspmcr::R](R) reader structure"]
impl crate::Readable for CSPMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cspmcr::W](W) writer structure"]
impl crate::Writable for CSPMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSPMCR to value 0"]
impl crate::Resettable for CSPMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
