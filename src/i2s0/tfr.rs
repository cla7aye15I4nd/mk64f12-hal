#[doc = "Register `TFR%s` reader"]
pub struct R(crate::R<TFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFP` reader - Read FIFO Pointer"]
pub type RFP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WFP` reader - Write FIFO Pointer"]
pub type WFP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Read FIFO Pointer"]
    #[inline(always)]
    pub fn rfp(&self) -> RFP_R {
        RFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Write FIFO Pointer"]
    #[inline(always)]
    pub fn wfp(&self) -> WFP_R {
        WFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "SAI Transmit FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfr](index.html) module"]
pub struct TFR_SPEC;
impl crate::RegisterSpec for TFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfr::R](R) reader structure"]
impl crate::Readable for TFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TFR%s to value 0"]
impl crate::Resettable for TFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
