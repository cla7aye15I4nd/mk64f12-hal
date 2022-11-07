#[doc = "Register `TCD%s_NBYTES_MLOFFNO` reader"]
pub struct R(crate::R<DMA_TCD_NBYTES_MLOFFNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TCD_NBYTES_MLOFFNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TCD_NBYTES_MLOFFNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TCD_NBYTES_MLOFFNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD%s_NBYTES_MLOFFNO` writer"]
pub struct W(crate::W<DMA_TCD_NBYTES_MLOFFNO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_TCD_NBYTES_MLOFFNO_SPEC>;
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
impl From<crate::W<DMA_TCD_NBYTES_MLOFFNO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_TCD_NBYTES_MLOFFNO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBYTES` reader - Minor Byte Transfer Count"]
pub type NBYTES_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NBYTES` writer - Minor Byte Transfer Count"]
pub type NBYTES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_TCD_NBYTES_MLOFFNO_SPEC, u32, u32, 30, O>;
#[doc = "Field `DMLOE` reader - Destination Minor Loop Offset enable"]
pub type DMLOE_R = crate::BitReader<DMLOE_A>;
#[doc = "Destination Minor Loop Offset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMLOE_A {
    #[doc = "0: The minor loop offset is not applied to the DADDR"]
    _0 = 0,
    #[doc = "1: The minor loop offset is applied to the DADDR"]
    _1 = 1,
}
impl From<DMLOE_A> for bool {
    #[inline(always)]
    fn from(variant: DMLOE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMLOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMLOE_A {
        match self.bits {
            false => DMLOE_A::_0,
            true => DMLOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMLOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMLOE_A::_1
    }
}
#[doc = "Field `DMLOE` writer - Destination Minor Loop Offset enable"]
pub type DMLOE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_TCD_NBYTES_MLOFFNO_SPEC, DMLOE_A, O>;
impl<'a, const O: u8> DMLOE_W<'a, O> {
    #[doc = "The minor loop offset is not applied to the DADDR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMLOE_A::_0)
    }
    #[doc = "The minor loop offset is applied to the DADDR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMLOE_A::_1)
    }
}
#[doc = "Field `SMLOE` reader - Source Minor Loop Offset Enable"]
pub type SMLOE_R = crate::BitReader<SMLOE_A>;
#[doc = "Source Minor Loop Offset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMLOE_A {
    #[doc = "0: The minor loop offset is not applied to the SADDR"]
    _0 = 0,
    #[doc = "1: The minor loop offset is applied to the SADDR"]
    _1 = 1,
}
impl From<SMLOE_A> for bool {
    #[inline(always)]
    fn from(variant: SMLOE_A) -> Self {
        variant as u8 != 0
    }
}
impl SMLOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMLOE_A {
        match self.bits {
            false => SMLOE_A::_0,
            true => SMLOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMLOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMLOE_A::_1
    }
}
#[doc = "Field `SMLOE` writer - Source Minor Loop Offset Enable"]
pub type SMLOE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DMA_TCD_NBYTES_MLOFFNO_SPEC, SMLOE_A, O>;
impl<'a, const O: u8> SMLOE_W<'a, O> {
    #[doc = "The minor loop offset is not applied to the SADDR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMLOE_A::_0)
    }
    #[doc = "The minor loop offset is applied to the SADDR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMLOE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:29 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 30 - Destination Minor Loop Offset enable"]
    #[inline(always)]
    pub fn dmloe(&self) -> DMLOE_R {
        DMLOE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub fn smloe(&self) -> SMLOE_R {
        SMLOE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:29 - Minor Byte Transfer Count"]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<0> {
        NBYTES_W::new(self)
    }
    #[doc = "Bit 30 - Destination Minor Loop Offset enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmloe(&mut self) -> DMLOE_W<30> {
        DMLOE_W::new(self)
    }
    #[doc = "Bit 31 - Source Minor Loop Offset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smloe(&mut self) -> SMLOE_W<31> {
        SMLOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_tcd_nbytes_mloffno](index.html) module"]
pub struct DMA_TCD_NBYTES_MLOFFNO_SPEC;
impl crate::RegisterSpec for DMA_TCD_NBYTES_MLOFFNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_tcd_nbytes_mloffno::R](R) reader structure"]
impl crate::Readable for DMA_TCD_NBYTES_MLOFFNO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_tcd_nbytes_mloffno::W](W) writer structure"]
impl crate::Writable for DMA_TCD_NBYTES_MLOFFNO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD%s_NBYTES_MLOFFNO to value 0"]
impl crate::Resettable for DMA_TCD_NBYTES_MLOFFNO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
