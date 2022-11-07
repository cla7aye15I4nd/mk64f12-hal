#[doc = "Register `FPROT3` reader"]
pub struct R(crate::R<FPROT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPROT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPROT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPROT3_SPEC>) -> Self {
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
#[doc = "Non-volatile P-Flash Protection 1 - Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot3](index.html) module"]
pub struct FPROT3_SPEC;
impl crate::RegisterSpec for FPROT3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fprot3::R](R) reader structure"]
impl crate::Readable for FPROT3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FPROT3 to value 0xff"]
impl crate::Resettable for FPROT3_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
