#[doc = "Register `PFB0CR` reader"]
pub struct R(crate::R<PFB0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFB0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFB0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFB0CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFB0CR` writer"]
pub struct W(crate::W<PFB0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFB0CR_SPEC>;
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
impl From<crate::W<PFB0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFB0CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B0SEBE` reader - Bank 0 Single Entry Buffer Enable"]
pub type B0SEBE_R = crate::BitReader<B0SEBE_A>;
#[doc = "Bank 0 Single Entry Buffer Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0SEBE_A {
    #[doc = "0: Single entry buffer is disabled."]
    _0 = 0,
    #[doc = "1: Single entry buffer is enabled."]
    _1 = 1,
}
impl From<B0SEBE_A> for bool {
    #[inline(always)]
    fn from(variant: B0SEBE_A) -> Self {
        variant as u8 != 0
    }
}
impl B0SEBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B0SEBE_A {
        match self.bits {
            false => B0SEBE_A::_0,
            true => B0SEBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0SEBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0SEBE_A::_1
    }
}
#[doc = "Field `B0SEBE` writer - Bank 0 Single Entry Buffer Enable"]
pub type B0SEBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFB0CR_SPEC, B0SEBE_A, O>;
impl<'a, const O: u8> B0SEBE_W<'a, O> {
    #[doc = "Single entry buffer is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0SEBE_A::_0)
    }
    #[doc = "Single entry buffer is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0SEBE_A::_1)
    }
}
#[doc = "Field `B0IPE` reader - Bank 0 Instruction Prefetch Enable"]
pub type B0IPE_R = crate::BitReader<B0IPE_A>;
#[doc = "Bank 0 Instruction Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0IPE_A {
    #[doc = "0: Do not prefetch in response to instruction fetches."]
    _0 = 0,
    #[doc = "1: Enable prefetches in response to instruction fetches."]
    _1 = 1,
}
impl From<B0IPE_A> for bool {
    #[inline(always)]
    fn from(variant: B0IPE_A) -> Self {
        variant as u8 != 0
    }
}
impl B0IPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B0IPE_A {
        match self.bits {
            false => B0IPE_A::_0,
            true => B0IPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0IPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0IPE_A::_1
    }
}
#[doc = "Field `B0IPE` writer - Bank 0 Instruction Prefetch Enable"]
pub type B0IPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFB0CR_SPEC, B0IPE_A, O>;
impl<'a, const O: u8> B0IPE_W<'a, O> {
    #[doc = "Do not prefetch in response to instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0IPE_A::_0)
    }
    #[doc = "Enable prefetches in response to instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0IPE_A::_1)
    }
}
#[doc = "Field `B0DPE` reader - Bank 0 Data Prefetch Enable"]
pub type B0DPE_R = crate::BitReader<B0DPE_A>;
#[doc = "Bank 0 Data Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0DPE_A {
    #[doc = "0: Do not prefetch in response to data references."]
    _0 = 0,
    #[doc = "1: Enable prefetches in response to data references."]
    _1 = 1,
}
impl From<B0DPE_A> for bool {
    #[inline(always)]
    fn from(variant: B0DPE_A) -> Self {
        variant as u8 != 0
    }
}
impl B0DPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B0DPE_A {
        match self.bits {
            false => B0DPE_A::_0,
            true => B0DPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0DPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0DPE_A::_1
    }
}
#[doc = "Field `B0DPE` writer - Bank 0 Data Prefetch Enable"]
pub type B0DPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFB0CR_SPEC, B0DPE_A, O>;
impl<'a, const O: u8> B0DPE_W<'a, O> {
    #[doc = "Do not prefetch in response to data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0DPE_A::_0)
    }
    #[doc = "Enable prefetches in response to data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0DPE_A::_1)
    }
}
#[doc = "Field `B0ICE` reader - Bank 0 Instruction Cache Enable"]
pub type B0ICE_R = crate::BitReader<B0ICE_A>;
#[doc = "Bank 0 Instruction Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0ICE_A {
    #[doc = "0: Do not cache instruction fetches."]
    _0 = 0,
    #[doc = "1: Cache instruction fetches."]
    _1 = 1,
}
impl From<B0ICE_A> for bool {
    #[inline(always)]
    fn from(variant: B0ICE_A) -> Self {
        variant as u8 != 0
    }
}
impl B0ICE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B0ICE_A {
        match self.bits {
            false => B0ICE_A::_0,
            true => B0ICE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0ICE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0ICE_A::_1
    }
}
#[doc = "Field `B0ICE` writer - Bank 0 Instruction Cache Enable"]
pub type B0ICE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFB0CR_SPEC, B0ICE_A, O>;
impl<'a, const O: u8> B0ICE_W<'a, O> {
    #[doc = "Do not cache instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0ICE_A::_0)
    }
    #[doc = "Cache instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0ICE_A::_1)
    }
}
#[doc = "Field `B0DCE` reader - Bank 0 Data Cache Enable"]
pub type B0DCE_R = crate::BitReader<B0DCE_A>;
#[doc = "Bank 0 Data Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B0DCE_A {
    #[doc = "0: Do not cache data references."]
    _0 = 0,
    #[doc = "1: Cache data references."]
    _1 = 1,
}
impl From<B0DCE_A> for bool {
    #[inline(always)]
    fn from(variant: B0DCE_A) -> Self {
        variant as u8 != 0
    }
}
impl B0DCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B0DCE_A {
        match self.bits {
            false => B0DCE_A::_0,
            true => B0DCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B0DCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B0DCE_A::_1
    }
}
#[doc = "Field `B0DCE` writer - Bank 0 Data Cache Enable"]
pub type B0DCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFB0CR_SPEC, B0DCE_A, O>;
impl<'a, const O: u8> B0DCE_W<'a, O> {
    #[doc = "Do not cache data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B0DCE_A::_0)
    }
    #[doc = "Cache data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B0DCE_A::_1)
    }
}
#[doc = "Field `CRC` reader - Cache Replacement Control"]
pub type CRC_R = crate::FieldReader<u8, CRC_A>;
#[doc = "Cache Replacement Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRC_A {
    #[doc = "0: LRU replacement algorithm per set across all four ways"]
    _000 = 0,
    #[doc = "2: Independent LRU with ways \\[0-1\\]
