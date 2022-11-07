#[doc = "Register `FPROT2` reader"]
pub struct R(crate::R<FPROT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPROT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPROT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPROT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROT` reader - P-Flash Region Protect"]
pub type PROT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - P-Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(self.bits)
    }
}
#[doc = "Non-volatile P-Flash Protection 1 - High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot2](index.html) module"]
pub struct FPROT2_SPEC;
impl crate::RegisterSpec for FPROT2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fprot2::R](R) reader structure"]
impl crate::Readable for FPROT2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FPROT2 to value 0xff"]
impl crate::Resettable for FPROT2_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
