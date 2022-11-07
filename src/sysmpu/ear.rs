#[doc = "Register `EAR%s` reader"]
pub struct R(crate::R<EAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EADDR` reader - Error Address"]
pub type EADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Error Address"]
    #[inline(always)]
    pub fn eaddr(&self) -> EADDR_R {
        EADDR_R::new(self.bits)
    }
}
#[doc = "Error Address Register, slave port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ear](index.html) module"]
pub struct EAR_SPEC;
impl crate::RegisterSpec for EAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ear::R](R) reader structure"]
impl crate::Readable for EAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EAR%s to value 0"]
impl crate::Resettable for EAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