for ifetches, \\[2-3\\]
for data"]
    _010 = 2,
    #[doc = "3: Independent LRU with ways \\[0-2\\]
for ifetches, \\[3\\]
for data"]
    _011 = 3,
}
impl From<CRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as _
    }
}
impl CRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRC_A> {
        match self.bits {
            0 => Some(CRC_A::_000),
            2 => Some(CRC_A::_010),
            3 => Some(CRC_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CRC_A::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CRC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CRC_A::_011
    }
}
#[doc = "Field `CRC` writer - Cache Replacement Control"]
pub type CRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PFB0CR_SPEC, u8, CRC_A, 3, O>;
impl<'a, const O: u8> CRC_W<'a, O> {
    #[doc = "LRU replacement algorithm per set across all four ways"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CRC_A::_000)
    }
    #[doc = "Independent LRU with ways \\[0-1\\]
for ifetches, \\[2-3\\]
for data"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CRC_A::_010)
    }
    #[doc = "Independent LRU with ways \\[0-2\\]
for ifetches, \\[3\\]
for data"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CRC_A::_011)
    }
}
#[doc = "Field `B0MW` reader - Bank 0 Memory Width"]
pub type B0MW_R = crate::FieldReader<u8, B0MW_A>;
#[doc = "Bank 0 Memory Width\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum B0MW_A {
    #[doc = "0: 32 bits"]
    _00 = 0,
    #[doc = "1: 64 bits"]
    _01 = 1,
    #[doc = "2: 128 bits"]
    _10 = 2,
}
impl From<B0MW_A> for u8 {
    #[inline(always)]
    fn from(variant: B0MW_A) -> Self {
        variant as _
    }
}
impl B0MW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<B0MW_A> {
        match self.bits {
            0 => Some(B0MW_A::_00),
            1 => Some(B0MW_A::_01),
            2 => Some(B0MW_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == B0MW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == B0MW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == B0MW_A::_10
    }
}
#[doc = "Invalidate Prefetch Speculation Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S_B_INV_AW {
    #[doc = "0: Speculation buffer and single entry buffer are not affected."]
    _0 = 0,
    #[doc = "1: Invalidate (clear) speculation buffer and single entry buffer."]
    _1 = 1,
}
impl From<S_B_INV_AW> for bool {
    #[inline(always)]
    fn from(variant: S_B_INV_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_B_INV` writer - Invalidate Prefetch Speculation Buffer"]
pub type S_B_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFB0CR_SPEC, S_B_INV_AW, O>;
impl<'a, const O: u8> S_B_INV_W<'a, O> {
    #[doc = "Speculation buffer and single entry buffer are not affected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(S_B_INV_AW::_0)
    }
    #[doc = "Invalidate (clear) speculation buffer and single entry buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(S_B_INV_AW::_1)
    }
}
#[doc = "Cache Invalidate Way x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CINV_WAY_AW {
    #[doc = "0: No cache way invalidation for the corresponding cache"]
    _0 = 0,
    #[doc = "1: Invalidate cache way for the corresponding cache: clear the tag, data, and vld bits of ways selected"]
    _1 = 1,
}
impl From<CINV_WAY_AW> for u8 {
    #[inline(always)]
    fn from(variant: CINV_WAY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CINV_WAY` writer - Cache Invalidate Way x"]
pub type CINV_WAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PFB0CR_SPEC, u8, CINV_WAY_AW, 4, O>;
impl<'a, const O: u8> CINV_WAY_W<'a, O> {
    #[doc = "No cache way invalidation for the corresponding cache"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINV_WAY_AW::_0)
    }
    #[doc = "Invalidate cache way for the corresponding cache: clear the tag, data, and vld bits of ways selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINV_WAY_AW::_1)
    }
}
#[doc = "Field `CLCK_WAY` reader - Cache Lock Way x"]
pub type CLCK_WAY_R = crate::FieldReader<u8, CLCK_WAY_A>;
#[doc = "Cache Lock Way x\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLCK_WAY_A {
    #[doc = "0: Cache way is unlocked and may be displaced"]
    _0 = 0,
    #[doc = "1: Cache way is locked and its contents are not displaced"]
    _1 = 1,
}
impl From<CLCK_WAY_A> for u8 {
    #[inline(always)]
    fn from(variant: CLCK_WAY_A) -> Self {
        variant as _
    }
}
impl CLCK_WAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLCK_WAY_A> {
        match self.bits {
            0 => Some(CLCK_WAY_A::_0),
            1 => Some(CLCK_WAY_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLCK_WAY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLCK_WAY_A::_1
    }
}
#[doc = "Field `CLCK_WAY` writer - Cache Lock Way x"]
pub type CLCK_WAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PFB0CR_SPEC, u8, CLCK_WAY_A, 4, O>;
impl<'a, const O: u8> CLCK_WAY_W<'a, O> {
    #[doc = "Cache way is unlocked and may be displaced"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLCK_WAY_A::_0)
    }
    #[doc = "Cache way is locked and its contents are not displaced"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLCK_WAY_A::_1)
    }
}
#[doc = "Field `B0RWSC` reader - Bank 0 Read Wait State Control"]
pub type B0RWSC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Bank 0 Single Entry Buffer Enable"]
    #[inline(always)]
    pub fn b0sebe(&self) -> B0SEBE_R {
        B0SEBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 0 Instruction Prefetch Enable"]
    #[inline(always)]
    pub fn b0ipe(&self) -> B0IPE_R {
        B0IPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 0 Data Prefetch Enable"]
    #[inline(always)]
    pub fn b0dpe(&self) -> B0DPE_R {
        B0DPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 0 Instruction Cache Enable"]
    #[inline(always)]
    pub fn b0ice(&self) -> B0ICE_R {
        B0ICE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bank 0 Data Cache Enable"]
    #[inline(always)]
    pub fn b0dce(&self) -> B0DCE_R {
        B0DCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Cache Replacement Control"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Bank 0 Memory Width"]
    #[inline(always)]
    pub fn b0mw(&self) -> B0MW_R {
        B0MW_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Cache Lock Way x"]
    #[inline(always)]
    pub fn clck_way(&self) -> CLCK_WAY_R {
        CLCK_WAY_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Bank 0 Read Wait State Control"]
    #[inline(always)]
    pub fn b0rwsc(&self) -> B0RWSC_R {
        B0RWSC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 0 Single Entry Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0sebe(&mut self) -> B0SEBE_W<0> {
        B0SEBE_W::new(self)
    }
    #[doc = "Bit 1 - Bank 0 Instruction Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0ipe(&mut self) -> B0IPE_W<1> {
        B0IPE_W::new(self)
    }
    #[doc = "Bit 2 - Bank 0 Data Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0dpe(&mut self) -> B0DPE_W<2> {
        B0DPE_W::new(self)
    }
    #[doc = "Bit 3 - Bank 0 Instruction Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0ice(&mut self) -> B0ICE_W<3> {
        B0ICE_W::new(self)
    }
    #[doc = "Bit 4 - Bank 0 Data Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn b0dce(&mut self) -> B0DCE_W<4> {
        B0DCE_W::new(self)
    }
    #[doc = "Bits 5:7 - Cache Replacement Control"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<5> {
        CRC_W::new(self)
    }
    #[doc = "Bit 19 - Invalidate Prefetch Speculation Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn s_b_inv(&mut self) -> S_B_INV_W<19> {
        S_B_INV_W::new(self)
    }
    #[doc = "Bits 20:23 - Cache Invalidate Way x"]
    #[inline(always)]
    #[must_use]
    pub fn cinv_way(&mut self) -> CINV_WAY_W<20> {
        CINV_WAY_W::new(self)
    }
    #[doc = "Bits 24:27 - Cache Lock Way x"]
    #[inline(always)]
    #[must_use]
    pub fn clck_way(&mut self) -> CLCK_WAY_W<24> {
        CLCK_WAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfb0cr](index.html) module"]
pub struct PFB0CR_SPEC;
impl crate::RegisterSpec for PFB0CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfb0cr::R](R) reader structure"]
impl crate::Readable for PFB0CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfb0cr::W](W) writer structure"]
impl crate::Writable for PFB0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFB0CR to value 0x3004_001f"]
impl crate::Resettable for PFB0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3004_001f;
}
