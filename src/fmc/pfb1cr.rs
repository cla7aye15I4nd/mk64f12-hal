#[doc = "Register `PFB1CR` reader"]
pub struct R(crate::R<PFB1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFB1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFB1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFB1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFB1CR` writer"]
pub struct W(crate::W<PFB1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFB1CR_SPEC>;
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
impl From<crate::W<PFB1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFB1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1SEBE` reader - Bank 1 Single Entry Buffer Enable"]
pub type B1SEBE_R = crate::BitReader<B1SEBE_A>;
#[doc = "Bank 1 Single Entry Buffer Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1SEBE_A {
    #[doc = "0: Single entry buffer is disabled."]
    _0 = 0,
    #[doc = "1: Single entry buffer is enabled."]
    _1 = 1,
}
impl From<B1SEBE_A> for bool {
    #[inline(always)]
    fn from(variant: B1SEBE_A) -> Self {
        variant as u8 != 0
    }
}
impl B1SEBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B1SEBE_A {
        match self.bits {
            false => B1SEBE_A::_0,
            true => B1SEBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1SEBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1SEBE_A::_1
    }
}
#[doc = "Field `B1SEBE` writer - Bank 1 Single Entry Buffer Enable"]
pub type B1SEBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFB1CR_SPEC, B1SEBE_A, O>;
impl<'a, const O: u8> B1SEBE_W<'a, O> {
    #[doc = "Single entry buffer is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1SEBE_A::_0)
    }
    #[doc = "Single entry buffer is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1SEBE_A::_1)
    }
}
#[doc = "Field `B1IPE` reader - Bank 1 Instruction Prefetch Enable"]
pub type B1IPE_R = crate::BitReader<B1IPE_A>;
#[doc = "Bank 1 Instruction Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1IPE_A {
    #[doc = "0: Do not prefetch in response to instruction fetches."]
    _0 = 0,
    #[doc = "1: Enable prefetches in response to instruction fetches."]
    _1 = 1,
}
impl From<B1IPE_A> for bool {
    #[inline(always)]
    fn from(variant: B1IPE_A) -> Self {
        variant as u8 != 0
    }
}
impl B1IPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B1IPE_A {
        match self.bits {
            false => B1IPE_A::_0,
            true => B1IPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1IPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1IPE_A::_1
    }
}
#[doc = "Field `B1IPE` writer - Bank 1 Instruction Prefetch Enable"]
pub type B1IPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFB1CR_SPEC, B1IPE_A, O>;
impl<'a, const O: u8> B1IPE_W<'a, O> {
    #[doc = "Do not prefetch in response to instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1IPE_A::_0)
    }
    #[doc = "Enable prefetches in response to instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1IPE_A::_1)
    }
}
#[doc = "Field `B1DPE` reader - Bank 1 Data Prefetch Enable"]
pub type B1DPE_R = crate::BitReader<B1DPE_A>;
#[doc = "Bank 1 Data Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1DPE_A {
    #[doc = "0: Do not prefetch in response to data references."]
    _0 = 0,
    #[doc = "1: Enable prefetches in response to data references."]
    _1 = 1,
}
impl From<B1DPE_A> for bool {
    #[inline(always)]
    fn from(variant: B1DPE_A) -> Self {
        variant as u8 != 0
    }
}
impl B1DPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B1DPE_A {
        match self.bits {
            false => B1DPE_A::_0,
            true => B1DPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1DPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1DPE_A::_1
    }
}
#[doc = "Field `B1DPE` writer - Bank 1 Data Prefetch Enable"]
pub type B1DPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFB1CR_SPEC, B1DPE_A, O>;
impl<'a, const O: u8> B1DPE_W<'a, O> {
    #[doc = "Do not prefetch in response to data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1DPE_A::_0)
    }
    #[doc = "Enable prefetches in response to data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1DPE_A::_1)
    }
}
#[doc = "Field `B1ICE` reader - Bank 1 Instruction Cache Enable"]
pub type B1ICE_R = crate::BitReader<B1ICE_A>;
#[doc = "Bank 1 Instruction Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1ICE_A {
    #[doc = "0: Do not cache instruction fetches."]
    _0 = 0,
    #[doc = "1: Cache instruction fetches."]
    _1 = 1,
}
impl From<B1ICE_A> for bool {
    #[inline(always)]
    fn from(variant: B1ICE_A) -> Self {
        variant as u8 != 0
    }
}
impl B1ICE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B1ICE_A {
        match self.bits {
            false => B1ICE_A::_0,
            true => B1ICE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1ICE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1ICE_A::_1
    }
}
#[doc = "Field `B1ICE` writer - Bank 1 Instruction Cache Enable"]
pub type B1ICE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFB1CR_SPEC, B1ICE_A, O>;
impl<'a, const O: u8> B1ICE_W<'a, O> {
    #[doc = "Do not cache instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1ICE_A::_0)
    }
    #[doc = "Cache instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1ICE_A::_1)
    }
}
#[doc = "Field `B1DCE` reader - Bank 1 Data Cache Enable"]
pub type B1DCE_R = crate::BitReader<B1DCE_A>;
#[doc = "Bank 1 Data Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1DCE_A {
    #[doc = "0: Do not cache data references."]
    _0 = 0,
    #[doc = "1: Cache data references."]
    _1 = 1,
}
impl From<B1DCE_A> for bool {
    #[inline(always)]
    fn from(variant: B1DCE_A) -> Self {
        variant as u8 != 0
    }
}
impl B1DCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B1DCE_A {
        match self.bits {
            false => B1DCE_A::_0,
            true => B1DCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1DCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1DCE_A::_1
    }
}
#[doc = "Field `B1DCE` writer - Bank 1 Data Cache Enable"]
pub type B1DCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFB1CR_SPEC, B1DCE_A, O>;
impl<'a, const O: u8> B1DCE_W<'a, O> {
    #[doc = "Do not cache data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1DCE_A::_0)
    }
    #[doc = "Cache data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1DCE_A::_1)
    }
}
#[doc = "Field `B1MW` reader - Bank 1 Memory Width"]
pub type B1MW_R = crate::FieldReader<u8, B1MW_A>;
#[doc = "Bank 1 Memory Width\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum B1MW_A {
    #[doc = "0: 32 bits"]
    _00 = 0,
    #[doc = "1: 64 bits"]
    _01 = 1,
    #[doc = "2: 128 bits"]
    _10 = 2,
}
impl From<B1MW_A> for u8 {
    #[inline(always)]
    fn from(variant: B1MW_A) -> Self {
        variant as _
    }
}
impl B1MW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<B1MW_A> {
        match self.bits {
            0 => Some(B1MW_A::_00),
            1 => Some(B1MW_A::_01),
            2 => Some(B1MW_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == B1MW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == B1MW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == B1MW_A::_10
    }
}
#[doc = "Field `B1RWSC` reader - Bank 1 Read Wait State Control"]
pub type B1RWSC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Bank 1 Single Entry Buffer Enable"]
    #[inline(always)]
    pub fn b1sebe(&self) -> B1SEBE_R {
        B1SEBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 1 Instruction Prefetch Enable"]
    #[inline(always)]
    pub fn b1ipe(&self) -> B1IPE_R {
        B1IPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 Data Prefetch Enable"]
    #[inline(always)]
    pub fn b1dpe(&self) -> B1DPE_R {
        B1DPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 1 Instruction Cache Enable"]
    #[inline(always)]
    pub fn b1ice(&self) -> B1ICE_R {
        B1ICE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bank 1 Data Cache Enable"]
    #[inline(always)]
    pub fn b1dce(&self) -> B1DCE_R {
        B1DCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Bank 1 Memory Width"]
    #[inline(always)]
    pub fn b1mw(&self) -> B1MW_R {
        B1MW_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Bank 1 Read Wait State Control"]
    #[inline(always)]
    pub fn b1rwsc(&self) -> B1RWSC_R {
        B1RWSC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 Single Entry Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1sebe(&mut self) -> B1SEBE_W<0> {
        B1SEBE_W::new(self)
    }
    #[doc = "Bit 1 - Bank 1 Instruction Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1ipe(&mut self) -> B1IPE_W<1> {
        B1IPE_W::new(self)
    }
    #[doc = "Bit 2 - Bank 1 Data Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1dpe(&mut self) -> B1DPE_W<2> {
        B1DPE_W::new(self)
    }
    #[doc = "Bit 3 - Bank 1 Instruction Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1ice(&mut self) -> B1ICE_W<3> {
        B1ICE_W::new(self)
    }
    #[doc = "Bit 4 - Bank 1 Data Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b1dce(&mut self) -> B1DCE_W<4> {
        B1DCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfb1cr](index.html) module"]
pub struct PFB1CR_SPEC;
impl crate::RegisterSpec for PFB1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfb1cr::R](R) reader structure"]
impl crate::Readable for PFB1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfb1cr::W](W) writer structure"]
impl crate::Writable for PFB1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFB1CR to value 0x3004_001f"]
impl crate::Resettable for PFB1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3004_001f;
}
