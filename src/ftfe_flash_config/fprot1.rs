#[doc = "Register `FPROT1` reader"]
pub struct R(crate::R<FPROT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPROT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPROT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPROT1_SPEC>) -> Self {
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
#[doc = "Non-volatile P-Flash Protection 0 - Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot1](index.html) module"]
pub struct FPROT1_SPEC;
impl crate::RegisterSpec for FPROT1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fprot1::R](R) reader structure"]
impl crate::Readable for FPROT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FPROT1 to value 0xff"]
impl crate::Resettable for FPROT1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
