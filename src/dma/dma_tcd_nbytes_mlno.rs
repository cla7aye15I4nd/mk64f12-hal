#[doc = "Register `TCD%s_NBYTES_MLNO` reader"]
pub struct R(crate::R<DMA_TCD_NBYTES_MLNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TCD_NBYTES_MLNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TCD_NBYTES_MLNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TCD_NBYTES_MLNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD%s_NBYTES_MLNO` writer"]
pub struct W(crate::W<DMA_TCD_NBYTES_MLNO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_TCD_NBYTES_MLNO_SPEC>;
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
impl From<crate::W<DMA_TCD_NBYTES_MLNO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_TCD_NBYTES_MLNO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBYTES` reader - Minor Byte Transfer Count"]
pub type NBYTES_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NBYTES` writer - Minor Byte Transfer Count"]
pub type NBYTES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_TCD_NBYTES_MLNO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Minor Byte Transfer Count"]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<0> {
        NBYTES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_tcd_nbytes_mlno](index.html) module"]
pub struct DMA_TCD_NBYTES_MLNO_SPEC;
impl crate::RegisterSpec for DMA_TCD_NBYTES_MLNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_tcd_nbytes_mlno::R](R) reader structure"]
impl crate::Readable for DMA_TCD_NBYTES_MLNO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_tcd_nbytes_mlno::W](W) writer structure"]
impl crate::Writable for DMA_TCD_NBYTES_MLNO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD%s_NBYTES_MLNO to value 0"]
impl crate::Resettable for DMA_TCD_NBYTES_MLNO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
