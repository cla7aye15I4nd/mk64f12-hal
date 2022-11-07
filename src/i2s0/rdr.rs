#[doc = "Register `RDR%s` reader"]
pub struct R(crate::R<RDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDR` reader - Receive Data Register"]
pub type RDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Data Register"]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new(self.bits)
    }
}
#[doc = "SAI Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdr](index.html) module"]
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdr::R](R) reader structure"]
impl crate::Readable for RDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDR%s to value 0"]
impl crate::Resettable for RDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
