#[doc = "Register `CSMR%s` reader"]
pub struct R(crate::R<CSMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSMR%s` writer"]
pub struct W(crate::W<CSMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSMR_SPEC>;
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
impl From<crate::W<CSMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `V` reader - Valid"]
pub type V_R = crate::BitReader<V_A>;
#[doc = "Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V_A {
    #[doc = "0: Chip-select is invalid."]
    _0 = 0,
    #[doc = "1: Chip-select is valid."]
    _1 = 1,
}
impl From<V_A> for bool {
    #[inline(always)]
    fn from(variant: V_A) -> Self {
        variant as u8 != 0
    }
}
impl V_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V_A {
        match self.bits {
            false => V_A::_0,
            true => V_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == V_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == V_A::_1
    }
}
#[doc = "Field `V` writer - Valid"]
pub type V_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSMR_SPEC, V_A, O>;
impl<'a, const O: u8> V_W<'a, O> {
    #[doc = "Chip-select is invalid."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(V_A::_0)
    }
    #[doc = "Chip-select is valid."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(V_A::_1)
    }
}
#[doc = "Field `WP` reader - Write Protect"]
pub type WP_R = crate::BitReader<WP_A>;
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP_A {
    #[doc = "0: Write accesses are allowed."]
    _0 = 0,
    #[doc = "1: Write accesses are not allowed. Attempting to write to the range of addresses for which the WP bit is set results in a bus error termination of the internal cycle and no external cycle."]
    _1 = 1,
}
impl From<WP_A> for bool {
    #[inline(always)]
    fn from(variant: WP_A) -> Self {
        variant as u8 != 0
    }
}
impl WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_A {
        match self.bits {
            false => WP_A::_0,
            true => WP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_A::_1
    }
}
#[doc = "Field `WP` writer - Write Protect"]
pub type WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSMR_SPEC, WP_A, O>;
impl<'a, const O: u8> WP_W<'a, O> {
    #[doc = "Write accesses are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP_A::_0)
    }
    #[doc = "Write accesses are not allowed. Attempting to write to the range of addresses for which the WP bit is set results in a bus error termination of the internal cycle and no external cycle."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP_A::_1)
    }
}
#[doc = "Field `BAM` reader - Base Address Mask"]
pub type BAM_R = crate::FieldReader<u16, BAM_A>;
#[doc = "Base Address Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BAM_A {
    #[doc = "0: The corresponding address bit in CSAR is used in the chip-select decode."]
    _0 = 0,
    #[doc = "1: The corresponding address bit in CSAR is a don't care in the chip-select decode."]
    _1 = 1,
}
impl From<BAM_A> for u16 {
    #[inline(always)]
    fn from(variant: BAM_A) -> Self {
        variant as _
    }
}
impl BAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BAM_A> {
        match self.bits {
            0 => Some(BAM_A::_0),
            1 => Some(BAM_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BAM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BAM_A::_1
    }
}
#[doc = "Field `BAM` writer - Base Address Mask"]
pub type BAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSMR_SPEC, u16, BAM_A, 16, O>;
impl<'a, const O: u8> BAM_W<'a, O> {
    #[doc = "The corresponding address bit in CSAR is used in the chip-select decode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BAM_A::_0)
    }
    #[doc = "The corresponding address bit in CSAR is a don't care in the chip-select decode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BAM_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Write Protect"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Base Address Mask"]
    #[inline(always)]
    pub fn bam(&self) -> BAM_R {
        BAM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    #[must_use]
    pub fn v(&mut self) -> V_W<0> {
        V_W::new(self)
    }
    #[doc = "Bit 8 - Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<8> {
        WP_W::new(self)
    }
    #[doc = "Bits 16:31 - Base Address Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bam(&mut self) -> BAM_W<16> {
        BAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Select Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csmr](index.html) module"]
pub struct CSMR_SPEC;
impl crate::RegisterSpec for CSMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csmr::R](R) reader structure"]
impl crate::Readable for CSMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csmr::W](W) writer structure"]
impl crate::Writable for CSMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSMR%s to value 0"]
impl crate::Resettable for CSMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
