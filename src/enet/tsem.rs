#[doc = "Register `TSEM` reader"]
pub struct R(crate::R<TSEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSEM` writer"]
pub struct W(crate::W<TSEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSEM_SPEC>;
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
impl From<crate::W<TSEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_SECTION_EMPTY` reader - Value Of The Transmit FIFO Section Empty Threshold"]
pub type TX_SECTION_EMPTY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_SECTION_EMPTY` writer - Value Of The Transmit FIFO Section Empty Threshold"]
pub type TX_SECTION_EMPTY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSEM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Value Of The Transmit FIFO Section Empty Threshold"]
    #[inline(always)]
    pub fn tx_section_empty(&self) -> TX_SECTION_EMPTY_R {
        TX_SECTION_EMPTY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of The Transmit FIFO Section Empty Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tx_section_empty(&mut self) -> TX_SECTION_EMPTY_W<0> {
        TX_SECTION_EMPTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Section Empty Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsem](index.html) module"]
pub struct TSEM_SPEC;
impl crate::RegisterSpec for TSEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsem::R](R) reader structure"]
impl crate::Readable for TSEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsem::W](W) writer structure"]
impl crate::Writable for TSEM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSEM to value 0"]
impl crate::Resettable for TSEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
